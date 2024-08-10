# Guessing-Game
- Simple first project in rust, put virtually all functionality in functions to preserve readability.
_________________________________________________________________________________
## Functionality

- Program prompts the user with a input asking for a number, then checks to see if it is valid. If the number enter is valid it compares it to a number generated in range from 0 to 100 using: `rng.gen_range(0..100)` from the library: `rand`. After comparing the values, tells the user if the entered value is higher, lower or the same. then either prompts the user to guess another number or asks if the user wants to play again.
