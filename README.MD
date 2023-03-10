# Guess the Number Game in Rust - My Learning Project

I am currently learning Rust and worked on this program that allows a user to guess a randomly generated number between 1 and 100. The program will indicate if the user's guess is too small, too big, or equal to the randomly generated number.

## How it Works

- The program generates a random number between 1 and 100 using the rand crate.
- A loop is initiated that will continue until the user correctly guesses the randomly generated number.
Inside the loop, the user is prompted to enter their guess.
- The user's guess is read in from standard input and parsed into an unsigned 32-bit integer.
- If the user's input cannot be parsed into an unsigned 32-bit integer, the loop continues to the next iteration.
- The user's guess is then compared to the randomly generated number using the cmp method from the std::cmp module.
- If the user's guess is less than the randomly generated number, the program will print "Too small!" and prompt the user to enter another guess.
- If the user's guess is greater than the randomly generated number, the program will print "Too big!" and prompt the user to enter another guess.
- If the user's guess is equal to the randomly generated number, the program will print "You win!" and exit the loop.
