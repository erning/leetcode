pub fn count_vowel_strings(n: i32) -> i32 {
    let mut vowels: Vec<i32> = vec![1; 5];
    for _ in 1..n {
        for i in 1..5 {
            vowels[i] += vowels[i - 1];
        }
    }
    vowels.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(count_vowel_strings(1), 5);
        assert_eq!(count_vowel_strings(2), 15);
        assert_eq!(count_vowel_strings(33), 66045);
    }
}
