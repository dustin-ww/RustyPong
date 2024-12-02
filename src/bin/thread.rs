use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use macroquad::input::KeyCode::N;

const NTHREADS: u32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let mut example = Arc::new(Mutex::new(0));

    let (sender, receiver) = channel();

    for i in 0..NTHREADS {
        let data = Arc::clone(&example);
        let sender = sender.clone();

        //let example = Arc::clone(&example);
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
            let mut ex = data.lock().unwrap();
            *ex = i;
            sender.send(()).unwrap();
            if i == 5 {
                panic!("exception in thread 5");
            }
        }));
    }

    for _ in 0..NTHREADS  {
        receiver.recv().unwrap()
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    println!("example is now {}", example.lock().unwrap())
}

/*fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}*/