fn main() {
    println!("Hello, world!");
}

fn do_stuff(qty: f64, oz: f64) ->f64{
    return qty * oz;

}

fn do_stuff2(qty: f64, oz: f64) ->f64{
    //println is a macro, supports varying multiple variables
    println!("{} {} -oz sarsaparilles!", qty, oz);

    //return last statement, no semicolon
    qty * oz

}