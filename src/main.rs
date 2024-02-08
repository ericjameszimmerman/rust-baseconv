mod base_converter;
use std::env;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let arg = &args[1];
        let mut converter = base_converter::BaseConverter::default();
        converter.parse_and_convert(arg);
        converter.print_values();
    } else {
        println!("Please provide an argument.");
    }
}
