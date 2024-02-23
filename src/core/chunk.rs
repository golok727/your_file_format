use std::mem::size_of;

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum ChunkDataType {
    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    F32,
    F64,

    String(usize),
}

impl ChunkDataType {
    pub fn get_size(&self) -> usize {
        match self {
            Self::U8 => size_of::<u8>(),
            Self::U16 => size_of::<u16>(),
            Self::U32 => size_of::<u32>(),
            Self::U64 => size_of::<u64>(),

            Self::I8 => size_of::<i8>(),
            Self::I16 => size_of::<i16>(),
            Self::I32 => size_of::<i32>(),
            Self::I64 => size_of::<i64>(),

            Self::F32 => size_of::<f32>(),
            Self::F64 => size_of::<f64>(),

            Self::String(len) => *len,
        }
    }
    pub fn get_flag(&self) -> u8 {
        match self {
            Self::U8 => b'b',
            Self::U16 => b'B',
            Self::U32 => b'u',
            Self::U64 => b'U',

            Self::I8 => b'i',
            Self::I16 => b'I',
            Self::I32 => b'n',
            Self::I64 => b'N',

            Self::F32 => b'f',
            Self::F64 => b'F',

            Self::String(_) => b'S',
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub flavor: ChunkDataType,
    pub chunk: Vec<u8>,
}

pub trait CreateChunk {
    fn create_chunk(&self) -> ChunkData;
}

impl CreateChunk for u8 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::U8,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for u16 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::U16,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for u32 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::U32,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for u64 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::U64,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for i8 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::I8,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for i16 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::I16,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for i32 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::I32,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for i64 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::I64,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for f32 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::F32,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for f64 {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::F64,
            chunk: self.to_le_bytes().to_vec(),
        }
    }
}

impl CreateChunk for String {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::String(self.len()),
            chunk: self.as_bytes().to_vec(),
        }
    }
}

impl CreateChunk for str {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::String(self.len()),
            chunk: self.as_bytes().to_vec(),
        }
    }
}

impl CreateChunk for bool {
    fn create_chunk(&self) -> ChunkData {
        ChunkData {
            flavor: ChunkDataType::U8,
            chunk: vec![*self as u8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_chunk_sizes() {
        // Uint
        assert_eq!(
            (10 as u8).create_chunk().flavor.get_size(),
            ChunkDataType::U8.get_size()
        );
        assert_eq!(
            (10 as u16).create_chunk().flavor.get_size(),
            ChunkDataType::U16.get_size()
        );
        assert_eq!(
            (10 as u32).create_chunk().flavor.get_size(),
            ChunkDataType::U32.get_size()
        );
        assert_eq!(
            (10 as u64).create_chunk().flavor.get_size(),
            ChunkDataType::U64.get_size()
        );

        // Int

        assert_eq!(
            (10 as i8).create_chunk().flavor.get_size(),
            ChunkDataType::I8.get_size()
        );
        assert_eq!(
            (10 as i16).create_chunk().flavor.get_size(),
            ChunkDataType::I16.get_size()
        );
        assert_eq!(
            (10 as i32).create_chunk().flavor.get_size(),
            ChunkDataType::I32.get_size()
        );
        assert_eq!(
            (10 as i64).create_chunk().flavor.get_size(),
            ChunkDataType::I64.get_size()
        );
    }

    #[test]
    fn string_size() {
        let str = "Radha";
        assert_eq!(str.create_chunk().flavor.get_size(), str.len());

        let str = String::from("Krsna");
        assert_eq!(str.create_chunk().flavor.get_size(), str.len());
    }
}
