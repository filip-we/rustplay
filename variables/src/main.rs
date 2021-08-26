use std::io;

const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 10;
    println!("The value of x is: {}", x);

    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("message");


    let index: usize = index
        .trim()
        .parse()
        .expect("needs to be number");


    let element = a[index];

    println!(
        "element at {} is {}",
        index,
        element
       );

}
