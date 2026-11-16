use std::hint::black_box;

fn main() {
    let mut i: i64 = black_box(0);
    let mut acc: i64 = 0;
    let limit: i64 = black_box(1_000_000);

    while i < limit {
        acc = (acc + i * 3) % 1_000_007;
        i += 1;
    }
    println!("{}", acc);
}