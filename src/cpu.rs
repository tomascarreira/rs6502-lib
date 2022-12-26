mod cpu {
	const MEM_CAP: usize  = 0x10000;
	
	use crate::MemoryMap;

	struct Cpu6502 {
		a: u8,
		x: u8,
		y: u8,
		sp: u8,
		pc: u16,
		p: u8, // todo! check bitflags macro
		decimal_mode: bool,
	}

	impl Cpu6502 {
		fn new() -> Self { 
			Cpu6502 { 
				a:0,
				x:0,
				y:0,
				sp:0xfd,
				pc:0,
				p:0,
				decimal_mode: true,
			}
		}
	}

	fn fetch_opcode(cpu: &Cpu6502, memory: &impl MemoryMap) {
		
	}
}
