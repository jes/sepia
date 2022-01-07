#![no_std]
#![no_main]

#![feature(global_asm)]
global_asm!(include_str!("asm/boot.S"));

mod uart;

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
			});
}
#[macro_export]
macro_rules! println
{
	() => ({
		   print!("\r\n")
		   });
	($fmt:expr) => ({
			print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			print!(concat!($fmt, "\r\n"), $($args)+)
			});
}

#[no_mangle]
extern "C" fn kinit() {
    println!("Hello, world!");
}
#[no_mangle]
extern "C" fn kinit_hart(_hartid: usize) {
	// We aren't going to do anything here until we get SMP going.
	// All non-0 harts initialize here.
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	println!("Aborting");
	abort();
}

#[no_mangle]
extern "C" fn abort() -> ! {
    loop {}
}
