use enigma::Enigma;
use reflector::Reflector;
use rotor::Rotor;

use crate::plugboard::Plugboard;

mod enigma;
mod enigma_int;
mod plugboard;
mod reflector;
mod rotor;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        println!("Usage: enigma --rotor ENCODING TURNOVER POSITION --message \"MESSAGE\"");
        return;
    }

    let mut rotors = Vec::new();
    let mut reflector = None;
    let mut plugboard = None;

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

            rotors.push(Rotor::from_args(&args[i + 1], &args[i + 2], &args[i + 3]));

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
        } else if current_argument == "--reflector" || current_argument == "-rf" {
            if i + 1 >= args.len() {
                panic!("Missing argument for reflector.\n --reflector \"PAIRS\"");
            }

            if args[i + 1].to_uppercase() == "DEFAULT" {
                reflector = Some(Reflector::default());
            } else {
                reflector = Some(Reflector::from_args(&args[i + 1]));
            }

            i += 1;
        } else if current_argument == "--plugboard" || current_argument == "-pb" {
            if i + 1 >= args.len() {
                panic!("Missing argument for plugboard.\n --plugboard \"PAIRS\"");
            }

            plugboard = Some(Plugboard::from_args(&args[i + 1]));
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

    if reflector.is_none() {
        panic!("No reflector specified.\n --reflector \"PAIRS\"")
    }
    let reflector = reflector.unwrap();

    let enigma = Enigma {
        rotors,
        reflector,
        plugboard,
    };

    println!("{}", enigma.encode(&message.unwrap()));
}
