type MathOp=fn (i32,i32)->i32;
fn math(op:&str)->MathOp{
	
	fn sum(a:i32,b:i32)->i32{
        	a+b
	}
	fn product(a:i32,b:i32)->i32{
        	a*b
	}
	match op{
		"sum" =>sum,
		"product" =>product,
		_ =>{
			println!("Warning: No implemented");	
			sum
		}
	}
}
fn main(){
        let (a,b)=(3,4);
	let sum=math("sum");
	let product=math("product");
        println!("{:?}",sum(a,b));
        println!("{:?}",product(a,b));
}

