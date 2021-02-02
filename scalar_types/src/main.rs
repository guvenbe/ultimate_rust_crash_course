fn main() {
    let x = 5u16;
    let y = 3.14f32;
    let z: i32 = 100_000;
    let my_bool: bool = false;
    let my_char: char = 'x'; //4 bytes utf8

    //tuples

    let info = (1, 3.3, 999);

    let info: (u8, f64, i32) = (1, 3.3, 999);

    let jets = info.0; // = 1
    let fuel = info.1; // = 3.3
    let jets = info.2; // = 99
    let (jets, fuel, ammo) = info; //arity max 12

    let buf = [1 ,2 ,3];
    let buf2 = [0; 3];
    let buf3:[u8; 3] =[1,2,6];
    println!("{}", buf3[2]) // 32 bytes fixed size
}
