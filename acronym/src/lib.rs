pub fn abbreviate(phrase: &str) -> String {
    let mut abbreviation = String::new();

    for word in phrase.split(|c: char| c == ' ' || c == '-' || c == '_') {
        let cleaned: String = word.chars().filter(|c| c.is_alphabetic()).collect();
        if cleaned.is_empty() {
            continue;
        }

        if let Some(first_char) = cleaned.chars().next() {
            abbreviation.push(first_char);
        }

        if !cleaned.chars().all(|c| c.is_uppercase()) {
            for ch in cleaned.chars().skip(1) {
                if ch.is_uppercase() {
                    abbreviation.push(ch);
                }
            }
        }
    }

    abbreviation.to_uppercase()
}
