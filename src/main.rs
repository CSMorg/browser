// src/main.rs

extern "C" {
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
}
