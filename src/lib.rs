use crate::time_table_checker::TimeTableChecker;
use chrono::Local;
use clap::{AppSettings, Clap};
use std::error::Error;
use std::fmt;

mod time_table_checker;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Justin P. <melotic@protonmail.ch>",
    about = "Notifies you when a class at VT has a seat available.",
    setting = AppSettings::ColoredHelp
)]
pub struct StoqedOptions {
    #[clap(about = "The \"SESSID\" cookie needed to authenticate to the timetable.")]
    pub session_id: String,

    #[clap(about = "The term to check for classes. Spring 2021 -> 202101")]
    pub term_year: u32,

    #[clap(about = "The CRNS to check for.")]
    pub crns: Vec<u32>,
}

pub fn run(opts: StoqedOptions) -> Result<(), Box<dyn Error>> {
    log::info!("Started at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    TimeTableChecker::new(opts).run()
}

#[derive(Debug)]
pub struct StoqedError<'a>(&'a str);

impl<'a> fmt::Display for StoqedError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for StoqedError<'_> {}
