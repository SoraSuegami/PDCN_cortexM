extern crate embedded_hal as hal;
use hal::{serial};
use wasmi::{MemoryRef, RuntimeValue};
use nb;
use core::marker::PhantomData;
use sp_std::fmt::Debug;
use sp_std::vec::Vec;

pub struct SerialSystem<Base,Word,B>
    where
        Base:SerialBase<Word,B>,
        Word:Into<B>+TryFrom<Vec<u8>>,
        B:AsRef<[u8]>
{
    base:Base,
    word:PhantomData<Word>,
    b:PhantomData<B>
}

impl<Base,Word,B> SerialSystem<Base,Word,B> 
    where
        Base:SerialBase<Word,B>,
        Word:Into<B>+TryFrom<Vec<u8>>,
        B:AsRef<[u8]>
{
    //interrupt
    fn read(&mut self, memory:&MemoryRef) -> Result<(Vec<RuntimeValue>,Vec<u8>),nb::Error<<Base as SerialBase<Word,B>>::Error>> {
        let word = self.base.read()?;
        let slice = word.into();
        let slice = slice.as_ref();
        Ok((Vec::new(),slice.to_vec()))
    }

    //externals
    fn write(&mut self, memory:&MemoryRef,data_ptr:u32,size:usize) -> Result<(),nb::Error<<Base as SerialBase<Word,B>>::Error>> {
        let data = memory.get(data_ptr,size).unwrap();
        let word = <Word as TryFrom<Vec<u8>>>::try_from(data).unwrap();
        self.base.write(word)?;
        Ok(())
    }
}

pub trait SerialBase<Word,B>:serial::Read<Word>+serial::Write<Word> 
    where 
        Word:Into<B>+TryFrom<Vec<u8>>,
        B:AsRef<[u8]>
{
    type Error:Debug;
    fn read(&mut self) -> nb::Result<Word, <Self as SerialBase<Word, B>>::Error>;
    fn write(&mut self, data: Word) -> nb::Result<(), <Self as SerialBase<Word, B>>::Error>;
}

pub trait TryFrom<T>:Sized {
    type Error:Debug;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
