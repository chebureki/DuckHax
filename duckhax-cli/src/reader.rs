use std::fs::File;
use std::io::Read;

/*
    a simple little-endian byte buffer reader for easy reading and error handling
*/

pub struct Reader{
    pos: usize,
    buf: Vec<u8>
}

pub fn from(buf: Vec<u8>)->Reader{
    Reader{
        pos: 0,
        buf,
    }
}

#[derive(Debug, Clone)]
pub enum ReaderError{
    EndOfBuffer,
}

macro_rules! valid_pos {
    ($self:expr, $pos:expr) => {
        if $pos >= $self.buf.len(){
            return Err(ReaderError::EndOfBuffer);
        }
    }
}

macro_rules! enough_bytes {
    ($self:expr, $bytes:expr) => {
        valid_pos!($self, $self.pos + $bytes);
    };
}

macro_rules! u8_lshifted{
    ($self:expr, $pos:expr) =>{
        ($self.buf[$self.pos+$pos] as u32) << ($pos * 8)
    }
}

impl Reader{
    pub fn seek(&mut self, pos: usize) -> Result<(), ReaderError>{
        valid_pos!(self, pos);
        self.pos = pos;
        Ok(())
    }

    pub fn seek_fw(&mut self, to_seek: usize) -> Result<(), ReaderError>{
        enough_bytes!(self, to_seek);
        self.pos += to_seek;
        Ok(())
    }

    pub fn read_u8(&mut self) -> Result<u8,ReaderError>{
        enough_bytes!(self, 1);
        let val = Ok(
            (
                u8_lshifted!(self, 0)
            ) as u8
        );
        self.pos += 1;
        val
    }

    pub fn read_u16(&mut self) -> Result<u16,ReaderError>{
        enough_bytes!(self, 2);
        let val = Ok(
            (
                u8_lshifted!(self, 0) |
                u8_lshifted!(self, 1)
            ) as u16
        );
        self.pos += 2;
        val
    }

    pub fn read_u32(&mut self) -> Result<u32,ReaderError>{
        enough_bytes!(self, 4);
        let val = Ok(
            (
                u8_lshifted!(self, 0) |
                u8_lshifted!(self, 1) |
                u8_lshifted!(self, 2) |
                u8_lshifted!(self, 3)
            ) as u32
        );
        self.pos += 4;
        val
    } 
}

pub fn from_file(file: &mut File)->Result<Reader,std::io::Error>{
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(
        Reader{
            pos: 0,
            buf,
        }
    )
}
 
pub fn from_file_path(file_path: &str)->Result<Reader,std::io::Error>{
    let mut file = File::open(file_path)?;
    from_file(&mut file)
}
