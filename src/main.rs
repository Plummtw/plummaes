use std::env;
use rand::Rng;
use aes::Aes256;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

fn read_key_file(file: &str) -> Vec<u8> {
    match std::fs::read(file) {
        Ok(bytes) if bytes.len() == 32 => bytes,
        Ok(_) => panic!("Key File Length Error"),
        Err(_) => panic!("Error reading key file")
    }
}

fn encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    let key = GenericArray::from_slice(key);

    let mut input_copy = Vec::from(input);
    if  input.len() % 16 != 0 {
        input_copy.resize(input.len() + (16 - input.len() % 16), 0);
    }
    let mut blocks = input_copy.chunks(16).map(|x| 
        GenericArray::clone_from_slice(x)).collect::<Vec<_>>();

    let cipher = Aes256::new(&key);
    cipher.encrypt_blocks(&mut blocks);
    let len_blocks = input.len().to_be_bytes();
    let mut ret = blocks.iter().map(|x| x.iter().cloned().collect::<Vec<_>>()).flatten().collect::<Vec<_>>();
    let _ = ret.splice(0..0, len_blocks);
    ret
}

fn encrypt_file(input: &str, output: &str, key: &[u8]) {
    match std::fs::read(input) {
        Ok(bytes) => {
            let result : Vec<u8> = encrypt(&bytes, key);
            let result : Vec<u8> = result.iter().enumerate().map(|(i, &byte)|
                 byte ^ key[i%key.len()]).collect();
            std::fs::write(output, result).unwrap();
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                panic!("please run again with appropriate permissions.");
            }
            panic!("{}", e);
        }
    }
}

fn decrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut length : usize = 0;
    for i in 0..8 {
        length <<= 8;
        length |= input[i] as usize;
    }
    let key = GenericArray::from_slice(key);
    let mut input_copy = Vec::from(&input[8..]);
    if  input.len() % 16 != 0 {
        input_copy.resize(input.len() + (16 - input.len() % 16), 0);
    }
    let mut blocks = input_copy.chunks(16).map(|x| 
        GenericArray::clone_from_slice(x)).collect::<Vec<_>>();

    let cipher = Aes256::new(&key);
    cipher.decrypt_blocks(&mut blocks);
    let mut ret = blocks.iter().map(|x| x.iter().cloned().collect::<Vec<_>>()).flatten().collect::<Vec<_>>();
    ret.truncate(length);
    ret
}

fn decrypt_file(input: &str, output: &str, key: &[u8]) {
    match std::fs::read(input) {
        Ok(bytes) => {
            let result : Vec<u8> = bytes.iter().enumerate().map(|(i, &byte)|
                byte ^ key[i%key.len()]).collect();            
            let result : Vec<u8> = decrypt(&result, key);
            std::fs::write(output, result).unwrap();
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                panic!("please run again with appropriate permissions.");
            }
            panic!("{}", e);
        }
    }
}

fn show_usage() {
    eprintln!("plummaes version 1.0.0 2022/03/20");
    eprintln!("usage: ");
    eprintln!("    plummaes generate <keyfile>");
    eprintln!("    plummaes encrypt <input> <output> <keyfile>");
    eprintln!("    plummaes decrypt <input> <output> <keyfile>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 5 {
        show_usage();
        return;    
    } else {
        if args.len() == 5 && args[1].to_lowercase() == "encrypt" {
            encrypt_file(&args[2], &args[3], &read_key_file(&args[4]));
        } else if args.len() == 5 && args[1].to_lowercase() == "decrypt" {
            decrypt_file(&args[2], &args[3], &read_key_file(&args[4]));
        } else if args.len() == 3 && args[1].to_lowercase() == "generate" {
            let mut key = [0u8; 32];
            let mut rng = rand::thread_rng();
            for i in 0..key.len() {
                key[i] = rng.gen();
            }
            std::fs::write(&args[2], key).unwrap();
        } else {
            show_usage();
        }
    }
}
