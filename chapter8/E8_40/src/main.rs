fn main() {
    let mut vec=Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}",vec.len());
    println!("{:?}",vec[0]);
    println!("{:?}",vec[1]);
    vec.pop();
    println!("{:?}",vec.len());
    vec[0]=7;
    vec.extend([1,2,3,4].iter().cloned());
    println!("{:?}",vec);
    let mut vec2=vec![5,6,7];
    vec.append(&mut vec2);
    println!("{:?}",vec);
    vec.swap(1,3);
    println!("{:?}",vec);
    let slice=[1,2,3,4,5,6,7];
    vec.copy_from_slice(&slice);
    assert_eq!(vec,slice);
    let slice=[4,3,2,1];
    vec.clone_from_slice(&slice);
    assert_eq!(vec,slice);
}
