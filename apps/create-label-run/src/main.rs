extern crate filecoin_proofs;
extern crate storage_proofs;
extern crate rand_xorshift;
extern crate rand;
extern crate paired;

use filecoin_proofs::*;
use storage_proofs::porep::stacked::StackedBucketGraph;
use crate::constants::{
    DefaultPieceHasher, DRG_DEGREE, EXP_DEGREE
};
use storage_proofs::drgraph::{Graph};
use storage_proofs::util::NODE_SIZE;
use crate::types::DataTree;
use storage_proofs::merkle::create_base_merkle_tree;
use storage_proofs::porep::stacked::vanilla::{create_label, create_label_exp};
use storage_proofs::hasher::Sha256Domain;

extern crate chrono;
use chrono::prelude::*;

const SIZE: usize = 1024 * 1024 * 1024 * 32;
const LAYERS: i8 = 3;
const START_LAYER: i8 = 1;
const NODES: usize = 0;
const PCOUNT: usize = 1000000;

fn main() -> std::io::Result<()> {
    fil_logger::init();

    let mut dt = Local::now();
    let mut start = dt.timestamp_millis();
    let array = vec![0; SIZE];

    println!("create DRG at {} with size {}", dt.timestamp_millis(), SIZE);
    let seed = [0u8; 32];
    let graph = StackedBucketGraph::<DefaultPieceHasher>::new_stacked(
        SIZE / NODE_SIZE, DRG_DEGREE, EXP_DEGREE, seed
    ).expect("Fail to create stackdrg");

    dt = Local::now();
    println!("Construct graph within {} ms graph size {}", dt.timestamp_millis() - start, graph.size());
    start = dt.timestamp_millis();

    // let _data_tree: DataTree =
    //     create_base_merkle_tree::<DataTree>(None, graph.size(), &array).expect("fail");

    let layer_size = graph.size() * NODE_SIZE;
    let mut labels_buffer = vec![0u8; 2 * layer_size];
    let replica_id = Sha256Domain::default();

    dt = Local::now();
    println!("Create merkle tree within {} ms", dt.timestamp_millis() - start);
    start = dt.timestamp_millis();
    let label_start = start;

    let layers = START_LAYER + LAYERS - 1;
    println!("Create Layer at {} nodes {}", dt.timestamp_millis(), graph.size());

	// How to get parent cache?
	let mut parent_cache = graph.parent_cache().expect("fail");
	let mut cache = Some(parent_cache);
	let mut nodes = graph.size();
	if 0 < NODES {
		nodes = NODES;
	}

    for layer in START_LAYER..=layers {
		dt = Local::now();
		let mut last = dt.timestamp_millis();
		if let Some(ref mut ccache) = cache {
			ccache.reset().expect("fail");
		}
        if 1 == layer {
            let layer_labels = &mut labels_buffer[0..layer_size];
            for node in 0..nodes {
                create_label(&graph, cache.as_mut(), &replica_id, layer_labels, layer as usize, node).expect("fail");
				if 0 == node % PCOUNT {
					dt = Local::now();
					println!("Current node {} in layer {} within {}", node, layer, dt.timestamp_millis() - last);
					last = dt.timestamp_millis();
				}
            }
        } else {
            let (layer_labels, exp_labels) = labels_buffer.split_at_mut(layer_size);
            for node in 0..nodes {
                create_label_exp(&graph, cache.as_mut(), &replica_id, exp_labels, layer_labels, layer as usize, node).expect("fail");
				if 0 == node % PCOUNT {
					dt = Local::now();
					println!("Current node {} in layer {} within {}", node, layer, dt.timestamp_millis() - last);
					last = dt.timestamp_millis();
				}
            }
        }
        dt = Local::now();
        println!("Create layer {} within {} ms nodes {}", layer, dt.timestamp_millis() - start, nodes);
        start = dt.timestamp_millis();
    }

    dt = Local::now();
    println!("Create layers within {} ms", dt.timestamp_millis() - label_start);
    println!("Create done at {}", dt.timestamp_millis());

	Ok(())
}
