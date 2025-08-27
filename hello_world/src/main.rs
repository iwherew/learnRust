fn greet_world(){
    let chinese = "你好，世界";
    let english = "hello world";
    let regions = [chinese, english];
    for region in regions.iter(){
        println!("{}", &region);
    }
}

fn main(){
    greet_world();
}
