# How I achieved this

The first thing I did was to import the `io` module from the standard library. This module allows me to read user input from the console.

Next, I defined a `main` function, which is the entry point of my program. I printed a message to the console to let the user know that my program is a temperature converter.

To allow the user to convert multiple temperatures, I used a `loop` statement. This statement allows me to repeatedly ask the user for a temperature to convert until they choose to exit the program.

Within the loop, I first asked the user to input a temperature value. To ensure that the user entered a valid number, I parsed the input as a floating-point number using the `parse()` method. If the input was not a valid number, I continued the loop to ask the user for another temperature.

I then asked the user to select the input unit of the temperature ("C" for Celsius or "F" for Fahrenheit). I used a match statement to determine which conversion formula to use based on the input unit.

Next, I applied the appropriate conversion formula to the temperature value to get the converted temperature. I then printed the result to the console using string formatting.

Finally, I asked the user if they wanted to convert another temperature. If the answer was "N" or "n", I exited the loop and ended the program.

## Note

You will have to install Rust🦀 to run this code.
