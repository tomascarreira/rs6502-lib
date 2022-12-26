mod cpu;
mod ram;


trait MemoryMap {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, val: u8);
}
