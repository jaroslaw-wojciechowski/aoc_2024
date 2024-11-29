mod utils;

use utils::read_lines;

const FILE_NAME: &str = "src/inputs/input-example.txt";

fn main() {
    parse_file(FILE_NAME);
}

fn parse_file(file_name: &str) {
    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            println!("{line}");
        }
    }
}
