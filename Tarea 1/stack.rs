pub const STACK_SIZE: usize = 500;

pub struct Stack {
    values: [i32; STACK_SIZE],
    cant: usize
}

impl Stack {
    pub fn new(values: [i32; STACK_SIZE]) -> Stack {
        Stack {
            values: values,
            cant: 0
        }
    }

    pub fn show_values(&self) {
        for n in 1..=self.cant {
            println!("element {}: {}", n, self.values[n-1]);
        }
    }

    pub fn push(&mut self, value: i32) {
        if self.cant < STACK_SIZE {
            self.values[self.cant] = value;
            self.cant += 1;
        } else {
            println!("Stack is full");
        }
    }

    pub fn pop(&mut self) {
        if self.cant > 0 {
            self.values[self.cant] = 0;
            self.cant -= 1;
        } else {
            println!("Stack is empty");
        }
    }

    pub fn front(&mut self) -> i32 {
        if self.cant > 0 {
            return self.values[self.cant];
        } else {
            println!("Stack is empty");
            return -1;
        }
    }

    pub fn is_empty(&mut self) -> bool {
        return self.cant > 0;
    }
}
