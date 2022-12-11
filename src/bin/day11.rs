use std::{
    cell::RefCell,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

const PART: usize = 2;

const ROUNDS: usize = match PART {
    1 => 20,
    2 => 10000,
    _ => panic!(),
};
const WORRY_DIV_FACTOR: usize = match PART {
    1 => 3,
    2 => 1,
    _ => panic!(),
};

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, Default)]
struct Monkey {
    items: VecDeque<usize>,
    op: Option<Op>,
    op_arg1: Option<usize>,
    op_arg2: Option<usize>,
    test_factor: usize,
    true_dest: usize,
    false_dest: usize,
    inspected_count: usize,
}

fn main() {
    let f = File::open("input/day11.txt").unwrap();
    let read = BufReader::new(f);
    let mut lines = read.lines();
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = vec![];

    loop {
        // skip Monkey line
        let Some(Ok(_)) = lines.next() else {
            panic!("expected line")
        };
        let mut monkey = Monkey::default();
        // items
        let Some(Ok(line)) = lines.next() else {
            panic!("expected line")
        };
        let line = line.replace("  Starting items: ", "");
        let toks = line.split(", ");
        for tok in toks {
            let worry: usize = tok.parse().expect("integer worry");
            monkey.items.push_back(worry);
        }
        // operation
        let Some(Ok(line)) = lines.next() else {
            panic!("expected line")
        };
        let line = line.replace("  Operation: new = ", "");
        let mut toks = line.split(' ');
        monkey.op_arg1 = match toks.next() {
            Some("old") => None,
            Some(tok) => {
                println!("{tok}");
                let n: usize = tok.parse().expect("integer n");
                Some(n)
            }
            _ => panic!("unknown first operand"),
        };
        monkey.op = match toks.next() {
            Some("+") => Some(Op::Add),
            Some("*") => Some(Op::Mul),
            _ => panic!("unknown op"),
        };
        monkey.op_arg2 = match toks.next() {
            Some("old") => None,
            Some(tok) => {
                let n: usize = tok.parse().expect("integer n");
                Some(n)
            }
            _ => panic!("unknown first operand"),
        };
        // test
        let Some(Ok(line)) = lines.next() else {
            panic!("expected line")
        };
        let line = line.replace("  Test: divisible by ", "");
        let n: usize = line.parse().expect("integer n");
        monkey.test_factor = n;
        // send to monkeys
        let Some(Ok(line)) = lines.next() else {
            panic!("expected line")
        };
        let line = line.replace("    If true: throw to monkey ", "");
        let n: usize = line.parse().expect("integer n");
        monkey.true_dest = n;
        let Some(Ok(line)) = lines.next() else {
            panic!("expected line")
        };
        let line = line.replace("    If false: throw to monkey ", "");
        let n: usize = line.parse().expect("integer n");
        monkey.false_dest = n;

        monkeys.push(Rc::new(RefCell::new(monkey)));

        match lines.next() {
            None => break,
            Some(Ok(s)) => assert!(s.is_empty()),
            _ => panic!("expected empty line or EOF"),
        }
    }
    // lcm of test factors
    // by working with this modulus, the worry levels wrap for all monkeys
    let modulus: usize = monkeys
        .iter()
        .map(|m| m.borrow().test_factor)
        .product();

    // simulate
    for _ in 0..ROUNDS {
        for monkey in monkeys.iter() {
            let mut monkey = monkey.borrow_mut();

            while !monkey.items.is_empty() {
                let worry_level = monkey.items.pop_front().unwrap();
                // operation
                let worry_level = {
                    let arg1 = match monkey.op_arg1 {
                        None => worry_level,
                        Some(x) => x,
                    };
                    let arg2 = match monkey.op_arg2 {
                        None => worry_level,
                        Some(x) => x,
                    };
                    match monkey.op {
                        None => worry_level,
                        Some(Op::Add) => arg1 + arg2,
                        Some(Op::Mul) => arg1 * arg2,
                    }
                };
                // update
                monkey.inspected_count += 1;
                // division
                let worry_level = worry_level / WORRY_DIV_FACTOR;
                // wrap worry level
                let worry_level = worry_level % modulus;
                // throw
                if worry_level % monkey.test_factor == 0 {
                    monkeys[monkey.true_dest]
                        .borrow_mut()
                        .items
                        .push_back(worry_level);
                } else {
                    monkeys[monkey.false_dest]
                        .borrow_mut()
                        .items
                        .push_back(worry_level);
                }
            }
        }
    }

    monkeys.sort_by_key(|m| m.borrow().inspected_count);
    let monkey_business = monkeys[monkeys.len() - 1].borrow().inspected_count
        * monkeys[monkeys.len() - 2].borrow().inspected_count;

    println!("{}", monkey_business);
}
