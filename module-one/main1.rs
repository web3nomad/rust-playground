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
    let v = *(map1.get("rust").unwrap());
    let v = map1.get("rust").unwrap();
    println!("{}", v);
}

fn test4() {
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2 = arr1;
    let arr3 = arr1;

    println!("arr1 addr {:p}", &arr1);
    println!("arr2 addr {:p}", &arr2);
    println!("arr3 addr {:p}", &arr3);
    // 三个地址是不一样的，但是值一样，发生了值拷贝

    fn return_string(x: &String) -> String {
        // x
        let s = String::from("Rust s");
        s
    }

    let s_p = return_string(&String::from("Rust a"));
    println!("{}", s_p);

    struct Foo {
        x: i32,
        y: (i32, bool),
        z: String,
    }

    // 'a:'b,表示'a不短于'b, 只能返回x, 因为最后要返回的 Foo 的生命周期是 'a
    fn bar2<'a: 'b, 'b>(x: &'a Foo, y: &'b Foo) -> &'a Foo {
        x
        // y
    }
}

fn test_trait() {
    trait Hacker {
        fn speak(&self) -> &str {
            "I'm hacking!"
        }
    }
    struct Person {}
    struct AnotherPerson {}
    impl Hacker for Person {}
    fn say_hi_to_hacker(hacker: &dyn Hacker) {
        println!("Hi, hacker! \n{}", hacker.speak());
    }
    let p = Person {};
    say_hi_to_hacker(&p);


    trait A {
        fn take_ownership(self);
    }
    
    #[derive(Debug)]
    struct X;

    impl A for X {
        fn take_ownership(self) {
            // do nothing
        }
    }
    let x = X {};
    println!("before take_ownership: {:?}", x);
    x.take_ownership();
    // println!("after take_ownership{:?}", x);  // 会失败，已经 take_ownership 了

}

fn test5() {
    let a = [1, 2, 4, 5, 6];
    let mut b: [i32; 3] = [1, 2, 3];
    b[1] = 10;
    let c = a[1..4].iter().cloned().collect::<Vec<i32>>();
    println!("{:?}", c);
    // use std::array;
    // convert a slice to an array
    // let c2: [i32; 3] = Array::from_slice(&a[1..4]);

    let data = "Rust is great!".to_string();
    let c = data;

    enum Message {
        // TODO: define the different variants used below
        Move { x: i32, y: i32 },
        Echo(String),
        ChangeColor(i32, i32, i32),
        Quit,
    }

    let messages: [Message; 4] = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        // 
    }

    let s1 = "Hello, world!";
    let s2 = s1.to_owned();
    let s3 = "  hello there ".trim();
    let s3 = s2.trim();
    let s4: String = "foo".into();
}

fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test_trait();
    test5();
}
