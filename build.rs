fn main() {
	println!("cargo:rustc-link-lib=inochi2d-c");
	println!("cargo:rustc-link-arg=-L.");
}
