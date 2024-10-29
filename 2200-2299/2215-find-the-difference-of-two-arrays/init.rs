pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut set = vec![0u8; 2001];
    for v in nums1.iter() {
        let i = (v + 1000) as usize;
        set[i] |= 0x01;
    }
    for v in nums2.iter() {
        let i = (v + 1000) as usize;
        set[i] |= 0x02;
    }
    let answer = |value| -> Vec<i32> {
        set.iter()
            .enumerate()
            .filter(|(_, &v)| v == value)
            .map(|(i, _)| (i as i32) - 1000)
            .collect()
    };
    vec![answer(0x01), answer(0x02)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(nums1: &[i32], nums2: &[i32], excepted: &[&[i32]]) {
        let mut output = find_difference(nums1.to_vec(), nums2.to_vec());
        output.iter_mut().for_each(|v| {
            v.sort_unstable();
        });
        assert_eq!(output, excepted, "{:?}, {:?}", nums1, nums2);
    }

    #[test]
    fn example() {
        tf(&[1, 2, 3], &[2, 4, 6], &[&[1, 3], &[4, 6]]);
        tf(&[1, 2, 3, 3], &[1, 1, 2, 2], &[&[3], &[]]);
    }
}
