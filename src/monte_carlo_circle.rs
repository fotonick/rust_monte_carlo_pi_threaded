extern crate rand;

use std::sync::mpsc::channel;
use std::thread::spawn;

static NTHREADS: i32 = 4;

// Description: Count how many of N uniform draws from a unit square fall
//              within a unit circle
// Input: number of trials to perform, drawing two floats from [0, 1)
// Output: number of trials that fell within the unit circle
fn monte_carlo_circle(n: i32) -> i32 {
    let mut num_in_circle = 0;
    for _ in 1..n {
        let x = rand::random::<f32>();
        let y = rand::random::<f32>();
        if x * x + y * y < 1. {
            num_in_circle += 1;
        }
    }
    num_in_circle
}

fn main() {
    let n = 10000000;

    // Spawn threads to split up the work. Communicate via a channel, which
    // is a thread-safe, multi-writer, single-reader FIFO.
    let (tx, rx) = channel();
    let chunk_size = n / NTHREADS;
    for _ in 0..NTHREADS {
        let tx_thread = tx.clone();
        spawn(move || tx_thread.send(monte_carlo_circle(chunk_size)));
    }

    // Grab results from the channel and sum them back up.
    let mut total_in_circle = 0;
    for _ in 0..NTHREADS {
        total_in_circle += rx.recv().unwrap();
    }

    let pi = 4. * total_in_circle as f32 / n as f32;
    println!("π is approximately {}", pi);
}
