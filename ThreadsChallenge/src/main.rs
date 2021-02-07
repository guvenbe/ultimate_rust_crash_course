// Challenge: Make two child threads and give them each a receiving end to a channel.  From the
// main thread loop through several values and print each out and then send it to the channel.
// On the child threads print out the values you receive. Close the sending side in the main
// thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
// the child threads.

use crossbeam::channel;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

fn pause_ms(ms : u64){
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let (tx,rx) = channel::unbounded();
    let rx2 = rx.clone();


    let handle_a = thread::spawn(move ||{
        for msg in rx2 {
            println!("Thread A received: {}", msg);
        }
    });

    let handle_b = thread::spawn(move ||{
        for msg in rx {
            println!("Thread B received: {}", msg);
        }
    });

    for i in 1..50 {
        tx.send(i).unwrap();
        pause_ms(20);
    }

    drop(tx);
    handle_a.join().unwrap();
    handle_b.join().unwrap();
}
