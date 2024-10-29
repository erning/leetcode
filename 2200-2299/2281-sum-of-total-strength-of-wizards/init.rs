pub fn total_strength(strength: Vec<i32>) -> i32 {
    const M: u64 = 1e9 as u64 + 7;
    let n = strength.len();
    let mut answer = 0u64;

    // a X X X i X X X X b
    // i is the smallest index between [a, b]. a, b inclueded
    let mut prev = vec![0; n];
    let mut next = vec![n - 1; n];
    let mut stack = Vec::new();
    for (i, &v) in strength.iter().enumerate() {
        while let Some(&j) = stack.last() {
            if v > strength[j] {
                prev[i] = j + 1;
                break;
            }
            next[j] = i - 1;
            stack.pop();
        }
        stack.push(i);
    }

    let prefix = {
        let mut p = (0, 0);
        let mut i = 0;
        let mut vec = vec![p];
        for &v in strength.iter() {
            let v = v as u64;
            i += 1;
            p = ((p.0 + v) % M, (p.1 + v * i) % M);
            vec.push(p);
        }
        vec
    };
    let suffix = {
        let mut p = (0, 0);
        let mut i = 0;
        let mut vec = vec![p];
        for &v in strength.iter().rev() {
            let v = v as u64;
            i += 1;
            p = ((p.0 + v) % M, (p.1 + v * i) % M);
            vec.push(p);
        }
        vec.reverse();
        vec
    };

    for (i, &v) in strength.iter().enumerate() {
        let a = prev[i];
        let b = next[i];

        let x = (i - a) as u64 + 1;
        let y = (b - i) as u64 + 1;

        let mut lhs = M + prefix[i].1 - prefix[a].1;
        lhs %= M;
        if a > 0 {
            lhs += M;
            lhs -= ((M + prefix[i].0 - prefix[a].0) * a as u64) % M;
            lhs %= M;
        }
        lhs *= y;
        lhs %= M;

        let mut rhs = M + suffix[i + 1].1 - suffix[b + 1].1;
        if b < n - 1 {
            rhs += M;
            rhs -= ((M + suffix[i + 1].0 - suffix[b + 1].0) * (n - 1 - b) as u64) % M;
            rhs %= M;
        }
        rhs *= x;
        rhs %= M;

        let mid = ((v as u64) * x * y) % M;

        answer += ((lhs + mid + rhs) % M) * (v as u64);
        answer %= M;
    }

    answer as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(total_strength(vec![1, 3, 1, 2]), 44);
        assert_eq!(total_strength(vec![5, 4, 6]), 213);

        assert_eq!(total_strength(vec![1, 2, 3, 100000, 100000]), 1799803);
        assert_eq!(total_strength(vec![99999, 99999]), 999199731);

        assert_eq!(
            total_strength(vec![
                99999999, 99999998, 9999997, 99999996, 99999998, 9999997, 99999996, 99999998,
                9999997, 99999996, 99999998, 9999997, 99999996
            ]),
            39004833
        );
    }
}
