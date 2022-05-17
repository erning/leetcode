pub fn simplify_path(path: String) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            parts.pop();
            continue;
        }
        parts.push(part);
    }
    let mut s = String::new();
    s.push('/');
    s.push_str(parts.join("/").as_str());
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(simplify_path("/home/".to_string()), "/home");
        assert_eq!(simplify_path("/../".to_string()), "/");
        assert_eq!(simplify_path("/home//foo".to_string()), "/home/foo");
    }
}
