import binascii
import sys

def load_umz(file_path):
    """Loads the UMZ binary file.
    The Universal Machine is big endian"""
    with open(file_path, mode='rb') as file:
        content = file.read()
        byte_array = []

        for b in range(0, len(content), 4):
            b1 = int(binascii.hexlify(content[b]), 16)   << 24  # msb
            b2 = int(binascii.hexlify(content[b+1]), 16) << 16
            b3 = int(binascii.hexlify(content[b+2]), 16) << 8
            b4 = int(binascii.hexlify(content[b+3]), 16)        # lsb
            byte = b1 | b2 | b3 | b4
            byte = byte & 0xFFFFFFFF
            byte_array.append(byte)

        return byte_array


# Universal Machine
class um:
    # Program Counter | Execution Finger
    pc = 0

    # 8 general registers
    registers = [0, 0, 0, 0, 0, 0, 0, 0]

    # Memory
    # 0 is program array, pc reads only from that array
    # array of arrays
    mem = []
    next_index = 0;
    available_mem_index = [];

    # Standard Operations
    def conditional_move(self, reg_a, reg_b, reg_c):
        """op 0
        Register A receives the value in register B,
        unless the register C contains 0"""
        if self.registers[reg_c] != 0:
            self.registers[reg_a] = self.registers[reg_b]
        return

    def array_index(self, reg_a, reg_b, reg_c):
        """op 1
        Register A recieves the value stored at offset
        in register C in the array identified by B"""
        arr_idx = self.registers[reg_b]
        offset = self.registers[reg_c]
        self.registers[reg_a] = self.mem[arr_idx][offset]
        return

    def array_amendment(self, reg_a, reg_b, reg_c):
        """op 2
        Array identified by A is amended at the offset
        in register B to store the value in register C"""
        arr_idx = self.registers[reg_a]
        offset = self.registers[reg_b]
        self.mem[arr_idx][offset] = self.registers[reg_c]
        return

    def addition(self, reg_a, reg_b, reg_c):
        """op 3
        Register A receives the value in register B plus
        the value in register C, modulo 2^32"""
        value = (self.registers[reg_b] + self.registers[reg_c]) & 0xFFFFFFFF
        self.registers[reg_a] = value
        return

    def multiplication(self, reg_a, reg_b, reg_c):
        """op 4
        Register A receives the value in register B times
        Fhe value in register C, modulo 2^32"""
        value = (self.registers[reg_b] * self.registers[reg_c]) & 0xFFFFFFFF
        self.registers[reg_a] = value
        return

    def division(self, reg_a, reg_b, reg_c):
        """op 5
        Register A receives the value in register B
        divided by the value in register C, if any, where
        each quantity is treated as an unsigned 32 bit number"""
        lhv = self.registers[reg_b]
        rhv = self.registers[reg_c]
        value = (lhv // rhv) & 0xFFFFFFFF
        self.registers[reg_a] = value
        return

    def not_and(self, reg_a, reg_b, reg_c):
        """op 6
        Each bit in register A receives a 1 bit if either
        register B or register C has a 0 bit in that position.
        Otherwise the bit in register A receives the 0 bit"""
        value = ~(self.registers[reg_b] & self.registers[reg_c])
        self.registers[reg_a] = (value & 0xFFFFFFFF)
        return

    # Other Operators
    def halt(self, reg_a, reg_b, reg_c):
        """op 7
        Universal machine stops computation"""
        #print('halt')
        print 'halt'
        self.on = False
        return

    def allocation(self, reg_a, reg_b, reg_c):
        """op 8
        New array is created with a capacity of platters
        commensurate to the value in the register C.  This
        new array is initialized entirely with platters
        holding the value 0.  A bit pattern not consisting of
        exclusively the 0 bit, and that identifies no other
        active allocated array, is placed in the B register"""
        arr = [0] * self.registers[reg_c]
        if len(self.available_mem_index) == 0:
            self.next_index = self.next_index + 1
            self.mem.append(arr)
            self.registers[reg_b] = self.next_index
        else:
            index = self.available_mem_index.pop()
            self.mem[index] = arr
            self.registers[reg_b] = index
        return

    def abandonment(self, reg_a, reg_b, reg_c):
        """op 9
        Array identified by the register C is abandoned.
        Future allocations may then reuse that identifier"""
        mem_index = self.registers[reg_c]
        #self.mem[mem_index].clear()
        self.mem[mem_index] = []
        self.available_mem_index.append(mem_index)
        return

    def output(self, reg_a, reg_b, reg_c):
        """op 10
        Value in the register C is displayed on the console
        immediately.  Only values between and including 0 and 255
        are allowed"""
        value = self.registers[reg_c] & 0xFF
        #print(chr(value), end="")
        sys.stdout.write(chr(value))
        return

    def _input(self, reg_a, reg_b, reg_c):
        """op 11
        Universal machine waits for input on the console.
        When input arrives, the register C is loaded with the
        input, which must be between and including 0 and 255.

        If the end of input has been signaled, then the
        register C is endowed with a uniform value pattern
        where every place is pregnant with the 1 bit"""
        try:
            self.registers[reg_c] = ord(sys.stdin.read(1)) & 0xFF
        except EOFError:
            self.registers[reg_c] = 0xFFFFFFFF
        return

    def load_program(self, reg_a, reg_b, reg_c):
        """op 12
        Array identified by the B register is duplicated
        and the duplicate shall replace the '0' array,
        regardless of size.  The execution finger is placed
        to indicate the platter of this array that is
        described by the offset given in C, where the value
        0 denotes the first platter, 1 the second, et cetera.

        The '0' array shall be the most sublime choice for
        loading, and shall be handled with the utmost velocity"""
        idx = self.registers[reg_b]
        arr = list(self.mem[idx])
        self.mem[0] = arr
        self.pc = self.registers[reg_c]
        return

    # Special Operators
    def orthography(self, reg_a, reg_b, reg_c):
        """op 13
        The value indicated is loaded into the register A
        forthwith"""
        reg_a = (self.op >> 25) & 0x7
        value = self.op & 0x1FFFFFF
        self.registers[reg_a] = value
        return

    op_map = [
        conditional_move,
        array_index,
        array_amendment,
        addition,
        multiplication,
        division,
        not_and,
        halt,
        allocation,
        abandonment,
        output,
        _input,
        load_program,
        orthography
    ]

    def step(self):
        # fetch, decode, execute
        self.op = self.mem[0][self.pc]
        op_code = (self.op & 0xF0000000) >> 28
        # registers a, b, c
        reg_a = (self.op >> 6) & 0x7
        reg_b = (self.op >> 3) & 0x7
        reg_c = (self.op & 0x7)

        self.pc = self.pc + 1

        self.op_map[op_code](self, reg_a, reg_b, reg_c)
#        print('op:%s, pc=%s, a=%s, b=%s, c=%s' % (op_code, self.pc, reg_a,
#            reg_b, reg_c))

    def power_on(self, file_path):
        self.on = True
        #program = load_umz('./sandmark.umz')
        #program = load_umz('./codex.umz')
        program = load_umz(file_path)
        self.mem.append(program)
        while self.on:
            self.step()


universal_machine = um()
universal_machine.power_on(sys.argv[1])
