use super::chunk::{ChunkData, CreateChunk};

pub struct BluePrint {
    pub id: &'static str,
    chunks: Vec<ChunkData>,
}

impl BluePrint {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            chunks: Vec::new(),
        }
    }

    pub fn add<T: CreateChunk>(&mut self, chunk: T) {
        self.chunks.push(chunk.create_chunk())
    }
}
