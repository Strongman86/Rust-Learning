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
fn show(name:Vec<&str>)->&str{
    match get(name){
        Some(shortest)=>shortest,
        None=>"Not found",
    }
}
fn main() {
    println!("{:?}",show(vec!["1234","456"]));
    println!("{:?}",show(vec![]));
}
