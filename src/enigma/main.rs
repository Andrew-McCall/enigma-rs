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

            reflector = Some(Reflector::from_args(&args[i + 1]));

            i += 1;
        } else if current_argument == "--plugboard" || current_argument == "-pb" {
            if i + 1 >= args.len() {
                panic!("Missing argument for plugboard.\n --plugboard \"PAIRS\"");
            }

            plugboard = Some(Plugboard::from_args(&args[i + 1]));
        } else if current_argument == "--help" || current_argument == "-h" {
            println!("Andrew McCall's Enigma Machine (HELP)\n\n--help (-h) | Shows all options\n--message (-m) | Sets the message to encode | --message \"PLAINTEXT MESSAGE\"\n--rotor (-r) | Adds a rotor | --rotor ENCODING TURNOVER POSITION\n--reflector (-rf) | Sets the reflector | --reflector \"PAIRS\"\n--plugboard (-pb) | Sets the plugboard | --plugboard \"PAIRS\"\n\nPAIRS is a list of letter pairs. For example, \"AB CD EF\" would map A to B, C to D, and E to F and vice versa.\nENCODING is a representation of the rotor's internal shifting. For example, \"DMTWSILRUYQNKFEJCAZBPGXOHV\" would cause A to become D and B to become M after passing through the rotor.\nTURNOVER is a letter of the rotor which when reached will cause the next rotor (left to right) to increment once. e.g \"A\".\nPOSITION is the current position of the rotor. This is based of it's encoding. For example, K would represent the rotor being one position forward if the example encoding is being used.\n\nEXAMPLE USAGE: ./enigma --rotor \"EKMFLGDQVZNTOWYHXUSPAIBRCJ\" V A --rotor \"AJDKSIRUXBLHWTMCQGZNPYFVOE\" E A --rotor \"BDFHJLCPRTXVZNYEIWGAKMUSQO\" Q A --reflector \"YR UH QS LD PX NG OK MI EB FZ CW VJ AT\" --message \"JDPEN XCJJO\"\nOUTPUT: \"HELLO WORLD\"");
            return;
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
