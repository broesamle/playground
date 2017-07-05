#[no_mangle]
pub extern fn return_it() -> u32 {
	0xEB7AAB
}

// needed for rustc to compile to wasm
fn main() {
    println!("rust-side main() was executed.");
}

#[cfg(test)]
mod tests {
	use super::return_it;

	#[test]
	fn it_works() {
		assert!( return_it() == 15432363);	// decimal value taken from python .-)
	}
}
