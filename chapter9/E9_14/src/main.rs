use std::num::ParseIntError;
fn square(number_str:&str)->Result<i32,ParseIntError>{
    number_str.parse::<i32>().map(|n| n.pow(2))
}
fn main() {
    match square("10"){
        Ok(n)=>assert_eq!(n,100),
        Err(err)=>println!("Error:{}",err),
    }
}
