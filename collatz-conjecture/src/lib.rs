pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut number = n;
    let mut steps = 0;

    while number != 1 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            number = number
                .checked_mul(3)?
                .checked_add(1)?;
        }
        steps += 1;
    }

    Some(steps)
}
