use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Assuming the argument is the second command line argument
    if args.len() > 1 {
        let arg = &args[1];

        // Try to parse as integer
        match arg.parse::<i64>() {
            Ok(i) => convert_integer(i),
            Err(_) => {
                match arg.parse::<u64>() {
                    Ok(u) => convert_unsigned_integer(u),
                    Err(_) => {
                        // Try to parse as floating-point
                        match arg.parse::<f64>() {
                            Ok(f) => convert_double(f),
                            Err(_) => {
                                // Try to parse as hexadecimal
                                if arg.starts_with("0x") || arg.starts_with("0X") {
                                    let hex_part = &arg[2..]; // Remove the "0x" prefix
                                    match i64::from_str_radix(hex_part, 16) {
                                        Ok(h) => println!("Parsed as hexadecimal: {}", h),
                                        Err(_) => println!("Failed to parse as hexadecimal."),
                                    }
                                } else if arg.ends_with("f") || arg.ends_with("F") {
                                    let float_part = &arg[..arg.len() - 1]; // Remove the "f" suffix
                                    match float_part.parse::<f32>() {
                                        Ok(f) => convert_float(f),
                                        Err(_) => println!("Failed to parse as float."),
                                    }
                                } else {
                                    println!("Argument didn't match any expected format.");
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Please provide an argument.");
    }
}

fn convert_integer(int_64: i64) {
    // Convert to byte array in little-endian format
    format_bytes(&int_64.to_le_bytes());
}

fn convert_unsigned_integer(uint_64: u64) {
    // Convert to byte array in little-endian format
    format_bytes(&uint_64.to_le_bytes());
}

fn convert_float(float_value: f32) {
    // Convert to byte array in little-endian format
    let float_bytes = float_value.to_le_bytes();
    let int32 = i32::from_le_bytes(float_bytes);
    let int64: i64 = int32 as i64; // Convert i32 to i64
    let int64_bytes = int64.to_le_bytes();
    format_bytes(&int64_bytes);
}

fn convert_double(double_value: f64) {
    // Convert to byte array in little-endian format
    let double_bytes = double_value.to_le_bytes();
    format_bytes(&double_bytes);
}

fn format_bytes(bytes: &[u8; 8]) {
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

    // Display integer, float, and double
    //println!("  Hexadecimal: 0x{:0>16x}", int_64);

    // Use iterator to reverse the byte array and map each byte to its hexadecimal representation
    let hex_string: String = bytes
        .iter()
        .rev()
        .map(|byte| format!("{:02X}", byte))
        .collect();

    // Prefix with "0x" to denote hexadecimal
    println!("  Hexadecimal: 0x{}", hex_string);

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
