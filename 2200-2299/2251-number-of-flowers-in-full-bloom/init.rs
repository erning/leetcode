pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, persons: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![0; persons.len()];

    let mut events: Vec<(i32, u8, usize)> = Vec::with_capacity(flowers.len() * 2 + persons.len());
    for v in flowers.into_iter() {
        events.push((v[0], 1, 0)); // start
        events.push((v[1], 3, 0)); // end
    }
    for (i, v) in persons.into_iter().enumerate() {
        events.push((v, 2, i)); // person with index
    }
    events.sort_unstable();

    let mut count = 0;
    for (_, o, i) in events.into_iter() {
        match o {
            1 => count += 1,
            2 => answer[i] = count,
            3 => count -= 1,
            _ => unimplemented!(),
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(flowers: &[[i32; 2]], persons: &[i32], expected: &[i32]) {
        let f: Vec<Vec<i32>> = flowers.iter().map(|v| v.to_vec()).collect();
        let p = persons.to_vec();
        let output = full_bloom_flowers(f, p);
        assert_eq!(
            output, expected,
            "flowers={:?}, persons={:?}",
            flowers, persons
        );
    }

    #[test]
    fn example() {
        tf(
            &[[1, 6], [3, 7], [9, 12], [4, 13]],
            &[2, 3, 7, 11],
            &[1, 2, 2, 2],
        );
        tf(&[[1, 10], [3, 3]], &[3, 3, 2], &[2, 2, 1]);
    }
}
