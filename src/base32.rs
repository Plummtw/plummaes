use rand::Rng;

pub(crate) fn encrypt_base32(input: &[u8]) -> [u8;5] {
  let mut result = [0u8; 5];
  let mut rng = rand::thread_rng();

  let input_copy = input.iter().map(|x| {
    // 0-25 for A-Z
    // 26-31 for other
    let c = *x as char;
    match c {
      'a'..='z' => (c as u8) - ('a' as u8),
      'A'..='Z' => (c as u8) - ('A' as u8),
      _ => rng.gen::<u8>() % 6 + 26,
    }
  }).collect::<Vec<_>>();

  // 5 3
  // 2 5 1
  // 4 4
  // 1 5 2
  // 3 5

  result[0] = input_copy[0] << 3 | input_copy[1] >> 2;
  result[1] = (input_copy[1] & 0b11) << 6 | input_copy[2] << 1 | input_copy[3] >> 4;
  result[2] = (input_copy[3] & 0b1111) << 4 | input_copy[4] >> 1;
  result[3] = (input_copy[4] & 0b1) << 7 | input_copy[5] << 2 | input_copy[6] >> 3;
  result[4] = (input_copy[6] & 0b111) << 5 | input_copy[7];

  return result;
}

pub(crate) fn decrypt_base32(input: &[u8]) -> [u8;8] {
  let mut result = [0u8; 8];

  // 0 0: 11111000
  // 1 0: 00000111 1:11000000
  // 2 1: 00111110
  // 3 1: 00000001 2:11110000
  // 4 2: 00001111 3:10000000
  // 5 3: 01111100
  // 6 3: 00000011 4:11100000
  // 7 4: 00011111

  result[0] = input[0] >> 3;
  result[1] = (input[0] & 0b111) << 2 | input[1] >> 6;
  result[2] = (input[1] & 0b111110) >> 1;
  result[3] = (input[1] & 0b1) << 4 | input[2] >> 4;
  result[4] = (input[2] & 0b1111) << 1 | input[3] >> 7;
  result[5] = (input[3] & 0b1111100) >> 2;
  result[6] = (input[3] & 0b11) << 3 | input[4] >> 5;
  result[7] = input[4] & 0b11111;

  for i in 0..8 {
    let c = match result[i] {
      n @ 0..=25 => n + ('a' as u8),
      _ => ' ' as u8,
    };
    result[i] = c;
  }

  return result;
}