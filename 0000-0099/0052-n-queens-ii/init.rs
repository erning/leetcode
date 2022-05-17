pub fn total_n_queens(n: i32) -> i32 {
    struct Env {
        n: usize,
        count: i32,
        m1: Vec<bool>,
        m2: Vec<bool>,
        m3: Vec<bool>,
        m4: Vec<bool>,
    }

    fn solve(env: &mut Env, y: usize) {
        macro_rules! set {
            ($x:expr, $y:expr, $v:expr) => {
                env.m1[$x] = $v;
                env.m2[$y] = $v;
                env.m3[$x + $y] = $v;
                env.m4[env.n + $x - $y] = $v;
            };
        }
        if y >= env.n {
            env.count += 1;
            return;
        }
        for x in 0..env.n {
            if env.m1[x] || env.m2[y] || env.m3[x + y] || env.m4[env.n + x - y] {
                continue;
            }
            set!(x, y, true);
            solve(env, y + 1);
            set!(x, y, false);
        }
    }

    let mut env = Env {
        n: n as usize,
        count: 0,
        m1: vec![false; n as usize],
        m2: vec![false; n as usize],
        m3: vec![false; (n << 1) as usize],
        m4: vec![false; (n << 1) as usize],
    };
    solve(&mut env, 0);

    env.count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rv = total_n_queens(8);
        assert_eq!(rv, 92);
    }
}
