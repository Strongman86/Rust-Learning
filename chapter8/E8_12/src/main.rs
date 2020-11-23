fn main() {
    let mut message=String::from("hello");
    message.extend([',','r','u'].iter());
    message.extend("st".chars());
    message.extend(" world".split_whitespace());
    println!("{:?}",message);
}
