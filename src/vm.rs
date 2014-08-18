pub fn perin_version() -> f32{
	0.1
}

#[deriving(FromPrimitive)]
enum Instruction {
	INTEGER = 0x00,
	STRING = 0x01,
	ADD = 0x02,
	SHOWINTEGER = 0x0A,
	SHOWVERSION = 0x0E,
	EXITVM = 0x0F
}

pub struct PerinVM{
	push: bool,
	stack: Vec<u8>
}

impl PerinVM{
	pub fn new() -> PerinVM{
		println!("Starting PerinVM instance");
		PerinVM{push: false, stack: vec![]}
	}
	pub fn interpreter(&mut self,bytecode: &'static str) -> (){
		for execbyte in bytecode.chars() {
			self.execute(execbyte as u8);
		}
	}
	fn execute(&mut self, execbyte: u8) -> () {
		if self.push {
			self.push(execbyte);
			self.push=false;
		}else{
			let op: Option<Instruction> = FromPrimitive::from_u8(execbyte);
			match op{
				None => {
					println!("Unknown instruction, skipping...");
				},
				Some(bc) => {
					match bc{
						INTEGER => {
							self.push=true;
						},
						ADD => {
							let a=self.pop() as int;
							let b=self.pop() as int;
							let c=a+b;
							self.push(c as u8);
						},
						SHOWINTEGER => {
							println!("Integer value {}",self.pop() as int);
						},
						SHOWVERSION => {
							println!("PerinVM v0.1.0");
						},
						EXITVM => {
							println!("Exit VM");
						},
						STRING => {
							println!("Unsupported instruction 'STRING' ");
						}
					}
				}
			}
			
		}
	}
	fn push(&mut self, value: u8) -> (){
		self.stack.push(value);
	}
	fn pop(&mut self) -> u8{
		let a: Option<u8>=self.stack.pop();
		match a{
			None => {
				println!("Failed to pop");
				0
			},
			Some(result) => {
				result
			}
		}
	}
}