use std::fmt;

pub struct Hit {
    data: String,
    file: String,
    line_number: usize,
}

impl Hit {
    pub fn new(data: String, file: String, line_number: usize) -> Self {
        Hit {
            data,
            file,
            line_number,
        }
    }
}

impl fmt::Display for Hit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}\t{}", self.file, self.line_number, self.data)?;
        Ok(())
    }
}
