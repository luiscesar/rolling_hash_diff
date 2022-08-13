# Rolling Hash Diff

## Description
Make a rolling hash based file diffing algorithm. When comparing original and an updated version of an input, it should return a description ("delta") which can be used to upgrade an original version of the file into the new file. The description contains the chunks which:

Can be reused from the original file
have been added or modified and thus would need to be synchronized
The real-world use case for this type of construct could be a distributed file storage system. This reduces the need for bandwidth and storage. If many people have the same file stored on Dropbox, for example, there's no need to upload it again.

A library that does a similar thing is rdiff. You don't need to fulfill the patch part of the API, only signature and delta.

## Requirements
Hashing function gets the data as a parameter. Separate possible filesystem operations.
Chunk size can be fixed or dynamic, but must be split to at least two chunks on any sufficiently sized data.
Should be able to recognize changes between chunks. Only the exact differing locations should be added to the delta.
Well-written unit tests function well in describing the operation, no UI necessary.

## Execution
### Build project
cargo build -r 
### Setup Environment
export PATH=./target/release:$PATH
### Execute command
#### Signature
rolling_hash_diff signature <file_name> <signature_file_name>

#### Delta
rolling_hash_diff delta <signature_file_name> <new_file_name> <delta_file_name> 
