struct PrintDrop(&'static str);
impl Drop for PrintDrop{
	fn drop(&mut self){
		println!("Droping {}",self.0)
	} 
}
fn main(){
//	let x=PrintDrop("x");
//	let y=PrintDrop("y");
	let tup1=(PrintDrop("a"),PrintDrop("b"),PrintDrop("c"));
//	let tup2=(PrintDrop("d"),PrintDrop("e"),PrintDrop("f"));
	let tup2=(PrintDrop("x"),PrintDrop("y"),panic!());
}	
