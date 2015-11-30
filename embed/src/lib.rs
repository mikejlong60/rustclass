extern crate time;
use std::thread;

#[no_mangle]
pub extern fn process() {
	println!("");
	let startTs = time::now;
	//let s = startTs.to_string();
	//println!("started at{} ", s);

	let handles: Vec<_> = (0..100).map(|_| {
		thread::spawn(|| {
			let mut x = 0;
			for _ in (0..5_000_000) {
				x += 1
			}
			x
		})
	}).collect();

	for h in handles {
		println!("Thread finished with count={}",
		h.join().map_err(|_| "Could not join a thread!").unwrap());
	}
	println!("done!");
}