// 232. Implement Queue using Stacks
// https://leetcode.com/problems/implement-queue-using-stacks/

#[allow(unused)]
#[derive(Default)]
struct MyQueue {
    stack_back: Vec<i32>,
    stack_front: Vec<i32>,
}

#[allow(unused)]
impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.stack_back.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.move_back_to_front();
        self.stack_front.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.move_back_to_front();
        *self.stack_front.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack_front.is_empty() && self.stack_back.is_empty()
    }

    fn move_back_to_front(&mut self) {
        if self.stack_front.is_empty() {
            while let Some(x) = self.stack_back.pop() {
                self.stack_front.push(x);
            }
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct MyQueueAnother {
    queue: Vec<i32>,
}

#[allow(unused)]
impl MyQueueAnother {
    fn new() -> Self {
        MyQueueAnother { queue: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        let mut temp: Vec<i32> = Vec::new();
        temp.push(x);

        for i in &self.queue {
            temp.push(*i);
        }

        self.queue = temp;
    }

    fn pop(&mut self) -> i32 {
        if let Some(x) = self.queue.pop() {
            return x;
        }

        return 0;
    }

    fn peek(&self) -> i32 {
        self.queue[self.queue.len() - 1]
    }

    fn empty(&self) -> bool {
        self.queue.len() == 0
    }
}
