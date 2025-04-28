// බාහිර C ශ්‍රිත අර්ථ දැක්වීම
#[link(name = "hello", kind = "static")]
#[link(name = "stdc++")]  // C++ සම්මත පුස්තකාලය සඳහා
extern "C" {
    fn hello_from_c();
    fn hello_from_cpp();
}

// මූලික Rust ශ්‍රිතය
#[no_mangle]
pub extern "C" fn main() {
    println!("Hello from Rust!");
    
    unsafe {
        hello_from_c();    // C ශ්‍රිතය කැඳවීම
        hello_from_cpp();  // C++ ශ්‍රිතය කැඳවීම
    }
}