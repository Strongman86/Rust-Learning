fn main() {
    let mut s=String::from("hαllo");
    s.remove(3);
    println!("{:?}",s);
    s.pop();

    s.pop();

    s.pop();

    let mut s=String::from("");
    s.truncate(3);

    s.clear();
    assert_eq!("",s);
    let mut s=String::from("α is alpha,β is beta");
    let beta_offset=s.find('β').unwrap_or(s.len());
    let t:String=s.drain(..beta_offset).collect();
    println!("{:?}",t);
    println!("{:?}",s);
}
