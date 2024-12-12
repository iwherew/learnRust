#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// 定义方法，一个构造器可以有多个impl
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    // 第一个参数不是self，视为关联函数（构造方法）
    fn square(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle{
        width: 35,
        height: 55,
    };
    let s = Rectangle::square(30);
    println!("{}",rect.area());
    println!("{:#?}",rect);
    println!("{:#?}",s);

    println!("{}",rect.can_hold(&rect2));
    println!("{}",rect.can_hold(&rect3));
}
