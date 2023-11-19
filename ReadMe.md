# Enigma Machine

## Description
### Introduction
The enigma was an encryption machine used in and post WWII. It used physical rotors to encode and decode message. My implementation is for the "Glühlampenmaschine" and later models; Complete with plugboard, rotors, and reflector. The enigma machine was deemed unbreakable at the time due it it's complexity but it's cipher is not secure by today's standards. 

### My Implementation
I have implemented the enigma machine as a command line tool using rust and standard library (std::env). My main goal was to create a command line interface to gain experience parsing arguments and flags to configure the encoding of the message. There are no other dependencies. 

Enigma-rs will be a tool for storing and more easily setting the configuration of the enigma machine using my core library.

### Soruces
- [Wikipedia 1 - Enigma Page](https://en.wikipedia.org/wiki/Enigma_machine)
- [Wikipedia 2 - Rotors and Internals](https://en.wikipedia.org/wiki/Enigma_rotor_details)

## Using
### Example
This example will decode "JDPEN XCJJO" into Hello World. 
*This can be found in the help page.*
```bash
./enigma --rotor "EKMFLGDQVZNTOWYHXUSPAIBRCJ" V A --rotor "AJDKSIRUXBLHWTMCQGZNPYFVOE" E A --rotor "BDFHJLCPRTXVZNYEIWGAKMUSQO" Q A --reflector "YR UH QS LD PX NG OK MI EB FZ CW VJ AT" --message "JDPEN XCJJO"
OUTPUT: "HELLO WORLD"
```
### Flags
#### --rotor | -r
Use `--rotor ENCODING TURNOVER POSITION` to add and configure a rotor. 

`ENCODING` is the encoding of the rotor. This is a string of 26 characters. Each character is a letter in the alphabet. A letter can only be used once but can be used in any order.

`TURNOVER` is the letter that will cause the next rotor to turn. This is a single character.

`POSITION` is the starting position of the rotor. This is a single character.
  
*You can have as many rotors as you want.*

###### Example:
```bash
./enigma ... --rotor "EKMFLGDQVZNTOWYHXUSPAIBRCJ" D A --rotor "AJDKSIRUXBLHWTMCQGZNPYFVOE" F B --rotor "BDFHJLCPRTXVZNYEIWGAKMUSQO" E C``
```

#### --reflector | -rf
Use `--reflector "PAIRS"` to configure the reflector.

`PAIRS` represents the pairs of letters that are connected in the reflector (required for symmetrical encryption). Each pair are two letters separated by a space. Each letter can only be used once but can be used in any order.

*The reflector must be included*

###### Example:
```bash
./enigma ... --reflector "AB CD EF GH IJ KL MN OP QR ST UV WX YZ"
```

#### --message | -m
Use `--message "MESSAGE"` to set the message to be encoded.

`MESSAGE` is the message to be encoded. This is a string of characters. Any non-alphabetic characters will be ignored and passed through.

*The message must be included*

###### Example:
```bash
./enigma ... --message "HELLO WORLD"
```

#### --plugboard | -pb
Use `--plugboard "PAIRS"` to configure the plugboard.

`PAIRS` represents the pairs of letters that are connected in the plugboard. Each pair are two letters separated by a space. Each letter can only be used once but can be used in any order. You can have as many pairs as you want.

*The plugboard is optional*

###### Example:
```bash
./enigma ... --plugboard "AB CD EF GH IJ KL MN OP QR ST UV WX YZ"
```

#### --help | -h
Use `--help` to print the help page.

*All other arguments will be ignored*

###### Example:
```bash
./enigma --help
```

#### --state | -s
Use `--state` to print the finial state of the enigma machine after encoding the message.

*This is optional*

###### Example:
```bash
./enigma ... --state
```

## Errors
Able to parse any number of rotors. Each turnover and position must be valid also.
```bash
$ ./enigma wooooop -rotor abcdefghijklmnopqrstuvwxyz a y -rotor abc a a
["$", "wooooop", "-rotor", "abcdefghijklmnopqrstuvwxyz", "a", "y", "-rotor", "abc", "a", "a"]
Error parsing rotor 2.
Encoding must be 26 characters long.
```


## Building
### Installation 
Clone the repo, build and run. (Cargo required)
```bash
$ git clone https://GitHub.com/Andrew-McCall/enigma-rs.git
$ cd enigma-rs
$ cargo build --release
```

### Cargo Build
Use `cargo build --release` to build the binary. 
```bash
$ ./enigma [flags]
```
The binary will be located in `target/release/enigma`.
### Cargo Run
Use `cargo run --release -- [flags]` to build and run the binary with one command.