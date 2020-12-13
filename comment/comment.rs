// This is a normal comment
/* This is another normal comment */
fn main() {
	cal();
}

/// This is a document comment
fn cal() {
	//! This is another document comment. Inner doc.
	// println!("Hello World!");
	let x = 1 + 2 * 3;
	println!("x is equal to {}", x);
}
