use rand::Rng;
use std::io::Read;
use std::fs::File;

pub(crate) fn read_key_file(file: &str) -> Vec<u8> {
  // match std::fs::read(file) {
  //     Ok(bytes) if bytes.len() >= 64 => bytes,
  //     Ok(_) => panic!("Key File Length Error"),
  //     Err(_) => panic!("Error reading key file")
  // }
  match File::options().read(true).open(file) {
      Ok(mut file) => {
          if file.metadata().unwrap().len() < 64 {
              panic!("Key File Length Error");
          }

          let mut buffer = [0u8; 64];
          // read exactly 10 bytes
          file.read_exact(&mut buffer).unwrap();
          buffer.to_vec()
      },
      Err(_) => panic!("Error reading key file")
  }
}

pub(crate) fn generate(file: &str) {
    let mut key = [0u8; 64];
    let mut rng = rand::thread_rng();
    for item in &mut key {
        *item = rng.gen();
    }
    std::fs::write(file, key).unwrap();
}