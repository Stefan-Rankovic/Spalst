//! SPDX-License-Identifier: GPL-3.0-only
use color_eyre::eyre::{Result, bail};
use std::fmt;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[derive(Debug)]
pub enum GetInputMode {
    Normal(String),
    Integer(isize),
    PositiveInteger(usize),
    Bool(bool),
    OneOf(Vec<String>),
}
impl fmt::Display for GetInputMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // It's going to be read as ("Expected {}", get_input_mode_instance)
        match self {
            Self::Normal(_) => unreachable!(),
            Self::Integer(_) => write!(f, "an integer"),
            Self::PositiveInteger(_) => write!(f, "a positive integer"),
            Self::Bool(_) => write!(f, "yes or no (y or n)"),
            Self::OneOf(acceptable) => write!(f, "one of {}", acceptable.join(", ")),
        }
    }
}

impl GetInputMode {
    pub async fn get_input(&mut self) -> Result<()> {
        loop {
            let mut input: String = String::new();
            BufReader::new(io::stdin()).read_line(&mut input).await?;
            let input: &str = input.trim();
            if input.is_empty() {
                // Ok.
                return Ok(());
            };
            match self.parse_from(input) {
                Ok(_) => break,
                Err(e) => {
                    eprint!("Expected {self}, not \"{input}\". Please enter again: ");
                    debug!(
                        "When parsing input \"{input}\" with the target type {:?}, got an error \"{e}\"",
                        e
                    );
                }
            };
        }
        // Ok.
        Ok(())
    }
    fn parse_from(&mut self, input: &str) -> Result<()> {
        match self {
            Self::Normal(_) => Self::Normal(input.parse()?),
            Self::Integer(_) => Self::Integer(input.parse()?),
            Self::PositiveInteger(_) => Self::PositiveInteger(input.parse()?),
            Self::Bool(_) => Self::Bool(input.parse()?),
            Self::OneOf(acceptable) => {
                if acceptable.contains(&input.to_string()) {
                    Self::Normal(input.parse()?)
                } else {
                    bail!(
                        "Invalid choice {input} from available choices {}.",
                        acceptable.join(", ")
                    );
                }
            }
        };
        // Ok.
        Ok(())
    }
}
