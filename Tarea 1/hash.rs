pub const HASH_SIZE: usize = 500;
// const EMPTY: String = String::new();

pub struct Hash {
    values: [i32; HASH_SIZE],
    hashes: Vec<String>
}

impl Hash {
    pub fn new(values: [i32; HASH_SIZE]) -> Hash {
        Hash {
            values: values,
            hashes: vec!["".to_string(); HASH_SIZE]
        }
    }

    pub fn show_values(&mut self) {
        for n in 1..=HASH_SIZE-1 {
            if self.hashes[n-1] != "" {
                println!("element {}: {} = {}", n, self.hashes[n-1], self.values[n-1]);
            }
        }
        println!("");
    }

    pub fn add(&mut self, hash: &str, value: i32) {
        let mut count: usize = 0;
        loop {
            if self.hashes[count] == "" {
                self.values[count] = value;
                self.hashes[count] = hash.to_string();
                break;
            }

            if self.hashes[count] == hash {
                println!("Hash already in use");
                break;
            }

            count += 1;

            if count >= HASH_SIZE {
                println!("Hash full");
                break;
            }
        }
    }

    pub fn find(&mut self, hash: &str) -> i32 {
        let mut count: usize = 0;
        let val;
        loop {
            if self.hashes[count] == hash {
                val = self.values[count];
                break;
            }

            count += 1;

            if count >= HASH_SIZE {
                println!("Hash not found");
                val = -1;
                break;
            }
        }

        return val;
    }

    pub fn erase(&mut self, hash: &str) {
        let mut count: usize = 0;
        loop {
            if self.hashes[count] == hash {
                self.values[count] = -1;
                self.hashes[count] = "".to_string();
                break;
            }

            count += 1;

            if count >= HASH_SIZE {
                println!("Hash not found");
                break;
            }
        }
    }
}