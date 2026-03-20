fn main() {
    let mut x = 5;
    println!("the num is {}",x);
    x = 6;
    println!("the num is {}",x);
    //i cant use mut with const
    const Y : i32 = 7;
    println!("the num is {}",Y);
}