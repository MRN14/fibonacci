use std::io;

fn main() {
    let mut n: String = String::new();

    println!("Choose a positive integer");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line");

    let n: u32 = n.trim().parse().expect("Choose a correct number");

    let result = fibonacci(n);

    println!("{n}th number of fibonacci is {result}!")
}

fn fibonacci(n: u32) -> u32 {
    let mut f0: u32 = 1;
    let mut f1: u32 = 0;
    let mut f2: u32;

    if n == 0 {
        f0 = 0;
    } else if n == 1 {
        f0 = 1;
    } else {
        for _i in 1..n {
            f2 = f1;
            f1 = f0;
            f0 = f1 + f2;
        }
    }

    f0
}
