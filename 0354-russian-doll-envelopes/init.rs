use std::cmp::Ordering;

pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    // envelopes.sort_unstable_by_key(|k| (k[0], -k[1]));
    envelopes.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
        Ordering::Equal => b[1].cmp(&a[1]),
        ordering => ordering,
    });
    let mut dp: Vec<i32> = Vec::with_capacity(envelopes.len());
    for v in envelopes.iter() {
        let w = v[1];
        if let Err(i) = dp.binary_search(&w) {
            if i == dp.len() {
                dp.push(w);
            } else {
                dp[i] = w;
            }
        }
    }
    dp.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[[i32; 2]], expected: i32) {
        let envelopes = input.iter().map(|v| v.to_vec()).collect();
        let output = max_envelopes(envelopes);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&[[5, 4], [6, 4], [6, 7], [2, 3]], 3);
        tf(&[[1, 1], [1, 1], [1, 1]], 1);

        tf(&[[1, 3], [3, 5], [6, 7], [6, 8], [8, 4], [9, 5]], 3);
        tf(&[[30, 50], [12, 2], [3, 4], [12, 15]], 3);
        tf(
            &[
                [1, 2],
                [2, 3],
                [3, 4],
                [3, 5],
                [4, 5],
                [5, 5],
                [5, 6],
                [6, 7],
                [7, 8],
            ],
            7,
        );
        tf(
            &[
                [2, 100],
                [3, 200],
                [4, 300],
                [5, 500],
                [5, 400],
                [5, 250],
                [6, 370],
                [6, 360],
                [7, 380],
            ],
            5,
        );

        tf(&[[46, 89], [50, 53], [52, 68], [72, 45], [77, 81]], 3);
    }
}
