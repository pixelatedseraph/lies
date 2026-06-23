//lies - lies is emulating systems
// R0 R1 R2 R3
// PC program counter

// LOAD reg,imm
// ADD dst,src
// PRINT reg
// HALT

// LOAD R0,10

enum Instruction{
    Load(usize,i64),
    Add(usize,usize),
    Sub(usize,usize),
    Mul(usize,usize),
    Div(usize,usize),
    Print(usize),
    Halt,
}

struct CPU{
    registers: [i64 ; 4],
    program_counter : usize,
    zero_flag       : bool,
    sign_flag       : bool,
    carry_flag      : bool,
    overflow_flag   : bool,
    halted_flag          : bool,
}


impl CPU {
    fn new() -> Self{
        Self { 
            registers: [0 ; 4],
            program_counter: 0,
            halted_flag: false, 
            zero_flag : false,
            
        }
    }

    fn step(&mut self,program: &[Instruction]){
        let instr = &program[self.program_counter];

        match instr {
            Instruction::Load(reg,value ) => {
                self.registers[*reg] = *value;
            }
            Instruction::Add(dst,src ) => {
                self.registers[*dst] += self.registers[*src];
            }
            Instruction::Sub(dst,src ) => {
                self.registers[*dst] -= self.registers[*src];
            }
            Instruction::Mul(dst,src ) => {
                self.registers[*dst] *= self.registers[*src];
            }
            Instruction::Div(dst,src ) => {
                self.registers[*dst] /= self.registers[*src];
            }
            Instruction::Print(reg) => {
                println!("{:?}",self.registers[*reg]);
            }
            Instruction::Halt =>{
                self.halted_flag = true;
            }
        }
    self.program_counter +=1;
    }

    fn run(&mut self,program: &[Instruction]){
        while !self.halted_flag{
            self.step(program);
        }
    }

}



fn main() {
    let program = vec![
        Instruction::Load(0, 10),
        Instruction::Load(1, 20),
        Instruction::Add(0,1),
        Instruction::Print(0),
        Instruction::Halt
    ];

    let mut cpu = CPU::new();
    cpu.run(&program);
}
