extern crate failure;
use failure::Fail;
use core::str;
/*
#[derive(Debug,Fail)]
pub enum SerialError<R,W,Word>
where
    R:serial::Read<Word>,
    W:serial::Write<Word>
{
    #[fail(display = "fail to read the serial")]
    ReadSerialError {
        error:nb::Error<R::Error>
    },
}

*/