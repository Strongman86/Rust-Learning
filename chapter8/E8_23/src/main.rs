fn main() {
    let s=" Hello\rust\t";
    println!("{:?}",s.trim());
    println!("{:?}",s.trim_right());
    println!("{:?}",s.trim_left());
}
