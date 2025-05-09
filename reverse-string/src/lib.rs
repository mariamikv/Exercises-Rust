use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let graphemes: Vec<&str> = input.graphemes(true).collect();

    let mut result = graphemes.clone();
    let mut first = 0;
    let mut second = result.len() - 1;

    while first < second {
        result.swap(first, second);
        first += 1;
        second -= 1;
    }

    result.join("")
}