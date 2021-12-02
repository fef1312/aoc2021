use std::io;

/// full-ascending circular stack
struct ShiftReg {
    vals: [i32; 4],
    pos: usize,
}

impl ShiftReg {
    pub const fn new() -> ShiftReg {
        ShiftReg {
            vals: [i32::MIN, i32::MIN, i32::MIN, i32::MIN],
            pos: 3,
        }
    }

    pub fn push(&mut self, val: i32) {
        self.pos += 1;
        self.pos %= 4;
        self.vals[self.pos] = val;
    }

    pub fn diff(&self) -> i32 {
        // i0  A
        // i1  A B
        // i2  A B
        // i3    B
        //
        // A = (i0 + (i1 + i2)     )
        // B = (     (i1 + i2) + i3)
        // B - A = i3 - i0
        let first = self.vals[self.pos]; // i3
        let last = self.vals[(self.pos + 1) % 4]; // i0
        first.wrapping_sub(last)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sr = ShiftReg::new();
    let mut count = 0;
    loop {
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer).unwrap() {
            0 => break,
            _ => {
                let val = buffer.trim().parse::<i32>().unwrap();
                sr.push(val);
                if sr.diff() > 0 {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
