fn main() {
    let mut target: u64 = 600851475143;
    let mut prime = 2;
    let mut results = vec![];

    while target != 1 {
        if (target % prime) == 0 {
            target = target / prime;
            results.push(prime);
        } else {
            prime += 1;
        }
    }
    let max = results.iter().fold(0, |m, v| std::cmp::max(m, *v));

    println!("{:?}", max);
}
