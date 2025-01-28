pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    /* 
        ASM INSTRUCTIONS
     */

    fn lda(&mut self, param: u8) {
        self.register_a = param;
        self.update_zero_and_negative_flags(self.register_a);
    }

    // Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    // Adds ine to the X register setting the zero and negitive flags as appropriate
    // Zero flag : Set if X is zero
    // Nega flag : Set if bit 7 of X is set
    // Rest is not affected
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {

        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    /* 
        CPU FUNCTIONS
     */

    /// Memory functions

    // Reads the byte at a given address in the memory
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    // Write `data` at `addr`
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }

    /// Machine functions
    
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }
    
    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opscode {
                0xA9 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    self.lda(param);
                }
                0x00 => {
                    return ;
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                _ => todo!()
            }
        }
    }

    // pub fn interpret(&mut self, program: Vec<u8>) {
    //     self.program_counter = 0;

    //     loop {
    //         let opscode = program[self.program_counter as usize];
    //         self.program_counter += 1;

    //         match opscode {
    //             0xA9 => {
    //                 let param = program[self.program_counter as usize];
    //                 self.program_counter += 1;
    //                 self.lda(param);
    //             }
    //             0x00 => {
    //                 return ;
    //             }
    //             0xAA => self.tax(),
    //             0xE8 => self.inx(),
    //             _ => todo!()
    //         }
    //     }
    // }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
   use super::*;
 

    // test for lda
   #[test]
   fn test_0xa9_lda_immediate_load_data() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
       assert_eq!(cpu.register_a, 0x05);
       assert!(cpu.status & 0b0000_0010 == 0b00);
       assert!(cpu.status & 0b1000_0000 == 0);
   }

    // test for lda's zero flags
    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    // test for tax
    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.load(vec![0xaa, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
  
        assert_eq!(cpu.register_x, 10)
    }

    // test for lda, tax, inx and brk
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
  
        assert_eq!(cpu.register_x, 0xc1)
    }
 
     #[test]
     fn test_inx_overflow() {
         let mut cpu = CPU::new();
         cpu.register_x = 0xff;
         cpu.load(vec![0xe8, 0xe8, 0x00]);
         cpu.program_counter = cpu.mem_read_u16(0xFFFC);
         cpu.run();
 
         assert_eq!(cpu.register_x, 1)
     }
 
 
}
