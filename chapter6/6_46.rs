fn main(){
	let s="hello";
	let c:Box<Fn()+'static>=Box::new(move||{s;});
}

