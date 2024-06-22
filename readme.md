## Overview

**Project Title**: Encryptor/Decryptor

**Project Description**: Followed tutorial to create a basic encryptor/decryptor

**Project Goals**: To understand some rust while having fun making random characters in a .txt file

## Instructions for Build and Use

Steps to build and/or run the software:

1. Make a new cargo file on your computer and copy and paste my main.rs program over to your main.rs file 
2. Make sure to type into the terminal Cargo build to build it
3. Then type cargo run to get it initialized (im not sure if its necessary but its smart to make sure)


Instructions for using the software:

1. You will need the hello.txt file in the folder
2. If you want to read the text file type "cat hello.txt" into the terminal
3. For encryption: in the terminal type cargo run hello.txt true
4. For decryption: in terminal type cargo run hello.txt false
5. You cna re-read the text file in step 2 if you want to see the encryption/decryption working

## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Using rust, so there needs to be rust all set up;
* I used the standard rust library as well as io: "use std::{fs, env, io::Write};"
* I got the rust-analyzer extension as well
* I also have Crates and Even Better TOML (Im not sure if those are necessary, the setup tutorials said I needed them but not sure why)

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
* *Tutorial I Followed* [How to make a simple FILE ENCRYPTOR (and a decryptor) in RUST](https://www.youtube.com/watch?v=3mSEVIGOqzY)
* *These tutorials helped me understand the basics* [Rust Programming Tutorial](https://www.youtube.com/playlist?list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] Make it more personalized with user input to see if they want to encrypt/decrypt
* [ ] Make the byte-shifting more random so it is a little more efficient as an encryptor 
* [ ] Make actual encryption/decryption software not just a byte-shifter
