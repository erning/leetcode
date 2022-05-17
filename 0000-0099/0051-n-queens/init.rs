pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut rv: Vec<Vec<String>> = Vec::new();

    let n = n as usize;
    let mut m1: Vec<bool> = vec![false; n]; // column
    let mut m2: Vec<bool> = vec![false; n]; // row
    let mut m3: Vec<bool> = vec![false; n << 1]; // slash
    let mut m4: Vec<bool> = vec![false; n << 1]; // backslash

    let mut stack: Vec<usize> = Vec::new();
    let mut x: usize = 0;

    macro_rules! set {
        ($x:expr, $y:expr, $v:expr) => {
            m1[$x] = $v;
            m2[$y] = $v;
            m3[$x + $y] = $v;
            m4[n + $x - $y] = $v;
        };
    }
    macro_rules! is_invalid {
        ($x:expr, $y:expr) => {
            m1[$x] || m2[$y] || m3[$x + $y] || m4[n + $x - $y]
        };
    }
    macro_rules! pop {
        () => {
            if let Some(a) = stack.pop() {
                set!(a, stack.len(), false);
                Some(a)
            } else {
                None
            }
        };
    }
    macro_rules! push {
        ($x:expr) => {
            set!($x, stack.len(), true);
            stack.push($x)
        };
    }

    loop {
        let y = stack.len();
        if y >= n {
            let v: Vec<String> = stack
                .iter()
                .map(|i| {
                    let mut c = vec!['.'; n];
                    c[*i] = 'Q';
                    c.into_iter().collect()
                })
                .collect();
            rv.push(v);

            if let Some(a) = pop!() {
                x = a + 1;
                continue;
            } else {
                break;
            }
        }

        if x >= n {
            if let Some(a) = pop!() {
                x = a + 1;
                continue;
            } else {
                break;
            }
        }

        if is_invalid!(x, y) {
            x += 1;
            continue;
        }

        push!(x);
        x = 0;
    }

    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rv = solve_n_queens(8);
        assert_eq!(rv.len(), 92);
    }
}
