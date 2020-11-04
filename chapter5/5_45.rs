fn main(){
        let a=Box::new("hello");
        let b=Box::new("Rsut".to_string());
        let c=*a;
        let d=*b;
        println!("{:?}",a);
}

