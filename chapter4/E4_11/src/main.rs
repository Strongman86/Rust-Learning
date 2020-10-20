use std::ops::Drop;
#[derive(Debug)]
struct S(i32);
impl Drop for S{
    fn drop(&mut self){
        println!("Drop {}",self.0);
    }
}
fn main(){
    let x=S(1);
    println!("Create x:{:?}",x);
    {
        let y=S(2);
        println!("Create y:{:?}",x);
        println!("exit inner scope");
    }
    println!("exit main");
}
