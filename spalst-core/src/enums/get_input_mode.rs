//! SPDX-License-Identifier: GPL-3.0-only

use color_eyre::eyre::{Result, bail};
use log::debug;
use std::fmt;
use tokio::io::{self, AsyncBufReadExt as _, BufReader};

#[derive(Debug)]
pub enum GetInputMode {
    Normal(String),
    Integer(isize),
    PositiveInteger(usize),
    Bool(bool),
    OneOf(Vec<String>),
}
impl fmt::Display for GetInputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // It's going to be read as ("Expected {}", get_input_mode_instance)
        match *self {
            Self::Normal(_) => unreachable!(),
            Self::Integer(_) => write!(f, "an integer"),
            Self::PositiveInteger(_) => write!(f, "a positive integer"),
            Self::Bool(_) => write!(f, "yes or no (y or n)"),
            Self::OneOf(ref acceptable) => write!(f, "one of {}", acceptable.join(", ")),
        }
    }
}

impl GetInputMode {
    pub async fn get_input(&mut self) -> Result<()> {
        loop {
            let input: String = {
                let mut inp: String = String::new();
                let _: usize = BufReader::new(io::stdin()).read_line(&mut inp).await?;
                inp.trim().to_string()
            };
            if input.is_empty() {
                // Ok.
                return Ok(());
            }
            match self.parse_from(&input) {
                Ok(()) => break,
                Err(error) => {
                    eprint!("Expected {self}, not \"{input}\". Please enter again: ");
                    debug!(
                        "When parsing input \"{input}\" with the target type {error:?}, got an error \"{error}\"",
                    );
                }
            }
        }
        // Ok.
        Ok(())
    }
    fn parse_from(&mut self, input: &str) -> Result<()> {
        *self = match *self {
            Self::Normal(_) => Self::Normal(input.parse()?),
            Self::Integer(_) => Self::Integer(input.parse()?),
            Self::PositiveInteger(_) => Self::PositiveInteger(input.parse()?),
            Self::Bool(_) => Self::Bool(input.parse()?),
            Self::OneOf(ref acceptable) => {
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
