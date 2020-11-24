fn main() {
    let s="hello world";
    let v=s.split(|c| (c as u32)>=(0x4E00 as u32)&&(c as u32)<=(0x9FA5 as u32)).collect::<Vec<&str>>();
    println!("{:?}",v);
    let v="asdffgg".split(|c| c=='1'||c=='x').collect::<Vec<&str>>();
    println!("{:?}",v);
    let v="abs test go".splitn(1,' ').collect::<Vec<&str>>();
    assert_eq!(v,["abs","test go"]);
    let v="abc def ghi".split_terminator(" ").collect::<Vec<&str>>();
    println!("{:?}",v);
}
