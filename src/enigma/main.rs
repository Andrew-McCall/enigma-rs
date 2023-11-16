fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    println!("{:?}", args);

    let mut rotors = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let current_argument = &args[i];

        if current_argument == "-rotor" || current_argument == "-r" {
            if i + 3 >= args.len() {
                println!(
                    "Missing arguments for rotor {}.\n -rotor ENCODING TURNOVER POSITION",
                    rotors.len() + 1
                );
                return;
            }

            match arg_to_rotor(&args[i + 1], &args[i + 2], &args[i + 3]) {
                Ok(rotor) => rotors.push(rotor),
                Err(e) => {
                    println!("Error parsing rotor {}.\n{}", rotors.len() + 1, e);
                    return;
                }
            }

            i += 3;
        }

        i += 1;
    }

    println!("Rotors: {:?}", rotors);
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
