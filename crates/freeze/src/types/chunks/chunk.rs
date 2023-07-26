use crate::types::FileError;
use crate::types::FileOutput;

use super::binary_chunk::BinaryChunk;
use super::chunk_ops::ChunkData;
use super::number_chunk::NumberChunk;

/// block chunk
pub type BlockChunk = NumberChunk;

/// transaction chunk
pub type TransactionChunk = BinaryChunk;

/// address chunk
pub type AddressChunk = BinaryChunk;

/// Chunk of data
#[derive(Debug, Clone)]
pub enum Chunk {
    /// block chunk
    Block(BlockChunk),

    /// transaction chunk
    Transaction(TransactionChunk),

    /// address chunk chunk
    Address(AddressChunk),
}

/// Chunk methods
impl Chunk {
    /// get filepath for chunk
    pub fn filepath(&self, name: &str, file_output: &FileOutput) -> Result<String, FileError> {
        match self {
            Chunk::Block(chunk) => chunk.filepath(name, file_output),
            Chunk::Transaction(chunk) => chunk.filepath(name, file_output),
            Chunk::Address(chunk) => chunk.filepath(name, file_output),
        }
    }
}

impl From<Vec<Chunk>> for Chunk {
    fn from(chunks: Vec<Chunk>) -> Self {
        match chunks.len() {
            0 => panic!("invalid empty chunk range"),
            1 => chunks.into_iter().next().unwrap(),
            _ => todo!("not implemented yet"),
        }
    }
}