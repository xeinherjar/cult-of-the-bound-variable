use std::fs::File;
use std::io::{ Read };

fn main() {
    let mut um = UM {
        power: true,
        pc: 0,
        registers: vec![0; 8],
        platters: vec![open_scroll()]
    };

    loop {
        let instr = um.step();
        let opcode = (instr & 0xF000_0000) >> 28;
        let reg_a  = ((instr >> 6) & 0x7) as usize;
        let reg_b  = ((instr >> 3) & 0x7) as usize;
        let reg_c  = (instr        & 0x7) as usize;

        println!("um registers, {:?}, pc {:?}", um.registers, um.pc);
        match opcode {
            0 => um.conditional_move(reg_a, reg_b, reg_c),
            3 => um.addition(reg_a, reg_b, reg_c),
            4 => um.multiplication(reg_a, reg_b, reg_c),
            5 => um.division(reg_a, reg_b, reg_c),
            6 => um.not_and(reg_a, reg_b, reg_c),
            7 => break,
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
    let mut file = match File::open("../codex.umz") {
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
    power: bool,
    pc: usize,
    registers: Vec<u32>,
    platters: Vec<Vec<u32>>
}

impl UM {
    fn step(&mut self) -> u32 {
        // Fetch, Decode, Execute

        let instruction = self.platters[0][self.pc];
        self.pc = self.pc + 1;

        instruction
    }

    fn conditional_move(&mut self, a: usize, b: usize, c: usize) {
        if self.registers[c] != 0 {
            self.registers[a] = self.registers[b];
        }

    }

    fn array_index(&mut self) {

    }

    fn array_amendment(&mut self) {

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

    fn halt(&mut self) {
        self.power = false;
    }

    fn allocation() {

    }

    fn abandonment() {

    }

    fn output() {

    }

    fn input() {

    }

    fn load_program(&mut self, b: usize, c: usize) {
        let index = self.registers[b] as usize;
        let platter_data = self.platters[index].clone();
        self.platters[index] = platter_data;
        self.pc = self.registers[c] as usize;

    }

    fn orthography(&mut self, instr: u32) {
        let reg_a = (instr as usize >> 25) & 0x7;
        self.registers[reg_a] = instr & 0x1FF_FFFF

    }

}
