use std::fmt::Display;
use std::mem;


//  We will call this method when we want to bite something.
trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    amount_left: i32,
}

// When you bite a Grapes, subtract 1 from how many grapes are left.
impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

fn main() {
    let mut b: i32 = 0;
    println!(" b = {}  size = {}", b, mem::size_of_val(&b));

    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);


    bunny_nibbles(&mut carrot);
    bunny_nibbles(&mut grapes);
    println!("Bunny nibbles for a while on carrot: {:?}", carrot);
    println!("Bunny nibbles for a while on grapes: {:?}", grapes);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

// generic `bunny_nibbles`
// function that:
// - takes a mutable reference to any type that implements Bite
// - calls `.bite()` several times

fn bunny_nibbles<T: Bite>(arg: &mut T) {
    for i in 1..20 {
        arg.bite();
    }
}
//