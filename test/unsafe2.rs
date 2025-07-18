fn main() -> () {
    unsafe {
        let address = 0x4000usize;
        let r :*mut i32 = address as *mut i32;
        *r = 2;
    }
}
