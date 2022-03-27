plummaes version 0.1.2 2022/03/20

use AES256(no iv) with key xor to encrypt/decrypt file

usage:
- plummaes generate <keyfile>
- plummaes encrypt <input> <output> <keyfile>
- plummaes decrypt <input> <output> <keyfile>

Note : keyfile must be 64 bytes or more

version:
- v0.1.0 : AES256(no iv) + key xor 
- v0.1.1 : Add Gzip Compress and File Header "Plumm 1.1"
- v0.1.2 : Allow encrypt input to be Plain Text (When File Not Found)
- v0.1.3 : Allow keyfile to be over 64 bytes (only first 64 bytes are used)