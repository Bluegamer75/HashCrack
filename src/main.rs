use std::io::{self, Write};
use std::sync::Arc;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;
use sha2::{Sha256, Sha512, Digest}; // Include Sha512
use sha1::Sha1;
use md5::Md5;
use sha3::{Sha3_256, Sha3_384};  // For SHA-3
use indicatif::{ProgressBar, ProgressStyle};

// Function to generate hash based on the provided algorithm
fn generate_hash(plaintext: &str, algorithm: &str) -> String {
    match algorithm {
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(plaintext);
            hex::encode(hasher.finalize())
        },
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(plaintext);
            hex::encode(hasher.finalize())
        },
        "sha512" => {
            let mut hasher = Sha512::new(); // Create hasher for SHA-512
            hasher.update(plaintext);
            hex::encode(hasher.finalize())
        },
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(plaintext);
            hex::encode(hasher.finalize())
        },
        "sha3" => {
            let mut hasher = Sha3_256::new();
            hasher.update(plaintext);
            hex::encode(hasher.finalize())
        },
        "sha384" => {
            let mut hasher = Sha3_384::new();
            hasher.update(plaintext);
            hex::encode(hasher.finalize())
        },
        _ => panic!("Unsupported algorithm"),
    }
}

// Function to perform brute-force attack
fn brute_force_crack(hash_value: &str, algorithm: &str, max_length: usize, charset: &str) -> Option<String> {
    println!("\nStarting Brute Force Attack, this will take time, relax and let Hashcrack do the work...");
    println!("---------------------------------------------------------------------------------------\n");

    let charset: Vec<char> = charset.chars().collect();
    let charset = Arc::new(charset);

    for length in 1..=max_length {
        let total = charset.len().pow(length as u32);
        let pb = ProgressBar::new(total as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        let result = (0..total).into_par_iter().find_map_any(|i| {
            let mut attempt = vec![0; length];
            let mut n = i;
            for j in (0..length).rev() {
                attempt[j] = (n % charset.len()) as u8;
                n /= charset.len();
            }
            let attempt: String = attempt.iter().map(|&i| charset[i as usize]).collect();
            pb.inc(1);
            if generate_hash(&attempt, algorithm) == hash_value {
                Some(attempt)
            } else {
                None
            }
        });

        pb.finish_with_message("done");

        if let Some(password) = result {
            return Some(password);
        }
    }

    None
}

// Function for dictionary attack
fn dictionary_attack(hash_value: &str, algorithm: &str, dictionary_path: &str) -> Option<String> {
    let file = File::open(dictionary_path).expect("Failed to open dictionary file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line.expect("Error reading line").trim().to_string();

        if generate_hash(&word, algorithm) == hash_value {
            return Some(word);
        }
    }

    None
}

// Main function of the program
fn main() {
    println!("Welcome to Hashcrack 1.0.0\n");
    println!("Hashcrack is a tool for cracking hashes (md5, sha1, sha256, sha512, sha3)");
    println!("This program should only be used for pentesting purposes and with explicit permission from the data owner.\n");
    println!("These are the options to run HashCrack:\n");
    println!("      -hash (generate hash with md5, sha1, sha256, sha512, or sha3)");
    println!("      -bruteforce (try to crack the hash with all the CPU)");
    println!("      -dictionary (try to crack the hash using a dictionary)");
    println!("      -exit to finish the program\n");

    loop {
        print!("Hashcrack:~$ ");
        io::stdout().flush().unwrap();
        
        let mut command = String::new(); 
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "-exit" => {
                println!("\nFarewell, old friend; in the silence of bits, the depths of hashes await us.\n");
                break;
            },
            "-hash" => {
                print!("Enter the text to hash: ");
                io::stdout().flush().unwrap();
                let mut plaintext = String::new();
                io::stdin().read_line(&mut plaintext).unwrap();
                let plaintext = plaintext.trim();

                print!("Enter the hash algorithm (md5, sha1, sha256, sha512, or sha3): ");
                io::stdout().flush().unwrap();
                let mut algorithm = String::new();
                io::stdin().read_line(&mut algorithm).unwrap();
                let algorithm = algorithm.trim();

                let hashed_text = generate_hash(plaintext, algorithm);
                println!("Your hash => {}", hashed_text);
            },
            "-bruteforce" => {
                print!("Enter the hash algorithm (md5, sha1, sha256, sha512, or sha3): ");
                io::stdout().flush().unwrap();
                let mut algorithm = String::new();
                io::stdin().read_line(&mut algorithm).unwrap();
                let algorithm = algorithm.trim();

                print!("Enter the hash to crack: ");
                io::stdout().flush().unwrap();
                let mut hash_to_crack = String::new();
                io::stdin().read_line(&mut hash_to_crack).unwrap();
                let hash_to_crack = hash_to_crack.trim();

                print!("Enter the maximum length for brute force attack: ");
                io::stdout().flush().unwrap();
                let mut max_length = String::new();
                io::stdin().read_line(&mut max_length).unwrap();
                let max_length: usize = max_length.trim().parse().unwrap();

                // Ask for the character set to use
                println!("Select character set:");
                println!("1. Lowercase letters (a-z)");
                println!("2. Uppercase letters (A-Z)");
                println!("3. Digits (0-9)");
                println!("4. Lowercase + Uppercase");
                println!("5. Lowercase + Digits");
                println!("6. Uppercase + Digits");
                println!("7. Lowercase + Uppercase + Digits");
                println!("8. Lowercase + Special characters");
                println!("9. Uppercase + Special characters");
                println!("10. Lowercase + Uppercase + Special characters");
                println!("11. Special characters only (e.g., !@#$%^&*)");
                println!("12. Lowercase + Uppercase + Digits + Special characters (e.g., !@#$%^&*)");
                print!("Enter your choice (1-12): ");
                io::stdout().flush().unwrap();
                let mut charset_choice = String::new();
                io::stdin().read_line(&mut charset_choice).unwrap();
                let charset_choice: usize = charset_choice.trim().parse().unwrap();

                let charset = match charset_choice {
                    1 => "abcdefghijklmnopqrstuvwxyz", // Lowercase only
                    2 => "ABCDEFGHIJKLMNOPQRSTUVWXYZ", // Uppercase only
                    3 => "0123456789", // Digits only
                    4 => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", // Lowercase + Uppercase
                    5 => "abcdefghijklmnopqrstuvwxyz0123456789", // Lowercase + Digits
                    6 => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", // Uppercase + Digits
                    7 => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", // Lowercase + Uppercase + Digits
                    8 => "abcdefghijklmnopqrstuvwxyz!@#$%^&*()-_=+[]{}|;:',.<>?/", // Lowercase + Special characters
                    9 => "ABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()-_=+[]{}|;:',.<>?/", // Uppercase + Special characters
                    10 => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()-_=+[]{}|;:',.<>?/", // Lowercase + Uppercase + Special characters
                    11 => "!@#$%^&*()-_=+[]{}|;:',.<>?/", // Special characters only
                    12 => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:',.<>?/", // All characters
                    _ => panic!("Invalid choice"),
                };

                match brute_force_crack(hash_to_crack, algorithm, max_length, charset) {
                    Some(cracked) => println!("\nHash cracked :D -> The plaintext is: {}", cracked),
                    None => println!("\nFailed to crack the hash :("),
                }
            },
            "-dictionary" => {
                print!("Enter the hash algorithm (md5, sha1, sha256, sha512, or sha3): ");
                io::stdout().flush().unwrap();
                let mut algorithm = String::new();
                io::stdin().read_line(&mut algorithm).unwrap();
                let algorithm = algorithm.trim();

                print!("Enter the dictionary path: ");
                io::stdout().flush().unwrap();
                let mut dictionary_path = String::new();
                io::stdin().read_line(&mut dictionary_path).unwrap();
                let dictionary_path = dictionary_path.trim();

                print!("Enter the hash to crack: ");
                io::stdout().flush().unwrap();
                let mut hash_to_crack = String::new();
                io::stdin().read_line(&mut hash_to_crack).unwrap();
                let hash_to_crack = hash_to_crack.trim();

                match dictionary_attack(hash_to_crack, algorithm, dictionary_path) {
                    Some(password) => println!("\nHash cracked :D -> The plaintext is: {}", password),
                    None => println!("\nFailed to crack the hash :("),
                }
            },
            _ => println!("Invalid command, please try again."),
        }
    }
}



