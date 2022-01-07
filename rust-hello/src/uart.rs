// https://raw.githubusercontent.com/sgmarz/osblog/master/risc_v/src/uart.rs
// uart.rs
// UART routines and driver

use core::fmt::{Error, Write};

pub struct Uart {
	base_address: usize,
}

impl Write for Uart {
	fn write_str(&mut self, out: &str) -> Result<(), Error> {
		for c in out.bytes() {
			self.put(c);
		}
		Ok(())
	}
}

impl Uart {
	pub fn new(base_address: usize) -> Self {
		Uart { base_address }
	}

	pub fn put(&mut self, c: u8) {
		let ptr = self.base_address as *mut u8;
		unsafe {
			ptr.add(0).write_volatile(c);
		}
	}
}
