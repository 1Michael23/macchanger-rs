use core::fmt;
use std::{num::ParseIntError, str::FromStr};
//use std::fmt::Error;

use rand::prelude::*;

#[derive(Debug)]
pub struct Mac {
    data: [u8; 6]
}

impl Mac {

    pub fn to_string(&self) -> String {
        format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5])
    }

    pub fn new() -> Mac{
        Mac { data: [0u8; 6]}
    }

    pub fn new_random() -> Mac{
        let mac: [u8; 6] = [
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
        ];
        Mac { data: mac}
    }

    pub fn new_random_with_vendor(vendor: [u8; 3]) -> Mac{
        let mac: [u8; 6] = [
            vendor[0],
            vendor[1],
            vendor[2],
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
        ];
        Mac {data: mac}
    }
}

impl fmt::Display for Mac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5])
    }
}

#[derive(Debug)]
pub enum ParseMacError {
    ParseIntError(ParseIntError),
    FormatError,
}

impl fmt::Display for ParseMacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseMacError::ParseIntError(e) => write!(f,"Failed to parse data in MAC: ({})",e),
            ParseMacError::FormatError => write!(f, "Mac has invalid format")
        }
    }
}

impl FromStr for Mac {
    type Err = ParseMacError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut out = Self::new();

        for (count, segment) in input.split(':').enumerate(){
            if count >= out.data.len(){
                return Err(ParseMacError::FormatError);
            }
            out.data[count] = u8::from_str_radix(segment, 16).map_err(|e| ParseMacError::ParseIntError(e))?;
        }
        Ok(out)
    }
}
