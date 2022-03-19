pub fn main() {
    let (mut x, mut y) = (0_u64, 1_u64);
    let mut next: u64;
    for _ in 0..25 {
        println!("{}", x);
        next = x + y;
        x = y;
        y = next;
    }
}
