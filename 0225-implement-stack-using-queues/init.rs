pub struct MyStack {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    pub fn new() -> Self {
        MyStack { data: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.data.pop().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        MyStack::new();
    }
}
