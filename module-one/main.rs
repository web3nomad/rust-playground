#![allow(dead_code)]
#![allow(unused_variables)]

fn test1() {
    use std::collections::HashMap;
    let mut map_of_vec = HashMap::<String, Vec<i32>>::new();
    map_of_vec.insert("one".to_string(), vec![1, 2, 3]);
    map_of_vec.insert("two".to_string(), vec![2, 3]);
    map_of_vec.insert("three".to_string(), vec![4, 5, 6]);
    println!("{:?}", map_of_vec);

    // let vec_res = map_of_vec.get("one");
    if let Some(vec) = map_of_vec.get_mut("one") {
        vec.insert(0, 100);
        vec.push(100);
    }
    println!("{:?}", map_of_vec);

    // create linked node list
    use std::collections::LinkedList;
    let mut list = LinkedList::<i32>::new();
    list.push_back(1);
    list.push_back(2);
    loop {
        if let Some(v) = list.pop_front() {
            println!("{}", v);
        } else {
            break;
        }
    }
    println!("{:?}", list);
}

fn test2() {
    
}

fn main() {
    // test1();
    test2();
}
