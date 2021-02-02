use module::greet;
use rand::{rngs, Rng};


fn main() {
    greet();
    let  x = rand::thread_rng().gen_range(0..100);
    println!("random num = {}",x);
}
