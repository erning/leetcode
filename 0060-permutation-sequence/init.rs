pub fn get_permutation(n: i32, k: i32) -> String {
    const FACTORIALS: [usize; 10] = [
        1,
        1,
        1 * 2,
        1 * 2 * 3,
        1 * 2 * 3 * 4,
        1 * 2 * 3 * 4 * 5,
        1 * 2 * 3 * 4 * 5 * 6,
        1 * 2 * 3 * 4 * 5 * 6 * 7,
        1 * 2 * 3 * 4 * 5 * 6 * 7 * 8,
        1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9,
    ];
    let mut used = [false; 9];
    let mut n = n as usize;
    let mut k = k as usize - 1;
    let mut rv: Vec<u8> = Vec::with_capacity(n);
    while n > 0 {
        let factorial = FACTORIALS[n - 1];
        let mut p = k / factorial;
        // for (i, _) in used
        //     .iter()
        //     .enumerate()
        //     .filter(|&(_, &v)| v == false)
        //     .skip(p)
        //     .take(1)
        // {
        //     p = i;
        // }
        for (i, &v) in used.iter().enumerate() {
            if !v {
                if p == 0 {
                    p = i;
                    break;
                }
                p -= 1;
            }
        }
        used[p] = true;
        rv.push(p as u8 + b'1');
        k = k % factorial;
        n -= 1;
    }
    String::from_utf8(rv).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(get_permutation(3, 3), "213");
        assert_eq!(get_permutation(4, 9), "2314");
        assert_eq!(get_permutation(3, 1), "123");
    }
}
