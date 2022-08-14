use crate::rdiff::{io::RdiffFile, constants::BLOCK_SIZE};

use super::{BufferedRdiffChunkIterator, RdiffChunkIterator};



#[test]
fn test_rdiff_chunk_iterator_buffered_new_case1() {
   let file_name = "resources/buffered_rdiff_chunk_iterator_file1.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   let iterator = BufferedRdiffChunkIterator::new(rdiff_file);
}

#[test]
fn test_rdiff_chunk_iterator_buffered_chunk_size_case1() {
   let file_name = "resources/buffered_rdiff_chunk_iterator_file1.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   assert_eq!(rdiff_file.size(), 3);
   let iterator = BufferedRdiffChunkIterator::new(rdiff_file);
   assert_eq!(iterator.get_chunk_size(), 2);
}

#[test]
fn test_rdiff_chunk_iterator_buffered_chunk_size_case2() {
   let file_name = "resources/buffered_rdiff_chunk_iterator_file2.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   assert!(rdiff_file.size() > BLOCK_SIZE);
   let iterator = BufferedRdiffChunkIterator::new(rdiff_file);
   assert_eq!(iterator.get_chunk_size(), BLOCK_SIZE);
}

#[test]
fn test_rdiff_chunk_iterator_buffered_get_rdiff_chunk_case1() {
   let file_name = "resources/buffered_rdiff_chunk_iterator_file1.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   let mut iterator = BufferedRdiffChunkIterator::new(rdiff_file);
   let mut number_of_chunks = 0;
   loop {
      let result = iterator.next_chunk().unwrap();
      match result {
         Some(rdiff_chunk) => {
            number_of_chunks += 1;
            println!("rdiff_chunk {:?}", rdiff_chunk)
         },
         None => break,
      }
   }
   println!("number_of_chunks {}", number_of_chunks);
   assert!(number_of_chunks >= 2);
}

#[test]
fn test_rdiff_chunk_iterator_buffered_get_rdiff_chunk_case2() {
   let file_name = "resources/buffered_rdiff_chunk_iterator_file2.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   let mut iterator = BufferedRdiffChunkIterator::new(rdiff_file);
   let mut number_of_chunks = 0;
   loop {
     let result = iterator.next_chunk().unwrap();
      match result {
         Some(rdiff_chunk) => {
            number_of_chunks += 1;
            println!("rdiff_chunk {:?}", rdiff_chunk)
         },
         None => break,
      }
   }
   println!("number_of_chunks {}", number_of_chunks);
   assert!(number_of_chunks >= 2);
}

#[test]
fn test_rdiff_chunk_iterator_buffered_get_rdiff_chunk_case3() {
   let file_name = "resources/test.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   let mut iterator = BufferedRdiffChunkIterator::new(rdiff_file);
   let mut number_of_chunks = 0;
   loop {
     let result = iterator.next_chunk().unwrap();
      match result {
         Some(rdiff_chunk) => {
            number_of_chunks += 1;
            println!("rdiff_chunk {:?}", rdiff_chunk)
         },
         None => break,
      }
   }
   println!("number_of_chunks {}", number_of_chunks);
   assert!(number_of_chunks >= 2);
}

#[test]
fn test_rdiff_chunk_iterator_buffered_get_rdiff_chunk_case4() {
   let file_name = "resources/poem.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   let mut iterator = BufferedRdiffChunkIterator::new(rdiff_file);
   let mut number_of_chunks = 0;
   loop {
      let result = iterator.next_chunk().unwrap();
      match result {
         Some(rdiff_chunk) => {
            number_of_chunks += 1;
            println!("rdiff_chunk {:?}", rdiff_chunk)
         },
         None => break,
      }
   }
   println!("number_of_chunks {}", number_of_chunks);
   assert!(number_of_chunks >= 2);
}

#[test]
fn test_rdiff_chunk_iterator_buffered_new_with_chunk_size_case1() {
   let file_name = "resources/buffered_rdiff_chunk_iterator_file1.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   let chunk_size:usize = 16;
   let iterator = 
      BufferedRdiffChunkIterator::new_with_chunk_size(chunk_size, rdiff_file).unwrap();

   let rdiff_file = RdiffFile::new(file_name).unwrap();   
   let buffer:Vec<u8> = Vec::new();
   let expected_iterator = 
      BufferedRdiffChunkIterator{chunk_size, buffer, rdiff_file};
   assert_eq!(iterator, expected_iterator);
}

#[test]
fn test_rdiff_chunk_iterator_buffered_with_chunk_size_next_chunk_case1() {
   let file_name = "resources/buffered_rdiff_chunk_iterator_file1.txt";
   let rdiff_file = RdiffFile::new(file_name).unwrap();
   let chunk_size:usize = 2;
   let mut iterator = 
      BufferedRdiffChunkIterator::new_with_chunk_size(chunk_size, rdiff_file).unwrap();
   let mut number_of_chunks = 0;
   loop {
      let result = iterator.next_chunk().unwrap();
      match result {
         Some(rdiff_chunk) => {
            number_of_chunks += 1;
            println!("rdiff_chunk {:?}", rdiff_chunk)
         },
         None => break,
      }
   }
   println!("number_of_chunks {}", number_of_chunks);
}