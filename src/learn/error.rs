use std::{
    error, fmt, fs,
    io::{self, BufRead},
    string,
};

#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Utf8(string::FromUtf8Error),
    General(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            MyError::General(s) => write!(f, "General error: {}", s),
        }
    }
}

impl error::Error for MyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Utf8(e) => Some(e),
            MyError::General(_) => None,
        }
    }
}

impl From<io::Error> for MyError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<string::FromUtf8Error> for MyError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

pub fn first_line(filename: &str) -> Result<String, MyError> {
    let file = fs::File::open(filename)?;
    let mut reader = io::BufReader::new(file);

    let mut buf = vec![];
    let len = reader.read_until(b'\n', &mut buf)?;
    let result = String::from_utf8(buf)?;

    if result.len() > 1 {
        return Err(MyError::General(format!("Line too long: {}", len)));
    }

    Ok(result)
}
