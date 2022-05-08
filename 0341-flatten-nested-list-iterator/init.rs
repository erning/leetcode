#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct NestedIterator {
    values: Vec<i32>,
    index: usize,
}

impl NestedIterator {
    #[allow(non_snake_case)]
    pub fn new(nestedList: Vec<NestedInteger>) -> Self {
        fn append(list: &Vec<NestedInteger>, values: &mut Vec<i32>) {
            for v in list.into_iter() {
                match v {
                    NestedInteger::Int(v) => values.push(*v),
                    NestedInteger::List(v) => append(v, values),
                }
            }
        }
        let mut values = Vec::new();
        append(&nestedList, &mut values);
        let index = 0;
        NestedIterator { values, index }
    }

    pub fn next(&mut self) -> i32 {
        let v = self.values[self.index];
        self.index += 1;
        v
    }

    pub fn has_next(&self) -> bool {
        self.index < self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut obj = NestedIterator::new(vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ]);
        let _: i32 = obj.next();
        let _: bool = obj.has_next();
    }
}
