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

SHA-1 (Secure Hash Algorithm 1) is a cryptographic hash function that takes an input (or 'message') and produces a fixed-size output (or 'hash') that is unique to that input. The output of a SHA-1 hash is typically represented as a hexadecimal string of 40 characters.

SHA-1 is considered to be a secure hashing algorithm, but it has been deprecated in favor of stronger algorithms such as SHA-2 and SHA-3 due to theoretical vulnerabilities that have been discovered. Despite this, it is still widely used in various applications.

The main properties of SHA-1 that make it useful as a hash function are:

It is easy to compute the hash of a message.
It is infeasible to generate the same hash output from two different input messages (collision resistance).
It is infeasible to modify a message without changing the hash output (integrity).
It is infeasible to determine the original message from the hash output alone (hiding).
SHA-1 is commonly used to verify the integrity of a file or message, as well as to create secure passwords. When a user creates a password, the password is hashed with SHA-1 (or another hashing algorithm) and the resulting hash is stored in the system. When the user tries to log in, their password is hashed and compared to the stored hash. If the hashes match, the password is considered to be correct. This ensures that the plaintext password is never stored in the system, making it more secure against potential attackers.

There are several hash algorithms that are widely used today, including:

SHA-2: This is a family of cryptographic hash functions that includes SHA-224, SHA-256, SHA-384, and SHA-512. It was developed as a successor to SHA-1 and is considered to be more secure.

SHA-3: This is a family of cryptographic hash functions that includes SHA3-224, SHA3-256, SHA3-384, and SHA3-512. It was developed as a successor to SHA-2 and is considered to be even more secure.

MD5 (Message-Digest Algorithm 5): This is a widely used cryptographic hash function that produces a 128-bit hash value. It is no longer considered to be secure due to the discovery of collision attacks, but it is still used in some applications for backward compatibility.

BLAKE2: This is a cryptographic hash function that is faster and more secure than SHA-2 and MD5. It is available in a variety of output sizes, including 256-bit and 512-bit.

Argon2: This is a password-hashing function that was designed to be resistant to GPU cracking attacks. It has won the Password Hashing Competition and is widely used in various applications.

It is important to note that the security of a hash function depends on the size of the output, as well as the strength of the underlying algorithm. In general, hash functions with larger output sizes (e.g. SHA-2, SHA-3, and BLAKE2) are considered to be more secure than those with smaller output sizes (e.g. MD5 and SHA-1). It is always a good idea to use the strongest and most up-to-date hash function available for your application.
