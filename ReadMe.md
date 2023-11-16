# Enigma Machine Simulation

> Enigma-Gui to come
## Using
```bash
enigma -rotor [encoding] [turnover] [position]
```
- You can have as many rotors as you want.
- The encoding must be 26 characters long.
- The turnover and position must be a single character which are included in the encoding.

## Errors
Able to parse any number of rotors. Each turnover and position must be valid also.
```bash
$ ./enigma wooooop -rotor abcdefghijklmnopqrstuvwxyz a y -rotor abc a a
["$", "wooooop", "-rotor", "abcdefghijklmnopqrstuvwxyz", "a", "y", "-rotor", "abc", "a", "a"]
Error parsing rotor 2.
Encoding must be 26 characters long.
``````