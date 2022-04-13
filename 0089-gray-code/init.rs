pub fn gray_code(n: i32) -> Vec<i32> {
    let mut rv = Vec::with_capacity(1 << n);
    for i in 0..1 << n {
        rv.push(i ^ (i >> 1));
    }
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        println!("{:?}", gray_code(5));
    }
}
