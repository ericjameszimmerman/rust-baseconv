use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let arg = &args[1];
        parse_and_convert(arg);
    } else {
        println!("Please provide an argument.");
    }
}

fn parse_and_convert(arg: &str) {
    use regex::Regex; // Add the import statement here

    if let Ok(i) = arg.parse::<i64>() {
        convert_and_print_bytes(&i.to_le_bytes());
    } else if let Ok(u) = arg.parse::<u64>() {
        convert_and_print_bytes(&u.to_le_bytes());
    } else if let Ok(f) = arg.parse::<f64>() {
        convert_and_print_bytes(&f.to_le_bytes());
    } else if arg.ends_with('f') || arg.ends_with('F') {
        parse_float(arg);
    } else {
        let pattern = r"^(0x|<0x|>0x|0X|<0X|>0X|>|<)?.*?[hH]?$";
        let re = Regex::new(pattern).unwrap();
        if re.is_match(arg) {
            parse_hex(arg);
        } else {
            println!("Argument didn't match any expected format.");
        }
    }
}

fn convert_and_print_float(float_value: f32) {
    // Convert to byte array in little-endian format
    let float_bytes = float_value.to_le_bytes();
    let int32 = i32::from_le_bytes(float_bytes);
    let int64: i64 = int32 as i64; // Convert i32 to i64
    let int64_bytes = int64.to_le_bytes();
    convert_and_print_bytes(&int64_bytes);
}

fn hex_to_byte_array(hex_str: &str, is_big_endian: bool) -> [u8; 8] {
    let mut bytes = Vec::new();

    // Convert hex string to bytes
    for chunk in hex_str.as_bytes().chunks(2) {
        let hex = std::str::from_utf8(chunk).expect("Invalid UTF-8 sequence");
        if let Ok(byte) = u8::from_str_radix(hex, 16) {
            bytes.push(byte);
        }
    }

    // If input is big-endian, reverse it for little-endian systems (most systems are little-endian)
    if is_big_endian {
        bytes.reverse();
    }

    // Initialize a fixed size array with zeros
    let mut fixed_bytes: [u8; 8] = [0; 8];

    // Copy the bytes into the fixed size array, truncating or padding as necessary
    for (i, &byte) in bytes.iter().enumerate().take(8) {
        fixed_bytes[i] = byte;
    }

    fixed_bytes
}

fn parse_float(arg: &str) {
    let float_arg = &arg[..arg.len() - 1]; // Remove the 'f' suffix
    match float_arg.parse::<f32>() {
        Ok(f) => convert_and_print_float(f),
        Err(_) => println!("Failed to parse as float."),
    }
}

fn parse_hex(arg: &str) {
    let (big_endian, hex_value) = parse_hex_string(arg);
    let bytes = hex_to_byte_array(&hex_value, big_endian);
    convert_and_print_bytes(&bytes);
}

fn parse_hex_string(input: &str) -> (bool, String) {
    // Determine the boolean value based on the starting character
    let starts_with = match input.chars().next() {
        Some('>') => true,
        Some('<') => false,
        _ => true,
    };

    // Remove '>' or '<' from the start if present
    let trimmed_start = if input.starts_with('>') || input.starts_with('<') {
        &input[1..]
    } else {
        input
    };

    // Remove "0x" or "0X" prefix if present
    let trimmed_prefix = if trimmed_start.to_lowercase().starts_with("0x") {
        &trimmed_start[2..]
    } else {
        trimmed_start
    };

    // Remove "h" or "H" suffix if present
    let hex_value = if trimmed_prefix.to_lowercase().ends_with('h') {
        &trimmed_prefix[..trimmed_prefix.len() - 1]
    } else {
        trimmed_prefix
    };

    (starts_with, hex_value.to_string())
}

fn convert_and_print_bytes(bytes: &[u8; 8]) {
    // Convert to various integer types
    let int64 = i64::from_le_bytes(bytes[0..8].try_into().unwrap());
    let int32 = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
    let int16 = i16::from_le_bytes(bytes[0..2].try_into().unwrap());
    let int8 = bytes[0] as i8; // Direct cast, since i8 and u8 have the same memory representation

    let uint64 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
    let uint32 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
    let uint16 = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
    let uint8 = bytes[0]; // Direct use, as u8 is the type of bytes' elements

    // Convert to floating-point types
    let float_value = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
    let double_value = f64::from_le_bytes(bytes[0..8].try_into().unwrap());

    let little_endian_bytes = uint64.to_le_bytes();
    let hex_string: String = little_endian_bytes
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
    let big_endian_bytes = uint64.to_be_bytes();
    let hex_string_big_endian: String = big_endian_bytes
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();

    // Prefix with "0x" to denote hexadecimal
    println!("     Hex (le): 0x{}", hex_string);
    println!("     Hex (be): 0x{}", hex_string_big_endian);

    println!("Integer (i64): {}", int64);
    println!("        (i32): {}", int32);
    println!("        (i16): {}", int16);
    println!("         (i8): {}", int8);

    println!("        (u64): {}", uint64);
    println!("        (u32): {}", uint32);
    println!("        (u16): {}", uint16);
    println!("         (u8): {}", uint8);

    println!("  Float (f32): {:.2e}", float_value);
    println!(" Double (f64): {:.2e}", double_value);
}
