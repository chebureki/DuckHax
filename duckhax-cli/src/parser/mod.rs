use crate::reader;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! import_parsers  {
    ($($parser: ident),*) => {
        $(mod $parser;)*
        const PARSERS : [&dyn Parser; count!($($parser)*)] = [
            $(&$parser::Parser{},)*
        ];
    };
}

//add parsers here!
import_parsers!(
    bitmap
);

pub trait Parser{
    fn name(&self) -> &str;
    fn file_extension(&self) -> &str; 

    //TODO: read from a byte buffer!
    fn parse(&self, reader: & mut reader::Reader) -> Result<Box<dyn ParserResult>, reader::ReaderError>;
}

pub trait ParserResult{
    fn inspect(&self) -> HashMap<&str, String>;
    //fn dump() //write to file buffer
}

fn detect_from_file_extension(extension: &str) -> Option<&dyn Parser>{
    for parser in PARSERS {
        if parser.file_extension() == extension{
            return Some(parser);
        }    
    }
    None
}

fn file_extension(path: &str) -> Option<&str>{
    Path::new(path).extension().and_then(OsStr::to_str)
}

fn probe_file_crc(path: &str) -> Result<Option<&dyn Parser>, std::io::Error>{
    let mut reader = reader::from_file_path(path)?;
    let crc = reader.read_u32();

    Ok(None)//TODO: CRC lookup
}

pub fn auto_detect_file(path_to_file: &str) -> Result<&dyn Parser, &str>{
    let path = Path::new(path_to_file);
    if !path.exists(){
        return Err("could not open file");
    }
    match path.extension().and_then(OsStr::to_str).and_then(detect_from_file_extension){
        Some(parser) => {return Ok(parser);}
        _ => {}
    }

    match probe_file_crc(path_to_file){
        Ok(Some(parser)) => {Ok(parser)}
        Err(err) => {Err("could not probe file")}
        _ => {Err("no parser found")}
    }
} 