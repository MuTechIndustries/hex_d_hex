// --------------------------------------------------------------------------------------------------------------------------------
// Module Name: HexDHex
// Module Purpose: Transform hex strings into a vector of u8's representing bytes from a string and turns a vector of 
// u8's representing the values of each pair of hex string bytes
// Organization of origin: MuTech Industries
// Current organization of upkeep: MuTech Industries
// Creating Author: Tyrel Richey
// Software License: SNGOG
// --------------------------------------------------------------------------------------------------------------------------------
// What Is a SNGOG License?
// A SNGOG or Situation Normal God Is Good License works as follows
// Any code released under a SNGOG License can be altered used and shared under any other license that is desired. 
// We would like if you made note of Mu Tech Industries however, it is not required.
// We are not liable for the actions of SNGOG users or results of the code base.
// On that note enjoy.
// Fine Print: Ussage of code results in MuTech Industries having the right to call upon you for a quest at any giving time.
// Final Note: The previous clause can be denied, however it is not recomended as who the heck turns down a quest?
// --------------------------------------------------------------------------------------------------------------------------------
// How is SNGOG pronounced?
// Sna-Gaug though to be honest it doesn't really matter.
// --------------------------------------------------------------------------------------------------------------------------------

// --------------------------------------------------------------------------------------------------------------------------------
// Begin Private Methods
// --------------------------------------------------------------------------------------------------------------------------------
// Function: utf8_byte_to_hex_position
// Purpose: Converts a UTF8 byte of 0-9 + a-f + A-F to its respective counting postion of 0-15 for hex base counting
// E.X. Input(65 | 97) -> Output(10)

#[inline]
fn utf8_byte_to_hex_position( byte: u8 ) -> u8 {
	match byte {
		e @ 48...57 => e - 48,
		65 | 97 => 10,
		66 | 98 => 11,
		67 | 99 => 12,
		68 | 100 => 13,
		69 | 101 => 14,
		70 | 102 => 15,
		_ => panic!("utf8_byte_to_hex_position called with a non-ASCII Hex character. Input must only contain ASCII('a' - 'f' | 'A' - 'F' | '0' - '9')")
	}
}

#[test]
fn test_utf8_byte_to_hex_position() {
	assert_eq!(utf8_byte_to_hex_position('0' as u8), 0);
	assert_eq!(utf8_byte_to_hex_position('1' as u8), 1);
	assert_eq!(utf8_byte_to_hex_position('2' as u8), 2);
	assert_eq!(utf8_byte_to_hex_position('3' as u8), 3);
	assert_eq!(utf8_byte_to_hex_position('4' as u8), 4);
	assert_eq!(utf8_byte_to_hex_position('5' as u8), 5);
	assert_eq!(utf8_byte_to_hex_position('6' as u8), 6);
	assert_eq!(utf8_byte_to_hex_position('7' as u8), 7);
	assert_eq!(utf8_byte_to_hex_position('8' as u8), 8);
	assert_eq!(utf8_byte_to_hex_position('9' as u8), 9);
	assert_eq!(utf8_byte_to_hex_position('a' as u8), 10);
	assert_eq!(utf8_byte_to_hex_position('b' as u8), 11);
	assert_eq!(utf8_byte_to_hex_position('c' as u8), 12);
	assert_eq!(utf8_byte_to_hex_position('d' as u8), 13);
	assert_eq!(utf8_byte_to_hex_position('e' as u8), 14);
	assert_eq!(utf8_byte_to_hex_position('f' as u8), 15);
	assert_eq!(utf8_byte_to_hex_position('A' as u8), 10);
	assert_eq!(utf8_byte_to_hex_position('B' as u8), 11);
	assert_eq!(utf8_byte_to_hex_position('C' as u8), 12);
	assert_eq!(utf8_byte_to_hex_position('D' as u8), 13);
	assert_eq!(utf8_byte_to_hex_position('E' as u8), 14);
	assert_eq!(utf8_byte_to_hex_position('F' as u8), 15);
}

fn hex_position_to_capital_utf8_byte( hex_position: u8 ) -> u8 {
	match hex_position {
		e @ 0...9 => e + 48,
		10 => 65,
		11 => 66,
		12 => 67,
		13 => 68,
		14 => 69,
		15 => 70,
		_ => panic!("Expected an input of 0-15 in hex_position_to_capital_utf8_byte. You entered {}.", hex_position),
	}
}

#[test]
fn test_hex_position_to_capital_utf8_byte() {
	assert_eq!(hex_position_to_capital_utf8_byte(0), '0' as u8);
}

fn hex_position_to_lower_utf8_byte( hex_position: u8 ) -> u8 {
	match hex_position {
		e @ 0...9 => e + 48,
		10 => 97,
		11 => 98,
		12 => 99,
		13 => 100,
		14 => 101,
		15 => 102,
		_ => panic!("Expected an input of 0-15 in hex_position_to_lower_utf8_byte. You entered {}.", hex_position),
	}
}

#[test]
fn test_hex_position_to_lower_utf8_byte() {
	assert_eq!(hex_position_to_lower_utf8_byte(0), '0' as u8);
}

// --------------------------------------------------------------------------------------------------------------------------------

// --------------------------------------------------------------------------------------------------------------------------------
// Begin Public Methods
// --------------------------------------------------------------------------------------------------------------------------------
// Function: dhex
// Purpose: To take a hex_string such as "0F12" and create a Vec<u8> for each pair of bytes
// Parameters: Should be an even length hex_string however if it's not than its last byte will be ignored as the length is devided by two.
// E.X. Input("OF12") -> Output(Vec<u8>[15,18])
pub fn dhex( hex_string: &str ) -> Box<Vec<u8>> {
	let hex_string_bytes: Vec<u8> = hex_string.bytes().collect();
	let num_of_bytes = hex_string_bytes.len() / 2;
	let mut dhex: Box<Vec<u8>> = Box::new(Vec::with_capacity(num_of_bytes));
	for iteration in 0..num_of_bytes {
		let base_index = iteration * 2;
		let new_val: u8 = (16 * utf8_byte_to_hex_position(hex_string_bytes[base_index])) + (utf8_byte_to_hex_position(hex_string_bytes[base_index + 1]));
		(*dhex).push(new_val);
	}
	dhex
}

#[test]
fn test_dhex() {
	let hex_string = "0F12FF00a7";
	let hex_vals = *dhex(hex_string);
	assert_eq!(hex_vals[0], 15);
	assert_eq!(hex_vals[1], 18);
	assert_eq!(hex_vals[2], 255);
	assert_eq!(hex_vals[3], 0);
	// Should allow mix of capital and lower case letters
	assert_eq!(hex_vals[4], 167);
}

// Function: hex
// Purpose: To take a Vec<u8> and create a hex_string representing the values in pairs of base 16 ansci bytes
// E.X. Input(Vec<u8>[15,18]) -> Output("0F12")
pub fn capital_hex( hex_values: &Vec<u8> ) -> Box<String> {
	let num_of_bytes = hex_values.len();
	let mut hex_string: Box<String> = Box::new(String::with_capacity(num_of_bytes));
	for index in 0..num_of_bytes {
		let byte: u8 = hex_values[index];
		let two_position: u8 = byte / 16;
		let one_position: u8 = byte - (two_position * 16);
		let left_char: u8 = hex_position_to_capital_utf8_byte(two_position);
		hex_string.push(left_char as char);
		let right_char: u8 = hex_position_to_capital_utf8_byte(one_position);
		hex_string.push(right_char as char);
	}
	hex_string
}

#[test]
fn test_capital_hex() {
	let hex_vals: Vec<u8> = vec![15,18,255,0,167];
	let hex_string = *capital_hex(&hex_vals);
	assert_eq!(hex_string, "0F12FF00A7");
}

pub fn lower_hex( hex_values: &Vec<u8> ) -> Box<String> {
	let num_of_bytes = hex_values.len();
	let mut hex_string: Box<String> = Box::new(String::with_capacity(num_of_bytes));
	for index in 0..num_of_bytes {
		let byte: u8 = hex_values[index];
		let two_position: u8 = byte / 16;
		let one_position: u8 = byte - (two_position * 16);
		let left_char: u8 = hex_position_to_lower_utf8_byte(two_position);
		hex_string.push(left_char as char);
		let right_char: u8 = hex_position_to_lower_utf8_byte(one_position);
		hex_string.push(right_char as char);
	}
	hex_string
}

#[test]
fn test_lower_hex() {
	let hex_vals: Vec<u8> = vec![15,18,255,0,167];
	let hex_string = *lower_hex(&hex_vals);
	assert_eq!(hex_string, "0f12ff00a7");
}
// --------------------------------------------------------------------------------------------------------------------------------

// MuTech Industries
// Coding is an art
// Stay excelent to each other