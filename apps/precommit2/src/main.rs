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

fn main() {
    fil_logger::init();
    println!("try to test precommit2 ~");
	let max_gpu_tree_batch_size = 700000;
    let max_gpu_column_batch_size = 400000;
    println!("try to construct builder channel ~");
    let (builder_tx, builder_rx) = mpsc::sync_channel::<(Vec<GenericArray<Fr, U11>>, bool)>(0);
    mpsc::sync_channel::<(Vec<GenericArray<Fr, U11>>, bool)>(
        max_gpu_column_batch_size * U11::to_usize() * 32,
    );
    println!("success construct builder channel ~");
    let config_count = 1;
    let nodes_count = 16777216; //1073741824
    let tree_count = 1;
    let layers = 11;

    rayon::scope(|s| {
        s.spawn(move |_| {
            for i in 0..config_count {
                let mut node_index = 0;
                let builder_tx = builder_tx.clone();
                println!("current config index {}", i);
                while node_index != nodes_count {
                    let chunked_nodes_count = std::cmp::min(nodes_count - node_index, max_gpu_column_batch_size);
                    println!("processing config {}/{} with column nodes {} node index {}", i + 1, tree_count, chunked_nodes_count, node_index);
                    let mut columns: Vec<GenericArray<Fr, U11>> = vec![
                        GenericArray::<Fr, U11>::generate(|_i: usize| Fr::zero());
                    	chunked_nodes_count
                    ];
                    let mut layer_data: Vec<Vec<Fr>> =
                        vec![Vec::with_capacity(chunked_nodes_count); layers];
                    rayon::scope(|s| {
                        let layer_data: &mut Vec<_> = &mut layer_data;
                        s.spawn(move |_| {
                            for (layer_index, layer_elements) in layer_data.iter_mut().enumerate() {
                                let start = (i * nodes_count) + node_index;
                                let end = start + chunked_nodes_count;
                                println!("current layer {} elements length {}/[{}, {}]", layer_index, layer_elements.len(), start, end);
								layer_elements.extend(&vec![
									Fr::zero();
									chunked_nodes_count
								]);
                            }
                        });
                    });

                    for layer_index in 0..layers {
                        for index in 0..chunked_nodes_count {
                            // println!("convert column {}/{}", layer_index, index);
                            columns[index][layer_index] = layer_data[layer_index][index];
                        }
                    }
                    println!("node index {}/{}/{} layer data len {}", node_index, chunked_nodes_count, nodes_count, layer_data.len());
                    drop(layer_data);
                    node_index += chunked_nodes_count;
                    let is_final = node_index == nodes_count;
                    builder_tx.send((columns, is_final)).expect("failed to send columns");
                }
            }
        });
		s.spawn(move |_| {
			let mut column_tree_builder = ColumnTreeBuilder::<U11, U8>::new(
				Some(BatcherType::GPU), nodes_count, max_gpu_column_batch_size, max_gpu_tree_batch_size)
				.expect("fail to create ColumnTreeBuilder");
			let mut i = 0;
			while i < 1 {
				let (columns, is_final): (Vec<GenericArray<Fr, U11>>, bool) = builder_rx.recv().expect("Failed to recv columns");
				if !is_final {
					column_tree_builder.add_columns(&columns).expect("Failed to add columns");
					continue;
				}
				i += 1;
			}
		});
    });
}
