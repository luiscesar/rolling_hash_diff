# Rolling Hash Diff

## Description
A rolling hash based file diffing algorithm. When comparing original and an updated version of an input, it should return a description ("delta") which can be used to upgrade an original version of the file into the new file.

A library that does a similar thing is rdiff. The patch part of the API has not been implemented.

## Requirements
Hashing function gets the data as a parameter. Separate possible filesystem operations.
Chunk size can be fixed or dynamic, but must be split to at least two chunks on any sufficiently sized data.
Should be able to recognize changes between chunks. Only the exact differing locations should be added to the delta.

## Verification
### Test Cases
* Signature generation
* Delta generation: Identical files
* Delta generation: Chunk removed
* Delta generation: Chunk changed
* Delta generation: Chunk shifted
* Delta generation: Additional data added between chunks

## Execution
### Build Project
cargo build -r 
### Setup Environment
export PATH=./target/release:$PATH
### Execute Command
#### Signature
rolling_hash_diff signature <file_name> <signature_file_name>

#### Delta
rolling_hash_diff delta <signature_file_name> <new_file_name> <delta_file_name> 
