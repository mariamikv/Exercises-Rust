pub fn is_armstrong_number(num: u32) -> bool {
    let mut number = num;
    let len = int_length(num);
    let mut armstrong_numbers = 0;

    let mut digits = Vec::new();

    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }

    for i in digits {
        armstrong_numbers += i.pow(len as u32);
    }

    armstrong_numbers == num
}

fn int_length(mut n: u32) -> usize {
    if n == 0 {
        return 1;
    }
    let mut len = 0;

    while n > 0 {
        n /= 10;
        len += 1;
    }
    len
}

