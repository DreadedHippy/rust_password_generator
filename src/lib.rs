use rand::{seq::SliceRandom, Rng};
use std::cmp::max;

const ALPHABETS: [char; 26] = [
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

const SPECIAL_CHARS: [char; 18] = [
	'?', '!', '£', '$', '%', '^', '&', '*', '.', '_', '-', ',', '/', '#', '~', '@', ';', ':'
];

const NUMERIC_CHARS: [char; 10] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

macro_rules! read_input {
	($t:tt) => {{
		let mut temp = String::new();
		std::io::stdin().read_line(&mut temp).expect("Invalid input");
		temp.trim().parse::<$t>().unwrap()
	}};
}

pub struct Params {
	pub password_length: u32,
	pub special_chars: u32,
	pub numeric_chars: u32
}

impl Params {
	pub fn generate_password(self) {
		let Params {special_chars, numeric_chars, password_length} = self;
		
    if special_chars + numeric_chars > password_length {
			println!("Password length must be greater than amount of special characters and digits");
			return
    }
		
		let (mut random_special_chars, mut random_numbers, mut random_alphabets) = (Vec::new(), Vec::new(), Vec::new());

		let max = max(max(special_chars, numeric_chars), password_length - (special_chars + numeric_chars));
		
		for i in 1..=max {
			if i <= special_chars {
				random_special_chars.push(SPECIAL_CHARS.choose(&mut rand::thread_rng()).unwrap().to_string());
			}
			
			if i <= numeric_chars {
				random_numbers.push(NUMERIC_CHARS.choose(&mut rand::thread_rng()).unwrap().to_string());
			}
			
			if i <= (password_length - (special_chars+numeric_chars)) {
				random_alphabets.push(generate_alphabet());
			}
		}
    
    let mut password = Vec::new();
    for _i in 1..=password_length {
			let choice: String = match rand::thread_rng().gen_range(0..=2) {
				0 => {
					random_special_chars.pop().unwrap_or_else(||
						random_numbers.pop().unwrap_or_else(||
							random_alphabets.pop().unwrap()
						)
					)
				},
				1 => {
					random_numbers.pop().unwrap_or_else(||
						random_alphabets.pop().unwrap_or_else(||
							random_special_chars.pop().unwrap()
						)
					)    
				},
				2 => {
					random_alphabets.pop().unwrap_or_else(||
						random_special_chars.pop().unwrap_or_else(||
							random_numbers.pop().unwrap()
						)
					)   
				},
				_ => { "This should not have happened".to_string()}
			};
			
			password.push(choice)        
    }
		
    let password = password.join("");
    println!("Here is your super secret password: \"{}\", guard it safely :)", password)
		
	}
}


fn generate_alphabet() -> String {
	let mut alphabet = ALPHABETS.choose(&mut rand::thread_rng()).unwrap().to_string();

	let is_to_uppercase: bool = rand::thread_rng().gen::<bool>();

	if is_to_uppercase {
		alphabet = alphabet.to_uppercase();
	}

	alphabet
}

pub fn get_user_input() -> u32 {
	let result = loop {    
			let input = read_input!(String);
			match input.trim().parse::<u32>() {
					Ok(n) => {
							break n
					},
					_ => println!("Invalid input, please try again")
			}
			
	};
	return result;  
}