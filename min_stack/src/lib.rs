struct Value {
    min: i32,
    value: i32
}

struct MinStack {
    stack: Vec<Value>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {stack: Vec::new()}
    }
    
    fn push(&mut self, val: i32) {
        if self.stack.len() > 0 {
        self.stack.push(Value {min: val.min(self.get_min()), value: val})
        } else {
        self.stack.push(Value {min: val, value: val})

        }
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1].value
    }
    
    fn get_min(&self) -> i32 {
        self.stack[self.stack.len() - 1].min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut minStack = MinStack::new();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
assert_eq!(minStack.get_min(), -3);
minStack.pop();
        assert_eq!(0, minStack.top());    // return 0
assert_eq!(minStack.get_min(), -2);
    }
}
