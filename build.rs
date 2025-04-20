fn main() {
    // Entry point and other linker arguments
    println!("cargo:rustc-link-arg=-e");
    println!("cargo:rustc-link-arg=__start");
    println!("cargo:rustc-link-arg=-nostartfiles");
}
