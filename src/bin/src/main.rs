mod cdylib {
    #[link(name = "cdylib", kind = "dylib")]
    extern "C" {
        pub fn add(a: usize, b: usize) -> usize;
    }
}

mod staticlib {
    #[link(name = "staticlib", kind = "static")]
    extern "C" {
        pub fn add(a: usize, b: usize) -> usize;
    }
}

fn main() {
    println!("Hello, world!");
    println!("cdylib:    1 + 1 = {}", unsafe { cdylib::add(1, 1) });
    println!("dylib:     1 + 1 = {}", dylib::add(1, 1));
    println!("rlib:      1 + 1 = {}", rlib::add(1, 1));
    println!("staticlib: 1 + 1 = {}", unsafe { staticlib::add(1, 1) });
}
