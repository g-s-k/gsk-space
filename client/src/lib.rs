#[no_mangle]
pub fn add_one(number: i32) -> i32 {
    eprintln!("add_one({:?}) was called", number);
    number + 1
}
