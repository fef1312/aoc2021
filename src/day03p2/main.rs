use std::io;

// there is no real reason to make this a struct i think
struct SortedNums {
    zero: Vec<u32>,
    one: Vec<u32>,
}

impl SortedNums {
    /// Sort all numbers in `v` by whether `bit` is a one or zero
    fn sort(v: &Vec<u32>, bit: usize) -> SortedNums {
        assert!(bit < 32);
        let mut zero = Vec::new();
        let mut one = Vec::new();

        for num in v {
            if (*num & (1 << bit)) != 0 {
                one.push(*num);
            } else {
                zero.push(*num);
            }
        }

        SortedNums {
            zero,
            one,
        }
    }
}

// i have a very strong suspicion this is in the standard library
enum Pick {
    More,
    Less
}

impl Pick {
    // also this should probably use references but the borrow
    // checker pissed me off way too hard for me to care
    pub fn pick<T>(&self, more: T, less: T) -> T {
        match self {
            Pick::More => more,
            Pick::Less => less,
        }
    }
}

fn do_single_sort(nums: &Vec<u32>, bit: usize, which: Pick) -> Vec<u32> {
    let sn = SortedNums::sort(nums, bit);

    if sn.one.len() < sn.zero.len() {
        which.pick(sn.zero, sn.one)
    } else {
        which.pick(sn.one, sn.zero)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut num_bits = 0usize;
    let mut oxy_nums = Vec::new();
    let mut co2_nums = Vec::new();

    // I don't really consider this part of the algorithm itself, it's just
    // obtaining the initial data.  You could further optimize this by doing
    // the first sorting round directly in here, though.
    loop {
        let mut buf = String::new();
        match stdin.read_line(&mut buf).unwrap() {
            0 => break,
            _ => {
                let numstr = buf.trim();
                if num_bits == 0 {
                    if numstr.len() > 32 {
                        panic!("Only up to 32 bits supported");
                    }
                    num_bits = numstr.len();
                } else if numstr.len() != num_bits {
                    panic!("Inconsistent number of bits");
                }

                let input = u32::from_str_radix(numstr, 2).unwrap();
                oxy_nums.push(input);
                co2_nums.push(input);
            }
        }
    }

    // if you wanted to get really fancy you could put co2
    // and oxygen into an array along with their Pick rule
    for bit in (0..num_bits).rev() {
        if oxy_nums.len() > 1 {
            oxy_nums = do_single_sort(&oxy_nums, bit, Pick::More);
        }
        if co2_nums.len() > 1 {
            co2_nums = do_single_sort(&co2_nums, bit, Pick::Less);
        }
    }

    assert_eq!(oxy_nums.len(), 1);
    assert_eq!(co2_nums.len(), 1);
    println!("oxygen: {}", oxy_nums[0]);
    println!("co2:    {}", co2_nums[0]);
    println!("answer: {}", oxy_nums[0] * co2_nums[0]);
}
