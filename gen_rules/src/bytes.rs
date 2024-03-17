use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

pub trait ReadByteArraysExt: io::Read {
    fn read_u8_array<const COUNT: usize>(&mut self) -> anyhow::Result<[u8; COUNT]> {
        let mut arr = [0; COUNT];
        self.read_exact(&mut arr)?;
        Ok(arr)
    }

    fn read_u16_array<const COUNT: usize>(&mut self) -> anyhow::Result<[u16; COUNT]> {
        let mut arr = [0; COUNT];
        self.read_u16_into::<LittleEndian>(&mut arr)?;
        Ok(arr)
    }

    fn read_u32_array<const COUNT: usize>(&mut self) -> anyhow::Result<[u32; COUNT]> {
        let mut arr = [0; COUNT];
        self.read_u32_into::<LittleEndian>(&mut arr)?;
        Ok(arr)
    }

    fn read_i16_array<const COUNT: usize>(&mut self) -> anyhow::Result<[i16; COUNT]> {
        let mut arr = [0; COUNT];
        self.read_i16_into::<LittleEndian>(&mut arr)?;
        Ok(arr)
    }

    fn read_i32_array<const COUNT: usize>(&mut self) -> anyhow::Result<[i32; COUNT]> {
        let mut arr = [0; COUNT];
        self.read_i32_into::<LittleEndian>(&mut arr)?;
        Ok(arr)
    }

    fn read_array<T, const COUNT: usize>(&mut self) -> anyhow::Result<[T; COUNT]>
    where
        T: ByteReadable + Default + Copy,
        Self: Sized,
    {
        let mut arr = [T::default(); COUNT];
        for elem in arr.iter_mut() {
            *elem = T::read(self)?;
        }
        Ok(arr)
    }
}

impl<R: ReadBytesExt> ReadByteArraysExt for R {}

pub trait ByteReadable: Sized {
    fn read(reader: &mut impl ReadBytesExt) -> anyhow::Result<Self>;
}
