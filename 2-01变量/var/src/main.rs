fn main() {
    // mut 设置为可变变量
    let mut x = 5;
    println!("value: {}",x);
    x = 6;
    println!("value: {}",x);

    // 使用下划线开头忽略未使用的变量
    let _x = 5;
    // let y = 10;

    let (a, mut b):(bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}",a,b);
    b = true;
    // 比较两值是否相等，相等继续执行，不相等中断并打印
    assert_eq!(a, b);

    // 解构赋值
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1,2,3,4,5];
    Struct {e, ..} = Struct {e: 5};
    assert_eq!([1,2,1,4,5],[a,b,c,d,e]);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}",MAX_POINTS);

    // 变量遮蔽
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner x = {}",x); // 12
    }
    println!("outer x = {}",x); // 6
}

struct Struct {
    e: i32
}


