use std::collections::HashMap;

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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
        if !dependencies.contains_key(&b) {
            dependencies.insert(b, 0);
        }
    }
    let mut n = 0;
    loop {
        if dependencies.is_empty() {
            break true;
        }
        let courses: Vec<i32> = dependencies
            .iter()
            .filter(|v| *v.1 == 0)
            .map(|v| *v.0)
            .collect();
        if courses.is_empty() {
            break false;
        }
        n += courses.len();
        if n as i32 >= num_courses {
            break true;
        }
        for v in courses.iter() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(num_courses: i32, prerequisites: &[[i32; 2]], expected: bool) {
        let v: Vec<Vec<i32>> = prerequisites.into_iter().map(|v| v.to_vec()).collect();
        let output = can_finish(num_courses, v);
        assert_eq!(
            output, expected,
            "num_courses: {:?}, prerequisites: {:?}",
            num_courses, &prerequisites
        );
    }

    #[test]
    fn example() {
        tf(2, &[[1, 0]], true);
        tf(2, &[[1, 0], [0, 1]], false);
        tf(1, &[], true);
    }
}
