use std::io::Write;
use std::io;
use std::fmt;
use clap;
use regex;
use std::process;


#[derive(Debug)]
pub struct UpdateDbError {
    error_source: DbErrorSource,
    message: String
}

impl UpdateDbError {
    pub fn new(message: &str, error: DbErrorSource) -> Self {
        UpdateDbError { message: message.to_owned(), error_source: error }
    }

    pub fn exit(&self) {
        let std_out = io::stdout();
        writeln!(&mut std_out.lock(), "{}", self.to_string()).expect("Error writing Error to stdout");
        process::exit(1)
    }
}

// Implementations
impl fmt::Display for UpdateDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error Type: {}\nDescription: {}", self.error_source.to_string(),self.message)
    }
}

impl From<io::Error> for UpdateDbError {
    fn from(err: io::Error) -> Self {
        UpdateDbError { message: err.to_string(), error_source: DbErrorSource::IoError}
    }
}

impl From<regex::Error> for UpdateDbError {
    fn from(err: regex::Error) -> Self {
        UpdateDbError { message: err.to_string(), error_source: DbErrorSource::RegexError}
    }
}

impl From<clap::Error> for UpdateDbError {
    fn from(err: clap::Error) -> Self {
        UpdateDbError { message: err.to_string(), error_source: DbErrorSource::ClapError}
    }
}

impl From<PostgresError> for UpdateDbError {
    fn from(err: PostgresError) -> Self {
        UpdateDbError { message: err.to_string(), error_source: DbErrorSource::PostgresError}
    }
}


// impl std::error::Error for UpdateDbError {
    
// }

#[derive(Debug)]
pub enum DbErrorSource {
    IoError,
    ClapError,
    RegexError,
    PostgresError,
    HerokuError
}

impl DbErrorSource {
    fn to_string(&self) -> &'static str {
        match *self {
            Self::IoError => "IO Error",
            Self::ClapError => "Clap Error",
            Self::RegexError => "Regex Error",
            Self::PostgresError => "Postgres Error",
            Self::HerokuError => "Heroku Error"
        }
    }
}


#[derive(Debug)]
pub struct PostgresError {
    message: String
}

impl PostgresError {
    pub fn new(message: &str) -> Self {
        PostgresError { message: message.to_owned() }
    }
}

impl fmt::Display for PostgresError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Postgres Error: {}", self.message)
    }
}


#[derive(Debug)]
pub struct HerokuError {
    message: String
}

impl HerokuError {
    pub fn new(message: &str) -> Self {
        HerokuError { message: message.to_owned() }
    }
}

impl fmt::Display for HerokuError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Heroku Error: {}", self.message)
    }
}

