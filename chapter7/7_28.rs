struct PrintDrop(&'static str);
impl Drop for PrintDrop{
        fn drop(&mut self){
                println!("Droping {}",self.0)
        }
}
enum E{
	Foo(PrintDrop,PrintDrop)
}
struct Foo{
	x:PrintDrop,
	y:PrintDrop,
	z:PrintDrop,
}
fn main(){
//	let e=E::Foo(PrintDrop("a"),PrintDrop("b"));
//	let f=Foo{
//		x:PrintDrop("x"),y:PrintDrop("y"),z:PrintDrop("z")
//	};
	let z=PrintDrop("z");
	let y=PrintDrop("y");
	let x=PrintDrop("x");
//	let closure=move ||{y;z;x};
	let closure=move||{
		{let z_ref=&z;}
		x;y;z;	
	};

}
