const MEMORY_SIZE: usize = 30000;
fn main() {
	let mut input = String::new();
	println!("Input: ");
  std::io::stdin().read_line(&mut input).unwrap();
  let mut memory = [0u8; MEMORY_SIZE];
  let mut pointer: usize = 0;
	// pc = porn_child
  let mut pc: usize = 0;
  while pc < input.len() {
		match input.chars().nth(pc).unwrap() {
			'>' => {
				if pointer < MEMORY_SIZE - 1 {
					pointer += 1;
				} else {
					panic!("ERROR: Out of memory");
				}
			}
			'<' => {
				if pointer > 0 {
					pointer -= 1;
				} else {
					panic!("ERROR: Memory pointer cannot be negative");
        }
      }
			'+' => memory[pointer] = memory[pointer].wrapping_add(1),
			'-' => memory[pointer] = memory[pointer].wrapping_sub(1),
      '.' => println!("{}", memory[pointer] as char),
			'[' => {
				if memory[pointer] == 0 {
					let mut depth = 1;
					while depth > 0 {
						pc += 1;
						let c = input.chars().nth(pc).unwrap();
						if c == '[' {
							depth += 1;
						} else if c == ']' {
							depth -= 1;
						}
					}
				}
      }
			']' => {
				if memory[pointer] != 0 {
					let mut depth = 1;
					while depth > 0 {
						pc -= 1;
						let c = input.chars().nth(pc).unwrap();
						if c == '[' {
							depth -= 1;
						} else if c == ']' {
							depth += 1;
						}
					}
				}
			}
			_ => (),
		}
		pc += 1;
	}
}
