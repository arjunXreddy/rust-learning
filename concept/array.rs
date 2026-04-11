use std::io;

fn main(){
    let a : [i32;5] = [1,2,3,4,5];
    let mut x = String::new();
    io::stdin()
        .read_line(& mut x)
        .expect("Failed to read line");

    let x : usize = x.trim().parse().expect("Index enter was not a number");

    let element = a[x];
    println!("this is num {}",element);
    
}