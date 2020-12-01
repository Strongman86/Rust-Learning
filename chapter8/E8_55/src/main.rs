use std::collections::HashMap;
fn merge_extend<'a>(
    map1:&mut HashMap<&'a str,&'a str>,
    map2:HashMap<&'a str,&'a str>
){
    map1.extend(map2);
}
fn merge_chain<'a>(
    map1:HashMap<&'a str,&'a str>,
    map2:HashMap<&'a str,&'a str>
)->HashMap<&'a str,&'a str>{
    map1.into_iter().chain(map2).collect()
}
fn merge_by_ref<'a>(
    map:&mut HashMap<&'a str,&'a str>,
    map_ref:&HashMap<&'a str,&'a str>
){
    map.extend(map_ref.into_iter().map(|(k,v)|(k.clone(),v.clone())));
}
fn main() {
    let mut book=HashMap::new();
    book.insert("book","rust");
    book.insert("pro","test");
    book.insert("one","go");
    let mut book2=HashMap::new();
    book2.insert("Action","good");
    book2.insert("Primer","nice");
    book2.insert("Matering","deep");
    merge_by_ref(&mut book, &book2);
    for key in book.keys(){
        println!("{}",key);
    }
}
