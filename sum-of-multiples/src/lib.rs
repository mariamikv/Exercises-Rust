pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut seen = vec![false; limit as usize];
    for &factor in factors {
        if factor == 0 {
            continue;
        }
        for multiple in (factor..limit).step_by(factor as usize) {
            seen[multiple as usize] = true;
        }
    }

    seen.iter()
        .enumerate()
        .filter_map(|(i, &is_multiple)| if is_multiple { Some(i as u32) } else { None })
        .sum()
}
