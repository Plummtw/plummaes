use aes::cipher::BlockDecrypt;
use aes::cipher::BlockEncrypt;
use aes::cipher::KeyInit;
use aes::Aes256;
use crate::GenericArray;

pub(crate) fn encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
  let key = GenericArray::from_slice(key);

  let mut input_copy = Vec::from(input);
  if  input.len() % 16 != 0 {
      input_copy.resize(input.len() + (16 - input.len() % 16), 0);
  }
  let mut blocks = input_copy.chunks(16).map(|x| 
      GenericArray::clone_from_slice(x)).collect::<Vec<_>>();

  let cipher = Aes256::new(&key);
  cipher.encrypt_blocks(&mut blocks);
  let mut header_block = [0u8; 16];
  let len_blocks = input.len().to_be_bytes();
  let mut ret = blocks.iter().map(|x| x.iter().cloned().collect::<Vec<_>>()).flatten().collect::<Vec<_>>();

  header_block[0..8].copy_from_slice(&"Plumm1.1".as_bytes()[0..8]);
  header_block[8..16].copy_from_slice(&len_blocks[0..8]);
  let _ = ret.splice(0..0, header_block);
  ret
}

pub(crate) fn decrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
  let mut length : usize = 0;
  for i in 8..16 {
      length <<= 8;
      length |= input[i] as usize;
  }
  let key = GenericArray::from_slice(key);
  let mut input_copy = Vec::from(&input[16..]);
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