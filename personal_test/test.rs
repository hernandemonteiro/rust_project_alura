fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    println!("Hello, world");
    println!("1 + 2 = {}", sum(1, 2));

    let arr: &[i8] = &[1, 2, 3, 4, 5];

    arr.iter().for_each(|i| {
        println!("i = {}", i);
    });
}
