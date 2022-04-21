use std::collections::HashSet;

use thiserror::Error;

pub const PROTOCOL_NAME_MIN_LENGTH: usize = {
    "basic".len()
};

lazy_static::lazy_static! {
    static ref PROTOCOL_NAME_SET: HashSet<char> = {
        create_name_hashset()
    };
}

fn create_name_hashset() -> HashSet<char> {
    let character_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut ret = HashSet::new();

    character_string
        .chars()
        .for_each(|e| {
            ret.insert(e);
        });

    ret
}

#[derive(Error, Debug)]
pub enum ProtocolNameError {
    #[error("character `{0}` is invalid")]
    InvalidCharacter(String),
    #[error("protocol name with length `{0}` is to short")]
    NameToShort(usize),
}

#[derive(Clone, Hash)]
pub struct ProtocolName {
    internal: String
}

impl ProtocolName {
    pub unsafe fn unchecked_new(input: String) -> ProtocolName {
        ProtocolName {
            internal: input
        }
    }

    pub fn validate_length(input: &str) -> bool {
        input.len() <= PROTOCOL_NAME_MIN_LENGTH
    }

    pub fn validate_characters(input: &str) -> Result<(), String> {
        let mut ret: bool = true;
        let mut invalid_chars = String::new();

        input
            .chars()
            .for_each(|e| {
                if !PROTOCOL_NAME_SET.contains(&e) {
                    ret = false;
                    invalid_chars.push(e);
                }
            });

        if !ret {
            return Err(invalid_chars);
        }

        Ok(())
    }

    pub fn characters_valid(input: &str) -> bool {
        input
            .chars()
            .any(|e| {
                !PROTOCOL_NAME_SET.contains(&e)
            })
    }


    pub fn new(input: String) -> Result<ProtocolName, ProtocolNameError> {
        if !Self::validate_length(&input) {
            return Err(ProtocolNameError::NameToShort(input.len()));
        }

        match Self::validate_characters(&input) {
            Ok(_) => {},
            Err(e) => {
                return Err(ProtocolNameError::InvalidCharacter(e));
            }
        }

        Ok(ProtocolName {
            internal: input
        })
    }

    pub fn to_str(&self) -> &str {
        self.internal.as_str()
    }

}

impl std::fmt::Debug for ProtocolName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.internal.fmt(f)
    }
}

impl ToString for ProtocolName {
    fn to_string(&self) -> String {
        self.internal.clone()
    }
}
