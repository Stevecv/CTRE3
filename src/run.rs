pub fn run_code(code: Vec<u8>) {
    //String builder stuff
    let mut str:String = "".to_string();
    let mut length = 0;

    let mut parameters = Vec::new();

    let mut environment = "o";
    let mut op_code = 0;
    for dec in code {
        /*
        o = search for opcode
        p = parameter collector
        s = string builder
        d = default (looks for a new job)
         */
        if environment == "o" {
            op_code = dec;
            environment = "p";
        } else if environment == "p" {
            if dec == 2 {
                environment = "s";
                str = "".to_string();
            }
        } else if environment == "s" {
            let len = str.len();


            if len == 0 {
                length = dec+1;
            } else if len == length as usize {
                let mut chars = str.chars();
                chars.next();
                parameters.push(chars.as_str().to_string());

                environment = "d";
            }

            str = str.to_owned().to_string() + &*(dec as char).to_string();

        } else if environment == "d" {
            if dec == 31 {
                run_opcode(op_code, &parameters)

                //reset
            }
        }
    }
}


pub fn run_opcode(op_code: u8, parameters: &Vec<String>) {
    match op_code {
        128 => println!("{}", parameters.join("")),

        _ => {println!("A unexpected error has occurred {}", op_code)},
    }
}