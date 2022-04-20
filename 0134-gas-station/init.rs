pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    let mut m = -1;
    let mut i = 0;
    let mut j = 0;
    let mut tank = 0;
    loop {
        if m < 0 {
            tank = gas[i];
            m = i as i32;
        }
        tank -= cost[i];
        if tank < 0 {
            tank = 0;
            m = -1;
        }
        i += 1;
        if i >= n {
            if j > 1 {
                m = -1;
                break;
            }
            j += 1;
            i = 0;
        }
        if i as i32 == m {
            break;
        }
        if m >= 0 {
            tank += gas[i];
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    }
}
