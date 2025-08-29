use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let _i1 = 10;
    let _i2:i32 = 11;
    let _i3 = 12i32;

    let _one_million = 1_000_000;

    let _f1 = 42.0;
    let _f2:f64 = 42.0;
    let _f3 = 42f64;
    let _f4 = 42_f64;
    let _f5 = 42.0_f64;

    println!("{},{}", _f4, type_of(&_f4));
}
