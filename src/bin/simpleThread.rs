use std::thread;

fn main() {
    let large_array: Vec<u64> = (1..=1_000_000).collect();
    let num_threads = 4;
    let chunk_size = large_array.len() / num_threads;

    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            large_array.len()
        } else {
            start + chunk_size
        };

        let chunk = large_array[start..end].to_vec();
        let handle = thread::spawn(move || -> u64 {
            let partial_sum: u64 = chunk.iter().sum();
            println!("Thread {} calculated partial sum: {}", i, partial_sum);
            partial_sum
        });

        handles.push(handle);
    }

    let total_sum: u64 = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();

    println!("Total sum: {}", total_sum);
}