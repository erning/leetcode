pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let n = arr.len() as i32 - 1;
    let mut a: i32 = 0;
    let mut b: i32 = n;
    while a <= b {
        let c = a + (b - a) / 2;
        let v = arr[c as usize];
        let prev = if c > 0 { arr[c as usize - 1] } else { i32::MIN };
        let next = if c < n { arr[c as usize + 1] } else { i32::MIN };
        if prev < v && v < next {
            a = c + 1
        } else if prev > v && v > next {
            b = c - 1
        } else {
            return c;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], expected: i32) {
        let output = peak_index_in_mountain_array(input.to_vec());
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(&[0, 1, 0], 1);
        tf(&[0, 2, 1, 0], 1);
        tf(&[0, 10, 5, 2], 1);

        tf(&[3, 5, 3, 2, 0], 1);
        tf(&[0, 2, 3, 5, 3], 3);
    }
}
