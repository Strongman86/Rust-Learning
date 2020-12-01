use std::collections::HashMap;
fn main() {
    let mut rev=HashMap::with_capacity(10);
    rev.insert("rust","test");
    rev.insert("program","rust");
    for key in rev.keys(){
        println!("{:?}",key);
    }
    for value in rev.values(){
        println!("{:?}",value);
    }
    if !rev.contains_key("book"){
        println!("find {} times",rev.len());
    }
    rev.remove("rust");
    let find=["rust","program"];
    for book in &find{
        match rev.get(book){
            Some(review)=>println!("{:?}",book),
            None=>println!("{:?}",book),
        }
    }
    for (book,review) in &rev{
        println!("{:?}",book);
    }
    
}
