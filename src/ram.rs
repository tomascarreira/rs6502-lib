mod ram{
	const MEM_CAP: usize = 0x10000;

	use crate::MemoryMap;

	struct Ram (Vec<u8>);

	impl Ram {
		fn new() -> Self {
			Ram(vec![0; MEM_CAP])
		}
	}

	impl MemoryMap for Ram {
		fn read(&self, addr: u16) -> u8 {
			self.0[(addr as usize) % 0xffff]
		}
		fn write(&mut self, addr: u16, val: u8) {
			self.0[(addr as usize) % 0xffff] = val;
		}
	}
}
