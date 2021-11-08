use std::collections::{HashSet, VecDeque};
use std::io;
use std::iter::FromIterator;

fn ackermann_3n(r7: u16, n: u32) -> u16 {
    // (x+1)^(n+2) + (x+1)^(n+1) + (x+1)^n + ... + (x+1)^2 + x
    let mut result = 0;
    let mut exp = n + 2;
    loop {
        result = (result as u32 + (r7 + 1).wrapping_pow(exp) as u32) as u16 & LITERAL;
        exp -= 1;
        if exp == 1 { break; }
    }
    result = (result as u32 + r7 as u32) as u16 & LITERAL;
    // println!("r7: {}, n: {}, af3n: {}", r7, n, result);
    result
}

fn find_r7() -> u16 {
    let mut r7 = 0;
    loop {
        // println!("Checking {}", r7);
        let af3x = ackermann_3n(r7, r7 as u32);
        if ackermann_3n(r7, af3x as u32) == 6 {
            break;
        }
        if r7 == LITERAL { break; }
        r7 += 1;
    };
    r7
}

struct SearchUnit {
    path: Vec<usize>,
    value: i32,
    op: i32,
}

impl SearchUnit {
    fn new() -> SearchUnit {
        SearchUnit { path: vec![0], value: 22, op: -3 }
    }
}

fn find_route(symbols: Vec<i32>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(SearchUnit::new() );
    let mut passed: HashSet<(usize, i32)> = HashSet::new();
    passed.insert((0, 22));
    loop {
        let current_unit = queue.pop_front().unwrap();
        let loc = *(current_unit.path.last().unwrap()) as isize;
        let (x, y) = (loc % 4, loc / 4);
        let mut neighbors: HashSet<isize> = HashSet::from_iter(
            vec![loc-1, loc+1, loc-4, loc+4].into_iter());
        if x == 0 { neighbors.remove(&(loc-1)); }
        else if x == 3 { neighbors.remove(&(loc+1)); }
        if y == 0 { neighbors.remove(&(loc-4)); }
        else if y == 3 { neighbors.remove(&(loc+4)); }
        let next: Vec<_> = neighbors.into_iter().map(|n| {
            let op = if symbols[n as usize] > -3 && symbols[n as usize] <= 0 { symbols[n as usize] } else { -3 };
            let value =
                if n == 0 { 22 }
                else if symbols[n as usize] <= 0 { current_unit.value }
                else {
                    match current_unit.op {
                        0 => current_unit.value * symbols[n as usize],
                        -1 => current_unit.value + symbols[n as usize],
                        _ => current_unit.value - symbols[n as usize]
                    }
                };
            let mut path = current_unit.path.clone();
            path.push(n as usize);
            SearchUnit { path, value, op }
        }).filter(|su| {
            let loc = *(su.path.last().unwrap());
            !passed.contains(&(loc, su.value)) && (loc != 15 || (loc == 15 && su.value == 30))
        }).collect();

        if let Some(su) = next.iter()
            .find(|&su| *su.path.last().unwrap() == 15 && su.value == 30) {
            break su.path.clone();
        }

        next.into_iter().for_each(|su| {
            passed.insert((*su.path.last().unwrap(), su.value));
            queue.push_back(su);
        });
    }
}

#[derive(Clone, Debug)]
pub struct SynacorVm {
    memory: Vec<u16>,
    registers: Vec<u16>,
    stack: Vec<u16>,
    ip: usize
}

const LITERAL: u16 = 32767;
const INVALID: u16 = 32776;

fn show_reg(x: u16) -> String {
    assert!(x > LITERAL && x < INVALID);
    format!("r{}", x - LITERAL - 1)
}

fn show_val(x: u16) -> String {
    assert!(x < INVALID);
    if x <= LITERAL { x.to_string() }
    else { show_reg(x) }
}

impl SynacorVm {
    pub fn new(program: Vec<u16>) -> SynacorVm {
        SynacorVm {
            memory: program,
            registers: vec![0; 8],
            stack: Vec::new(),
            ip: 0
        }
    }

    fn val(&self, x: u16) -> u16 {
        assert!(x < INVALID);
        if x <= LITERAL { x }
        else { self.registers[(x - LITERAL) as usize - 1] }
    }

    fn set_reg(&mut self, x: u16, val: u16) {
        assert!(x > LITERAL && x < INVALID);
        self.registers[(x - LITERAL) as usize - 1] = val;
    }

    pub fn run(&mut self, prepared: &str, second_prepared: &str) -> u32 {
        let mut in_buffer: VecDeque<char> = VecDeque::from(prepared.chars().collect::<Vec<_>>());
        let mut out_buffer = String::new();
        let mut extracting = false;
        let mut count = 0;
        let r7 = 25734; // find_r7();
        let mut stop = 0;

        let result = loop {
            let mem_len = self.memory.len();
            if self.ip >= mem_len {
                break 1;
            }

            // skip confirmation process
            if self.ip == 5489 {
                self.registers[0] = 6;
                self.ip = 5491;
            }
            // after confirmation process
            // if self.ip == 5491 {
            //     extracting = true;
            // }
            let a = if self.ip + 1 < mem_len { self.memory[self.ip + 1] } else { 0 };
            let b = if self.ip + 2 < mem_len { self.memory[self.ip + 2] } else { 0 };
            let c = if self.ip + 3 < mem_len { self.memory[self.ip + 3] } else { 0 };
            match self.memory[self.ip] {
                0 => break 0, // halt
                1 => { // set a b
                    self.set_reg(a, self.val(b));
                    if extracting {
                        println!("{}: set {} {}", self.ip, show_reg(a), show_val(b));
                    }
                    self.ip += 3;
                }
                2 => { // push a
                    self.stack.push(self.val(a));
                    if extracting {
                        println!("{}: push {}", self.ip, show_val(a));
                    }
                    self.ip += 2;
                }
                3 => { // pop a
                    assert!(!self.stack.is_empty());
                    let val = self.stack.pop().unwrap();
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: pop {}", self.ip, show_reg(a));
                    }
                    self.ip += 2;
                }
                4 => { // eq a b c
                    let val = if self.val(b) == self.val(c) { 1 } else { 0 };
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: eq {} {} {}", self.ip, show_reg(a), show_val(b), show_val(c));
                    }
                    self.ip += 4;
                }
                5 => { // gt a b c
                    let val = if self.val(b) > self.val(c) { 1 } else { 0 };
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: gt {} {} {}", self.ip, show_reg(a), show_val(b), show_val(c));
                    }
                    self.ip += 4;
                }
                6 => { // jmp a
                    if extracting {
                        println!("{}: jmp {}", self.ip, show_val(a));
                    }
                    self.ip = self.val(a) as usize;
                }
                7 => { // jt a b
                    if extracting {
                        println!("{}: jt {} {}", self.ip, show_val(a), show_val(b));
                    }
                    if self.val(a) != 0 { self.ip = self.val(b) as usize; }
                    else { self.ip += 3; }
                }
                8 => { // jf a b
                    if extracting {
                        println!("{}: jf {} {}", self.ip, show_val(a), show_val(b));
                    }
                    if self.val(a) == 0 { self.ip = self.val(b) as usize; }
                    else { self.ip += 3; }
                }
                9 => { // add a b c
                    let val = (self.val(b) + self.val(c)) & LITERAL;
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: add {} {} {}", self.ip, show_reg(a), show_val(b), show_val(c));
                    }
                    self.ip += 4;
                }
                10 => { // mult a b c
                    let val = ((self.val(b) as u32 * self.val(c) as u32) & LITERAL as u32) as u16;
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: mult {} {} {}", self.ip, show_reg(a), show_val(b), show_val(c));
                    }
                    self.ip += 4;
                }
                11 => { // mod a b c
                    let val = self.val(b) % self.val(c);
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: mod {} {} {}", self.ip, show_reg(a), show_val(b), show_val(c));
                    }
                    self.ip += 4;
                }
                12 => { // and a b c
                    let val = self.val(b) & self.val(c);
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: and {} {} {}", self.ip, show_reg(a), show_val(b), show_val(c));
                    }
                    self.ip += 4;
                }
                13 => { // or a b c
                    let val = self.val(b) | self.val(c);
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: or {} {} {}", self.ip, show_reg(a), show_val(b), show_val(c));
                    }
                    self.ip += 4;
                }
                14 => { // not a b
                    let val = !self.val(b) & LITERAL;
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: not {} {}", self.ip, show_reg(a), show_val(b));
                    }
                    self.ip += 3;
                }
                15 => { // rmem a b
                    let val = self.memory[self.val(b) as usize];
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: rmem {} {}", self.ip, show_reg(a), show_val(b));
                    }
                    self.ip += 3;
                }
                16 => { // wmem a b
                    let loc = self.val(a);
                    self.memory[loc as usize] = self.val(b);
                    if extracting {
                        println!("{}: wmem {} {}", self.ip, show_val(a), show_val(b));
                    }
                    self.ip += 3;
                }
                17 => { // call a
                    self.stack.push(self.ip as u16 + 2);
                    if extracting {
                        println!("{}: call {}", self.ip, show_val(a));
                    }
                    self.ip = self.val(a) as usize;
                }
                18 => { // ret
                    if extracting {
                        println!("{}: ret", self.ip);
                    }
                    if self.stack.is_empty() { break 0; }
                    else { self.ip = self.stack.pop().unwrap() as usize; }
                }
                19 => { // out a
                    out_buffer.push((self.val(a) as u8) as char);
                    // confirmation process
                    // if out_buffer.ends_with("1 billion years.\"") {
                    //     extracting = true;
                    // }
                    // print!("{}", (self.val(a) as u8) as char);
                    if extracting {
                        println!("{}: out {}", self.ip, (self.val(a) as u8) as char);
                    }
                    self.ip += 2;
                }
                20 => { // in a
                    print!("{}", out_buffer);

                    if in_buffer.is_empty() {
                        if stop == 0 {
                            self.registers[7] = r7;
                            println!("r7: {}", r7);
                            in_buffer = "use teleporter\n".chars().collect();
                            stop += 1;
                        } else if stop == 1 {
                            let symbols = vec![22, -2, 9, 0, -1, 4, -2, 18, 4, 0, 11, 0, 0, 8, -2, 1];
                            println!("route: {:?}", find_route(symbols));
                            in_buffer = second_prepared.chars().collect();
                            stop += 1;
                        } else {
                            let mut temp = String::new();
                            io::stdin().read_line(&mut temp).unwrap();
                            in_buffer = temp.chars().collect();
                        }
                    }
                    let val = in_buffer.pop_front().unwrap() as u16;
                    self.set_reg(a, val);
                    if extracting {
                        println!("{}: in {}", self.ip, show_reg(a));
                    }
                    self.ip += 2;
                    out_buffer = String::new();
                }
                n => {
                    if extracting {
                        println!("{}: noop {}", self.ip, n);
                    }
                    self.ip += 1;
                }
            }
            if extracting {
                count += 1;
                if count >= 1000 { break 2; }
            }
        };
        println!("{}", out_buffer);
        println!("result {}", result);
        result
    }
}
