fn main() {
    let mut s = String::from("hello");
    println!("s: {}", s);
    s.push_str(", world");
    println!("s: {}", s);

    // 可Copy类型：基础类型及其组成的元组
    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);

    // 所有权发生转移，原变量已被drop，不可用
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("s2: {}", s2);
}
