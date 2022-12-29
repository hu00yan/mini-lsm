mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes};
pub use iterator::BlockIterator;

const SIZEOF_U16: usize = std::mem::size_of::<u16>();

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted
/// key-value pairs.
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl Block {
    pub fn encode(&self) -> Bytes {
        let mut buf = self.data.clone();
        let offsets_len = self.offsets.len();
        for offset in &self.offsets {
            buf.put_u16(*offset);
        }
        buf.put_u16(offsets_len as u16);
        buf.into()
    }

    pub fn decode(data: &[u8]) -> Self {
        let entry_num = (&data[data.len() - SIZEOF_U16..]).get_u16() as usize;
        let offsets_start = data.len() - SIZEOF_U16 - 2 * entry_num;
        let offsets_raw = &data[offsets_start..data.len() - SIZEOF_U16];
        let offsets = offsets_raw
            .chunks(SIZEOF_U16)
            .map(|mut x| x.get_u16())
            .collect();
        let data = data[0..offsets_start].to_vec();
        Self { data, offsets }
    }
}

#[cfg(test)]
mod tests;
