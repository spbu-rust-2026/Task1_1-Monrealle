use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut numbers = input.split_whitespace();

    let a: i64 = numbers.next().unwrap().parse().unwrap();
    let b: i64 = numbers.next().unwrap().parse().unwrap();

    println!("{}", a + b);
}
