use std::thread;

fn main() {
    let handle = thread::spawn(move ||{
        for i in 0..100{
            println!("hello")
        }
    });

    let handle2 = thread::spawn(move ||{
        for i in 0..100{
            println!("goodbye")
        }
    });

    handle.join().unwrap();
    handle2.join().unwrap();
}
