fn main() {
    // 字符串字面值不可修改，如 "hello"
    // String 类型 可修改，需要在heap上分配内存保存未知的文本内容
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let mut y = x;
    y = 6;
    println!("x = {}, y = {}", x, y);

    // move
    let s1 = String::from("hello");
    let s2 = s1; // move, 让s1失效
    // println!("s1 = {}, s2 = {}", s1, s2); // s1 value borrowed here after move

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 克隆，消耗资源
    println!("s1 = {}, s2 = {}", s1, s2); // s1 = hello, s2 = hello

    // copy: stack栈内存复制，旧变量可用
    // 整数 bool char 浮点数 tuple元组（内容全copy）
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 把一个值赋给其他变量时会发生移动move
    // 当一个包含heap数据的变量离开作用域时，它的值就会被drop函数清理，除非数据的所有权移动到另一个变量上了
    let s = String::from("hello");
    let s2 = takes_ownership(s);
    // println!("s = {}", s); // borrow of moved value: `s`
    println!("s2 = {}", s2);

    let x = 5;
    makes_copy(x);
    println!("x:{}",x);

    // 引用&，允许引用值而不取得所有权
    let s1= String::from("hello");
    let len = calculate_length(&s1);
    println!("s1 = {}, len = {}", s1, len);

    // 加上mut转为可变引用
    let mut s1 = String::from("hello");
    let len = change_length(&mut s1);

    // 不能超过一个可变引用，防止数据竞争
    let s2 = &mut s1;
    let s3 = &mut s1;
    // println!("s1 = {}, s2 = {}", s2, s3); //  cannot borrow `s1` as mutable more than once at a time

    // 可以通过创建新的作用域，来允许非同时创建的多个可变引用
    {
        let s2 = &mut s1;
    }
    let s3 = &mut s1;

    // 不可以同时拥有可变引用和不可变应用
    // 多个不可变应用是可以的
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let s1 = &mut s;
    // println!("r1 = {}, r2 = {}, s1 = {}", r1, r2, s1); //  cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("r1 = {}, r2 = {}", r1, r2); //  cannot borrow `s` as mutable because it is also borrowed as immutable

    let mut s = String::from("hello world");
    let word_index = first_word(&s);
    // s.clear();
    println!("first wordIndex = {}", word_index);

    // 字符串切片： 前包含，后不包含
    let mut s = String::from("hello world");
    // let hello = &s[0..5];
    // let world = &s[6..11];
    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];
    println!("hello = {}, world = {}", hello, world);
    println!("whole = {}", whole);

    // 字符串字面值就是切片
    let s = "hello";

    // 字符串字面值通过切片传入
    let my_string = String::from("hello world");
    let word_index = first_word(&my_string[..]);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}
// 参数类型定义为&str，可以同时接收String和&str类型的参数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn change_length(s: &mut String) -> usize {
    s.push_str(",world");
    s.len()
}
// 借用 borrow ，不可修改借用的值
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
