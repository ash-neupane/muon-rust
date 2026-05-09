
use std::io;
use std::num::ParseIntError;

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::Io(e)
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

enum MyError{
    Io(io::Error),
    Parse(ParseIntError),
}


fn read_file(fname: &String) -> Result<String, io::Error> {
    // stub: pretend we read the file
    Ok(String::from(fname))
}

fn parse(_ch: &String) -> Result<i32, ParseIntError> {
    // do stuff
    let x: i32 = 5;
    Ok(x)
}

fn read_and_parse(fname: &String) -> Result<i32, MyError> {
    let s: String = read_file(fname)?;
    let n: i32 = parse(&s)?;
    Ok(n)
}

fn main() {
    let v: Vec<i32> = vec![10, 20, 30];
    // println!("{}", v[5]);
    // let val: Option<&i32> = v.get(5);
    match v.get(2) { 
        Some(x) => println!("{}", x),
        None => println!("vec was empty")

    }
    // println!("{:?}", val);
    let fname: String = String::from("hello");
    match read_and_parse(&fname) {
        Ok(v) => println!("{}", v),
        Err(MyError::Io(e)) => println!("{}", e),
        Err(MyError::Parse(e)) => println!("{}", e)
    }
}