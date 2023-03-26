use std::io;

fn main() {
    println!("Hello, world!");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("error input, we need string!!");

    let x : i32 = number.trim().parse().expect("Input not number!");
    if x > 3 {
        println!("Hi, this is bigger then 3.");
    } else {
        println!("Hi, this is less then 3.");
    }

    let mut count = 0;
    let result = loop {
        count += 1;

        if count > x {
            break count * 2;
        }
    };

    println!("the result is {result}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
}
