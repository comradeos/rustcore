fn main() {
    let n: i32 = 17;

    let mut a: i64 = 0;
    let mut b: i64 = 1;

    if n == 0 {
        println!("F({}) = {}", n, a);
        return;
    }

    if n == 1 {
        println!("F({}) = {}", n, b);
        return;
    }

    let mut i: i32 = 2;

    loop {
        if i > n {
            break;
        }

        let t = a + b;
        a = b;
        b = t;

        i += 1;
    }

    println!("F({}) = {}", n, b);
}