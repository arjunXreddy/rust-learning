fn main(){
    let x : i32 = 6;
    println!("this num is {}",x);

    let n : (i32,f32,u8) = (500,3.4,9);
    let (a,b,c) = n;
    println!("this num are {},{},{}",a,b,c);
}