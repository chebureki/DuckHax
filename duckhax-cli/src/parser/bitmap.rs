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

    fn parse(&self, reader: &mut reader::Reader) -> Result<Box<dyn parser::ParserResult>, reader::ReaderError>{
        reader.seek_fw(4)?; //skip file size
        reader.seek_fw(4)?; //skip file magic //TODO: have some sort of check
        reader.seek_fw(4)?; //skip file crc//TODO: consider this!

        let crc1 = reader.read_u32()?;
        let width = reader.read_u32()?;
        let height = reader.read_u32()?;
        let u1 = reader.read_u32()?;
        let bitmap_type = match bitmap_type_from_u32(reader.read_u8()?){
            Some(t) => {t}
            None => {panic!("unsupported bitmap type");}
        };
        let u2 = reader.read_u32()?;
        let mut pixels: Vec<Pixel> = Vec::with_capacity((width*height) as usize);
        let parse_pixel = match bitmap_type{
            //TODO: document this, it looks like genuine magic
            BitmapType::RGB => |p: u16| Pixel{r: (((p&0xf800)>>11) * 8)as u8, g: (((p&(0x07C0))>>6) * 8)as u8, b: (((p&0x001F)>>0) * 8)as u8, a:255},
            BitmapType::RGBA => |p: u16| Pixel{r: (((p&0x0f00)>>8)*16)as u8, g: (((p&0x00f0)>>4)*16)as u8, b: (((p&0x000f)>>0)*16)as u8, a:(((p&0xf000)>>12)*16) as u8},
            BitmapType::RGBMonoAlpha => |p: u16| Pixel{r: (((p&0x7C00)>>10)*8)as u8, g: (((p&0x03E0)>>5)*8)as u8, b: (((p&0x001F)>>0)*8)as u8, a: (((p&0x8000)>>15)*255)as u8},
            _ => {panic!("missing pixel parser")}
        };

        for _ in 0..(width*height){
            pixels.push(parse_pixel(reader.read_u16()?));
        }

        Ok(
            Box::new(
                ParserResult{crc1,width,height,u1,bitmap_type,pixels}
            )
        )
    }
}

#[derive(Debug)]
enum BitmapType{
    RGBA,
    RGBMonoAlpha,
    RGB,
}

pub struct Pixel{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct ParserResult{
    crc1: u32,
    width: u32,
    height: u32,
    u1: u32,
    bitmap_type: BitmapType,
    pixels: Vec<Pixel>,
}

fn bitmap_type_from_u32(t: u8) -> Option<BitmapType> {
    match t {
        0xA => Some(BitmapType::RGBA),
        0x7 => Some(BitmapType::RGBMonoAlpha),
        0x8 => Some(BitmapType::RGB),
        _ => {None}
    }
}

impl ParserResult{

}

impl parser::ParserResult for ParserResult{
    fn inspect(&self)->HashMap<&str, String>{
        let mut map = HashMap::new();
        map.insert("dimensions", format!("{}x{}",self.width, self.height));
        map.insert("type", format!("{:?}",self.bitmap_type));
        map
    }
}
