sha1-cracker
A command-line tool for cracking SHA-1 hashed passwords using a wordlist.

Usage
To use the tool, run the following command:

Copy code
shar1-cracker <wordlist.txt> <sha1_hash>
The wordlist.txt file should contain a list of words, one per line, that will be used to attempt to crack the hash. The sha1_hash argument is the SHA-1 hash that you want to crack.

Example
Here is an example of using the tool to crack a SHA-1 hash:

Copy code
shar1-cracker wordlist.txt b7a875fc1ea228bcef32e5fbc5ba0c1b1f6c70eb
If the tool is successful, it will print the plaintext password to the console. If it is not successful, it will print a message indicating that the password could not be found in the wordlist.

Dependencies
This tool requires the Rust programming language to be installed on your system. You can install Rust by following the instructions on the official Rust website.

Code Explanation
The main function of the program starts by using the env::args function to retrieve a list of command-line arguments passed to the program. These arguments are then collected into a Vec<String> and stored in the args variable.

Next, the program checks the length of the args vector. If it is not equal to 3 (i.e. the program name, the wordlist file, and the hash), it prints a usage message to the console and returns. This is to ensure that the correct number of arguments have been passed to the program.

If the correct number of arguments are present, the program can proceed with attempting to crack the hash using the provided wordlist. However, this functionality is not implemented in the code snippet provided.
