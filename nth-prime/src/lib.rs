pub fn nth(n: u32) -> u32 {
    nth_prime(n)
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn nth_prime(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate = 2;

    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 1;
    }
}