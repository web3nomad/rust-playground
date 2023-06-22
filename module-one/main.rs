#![allow(dead_code)]
#![allow(unused_variables)]

fn test1() {
    const RUST: &str = "rust";
    const WEIGHT: u64 = 100;

    println!("Hello Rust");
    println!("{}", RUST);
    println!("{}", WEIGHT);

    let name = "2001";
    println!("{}", name);

    let name_to_i32 = match name.parse::<i32>() {
        Ok(n) => n,
        Err(_) => panic!("Not a number"),
    };
    println!("{}", name_to_i32);
    let name_to_i32_alt = name.parse::<i32>().unwrap();
    println!("{}", name_to_i32_alt);
}

fn test2() {
    let t = (1, "hello", true, 9.1);
    println!("{:?}", t);
    println!("{}", t.1);

    let arr: [i32; 6] = [1, 2, 3, 4, 5, 0];
    println!("{:?}", arr);
    println!("{}", arr[1]);

    let s1 = "Hello Rust";

    let s2 = String::new();         // 空字符串
    let s2 = "Hello Rust".to_string();
    let s2 = s1.to_string();
    let s2: String = "Hello Rust".into();
    let s2: String = s1.into();
    let s2 = String::from("Hello Rust");

    let s3 = &s2[0..5];
    println!("{}", s3);
}

fn test3() {
    let mut v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    v.push(10);

    let mut vec1 = Vec::new();
    let mut vec2 = vec![];

    vec1.push("rust");
    // vec1.push(1);
    vec2.push(true);

    use std::collections::HashMap;
    let mut map1 = HashMap::new();
    map1.insert(String::from("rust"), 1);
    println!("{:?}", map1);
}

fn main() {
    // test1();
    // test2();
    test3();
}
