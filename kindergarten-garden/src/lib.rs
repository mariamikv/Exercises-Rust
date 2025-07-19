pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    const STUDENTS: [&str; 12] = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let student_index = STUDENTS
        .iter()
        .position(|&s| s == student)
        .expect("Student not found");

    let mut result = Vec::new();
    for line in diagram.lines() {
        let chars: Vec<char> = line.chars().collect();
        let i = student_index * 2;
        result.push(flower_name(chars[i]));
        result.push(flower_name(chars[i + 1]));
    }

    result
}

fn flower_name(c: char) -> &'static str {
    match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "unknown",
    }
}
