const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main(){
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles.....", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", ready);

    let (mut x ,y):(i32, f64) = (6, 5.0);
    let (mut q ,r) = (6, 5.0);
    println!("{}  {}", x, y);
}
