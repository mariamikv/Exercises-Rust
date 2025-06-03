pub fn build_proverb(list: &[&str]) -> String {
    let mut main_text = String::new();

    if list.is_empty() { return main_text; }

    if list.len() == 1 {
        main_text.push_str(&format!("And all for the want of a {}.", list[0]));
    } else {
        for i in 0..list.len() - 1 {
            main_text.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]));
            if i == list.len() - 1 {
                break;
            }
        }

        main_text.push_str(&format!("And all for the want of a {}.", list[0]));
    }

    main_text
}
