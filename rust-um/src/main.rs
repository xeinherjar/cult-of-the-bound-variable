use std::fs::File;
use std::io::{ Read, stdin };

fn main() {
    let mut um = UM {
        pc: 0,
        registers: [0, 0, 0, 0, 0, 0, 0, 0],
        platters: vec![open_scroll()],
        memory_index_next: 0,
        memory_index_available: vec![],
    };

    loop {
        let instr = um.step();
        let opcode = (instr & 0xF000_0000) >> 28;
        let reg_a  = ((instr >> 6) & 0x7) as usize;
        let reg_b  = ((instr >> 3) & 0x7) as usize;
        let reg_c  = (instr        & 0x7) as usize;

        match opcode {
            0 => um.conditional_move(reg_a, reg_b, reg_c),
            1 => um.array_index(reg_a, reg_b, reg_c),
            2 => um.array_amendment(reg_a, reg_b, reg_c),
            3 => um.addition(reg_a, reg_b, reg_c),
            4 => um.multiplication(reg_a, reg_b, reg_c),
            5 => um.division(reg_a, reg_b, reg_c),
            6 => um.not_and(reg_a, reg_b, reg_c),
            7 => break,
            8 => um.allocation(reg_b, reg_c),
            9 => um.abandonment(reg_c),
            10 => um.output(reg_c),
            11 => um.input(reg_c),
            12 => um.load_program(reg_b, reg_c),
            13 => um.orthography(instr),
            _ => {
                println!("Unknown Opcode {:?}", opcode);
                break
            }

        }
    }



}

fn open_scroll() -> Vec<u32> {
    //let mut file = match File::open("./test2") {
    let mut file = match File::open("../codex.umz") {
    //let mut file = match File::open("../sandmark.umz") {
        Ok(file) => file,
        Err(_)  => panic!("Failed to open file"),
    };

    let mut buffer: Vec<u8> = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => { },
        Err(_) => panic!("Failed to load scroll into vector")
    };

    let scroll: Vec<u32> = buffer.chunks(4)
    .map(| b | {
        let msb = (b[0] as u32) << 24;
        let lov = (b[1] as u32) << 16;
        let med = (b[2] as u32) <<  8;
        let lsb = (b[3] as u32) <<  0;

        msb | lov | med | lsb
    })
    .collect();

    scroll
}

#[derive(Debug)]
struct UM {
    pc: u32,
    registers: [u32; 8],
    platters: Vec<Vec<u32>>,
    memory_index_next: u32,
    memory_index_available: Vec<u32>,
}

impl UM {
    fn step(&mut self) -> u32 {
        let instruction = self.platters[0][self.pc as usize];
        self.pc = self.pc + 1;

        instruction
    }

    fn conditional_move(&mut self, a: usize, b: usize, c: usize) {
        if self.registers[c] != 0 {
            self.registers[a] = self.registers[b];
        }
    }

    fn array_index(&mut self, a: usize, b: usize, c: usize) {
        let index = self.registers[b] as usize;
        let offset = self.registers[c] as usize;
        self.registers[a] = self.platters[index][offset];
    }

    fn array_amendment(&mut self, a: usize, b: usize, c: usize) {
        let index = self.registers[a] as usize;
        let offset = self.registers[b] as usize;
        self.platters[index][offset] = self.registers[c];
    }

    fn addition(&mut self, a: usize, b: usize, c: usize) {
        self.registers[a] = self.registers[b].wrapping_add(self.registers[c]);
    }

    fn multiplication(&mut self, a: usize, b: usize, c: usize) {
        self.registers[a] = self.registers[b].wrapping_mul(self.registers[c]);
    }

    fn division(&mut self, a: usize, b: usize, c: usize) {
        self.registers[a] = self.registers[b].wrapping_div(self.registers[c]);
    }

    fn not_and(&mut self, a: usize, b: usize, c: usize) {
        let value = !(self.registers[b] & self.registers[c]);
        self.registers[a] = value;
    }

    /*
    fn halt(&mut self) {
    }
    */

    fn allocation(&mut self, b: usize, c: usize) {
        let new_platter: Vec<u32> = vec![0; self.registers[c] as usize];
        if self.memory_index_available.len() == 0 {
            self.memory_index_next = self.memory_index_next + 1;
            self.platters.push(new_platter);
            self.registers[b] = self.memory_index_next;
        } else {
            let index = match self.memory_index_available.pop() {
                Some(index) => index,
                None => panic!("Whoa, how did this fail.....")
            };
            self.platters[index as usize] = new_platter;
            self.registers[b] = index;
        }

    }

    fn abandonment(&mut self, c: usize) {
        let index = self.registers[c];
        self.platters[index as usize] = vec![];
        self.memory_index_available.push(index);
    }

    fn output(&mut self, c: usize) {
        let glyph = (self.registers[c] & 0xFF) as u8;
        let glyph = glyph as char;
        print!("{}", glyph);
    }

    fn input(&mut self, c: usize) {
        for byte in stdin().bytes() {
            match byte {
                Ok(byte) => {
                    self.registers[c] = (byte & 0xFF) as u32;
                    break;
                },
                Err(_) => panic!("Error reading from stdin")
            }
        }
    }

    fn load_program(&mut self, b: usize, c: usize) {
        let index = self.registers[b];
        if index != 0 {
            self.platters[0] = self.platters[index as usize].clone();
        }
        self.pc = self.registers[c];
    }

    fn orthography(&mut self, instr: u32) {
        let reg_a = (instr >> 25) & 0x7;
        self.registers[reg_a as usize] = instr & 0x1FF_FFFF;
    }

}
