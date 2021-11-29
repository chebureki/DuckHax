use crate::parser;
use crate::reader;

use std::collections::HashMap;

pub struct Parser{

}

impl Parser{

}

impl parser::Parser for Parser{
    fn name(&self) -> &str {return "Bitmap";}
    fn file_extension(&self) -> &str{return "Bitmap_Z"}

    fn parse(&self, reader: &mut reader::Reader) -> Result<&dyn parser::ParserResult, reader::ReaderError>{
        println!("Read file-size: {}", reader.read_u32()?);
        Err(reader::ReaderError::EndOfBuffer)
    }
}

enum BitmapType{
    RGBA,
    RGBMonoAlpha,
    RGB,
}

pub struct ParserResult{
    uCRC1: u32,
    uCRC2: u32,
    width: u32,
    height: u32,
    u1: u32,
    byte: BitmapType,
    uint16: [u32],
}

impl ParserResult{

}

impl parser::ParserResult for ParserResult{
    fn inspect(&self)->HashMap<&str, String>{
        let mut map = HashMap::new();
        map.insert("dimensions", format!("{}x{}",self.width, self.height));
        map
    }
}
