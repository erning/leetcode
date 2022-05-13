// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

pub struct Solution {
    bad_version: i32,
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut a = 0;
        let mut b = n;
        while a < b {
            let c = a + (b - a) / 2;
            if self.isBadVersion(c) {
                b = c - 1;
            } else {
                a = c + 1;
            }
        }
        if self.isBadVersion(a) {
            a
        } else {
            a + 1
        }
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, n: i32) -> bool {
        n == self.bad_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(n: i32, bad: i32) {
        let s = Solution { bad_version: bad };
        let output = s.first_bad_version(n);
        assert_eq!(output, bad);
    }

    #[test]
    fn example() {
        tf(5, 4);
        tf(1, 1);
    }
}
