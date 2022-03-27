use std::env;
use std::io::Read;
use std::io::Write;
use aes::cipher::{
    generic_array::GenericArray,
};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
mod base32;
mod key;
mod crypt;

fn encrypt_file(input: &str, output: &str, key: &[u8]) {
    let input_bytes = match std::fs::read(input) {
        Ok(bytes) => {
           bytes.clone()
        },
        Err(_) => {
            input.to_owned().into_bytes()
        }
    };
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    let _ = e.write_all(&input_bytes).unwrap();
    let buf = e.finish().unwrap();

    let bytes = buf.clone();

    let key_first = &key[0..32];
    let key_second = &key[32..64];

    let result : Vec<u8> = crypt::encrypt(&bytes, key_first);
    let mut result : Vec<u8> = result.iter().enumerate().map(|(i, &byte)|
        if i < 16 { byte }
        else {
            byte ^ key_second[i%key_second.len()]}).collect();

    let mut header_block = [0u8; 8];
    header_block[0..8].copy_from_slice(&"Plumm1.1".as_bytes()[0..8]);

    let _ = result.splice(0..8, header_block);
    std::fs::write(output, result).unwrap();
}

fn decrypt_file(input: &str, output: &str, key: &[u8]) {
    match std::fs::read(input) {
        Ok(bytes) => {
            // Check Header
            if String::from_utf8(bytes[0..8].to_vec()).unwrap() != "Plumm1.1" {
                panic!("Invalid Header");
            }

            let key_first = &key[0..32];
            let key_second = &key[32..64];

            let result : Vec<u8> = bytes.iter().enumerate().map(|(i, &byte)|
                if i < 16 { byte }
                else {
                    byte ^ key_second[i%key_second.len()]}).collect();
            let result : Vec<u8> = crypt::decrypt(&result, key_first);

            let mut decoder = GzDecoder::new(result.as_slice());
            let mut buf : Vec<u8> = Vec::new();
            match decoder.read_to_end(&mut buf) {
                Ok(_) => {
                    std::fs::write(output, buf).unwrap();
                },
                Err(_) => {
                    std::fs::write(output, result).unwrap();
                }
            };
            
            // std::fs::write(output, buf).unwrap();
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                panic!("please run again with appropriate permissions.");
            }
            panic!("{}", e);
        }
    }
}

fn enbase32_file(input: &str, output: &str, key: &[u8]) {
    let mut input_bytes = match std::fs::read(input) {
        Ok(bytes) => {
           bytes.clone()
        },
        Err(_) => {
            input.to_owned().into_bytes()
        }
    };

    if input_bytes.len() % 8 != 0 {
        input_bytes.resize(input.len() + (8 - input_bytes.len() % 8), 0);
    }

    let bytes = input_bytes.chunks(8).map(|chunk| {
        return base32::encrypt_base32(chunk);
    }).flatten().collect::<Vec<_>>();

    let key_first = &key[0..32];
    let key_second = &key[32..64];

    let result : Vec<u8> = crypt::encrypt(&bytes, key_first);
    let mut result : Vec<u8> = result.iter().enumerate().map(|(i, &byte)|
        if i < 16 { byte }
        else {
            byte ^ key_second[i%key_second.len()]}).collect();

    let mut header_block = [0u8; 8];
    header_block[0..8].copy_from_slice(&"Plumm1.4".as_bytes()[0..8]);

    let _ = result.splice(0..8, header_block);
    std::fs::write(output, result).unwrap();
}

fn debase32_file(input: &str, output: &str, key: &[u8]) {
    match std::fs::read(input) {
        Ok(bytes) => {
            // Check Header
            if String::from_utf8(bytes[0..8].to_vec()).unwrap() != "Plumm1.4" {
                panic!("Invalid Header");
            }

            let key_first = &key[0..32];
            let key_second = &key[32..64];

            let result : Vec<u8> = bytes.iter().enumerate().map(|(i, &byte)|
                if i < 16 { byte }
                else {
                    byte ^ key_second[i%key_second.len()]}).collect();
            let result : Vec<u8> = crypt::decrypt(&result, key_first);

            let result : Vec<u8> = result.chunks(5).map(|chunk| {
                return base32::decrypt_base32(chunk);
            }).flatten().collect::<Vec<_>>();

            std::fs::write(output, result).unwrap();

            // let mut decoder = GzDecoder::new(result.as_slice());
            // let mut buf : Vec<u8> = Vec::new();
            // match decoder.read_to_end(&mut buf) {
            //     Ok(_) => {
            //         std::fs::write(output, buf).unwrap();
            //     },
            //     Err(_) => {
            //         std::fs::write(output, result).unwrap();
            //     }
            // };
            
            // std::fs::write(output, buf).unwrap();
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
    eprintln!("plummaes version 0.1.4 2022/03/27");
    eprintln!("usage: ");
    eprintln!("    plummaes generate <keyfile>");
    eprintln!("    plummaes encrypt <input> <output> <keyfile>");
    eprintln!("    plummaes decrypt <input> <output> <keyfile>");
    eprintln!("    plummaes enbase32 <input> <output> <keyfile>");    
    eprintln!("    plummaes debase32 <input> <output> <keyfile>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 5 {
        show_usage();
        return;    
    } else {
        if args.len() == 5 && args[1].to_lowercase() == "encrypt" {
            encrypt_file(&args[2], &args[3], &key::read_key_file(&args[4]));
        } else if args.len() == 5 && args[1].to_lowercase() == "decrypt" {
            decrypt_file(&args[2], &args[3], &key::read_key_file(&args[4]));
        } else if args.len() == 5 && args[1].to_lowercase() == "enbase32" {
            enbase32_file(&args[2], &args[3], &key::read_key_file(&args[4]));
        } else if args.len() == 5 && args[1].to_lowercase() == "debase32" {
            debase32_file(&args[2], &args[3], &key::read_key_file(&args[4]));
        } else if args.len() == 3 && args[1].to_lowercase() == "generate" {
            key::generate(&args[2]);
        } else {
            show_usage();
        }
    }
}
