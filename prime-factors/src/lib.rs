pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut count = 2;
    let mut number = n;

    while number > 1 {
        if number % count == 0 {
            result.push(count);
            number = number / count;
        } else {
            count += 1
        }
    }

    result
}
