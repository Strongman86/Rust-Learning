fn main() {
    let s:String=format!("{}Rust","hello");
    assert_eq!(s,"helloRust");
    assert_eq!(format!("{:5}","hellorust"),"hellorust");
    assert_eq!(format!("{:5.3}","hellorust"),"hel  ");
    println!("{:?}",format!("{:15}","hellorust"));
    
    println!("{:?}",format!("{:<12}","hellorust"));
    println!("{:?}",format!("{:>12}","hellorust"));
    println!("{:?}",format!("{:^12}","hellorust"));
    println!("{:?}",format!("{:=^12}","hellorust"));
    println!("{:?}",format!("{:*^12}","hellorust"));
}

