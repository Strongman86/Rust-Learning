fn main() {
    let s="testing the code".to_string();
    println!("{:?}",s.find('t'));//shown by Some(x),x is a value
    println!("{:?}",s.rfind('h'));
    println!("{:?}",s.find("the"));
}
