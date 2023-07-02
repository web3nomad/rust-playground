#![allow(dead_code)]

fn test1() {
    let mut a = 1;

    let mut c = || {
        a += 1;
        println!("a: {}", a);
        // println!("a");
    };

    c();

    let _x = [1,2,3,4,5];
    let y = String::from("hello");
    {
        let z = &(*y);
        println!("y: {:?}", z);
    }
    let zz = y;
    println!("y: {:?}", zz);
}

fn test2() {
    fn f() -> Option<i32> {
        let a = Some(12);
        let b: Option<i32> = None;
        let _c = b?;  // 这里会直接 return None;
        a
    }
    let c = f();
    println!("c: {:?}", c);

    ///

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    // 上面要用 ref p 来获取 y 的引用，否则会报错，因为 Partial Move 导致下面一行 y 不能再被访问
    println!("y: {:?}", y);
}

fn test_move() {
    let mut var1 = vec![1,2,3];
    let mut f = move || {
        println!("Hello from closure! {:?}", var1);
        var1.push(5);
    };
    f();
    // 之后无法再使用 var1, 因为已经被 move 了
    // var1.push(4);
    // println!("Hello from outside of closure! {:?}", var1);
}

fn test_thread() {
    use std::thread;
    use std::any::Any;
    use std::sync::Arc;
    use std::sync::Mutex;

    // let mut counter = vec![1];
    let counter = Arc::new(Mutex::new(1));
    let counter1 = Arc::new(1);
    let mut threads = vec![];
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let mut counter1 = Arc::clone(&counter1);

        let handle = thread::spawn(move || {
            let mut counter_v = counter.lock().unwrap();
            let counter1_v = Arc::make_mut(&mut counter1);
            *counter_v += 1;
            *counter1_v += 1;
            println!("Hello from thread {}, counter = {:?}, counter1 = {:?}!", i, counter_v, counter1_v);
            // if i == 3 {
            //     panic!("oh no!");
            // }
        });
        threads.push(handle);
    }

    // iter() 返回一个不可变的引用迭代器，而 into_iter() 返回一个拥有所有权的迭代器。
    // 这意味着，如果您需要在迭代期间修改集合，则必须使用 into_iter()。如果您只需要读取集合中的元素，则可以使用 iter()。
    // 请注意，into_iter() 会消耗集合，因此在迭代之后，您将无法再次使用该集合。
    for (index, thread) in threads.into_iter().enumerate() {
        let result: Result<(), Box<dyn Any + Send>> = thread.join();
        println!("Result {} {:?}", index, result);
        result.unwrap();
    }
}


fn main() {
    // test1();
    test2();
    // test_move();
    // test_thread();
}
