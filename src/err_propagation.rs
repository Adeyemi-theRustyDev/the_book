use std::fs::File;
use std::io::{self, Read};

pub fn read_username(path: &str) -> Result<String, io::Error>{
    let f = File::open(path);
    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

pub fn read_uname(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    let  _f = File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}