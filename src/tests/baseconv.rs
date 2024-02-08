use crate::base_converter::BaseConverter;

// The module where our tests will reside
#[cfg(test)]
mod tests {
    // Bring the outer scope (Rectangle struct) into the scope of this module
    use super::*;
    // Import the BaseConverter struct
    // Define a test case
    #[test]
    fn converter_test1() {
        // Create an instance of BaseConverter
        let mut converter = BaseConverter::default();

        // Assert that the area method returns the expected result
        converter.parse_and_convert("0x3f9df3b6");

        assert_eq!(converter.float_value, 1.234);
    }

    use parameterized::parameterized;

    #[parameterized(input = {
        "0x3f9df3b6", "0xc0add2f2", "0xc59c4800"
    }, expected = {
        1.234, -5.432, -5001.0
    })]
    fn test_convert_to_float(input: &str, expected: f32) {
        let mut converter = BaseConverter::default();

        // Assert that the area method returns the expected result
        converter.parse_and_convert(&input);

        assert_eq!(converter.float_value, expected);
    }
}
