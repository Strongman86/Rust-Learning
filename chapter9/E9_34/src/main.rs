use std::panic;
fn sum(a:i32,b:i32)->i32{
    a+b
}
fn main() {
    let result=panic::catch_unwind(||{println!("hello");});
    assert!(result.is_ok());
    panic::set_hook(Box::new(|panic_info|{
        if let Some(location)=panic_info.location(){
            println!("panic occured '{}' at {}",location.file(),location.line());
        }
        else{
            println!("Can't get location information...");
        }
    }));
    let result=panic::catch_unwind(||{println!("oh no!");});
    assert!(result.is_err());
    println!("{}",sum(1,2));
}
