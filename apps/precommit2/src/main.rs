extern crate filecoin_proofs;
extern crate storage_proofs;
extern crate storage_proofs_core;
extern crate rand_xorshift;
extern crate rand;
extern crate paired;
extern crate rayon;
extern crate generic_array;
extern crate ff;
extern crate neptune;
extern crate log;

use generic_array::typenum::{self, Unsigned};
use rand::{Rng, SeedableRng};
use filecoin_proofs::*;
use rand_xorshift::XorShiftRng;
// use storage_proofs::hasher::Hasher;
use std::io::SeekFrom;
use std::io::Seek;

extern crate chrono;
use chrono::prelude::*;
use rayon::prelude::*;
use std::sync::{mpsc, Arc};
use typenum::{U11, U8};
use ff::Field;
use paired::bls12_381::Fr;
use storage_proofs_core::measurements::{
    measure_op,
    Operation::{CommD, EncodeWindowTimeAll, GenerateTreeC, GenerateTreeRLast},
};
use generic_array::{sequence::GenericSequence, GenericArray};
use neptune::batch_hasher::BatcherType;
use neptune::column_tree_builder::{ColumnTreeBuilder, ColumnTreeBuilderTrait}; 
use log::info;

fn main() {
    fil_logger::init();
    info!("try to test precommit2 ~");
    let max_gpu_tree_batch_size = 700000;
    let max_gpu_column_batch_size = 400000;
    info!("try to construct builder channel ~");
    let (builder_tx, builder_rx) = mpsc::sync_channel::<(Vec<GenericArray<Fr, U11>>, bool)>(0);
    mpsc::sync_channel::<(Vec<GenericArray<Fr, U11>>, bool)>(
        max_gpu_column_batch_size * U11::to_usize() * 32,
    );
    info!("success construct builder channel ~");
    let graph_size = 1073741824;
    let tree_count = 8;
    let layers = 11;
    // let graph_size = 16777216;
    // let layers = 2;
    // let tree_count = 1;
    let nodes_count = graph_size / tree_count;
    let config_count = tree_count;

    let mut dt = Local::now();
    let mut start = dt.timestamp_millis();

    info!("start precommit2 at {} ms", start);
    rayon::scope(|s| {
        s.spawn(move |_| {
            for i in 0..config_count {
                let mut node_index = 0;
                let builder_tx = builder_tx.clone();
                info!("<PRODUCER> urrent config index {}", i);
                while node_index != nodes_count {
                    let chunked_nodes_count = std::cmp::min(nodes_count - node_index, max_gpu_column_batch_size);
                    info!("<PRODUCER> processing config {}/{} with column nodes {} node index {}", i + 1, tree_count, chunked_nodes_count, node_index);
                    let mut columns: Vec<GenericArray<Fr, U11>> = vec![
                        GenericArray::<Fr, U11>::generate(|_i: usize| Fr::zero());
                        chunked_nodes_count
                    ];
                    info!("<PRODUCER> start to create layer data ~");
                    let mut layer_data: Vec<Vec<Fr>> =
                        vec![Vec::with_capacity(chunked_nodes_count); layers];
                    info!("<PRODUCER> start to fill column elements ~");
                    rayon::scope(|s| {
                        let layer_data: &mut Vec<_> = &mut layer_data;
                        s.spawn(move |_| {
                            for (layer_index, layer_elements) in layer_data.iter_mut().enumerate() {
                                let start = (i * nodes_count) + node_index;
                                let end = start + chunked_nodes_count;
                                layer_elements.extend(&vec![
                                    Fr::zero();
                                    chunked_nodes_count
                                ]);
                                info!("<PRODUCER> current layer {} elements length {}/[{}, {}]", layer_index, layer_elements.len(), start, end);
                            }
                        });
                    });

                    info!("<PRODUCER> start to reorganize column elements ~");
                    for layer_index in 0..layers {
                        for index in 0..chunked_nodes_count {
                            // info!("convert column {}/{}", layer_index, index);
                            columns[index][layer_index] = layer_data[layer_index][index];
                        }
                    }
                    info!("<PRODUCER> node index {}/{}/{} layer data len {}", node_index, chunked_nodes_count, nodes_count, layer_data.len());
                    drop(layer_data);
                    node_index += chunked_nodes_count;
                    let is_final = node_index == nodes_count;
                    builder_tx.send((columns, is_final)).expect("failed to send columns");
                    info!("<PRODUCER> send columns done ~");
                }
            }
        });
        s.spawn(move |_| {
			info!("<CONSUMER> create column tree builder ~");
            let mut column_tree_builder = ColumnTreeBuilder::<U11, U8>::new(
                Some(BatcherType::GPU), nodes_count, max_gpu_column_batch_size, max_gpu_tree_batch_size)
                .expect("fail to create ColumnTreeBuilder");
			info!("<CONSUMER> start to feed GPU ~");
            let mut i = 0;
            while i < config_count {
                info!("<CONSUMER> waiting for next columns ~");
                let (columns, is_final): (Vec<GenericArray<Fr, U11>>, bool) = builder_rx.recv().expect("Failed to recv columns");
                if !is_final {
                    info!("<CONSUMER> next columns received ~");
                    column_tree_builder.add_columns(&columns).expect("Failed to add columns");
                    continue;
                }
                info!("<CONSUMER> last columns received ~");
                column_tree_builder.add_final_columns(&columns).expect("failed to add final columns");
                info!("<CONSUMER> last column processed ~");
                i += 1;
            }
        });
    });

    dt = Local::now();
    let mut end = dt.timestamp_millis();
    info!("precommit2 elapse {} ms", end - start);
}
