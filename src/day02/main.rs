use std::io;

struct Location {
    pos: i32,
    depth: i32,
    aim: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut loc = Location {
        pos: 0,
        depth: 0,
        aim: 0,
    };

    loop {
        let mut buf = String::new();
        match stdin.read_line(&mut buf).unwrap() {
            0 => break,
            _ => handle_cmd(&mut loc, buf.trim())
        }
    }

    println!("Submarine is at pos {}, depth {}", loc.pos, loc.depth);
    println!("Answer: {}", loc.pos * loc.depth);
}

fn handle_cmd(loc: &mut Location, cmd: &str) {
    let mut iter = cmd.split_whitespace();
    match iter.next() {
        Some("up") => loc.aim -= iter.next().unwrap().parse::<i32>().unwrap(),
        Some("down") => loc.aim += iter.next().unwrap().parse::<i32>().unwrap(),
        Some("forward") => {
            let val = iter.next().unwrap().parse::<i32>().unwrap();
            loc.pos += val;
            loc.depth += val * loc.aim;
            if loc.depth < 0 {
                panic!("submarine attempted to fly")
            }
        },
        Some(s) => panic!("unknown command \"{}\"", s),
        None => panic!("empty command"),
    }
}
