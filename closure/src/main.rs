use std::fmt::Debug;

fn main() {
    let mut v = vec![2, 4, 6];
    let sum =v.iter()
        .map(|x| x*3)
        .filter(|x| *x >10)
        .fold(0, | acc, x|  acc + x);


        println!("sum: {}" ,sum);

    let viter =v.iter()
        .map(|x| x*3)
        .filter(|x| *x >10);

    for val in viter{
        println!("got: {}",val);
    }

}
