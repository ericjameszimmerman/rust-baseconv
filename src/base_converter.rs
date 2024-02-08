// Define the BaseConverter struct
pub struct BaseConverter {
    pub int64: i64,
    pub int32: i32,
    pub int16: i16,
    pub int8: i8,
    pub uint64: u64,
    pub uint32: u32,
    pub uint16: u16,
    pub uint8: u8,
    pub float_value: f32,
    pub double_value: f64,
    pub little_endian_bytes: [u8; 8],
    pub little_endian_hex_string: String,
    pub big_endian_bytes: [u8; 8],
    pub big_endian_hex_string: String,
}

impl Default for BaseConverter {
    fn default() -> BaseConverter {
        BaseConverter {
            int64: 0,
            int32: 0,
            int16: 0,
            int8: 0,
            uint64: 0,
            uint32: 0,
            uint16: 0,
            uint8: 0,
            float_value: 0.0,
            double_value: 0.0,
            little_endian_bytes: Default::default(),
            little_endian_hex_string: Default::default(),
            big_endian_bytes: Default::default(),
            big_endian_hex_string: Default::default(),
        }
    }
}

// Implement methods for BaseConverter
impl BaseConverter {
    // Method to calculate the area of the rectangle
    pub fn parse_and_convert(&mut self, arg: &str) -> () {
        use regex::Regex; // Add the import statement here

        if let Ok(i) = arg.parse::<i64>() {
            self.convert_and_print_bytes(&i.to_le_bytes());
        } else if let Ok(u) = arg.parse::<u64>() {
            self.convert_and_print_bytes(&u.to_le_bytes());
        } else if let Ok(f) = arg.parse::<f64>() {
            self.convert_and_print_bytes(&f.to_le_bytes());
        } else if arg.ends_with('f') || arg.ends_with('F') {
            self.parse_float(arg);
        } else {
            let pattern = r"^(0x|<0x|>0x|0X|<0X|>0X|>|<)?.*?[hH]?$";
            let re = Regex::new(pattern).unwrap();
            if re.is_match(arg) {
                self.parse_hex(arg);
            } else {
                println!("Argument didn't match any expected format.");
            }
        }
    }

    fn convert_and_print_float(&mut self, float_value: f32) {
        // Convert to byte array in little-endian format
        let float_bytes = float_value.to_le_bytes();
        let int32 = i32::from_le_bytes(float_bytes);
        let int64: i64 = int32 as i64; // Convert i32 to i64
        let int64_bytes = int64.to_le_bytes();
        self.convert_and_print_bytes(&int64_bytes);
    }

    fn hex_to_byte_array(&self, hex_str: &str, is_big_endian: bool) -> [u8; 8] {
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

    fn parse_float(&mut self, arg: &str) {
        let float_arg = &arg[..arg.len() - 1]; // Remove the 'f' suffix
        match float_arg.parse::<f32>() {
            Ok(f) => self.convert_and_print_float(f),
            Err(_) => println!("Failed to parse as float."),
        }
    }

    fn parse_hex(&mut self, arg: &str) {
        let (big_endian, hex_value) = self.parse_hex_string(arg);
        let bytes = self.hex_to_byte_array(&hex_value, big_endian);
        self.convert_and_print_bytes(&bytes);
    }

    fn parse_hex_string(&mut self, input: &str) -> (bool, String) {
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

    fn convert_and_print_bytes(&mut self, bytes: &[u8; 8]) {
        // Convert to various integer types
        self.int64 = i64::from_le_bytes(bytes[0..8].try_into().unwrap());
        self.int32 = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
        self.int16 = i16::from_le_bytes(bytes[0..2].try_into().unwrap());
        self.int8 = bytes[0] as i8; // Direct cast, since i8 and u8 have the same memory representation

        self.uint64 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        self.uint32 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        self.uint16 = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        self.uint8 = bytes[0]; // Direct use, as u8 is the type of bytes' elements

        // Convert to floating-point types
        self.float_value = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
        self.double_value = f64::from_le_bytes(bytes[0..8].try_into().unwrap());

        self.little_endian_bytes = self.uint64.to_le_bytes();
        self.little_endian_hex_string = self
            .little_endian_bytes
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect();
        self.big_endian_bytes = self.uint64.to_be_bytes();
        self.big_endian_hex_string = self
            .big_endian_bytes
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect();
    }

    pub fn print_values(&self) {
        // Prefix with "0x" to denote hexadecimal
        println!("     Hex (le): 0x{}", self.little_endian_hex_string);
        println!("     Hex (be): 0x{}", self.big_endian_hex_string);

        println!("Integer (i64): {}", self.int64);
        println!("        (i32): {}", self.int32);
        println!("        (i16): {}", self.int16);
        println!("         (i8): {}", self.int8);

        println!("        (u64): {}", self.uint64);
        println!("        (u32): {}", self.uint32);
        println!("        (u16): {}", self.uint16);
        println!("         (u8): {}", self.uint8);

        println!("  Float (f32): {:.2e}", self.float_value);
        println!(" Double (f64): {:.2e}", self.double_value);
    }
}
