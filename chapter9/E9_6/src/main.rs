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
fn show(names:Vec<&str>)->&str{
    //get(names).unwrap()
    get(names).unwrap_or("Nor Found")
    //get(names).unwrap_or_else(|| "Not Found")
    //get(names).expect("Not Found")
}
fn main() {
    assert_eq!(show(vec!["Uku","Felipe"]),"Uku");
    assert_eq!(show(Vec::new()),"Not Found");
}
