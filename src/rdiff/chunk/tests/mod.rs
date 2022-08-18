use std::collections::HashMap;

use super::{RdiffChecksum, RdiffChunkDigest, RdiffChunkTable};

#[test]
fn test_rdiff_chunk_table_new_case1() {
    let rdiff_chunk_table = RdiffChunkTable::new();
    let chunk_table: HashMap<RdiffChecksum, Vec<RdiffChunkDigest>> = HashMap::new();
    let expected = RdiffChunkTable { chunk_table };
    assert_eq!(rdiff_chunk_table, expected);
}
