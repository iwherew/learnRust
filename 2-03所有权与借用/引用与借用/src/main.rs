fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 不可变引用
    let s1 = String::from("hello");
    let len = cal_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2: {}", s2);

    // 注意，引用的作用域从创建开始，一直持续到它最后一次使用的地方 println!(....)，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }
    // 同一作用域，特定数据只能有一个可变引用
    let mut s = String::from("hello");
    let _p1 = &mut s;
    println!("{}", _p1);
    let _p2 = &mut s;
    // 出错的原因在于，第一个可变借用 p1 必须要持续到最后一次使用的位置 println!，在 p1 创建和最后一次使用之间，我们又尝试创建第二个可变借用 p2。
    // println!("{}, {}", _p1, _p2); // cannot borrow `s` as mutable, as it is not declared as mutable
    // 在使用p1完成后再去创建p2并使用是没问题的
    println!("{}",_p2);

    // 可变引用与不可变引用不能同时存在。在不可变引用使用完毕前，不能使用可变引用
    let _p3 = &s;
    let _p4 = &mut s;
    // println!("{},{}",_p3, _p4);

}

fn cal_length(s: &String) -> usize {
    s.len()
}

fn change(a_str: &mut String){
    a_str.push_str(", world");
}
