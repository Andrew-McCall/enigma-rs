use enigma::Enigma;
use enigma_int::ToEnigmaInt;
use reflector::Reflector;
use rotor::Rotor;

mod enigma;
mod enigma_int;
mod reflector;
mod rotor;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut rotors = Vec::new();

    let mut message = None;

    let mut i = 1;
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

            message = Some(args[i + 1].to_string());
            i += 1
        } else {
            panic!("Unknown argument: {}", current_argument);
        }

        i += 1;
    }

    if rotors.len() == 0 {
        panic!("No rotors specified.\n --rotor ENCODING TURNOVER POSITION");
    }

    if message.is_none() {
        panic!("No message specified.\n --message \"MESSAGE\"");
    }

    let enigma = Enigma {
        rotors,
        reflector: Reflector::default(),
    };

    println!("Encoded: {}", enigma.encode(&message.unwrap()));
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

    let wiring = encoding
        .chars()
        .map(|c| c.to_internal_int())
        .collect::<Vec<usize>>();

    // Validate turnover
    let turnover = turnover.chars().next().unwrap().to_internal_int();

    let turnover_index = match wiring.iter().position(|&c| c == turnover) {
        Some(i) => i,
        None => return Err("Turnover was not a character of the rotor.".to_string()),
    };

    // Validate position
    let position = position.chars().next().unwrap().to_internal_int();

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
