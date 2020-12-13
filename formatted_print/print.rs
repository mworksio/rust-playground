fn main() {
	println!("Hello {}", 28);

	println!("{0}, This is {1}; {1}, This is {0}", "Alice", "Bob");

	println!("{subject}, {grade}, {name}", subject="Japanese", grade="A", name="John");

	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

	println!("{number:>width$}", number=1, width=6);

	println!("{number:>0width$}", number=1, width=6);

	//println!("My name is {0}, {1} {0}", "Bond");
	
	#[allow(dead_code)]
    	struct Structure(i32);

    	println!("This struct `{}` won't print...", Structure(3));

}
