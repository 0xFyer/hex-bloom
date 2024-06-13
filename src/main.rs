mod linked_hash_x;

use std::time::Instant;

fn main() {
    benchmark(500_000);
    benchmark(1_000_000);
    benchmark(2_000_000);
    benchmark(3_000_000);
}

fn benchmark(num_operations: i32) {
    let mut lhx = linked_hash_x::LinkedHashX::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);

    let start = Instant::now();

    for i in 0..num_operations {
        lhx.insert(vec![i as u8; 32]);
    }
    let duration = start.elapsed();
    println!(
        "Time taken for {} insertions: {:?}",
        num_operations, duration
    );

    let start = Instant::now();
    for i in 0..num_operations {
        lhx.update(vec![i as u8; 32], vec![(i + 1) as u8; 32]);
    }
    let duration = start.elapsed();
    println!("Time taken for {} updates: {:?}", num_operations, duration);

    let start = Instant::now();
    for i in 0..num_operations {
        lhx.delete(vec![(i + 1) as u8; 32]);
    }
    let duration = start.elapsed();
    println!(
        "Time taken for {} deletions: {:?}",
        num_operations, duration
    );
}
