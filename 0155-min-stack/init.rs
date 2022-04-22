pub struct MinStack {
    vec: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack {
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.vec.push(val);
    }

    pub fn pop(&mut self) {
        self.vec.pop();
    }

    pub fn top(&self) -> i32 {
        self.vec.last().unwrap().clone()
    }

    pub fn get_min(&self) -> i32 {
        // TODO:
        self.vec.iter().min().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // ["MinStack","push","push","push","getMin","pop","top","getMin"]
        // [[],        [-2],  [0],   [-3],  [],      [],   [],   []]
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }
}
