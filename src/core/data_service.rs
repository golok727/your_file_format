use crate::error::crab_error::Error;

pub struct Serializer {
    buffer: Vec<u8>,
}

impl Serializer {
    pub fn put_str(&mut self, _val: &str) -> &mut Self {
        dbg!(&self.buffer);
        self
    }
    pub fn put_string(&mut self, _val: String) -> &mut Self {
        self
    }
    pub fn put_u8(&mut self, _val: u8) -> &mut Self {
        self
    }
    pub fn put_u16(&mut self, _val: u16) -> &mut Self {
        self
    }

    pub fn put_u32(&mut self, _val: u32) -> &mut Self {
        self
    }

    pub fn put_u64(&mut self, _val: u64) -> &mut Self {
        self
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self { buffer: Vec::new() }
    }
}

pub struct Deserializer {
    raw: Vec<u8>,
}

impl Deserializer {
    pub fn get_str(&mut self, _val: &str) -> Result<&str, Error> {
        dbg!(&self.raw);
        todo!()
    }
    pub fn get_string(&mut self, _val: String) -> Result<String, Error> {
        todo!()
    }
    pub fn get_u8(&mut self, _val: u8) -> Result<u8, Error> {
        todo!()
    }
    pub fn get_u16(&mut self, _val: u16) -> Result<u16, Error> {
        todo!()
    }

    pub fn get_u32(&mut self, _val: u32) -> Result<u32, Error> {
        todo!()
    }

    pub fn get_u64(&mut self, _val: u64) -> Result<u64, Error> {
        todo!()
    }
}

impl From<Vec<u8>> for Deserializer {
    fn from(raw: Vec<u8>) -> Self {
        Self { raw }
    }
}
