use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const SENSE_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const SCREEN_W: usize = 40;
const SCREEN_H: usize = 6;

fn main() {
    let f = File::open("input/day10.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let mut x: i32 = 1;
    let mut cycle: usize = 0;
    let mut acc: i32 = 0;
    let mut screen = [false; SCREEN_W * SCREEN_H];

    for line in lines {
        cycle += 1;
        sense(cycle, x, &mut acc);
        draw(cycle, x, &mut screen);
        let line = line.unwrap();
        let mut toks = line.split(' ');
        let op = toks.next().expect("operation");
        match op {
            "noop" => println!("{:0>3} noop", cycle),
            "addx" => {
                let amt: i32 = toks.next().expect("amount").parse().expect("integer");
                println!("{:0>3} addx {}", cycle, amt);
                cycle += 1;
                sense(cycle, x, &mut acc);
                draw(cycle, x, &mut screen);
                println!("{:0>3} x: {} += {}", cycle, x, amt);
                x += amt;
            }
            _ => panic!("unknown operation"),
        }
    }
    println!();
    print_screen(&screen);
    println!();

    println!("{}", acc);
}

fn sense(cycle: usize, x: i32, acc: &mut i32) {
    if SENSE_CYCLES.contains(&cycle) {
        let signal_stength: i32 = cycle as i32 * x;
        println!("{:0>3} sense x: {}, strength: {}", cycle, x, signal_stength);
        *acc += signal_stength;
    }
}

fn draw(cycle: usize, x: i32, screen: &mut [bool; SCREEN_W * SCREEN_H]) {
    let pos = cycle - 1;
    let j = pos % SCREEN_W;

    if (j as i32 - x).abs() < 2 {
        screen[pos] = true;
    }
}

fn print_screen(screen: &[bool; SCREEN_W * SCREEN_H]) {
    for (pos, &pixel) in screen.iter().enumerate() {
        let j = pos % SCREEN_W;
        if pixel {
            print!("#");
        } else {
            print!(".");
        }
        if j == SCREEN_W - 1 {
            println!();
        }
    }
}
