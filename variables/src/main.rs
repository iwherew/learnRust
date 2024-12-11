// const MAX_POINTS:u32 = 100_000;

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    let x = "test";
    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // 元组长度不可变，类型可不同，用小括号()
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let(x,y,z) = tup;
    println!("{},{},{}", tup.0, tup.1, tup.2);
    println!("{},{},{}", x,y,z);

    // 数组长度不可变，类型必须相同，用中括号[]
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March","April", "May", "June", "July","August", "September", "October", "November","December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 指定类型
    let a = [3; 5]; // let a = [3,3,3,3,3]
    println!("{}",months[0]);

    another_function(5,6);
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}