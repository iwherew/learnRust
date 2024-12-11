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

    let sum = another_function(5,6);
    println!("{}",sum);

    let num = 3;
    if num < 5 {
        println!("The number is less than 5");
    } else if num == 5 {
        println!("The number is equal to 5");
    } else{
        println!("The number is greater than 5");
    }

    let condition = true;
    let num = if condition { 10 } else { 20 };
    let num = if num < 5 { 0 } else { 1 };
    println!("The value of num is: {}", num);

    let mut count = 0;

    loop{
        println!("loop again!");
        count = count + 1;
        if(count == 5){
            break;
        }
    }

    let mut count = 0;
    let mut num = 2;
    // loop with result
    let result = loop{
        num = num * 2;
        println!("loop result again!");
        count = count + 1;
        if(count == 5){
            break num;
        }
    };
    println!("The value of result is: {}", result);

    let mut count = 0;
    while (count < 5) {
        count = count + 1;
        println!("while");
    }

    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("The value of element is: {}", element);
    }

    // Range 含左不含右
    for number in (1..4).rev() {
        println!("倒计时{}!", number);
    }
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // x + y
    return x+y;
}