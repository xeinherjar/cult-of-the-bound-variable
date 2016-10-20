use std::fs::File;
use std::io::{ Read };

fn main() {
    let scroll = open_scroll();
    let mut um = UM {
        power: true,
        pc: 0,
        registers: vec![0, 8],
        platters: vec![0, 1]
    };

    println!("CODEC, {:?}", scroll[33]);
    println!("UM, {:?}", um);

    um.step();
    println!("UM, {:?}", um);
}

fn open_scroll() -> Vec<u8> {
    let mut file = match File::open("../codex.umz") {
        Ok(file) => file,
        Err(_)  => panic!("Failed to open file"),
    };

    let mut buffer: Vec<u8> = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => buffer,
        Err(_) => panic!("Failed to load codec into vector")
    }
}

#[derive(Debug)]
struct UM {
    power: bool,
    pc: i32,
    registers: Vec<u32>,
    platters: Vec<u32>
}

impl UM {
    fn step(&mut self) {
        // Fetch, Decode, Execute

        self.pc = self.pc + 1;
    }

    fn conditional_move(&mut self) {

    }

    fn array_index(&mut self) {

    }

    fn array_amendment(&mut self) {

    }

    fn addition(&mut self) {

    }

    fn multiplication(&mut self) {

    }

    fn division(&mut self) {

    }

    fn not_and(&mut self) {

    }

    fn halt(&self) {
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

    fn load_program() {

    }

    fn orthography() {

    }

}
