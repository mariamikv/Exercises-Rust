pub fn egg_count(display_value: u32) -> usize {
    if display_value == 0 { return 0 };

    let mut count = 0;
    let mut binary = String::new();
    let mut number = display_value;
    while number > 0 {
        let mut temp = number % 2;
        binary.push_str(temp.to_string().as_str());
        number = number / 2;
    }

    for i in binary.chars() {
        if i == '1' { count += 1; }
    }

    count as usize
}
