fn main() {
    let mut result = 0;
    let mut counter = 1;

    while counter < 1000 {
        if (counter % 3) == 0 || (counter % 5) == 0 {
            println!("{}", counter);
            result += counter;
        }
        counter += 1;
    }

    println!("result: {}", result);
}
