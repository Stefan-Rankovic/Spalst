use crate::enums::LevelFilterWrapper;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "spalst")]
#[command(about = "RPG Game", version)]
pub struct ArgsParser {
    #[arg(
        long,
        value_enum,
        default_value_t = LevelFilterWrapper::Warn,
        help = "Log actions inside a spalst.log file with the level passed."
    )]
    pub log_level: LevelFilterWrapper,
    #[arg(
        long,
        default_value_t = false,
        help = "Clean all previous logs, leaving only the log of the current program instance."
    )]
    pub clean_logs: bool,
}
