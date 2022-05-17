use std::collections::BinaryHeap;

pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // event: (start, height, expire)
    // higher first for begining event, while lower first for ending events
    let mut events: Vec<(i32, i32, i32)> = buildings
        .into_iter()
        .flat_map(|v| vec![(v[0], -v[2], v[1]), (v[1], v[2], v[1])])
        .collect();
    events.sort_unstable();

    let mut rv: Vec<Vec<i32>> = Vec::new();
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut prev = 0;
    for (start, height, expire) in events.into_iter() {
        heap.push((height.abs(), expire));
        let curr = loop {
            if let Some(&(h, e)) = heap.peek() {
                if start >= e {
                    heap.pop(); // pop the expired event
                } else {
                    break h;
                }
            } else {
                break 0;
            }
        };
        if curr != prev {
            rv.push(vec![start, curr]);
            prev = curr;
        }
    }

    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            get_skyline(vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8]
            ]),
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0]
            ]
        );
        assert_eq!(
            get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]),
            vec![vec![0, 3], vec![5, 0]]
        );
    }
}
