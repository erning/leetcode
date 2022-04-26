pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let n = tasks.len();
    if n == 1 {
        return -1;
    }
    if n == 2 {
        return if tasks[0] == tasks[1] { 1 } else { -1 };
    }
    let mut tasks = tasks;
    tasks.sort_unstable();
    if tasks[0] != tasks[1] {
        return -1;
    }

    let mut prev3: Option<i32> = Some(0);
    let mut prev2: Option<i32> = None;
    let mut prev1: Option<i32> = Some(1);
    let mut curr: Option<i32> = None;

    for i in 3..n + 1 {
        if let Some(mut p) = prev3 {
            if tasks[i - 3] == tasks[i - 2] && tasks[i - 2] == tasks[i - 1] {
                p += 1;
                if let Some(v) = curr {
                    if p < v {
                        curr.replace(p);
                    }
                } else {
                    curr.replace(p);
                }
            }
        }
        if let Some(mut p) = prev2 {
            if tasks[i - 2] == tasks[i - 1] {
                p += 1;
                if let Some(v) = curr {
                    if p < v {
                        curr.replace(p);
                    }
                } else {
                    curr.replace(p);
                }
            }
        }
        prev3 = prev2.take();
        prev2 = prev1.take();
        prev1 = curr.take();
    }
    prev1.unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]), 4);
        assert_eq!(minimum_rounds(vec![2, 3, 3]), -1);

        assert_eq!(minimum_rounds(vec![2, 2, 2]), 1);
        assert_eq!(minimum_rounds(vec![2, 2, 3]), -1);
        assert_eq!(minimum_rounds(vec![2, 2]), 1);
        assert_eq!(minimum_rounds(vec![1, 2]), -1);
    }

    #[test]
    fn fixed_wrong_answer() {
        assert_eq!(
            minimum_rounds(vec![
                69, 65, 62, 64, 70, 68, 69, 67, 60, 65, 69, 62, 65, 65, 61, 66, 68, 61, 65, 63, 60,
                66, 68, 66, 67, 65, 63, 65, 70, 69, 70, 62, 68, 70, 60, 68, 65, 61, 64, 65, 63, 62,
                62, 62, 67, 62, 62, 61, 66, 69
            ]),
            20
        );
    }
}
