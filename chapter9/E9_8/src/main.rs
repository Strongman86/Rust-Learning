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
    get(name).map(|name| name.len())
}
fn main() {
    println!("{:?}",get_length(vec!["abc","defg"]));
    println!("{:?}",get_length(Vec::new()));
}
