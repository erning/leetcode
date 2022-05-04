use std::collections::HashMap;

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut dependencies: HashMap<i32, i32> = HashMap::new();
    let mut dependants: HashMap<i32, Vec<i32>> = HashMap::new();
    for v in prerequisites.iter() {
        let (a, b) = (v[0], v[1]);
        if let Some(v) = dependants.get_mut(&b) {
            v.push(a);
        } else {
            dependants.insert(b, vec![a]);
        }
        if let Some(v) = dependencies.get_mut(&a) {
            *v += 1;
        } else {
            dependencies.insert(a, 1);
        }
    }
    for i in 0..num_courses {
        if !dependencies.contains_key(&i) {
            dependencies.insert(i, 0);
        }
    }

    let mut answer = Vec::new();
    loop {
        if dependencies.is_empty() {
            break;
        }
        let courses: Vec<i32> = dependencies
            .iter()
            .filter(|v| *v.1 == 0)
            .map(|v| *v.0)
            .collect();
        if courses.is_empty() {
            break;
        }
        for v in courses.iter() {
            answer.push(*v);
            if let Some(v) = dependants.get(v) {
                for v in v.iter() {
                    if let Some(v) = dependencies.get_mut(v) {
                        *v -= 1;
                    }
                }
            }
            dependencies.remove(v);
        }
    }

    if answer.len() < num_courses as usize {
        answer.clear()
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(num_courses: i32, prerequisites: &[[i32; 2]], excepted: &[&[i32]]) {
        let v: Vec<Vec<i32>> = prerequisites.into_iter().map(|v| v.to_vec()).collect();
        let output = find_order(num_courses, v);
        for e in excepted.into_iter() {
            let v: Vec<i32> = e.into_iter().map(|&v| v).collect();
            if output == v {
                return;
            }
        }
        assert!(
            false,
            "num_courses: {:?}, prerequisites: {:?}",
            num_courses, &prerequisites
        );
    }

    #[test]
    fn example() {
        tf(2, &[[1, 0]], &[&[0, 1]]);
        tf(
            2,
            &[[1, 0], [2, 0], [3, 1], [3, 2]],
            &[&[0, 2, 1, 3], &[0, 1, 2, 3]],
        );
        tf(1, &[], &[&[0]]);

        tf(3, &[[1, 0], [1, 2], [0, 1]], &[&[]]);
    }
}
