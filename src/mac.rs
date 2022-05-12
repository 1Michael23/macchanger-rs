use core::str::FromStr;
use owo_colors::OwoColorize;
use rand::prelude::*;

use crate::SET_LOCAL_BIT;

#[derive(Debug)]
pub struct Mac {
    pub data: [u8; 6],
}

impl Mac {
    pub fn to_string(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5]
        )
    }

    pub fn new() -> Mac {
        Mac { data: [0u8; 6] }
    }

    pub fn new_random() -> Mac {
        let mac: [u8; 6] = [
            Mac::rand_octet(true),
            Mac::rand_octet(false),
            Mac::rand_octet(false),
            Mac::rand_octet(false),
            Mac::rand_octet(false),
            Mac::rand_octet(false),
        ];
        Mac { data: mac }
    }

    pub fn rand_octet(first: bool) -> u8 {
        //First boolean indicates whether the least significant bits will be flipped
        let array: [bool; 8];
        if first {
            array = [
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                SET_LOCAL_BIT,
                false,
            ];

            if !SET_LOCAL_BIT {
                println!(
                    "{} Global bit generated Mac is set, This Mac address may cause problems.",
                    "Warn:".yellow()
                );
            }
        } else {
            array = [
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
            ];
        }

        Mac::octet_from_bool_arr(array)
    }

    fn octet_from_bool_arr(array: [bool; 8]) -> u8 {
        array
            .iter()
            .fold(0, |result, &bit| (result << 1) ^ bit as u8)
    }

    pub fn bool_array_from_octet(octet: u8) -> [bool; 8] {
        let mut result: [bool; 8] = [false; 8];

        for (i, bool) in result.iter_mut().enumerate().take(7) {
            *bool = octet & (1 << i) != 0;
        }

        result
    }
}

impl FromStr for Mac {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut out = Self::new();

        for (count, segment) in input.split(':').enumerate() {
            if count >= out.data.len() {
                return Err("Invalid number of segments".to_string());
            }
            match u8::from_str_radix(segment, 16) {
                Ok(data) => {
                    if count == 0 {
                        if Mac::bool_array_from_octet(data)[0] {
                            return Err("Multicast bit is set".to_string());
                        } else if !Mac::bool_array_from_octet(data)[1] {
                            if !SET_LOCAL_BIT {
                                println!(
                                    "{} Global bit is set, This Mac address may cause problems.",
                                    "Warn:".yellow()
                                );
                                out.data[count] = data;
                            } else {
                                return Err("Global bit is set".to_string());
                            }
                        } else {
                            out.data[count] = data;
                        }
                    } else {
                        out.data[count] = data;
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(out)
    }
}
