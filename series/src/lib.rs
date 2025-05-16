//sliding window
pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut vec = Vec::new();

    if len > digits.len() || digits.is_empty() {
        return vec
    }

    for i in 0..=digits.len() - len {
        let substr = &digits[i..i+len];
        vec.push(substr.to_string());
    }

    vec
}
