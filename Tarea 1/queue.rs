pub const QUEUE_SIZE: usize = 500;

pub struct Queue {
    values: [i32; QUEUE_SIZE],
    cant: usize
}

impl Queue {
    pub fn new(values: [i32; QUEUE_SIZE]) -> Queue {
        Queue {
            values: values,
            cant: 0
        }
    }

    pub fn show_values(&self) {
        for n in 1..=self.cant {
            println!("element {}: {}", n, self.values[n-1]);
        }
        println!("");
    }

    pub fn push(&mut self, value: i32) {
        if self.cant < QUEUE_SIZE {
            self.values[self.cant] = value;
            self.cant += 1;
        } else {
            println!("Stack is full");
        }
    }

    pub fn pop(&mut self) {
        if self.cant > 0 {
            for n in 0..=self.cant-1 {
                self.values[n] = self.values[n + 1];
            }
            self.values[self.cant] = 0;
            self.cant -= 1;
        } else {
            println!("Stack is empty");
        }
    }

    pub fn front(&mut self) -> i32 {
        if self.cant > 0 {
            return self.values[0];
        } else {
            println!("Stack is empty");
            return -1;
        }
    }

    pub fn is_empty(&mut self) -> bool {
        return self.cant > 0;
    }
}