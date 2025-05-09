pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();
    let mut bottles = start_bottles;

    for _ in 1..=take_down {
        result.push_str(&start_str(bottles));
        result.push_str(&start_str(bottles));
        result.push_str("And if one green bottle should accidentally fall,\n");
        result.push_str(&end_str(bottles - 1));
        result.push('\n');

        bottles -= 1;
    }

    result
}

fn start_str(bottles: u32) -> String {
    format!(
        "{} green {} hanging on the wall,\n",
        number_to_word(bottles),
        bottle_word(bottles)
    )
}

fn end_str(bottles: u32) -> String {
    format!(
        "There'll be {} green {} hanging on the wall.\n",
        number_to_word(bottles).to_lowercase(),
        bottle_word(bottles)
    )
}

fn bottle_word(n: u32) -> &'static str {
    if n == 1 {
        "bottle"
    } else {
        "bottles"
    }
}

fn number_to_word(n: u32) -> &'static str {
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "No",
    }
}
