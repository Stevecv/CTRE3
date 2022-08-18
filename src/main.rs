mod run;

use std::fs;

fn main() {
    let file_path = "C:\\Users\\jacpe\\Temporary\\code.itm";

    let code = fs::read(file_path)
        .expect("Should have been able to read the file");

    run::run_code(code);
}