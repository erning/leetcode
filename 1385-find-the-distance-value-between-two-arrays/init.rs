pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    let arr2 = {
        let mut v = arr2;
        v.sort_unstable();
        v
    };
    let n = arr2.len();
    let mut count = 0;

    for a in arr1.iter() {
        let i = match arr2.binary_search(a) {
            Ok(i) | Err(i) => i,
        };
        let c = if i == 0 {
            i32::abs(a - arr2[0])
        } else if i == n {
            i32::abs(a - arr2[n - 1])
        } else {
            i32::min(i32::abs(a - arr2[i - 1]), i32::abs(a - arr2[i]))
        };
        if c > d {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(arr1: &[i32], arr2: &[i32], d: i32, expected: i32) {
        let output = find_the_distance_value(arr1.to_vec(), arr2.to_vec(), d);
        assert_eq!(output, expected, "{:?}", (arr1, arr2, d));
    }

    #[test]
    fn example() {
        tf(&[4, 5, 8], &[10, 9, 1, 8], 2, 2);
        tf(&[1, 4, 2, 3], &[-4, -3, 6, 10, 20, 30], 3, 2);
        tf(&[2, 1, 100, 3], &[-5, -2, 10, -3, 7], 6, 1);
    }
}
