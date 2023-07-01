
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
}

fn main() {
    test1();
    test2();
}
