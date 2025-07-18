fn main() -> () {
    let mut x = 4;
    let y = &mut x;
    *y = 5;
    println!("x: {}", x);
}
