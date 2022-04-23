use std::io;

fn main() {
    println!("Enter a number:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line!");

    let num: u32 = num.trim().parse().expect("NaN");

    let result = fib(num);

    println!("{} is the {}(th) fibonacci number", result, num);
}

fn fib(x: u32) -> u32 {
    if x < 1 {
        return 0;
    }
    if x < 2 {
        return 1;
    }
    return fib(x - 1) + fib(x - 2);
}
