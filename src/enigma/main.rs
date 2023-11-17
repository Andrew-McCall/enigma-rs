mod enigma_converts;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    println!("{:?}", args);

    let mut rotors = Vec::new();

    let mut message = None;

    let mut i = 0;
    while i < args.len() {
        let current_argument = &args[i];

        if current_argument == "--rotor" || current_argument == "-r" {
            if i + 3 >= args.len() {
                panic!(
                    "Missing arguments for rotor {}.\n --rotor ENCODING TURNOVER POSITION",
                    rotors.len() + 1
                );
            }

            match arg_to_rotor(&args[i + 1], &args[i + 2], &args[i + 3]) {
                Ok(rotor) => rotors.push(rotor),
                Err(e) => {
                    panic!("Error parsing rotor {}.\n{}", rotors.len() + 1, e);
                }
            }

            i += 3;
        } else if current_argument == "--message" || current_argument == "-m" {
            if message.is_some() {
                panic!("--message can only be used once.");
            }

            if i + 1 >= args.len() {
                panic!("Missing argument for message.\n --message \"MESSAGE\"");
            }

            let remaining_args = &args[(i + 1)..];

            let input = remaining_args.join(" ");
            let mut first_quote = None;
            let mut last_quote = None;
            input.char_indices().for_each(|c| {
                if c.1 == '"' {
                    if first_quote.is_none() {
                        first_quote = Some(c.0);
                    } else {
                        last_quote = Some(c.0);
                    }
                }
            });

            if first_quote.is_none() && last_quote.is_none() {
                message = Some(args[i + 1].clone());
                i += 1;
            } else if first_quote.is_none() && last_quote.is_none() {
                message = Some(input[(first_quote.unwrap() + 1)..last_quote.unwrap()].to_string());
                i += 1;
            } else {
                panic!("Invalid message format. Must be --message \"MESSAGE TEXT\"");
            }
        }

        i += 1;
    }

    println!("Rotors: {:?}", rotors);

    let enigma = Enigma { rotors };
}

fn arg_to_rotor(encoding: &String, turnover: &String, position: &String) -> Result<Rotor, String> {
    // Basic Checks
    if encoding.len() != 26 {
        return Err("Encoding must be 26 characters long.".to_string());
    }

    if turnover.len() != 1 {
        return Err("Turnover must be 1 character.".to_string());
    }

    if position.len() != 1 {
        return Err("Position must be 1 character.".to_string());
    }

    let wiring = encoding.chars().collect::<Vec<char>>();

    // Validate turnover
    let turnover = turnover.chars().next().unwrap();

    let turnover_index = match wiring.iter().position(|&c| c == turnover) {
        Some(i) => i,
        None => return Err("Turnover was not a character of the rotor.".to_string()),
    };

    // Validate position
    let position = position.chars().next().unwrap();

    let position_index = match wiring.iter().position(|&c| c == position) {
        Some(i) => i,
        None => return Err("Position was not a character of the rotor.".to_string()),
    };

    Ok(Rotor {
        wiring,
        turnover: turnover_index,
        position: position_index,
    })
}

#[derive(Debug)]
struct Rotor {
    wiring: Vec<char>,
    turnover: usize,
    position: usize,
}

struct Enigma {
    rotors: Vec<Rotor>,
}
