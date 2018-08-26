fn main() {
    let mut n = 0;
    let mut sum = 0;
    while fib(n) < 4000000 {
        let fib_cache = fib(n);
        if (fib_cache % 2) == 0 {
            sum = sum + fib_cache;
        }
        n += 1;
    }
    println!("sum: {}", sum);
}

fn fib(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}
