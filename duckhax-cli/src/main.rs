mod parser;
mod reader;

fn main() {
    let path = "testing/1.Bitmap_Z";
    let parser: &dyn parser::Parser = match parser::auto_detect_file(path){
        Ok(parser) => {parser}
        Err(err) => {panic!("{}", err)}
    };
    println!("Using parser: {}", parser.name());

    let mut reader = match reader::from_file_path(path){
        Ok(reader) => {reader}
        Err(err) => {panic!("{}", err);}
    };
    let parse_result = match parser.parse(&mut reader){
        Ok(res) => {res}
        Err(err) => {panic!("{:?}", err)}
    };
    let inspection = parse_result.inspect();
    for kv in inspection{
        let (k,v) = kv;
        println!("{}: {}", k, v);
    }
    //let inspection = //parse_result

}
