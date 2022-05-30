pub fn digit_count(num: String) -> bool {
    let mut map: [usize; 10] = [0; 10];
    for v in num.bytes().map(|v| (v - b'0') as usize)  {
        map[v] +=1;
    }

    for (i, v) in num.bytes().map(|v| (v-b'0') as usize).enumerate() {
        if v != map[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(digit_count("1210".to_string()), true);
        assert_eq!(digit_count("030".to_string()), false);
    }
}
