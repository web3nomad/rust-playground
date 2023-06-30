
fn main() {
    let mut a = 1;

    let mut c = || {
        a += 1;
        println!("a: {}", a);
        // println!("a");
    };

    c();
}
