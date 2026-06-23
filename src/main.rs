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
    Jmp(usize),
    Cmp(usize,usize),
    Je(usize),
    Jne(usize),
    Inc(usize),
    Dec(usize),
}

struct CPU{
    registers: [i64 ; 4],
    program_counter : usize,
    halted_flag     : bool,
    zero_flag       : bool,
    sign_flag       : bool,
}


impl CPU {
    fn new() -> Self{
        Self { 
            registers: [0 ; 4],
            program_counter: 0,
            halted_flag: false, 
            zero_flag : false,
            sign_flag : false,
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
                if self.registers[*dst] == 0{
                    self.zero_flag = true;
                }
                else {
                    self.zero_flag = false;
                }
            }
            Instruction::Sub(dst,src ) => {
                self.registers[*dst] -= self.registers[*src];
                if self.registers[*dst] == 0{
                    self.zero_flag = true;
                }
                else {
                    self.zero_flag = false;
                }
            }
            Instruction::Mul(dst,src ) => {
                self.registers[*dst] *= self.registers[*src];
                if self.registers[*dst] == 0{
                    self.zero_flag = true;
                }
                else {
                    self.zero_flag = false;
                }
            }
            Instruction::Div(dst,src ) => {
                self.registers[*dst] /= self.registers[*src];
                if self.registers[*dst] == 0{
                    self.zero_flag = true;
                }
                else {
                    self.zero_flag = false;
                }
            }
            Instruction::Print(reg) => {
                println!("{:?}",self.registers[*reg]);
            }
            Instruction::Halt =>{
                self.halted_flag = true;
            }
            Instruction::Jmp(pc_set) =>{
                self.program_counter = *pc_set-1;
            }
            Instruction::Cmp(reg1,reg2 ) =>{
                if self.registers[*reg1] - self.registers[*reg2] == 0{
                    self.zero_flag = true;
                }
                else if self.registers[*reg1] - self.registers[*reg2] > 0{
                    self.sign_flag = false;
                }
                else {
                    self.sign_flag = true;
                }
            }
            Instruction::Je(pc_set) => {
                if self.zero_flag == true{
                    self.program_counter = *pc_set - 1;
                }
            }
            Instruction::Jne(pc_set) => {
                if self.zero_flag == false{
                    self.program_counter = *pc_set - 1;
                }
            }
            Instruction::Inc(reg) => {
                self.registers[*reg] += 1;
            }
            Instruction::Dec(reg) => {
                self.registers[*reg] -= 1;
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
        Instruction::Load(0, 0),
        Instruction::Load(2, 10),
        Instruction::Inc(0),
        Instruction::Print(0),
        Instruction::Cmp(0,2),
        Instruction::Jne(2),
        Instruction::Halt,
    ];

    let mut cpu = CPU::new();
    cpu.run(&program);
}
