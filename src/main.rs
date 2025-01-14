use std::time::Instant;

use clap::Parser;
use miden_crypto::{
    hash::rpo::{Rpo256, RpoDigest},
    merkle::{MerkleError, Smt},
    Felt, Word, ONE,
};
use rand_utils::rand_value;

#[derive(Parser, Debug)]
#[clap(name = "Benchmark", about = "SMT benchmark", version, rename_all = "kebab-case")]
pub struct BenchmarkCmd {
    /// Size of the tree
    #[clap(short = 's', long = "size")]
    size: u64,
}

fn main() {
    benchmark_smt();
}

/// Run a benchmark for [`Smt`].
pub fn benchmark_smt() {
    let args = BenchmarkCmd::parse();
    let tree_size = args.size;

    // prepare the `leaves` vector for tree creation
    let mut entries = Vec::new();
    for i in 0..tree_size {
        let key = rand_value::<RpoDigest>();
        let value = [ONE, ONE, ONE, Felt::new(i)];
        entries.push((key, value));
    }

    let mut tree = construction(entries, tree_size).unwrap();
    insertion(&mut tree, tree_size).unwrap();
    proof_generation(&mut tree, tree_size).unwrap();
}

/// Runs the construction benchmark for [`Smt`], returning the constructed tree.
pub fn construction(entries: Vec<(RpoDigest, Word)>, size: u64) -> Result<Smt, MerkleError> {
    println!("Running a construction benchmark:");
    let now = Instant::now();
    let tree = Smt::with_entries(entries)?;
    let elapsed = now.elapsed();
    println!(
        "Constructed a SMT with {} key-value pairs in {:.3} seconds",
        size,
        elapsed.as_secs_f32(),
    );

    println!("Number of leaf nodes: {}\n", tree.leaves().count());

    Ok(tree)
}

/// Runs the insertion benchmark for the [`Smt`].
pub fn insertion(tree: &mut Smt, size: u64) -> Result<(), MerkleError> {
    println!("Running an insertion benchmark:");

    let mut insertion_times = Vec::new();

    for i in 0..20 {
        let test_key = Rpo256::hash(&rand_value::<u64>().to_be_bytes());
        let test_value = [ONE, ONE, ONE, Felt::new(size + i)];

        let now = Instant::now();
        tree.insert(test_key, test_value);
        let elapsed = now.elapsed();
        insertion_times.push(elapsed.as_secs_f32());
    }

    println!(
        "An average insertion time measured by 20 inserts into a SMT with {} key-value pairs is {:.3} milliseconds\n",
        size,
        // calculate the average by dividing by 20 and convert to milliseconds by multiplying by 
        // 1000. As a result, we can only multiply by 50
        insertion_times.iter().sum::<f32>() * 50f32,
    );

    Ok(())
}

/// Runs the proof generation benchmark for the [`Smt`].
pub fn proof_generation(tree: &mut Smt, size: u64) -> Result<(), MerkleError> {
    println!("Running a proof generation benchmark:");

    let mut insertion_times = Vec::new();

    for i in 0..20 {
        let test_key = Rpo256::hash(&rand_value::<u64>().to_be_bytes());
        let test_value = [ONE, ONE, ONE, Felt::new(size + i)];
        tree.insert(test_key, test_value);

        let now = Instant::now();
        let _proof = tree.open(&test_key);
        let elapsed = now.elapsed();
        insertion_times.push(elapsed.as_secs_f32());
    }

    println!(
        "An average proving time measured by 20 value proofs in a SMT with {} key-value pairs in {:.3} microseconds",
        size,
        // calculate the average by dividing by 20 and convert to microseconds by multiplying by
        // 1000000. As a result, we can only multiply by 50000
        insertion_times.iter().sum::<f32>() * 50000f32,
    );

    Ok(())
}
