use std::io;

fn main() {
    let stdin = io::stdin();
    let mut line_count = 0u32;
    let mut bit_counts = [0u32; 32];
    let mut num_bits = 0usize;

    loop {
        let mut buf = String::new();
        match stdin.read_line(&mut buf).unwrap() {
            0 => break,
            _ => {
                let numstr = buf.trim();
                if line_count == 0 {
                    if numstr.len() > bit_counts.len() {
                        panic!("Only up to 32 bits supported");
                    }
                    num_bits = numstr.len();
                } else if numstr.len() != num_bits {
                    panic!("Inconsistent number of bits");
                }
                line_count += 1;

                let mut input = u32::from_str_radix(numstr, 2).unwrap();
                for i in 0..num_bits {
                    bit_counts[i] += input & 1;
                    input >>= 1;
                }
            }
        }
    }

    let line_count = line_count;
    let bit_counts = bit_counts;

    let mut gamma = 0u32;
    for i in 0..num_bits {
        let count = bit_counts[i];
        println!("number of 1's in bit {}: {}", i, count);
        if count > (line_count / 2) {
            gamma |= 1 << i;
        }
    }

    let epsilon = gamma ^ ((1 << num_bits) - 1);
    println!("gamma = {}, epsilon = {}", gamma, epsilon);
    println!("answer: {}", gamma * epsilon);
}
