#![feature(core)]
mod vm;

fn main(){
	let bytecode = "\x0E\x00\x03\x00\x02\x02\x0A";
	println!("Perin v{}",vm::perin_version());
	println!("Perin VM executes FlopFlip bytecode");
	
	let mut perin = vm::PerinVM::new();
	perin.interpreter(bytecode);
}
