use std::fmt::Debug;
trait Dosomething<T>{
	fn do_sth(&self,value:T);
}
impl<'a ,T:Debug>Dosomething<T> for &'a usize{
	fn do_sth(&self,value:T){
		println!("{:?}",value);
	}
}
fn bar(b:Box<for<'f> Dosomething<&'f usize>>){
	let s:usize=10;
	b.do_sth(&s);
}
fn main(){
	let x=Box::new(&2usize);
	bar(x);
}
