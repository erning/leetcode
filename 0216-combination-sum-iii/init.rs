pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn sum(k: i32, n: i32, i: usize, used: &mut Vec<bool>, rv: &mut Vec<Vec<i32>>) {
        if k == 0 {
            if n == 0 {
                rv.push(
                    used.iter()
                        .enumerate()
                        .filter(|v| *v.1)
                        .map(|v| v.0 as i32)
                        .collect(),
                );
            }
            return;
        }
        if n < 0 {
            return;
        }
        for i in i..=9 {
            if used[i] {
                continue;
            }
            used[i] = true;
            sum(k - 1, n - i as i32, i + 1, used, rv);
            used[i] = false;
        }
    }

    let mut rv = Vec::new();
    let mut used = vec![false; 10];
    sum(k, n, 1, &mut used, &mut rv);
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(combination_sum3(3, 7), vec![vec![1, 2, 4]]);
        assert_eq!(
            combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
        assert_eq!(combination_sum3(4, 1), Vec::<Vec<i32>>::new());
    }
}
