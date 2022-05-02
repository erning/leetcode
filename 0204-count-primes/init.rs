pub fn count_primes(n: i32) -> i32 {
    let n = n as usize;
    if n < 2 {
        return 0;
    }
    let mut nums: Vec<bool> = vec![true; n];
    nums[0] = false;
    nums[1] = false;
    let mut i = 2;
    while i * i < n {
        if nums[i] {
            let mut j = i * i;
            while j < n {
                nums[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    // for i in 2..n {
    //     let mut j = i + i;
    //     while j < n {
    //         nums[j] = false;
    //         j += i;
    //     }
    // }
    nums.into_iter().filter(|&v| v).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(count_primes(10), 4);
        assert_eq!(count_primes(0), 0);
        assert_eq!(count_primes(1), 0);

        assert_eq!(count_primes(5 * 10i32.pow(6)), 348513);
    }
}
