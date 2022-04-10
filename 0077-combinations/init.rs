pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut stack = [0; 20];
    let mut p = 0;
    let mut rv: Vec<Vec<i32>> = Vec::new();
    loop {
        if p >= k as usize {
            rv.push(stack[..p].to_vec());
            p -= 1;
            continue;
        }
        let v = &mut stack[p];
        *v += 1;
        if *v > n {
            if p == 0 {
                break;
            }
            p -= 1;
        } else {
            p += 1;
            stack[p] = *v;
        }
    }
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(n: i32, k: i32, expected: Vec<Vec<i32>>) {
        let mut output = combine(n, k);
        output.sort();
        assert_eq!(output, expected, "input={:?}", (n, k));
    }

    #[test]
    fn example() {
        tf(
            4,
            2,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
        );
        tf(1, 1, vec![vec![1]]);
    }
}
