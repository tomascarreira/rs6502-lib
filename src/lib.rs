const MEM_CAP: usize  = 0x10000;

struct Cpu6502<'a> {
	a: u8,
	x: u8,
	y: u8,
	sp: u8,
	pc: u16,
	p: u8,
	decimal_mode: bool,
}

impl Cpu6502<'_> {
	fn new() -> Self {
		let instr_vec = Vec::new(); 
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

struct Ram (Vec<u8>);

impl Ram {
	fn new() -> Self {
		Ram(vec![0; MEM_CAP])
	}
}

trait MemoryMap {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, val: u8);
}

impl MemoryMap for Ram {
	fn read(&self, addr: u16) -> u8 {
		self.0[(addr as usize) % 0xffff]
	}
	fn write(&mut self, addr: u16, val: u8) {
		self.0[(addr as usize) % 0xffff] = val;
	}
}
