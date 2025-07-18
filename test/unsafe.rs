fn main() -> () {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        *r2 += 4;
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }
}
