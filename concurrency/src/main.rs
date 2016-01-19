use std::thread;

fn main() {
    for x in 1 .. 100000 {
    thread::spawn(||
        {
            println!("Hello, Julia, Chloe, Waverly, Maddie, and Denise!");
            example2();
        }
    );
  }
}


fn example2() {
    let handle = thread::spawn(|| {
        safe_shared_data();
        "From thread spawned by the greeting thread: You are even prettier than think you are!."

    });
    println!("{}", handle.join().unwrap());
}

use std::sync::{Arc, Mutex};

fn safe_shared_data() {
//    let mut data = vec![1,2,3];  compile error: capture of moved value: `data`
//    let mut data = Arc::new(vec![1,2,3]);
//    let mut data = Arc::new(vec![1,2,3]);
    let mut data = Arc::new(Mutex::new(vec![1,2,3]));

    for i in 0 .. 3 {
        let data = data.clone();
        thread::spawn(move || {
//        data[i] += 1; compile error: cannot borrow immutable borrowed content as mutable
        let mut data = data.lock().unwrap();
        data[i] += 1;
        });
    }
    //thread::sleep_ms(50);
}