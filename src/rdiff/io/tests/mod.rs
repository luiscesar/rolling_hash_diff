use crate::rdiff::{io::RdiffFile, constants::BLOCK_SIZE};

#[test]
fn test_rdiff_file_read_block_case1() {
    let filename = "resources/poem.txt";
    let mut rdiff_file = RdiffFile::new(filename).unwrap();
    let (size,block) = rdiff_file.read_block().unwrap().unwrap();
    assert_eq!(size,BLOCK_SIZE);
}

#[test]
fn test_rdiff_file_read_block_case2() {
    let filename = "resources/poem.txt";
    let mut rdiff_file = RdiffFile::new(filename).unwrap();
    let mut count:usize = 0;
    while let Some(x) = rdiff_file.read_block().unwrap() {
        let (size, block) = x;
        count += 1;
        assert!((size == BLOCK_SIZE) || 
            (size == (rdiff_file.size() as usize) - (count - 1) * BLOCK_SIZE));
    }   
}