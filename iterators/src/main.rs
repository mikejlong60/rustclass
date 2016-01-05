fn main() {
    for x in 0 .. 10 {
        println!("Hello, {}", x);
    }

    let nums = vec![1,2,3,4,5,6];
    for x in &nums {
        println!("Hello Vector, {}", *x);
    }
}
