fn get(name:Vec<&str>)->Option<&str>{
    if name.len()>0{
        let mut shortest=name[0];
        for i in name.iter(){
            if i.len()<shortest.len(){
                shortest=*i;
            }
        }
        Some(shortest)
    }else{
        None
    }
}
fn get_length(name:Vec<&str>)->Option<usize>{
    match get(name){
        Some(shortest)=>Some(shortest.len()),
        None=>None,
    } 
}
fn main() {
    println!("{:?}",get_length(vec!["1234","456"]));
    println!("{:?}",get_length(Vec::new()));
}
