plummaes version 1.1.0 2022/03/20

use AES256(no iv) with key xor to encrypt/decrypt file

usage:
- plummaes generate <keyfile>
- plummaes encrypt <input> <output> <keyfile>
- plummaes decrypt <input> <output> <keyfile>

Note : keyfile must be 64 bytes

version:
- v1.0 : AES256(no iv) + key xor 
- v1.1 : Add Gzip Compress and File Header "Plumm 1.1"
