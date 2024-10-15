use std::io::{self, Write};
use std::sync::Arc;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;
use sha2::{Sha256, Digest};
use md5::Md5;
use indicatif::{ProgressBar, ProgressStyle};

// Función para generar hash basado en el algoritmo proporcionado
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
        _ => panic!("Unsupported algorithm"),
    }
}

// Función para realizar ataque de fuerza bruta
fn brute_force_crack(hash_value: &str, algorithm: &str, max_length: usize) -> Option<String> {
    println!("\nStarting Brute Force Attack, this will take time, relax and let Hashcrack do the work...");
    println!("---------------------------------------------------------------------------------------\n");

    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()".chars().collect();
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

// Dictionary attack function
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

// Main function in the program
fn main() {
    println!("Welcome to Hashcrack 0.9.5");
    println!("");
    println!("Hashcrack is a tool for cracking hashes (md5 and sha256) using brute-force or dictionary attacks.");
    println!("This program should only be used for pentesting purposes and with explicit permission from the data owner.");
    println!("");
    println!("These are the options to run HashCrack:");
    println!("");
    println!("      -hash (generate hash with md5 or sha256)");
    println!("      -bruteforce (try to crack the hash with all the CPU)");
    println!("      -dictionary (try to crack the hash using a dictionary)");
    println!("      -exit to finish the program");
    println!("");

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

                print!("Enter the hash algorithm (md5 or sha256): ");
                io::stdout().flush().unwrap();
                let mut algorithm = String::new();
                io::stdin().read_line(&mut algorithm).unwrap();
                let algorithm = algorithm.trim();

                let hashed_text = generate_hash(plaintext, algorithm);
                println!("Your hash => {}", hashed_text);
            },
            "-bruteforce" => {
                print!("Enter the hash algorithm (md5 or sha256): ");
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

                match brute_force_crack(hash_to_crack, algorithm, max_length) {
                    Some(cracked) => println!("\nHash cracked :D -> The plaintext is: {}", cracked),
                    None => println!("\nFailed to crack the hash :("),
                }
            },
            "-dictionary" => {
                print!("Enter the hash algorithm (md5 or sha256): ");
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
                    Some(cracked) => println!("\nHash cracked :D -> The plaintext is: {}", cracked),
                    None => println!("\nFailed to crack the hash with the dictionary :("),
                }
            },
            _ => println!("Invalid command. Available commands: -exit, -hash, -bruteforce, -dictionary"),
        }
    }
}

