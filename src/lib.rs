// use crate::errors::{ UpdateDbError, HerokuError };
use regex;
use std::collections::HashMap;
use std::process;
use std::env;
use std::fs;
use std::path;
use clap::{ Arg, App };

mod errors ;

pub struct UpdateDbCli ;

impl UpdateDbCli {
    pub fn new(app_name: &str) -> App {
        let cli_app = App::new(app_name)
            .version("0.0.1")
            .about("Update local db with Heroku backup db. Uses pg_dump and pg_restore commands")
            .author("Chidi Nnadi");

        // -d, --db-name
        let local_db_name_arg = Arg::with_name("local_db_name")
            .help("The name of the local db")
            .required(true)
            .short("d")
            .long("db-name")
            .value_name("LOCAL_DB_NAME")
            .takes_value(true);

        let cli_app = cli_app.arg(local_db_name_arg);

        // -u, --username
        let db_username_arg = Arg::with_name("db_username")
            .help("The username of the local db")
            .required(true)
            .short("u")
            .long("username")
            .value_name("DB_USERNAME")
            .takes_value(true);

        let cli_app = cli_app.arg(db_username_arg);

        // Heroku app name
        // -a, -app 
        let app_name_arg = Arg::with_name("app_name")
            .help("The name of the heroku app")
            .required(true)
            .short("a")
            .long("app")
            .value_name("APP_NAME")
            .takes_value(true);

        cli_app.arg(app_name_arg)
    }
}


pub fn get_db_cred(app_name: &str, db_name: Option<&str>) -> Result<HashMap<String, String>, errors::UpdateDbError> {
    let mut db_name = db_name;
    if db_name.is_none() {
        db_name = Some("DATABASE_URL");
    }

    let database_cred_output = process::Command::new("heroku")
        .arg("pg:credentials:url")
        .arg(db_name.unwrap())
        .arg("-a")
        .arg(app_name)
        .output()?;

    let database_cred_output = String::from_utf8_lossy(&database_cred_output.stdout);

    Ok(db_cred_from_string(&*database_cred_output)?)
}

// Postgres domain
pub fn pg_restore(user_name: &str, db_name: &str) -> Result<(), errors::UpdateDbError>{
    println!("Beginning restore process");
    let pg_restore_output = process::Command::new("pg_restore")
        .arg("--verbose")
        .arg("--clean")
        .arg("--no-acl")
        .arg("--no-owner")
        .arg("-h")
        .arg("localhost")
        .arg("-U")
        .arg(user_name)
        .arg("-d")
        .arg(db_name)
        .arg("latest.dump")
        .output()?;

    if pg_restore_output.status.success(){
        println!("Local database successfully restored from remote Heroku db.");
        return Ok(())
    } else {
        let err_message = String::from_utf8_lossy(&pg_restore_output.stderr);
        return Err(errors::UpdateDbError::new(&err_message.to_string(), errors::DbErrorSource::PostgresError))
    }
}


pub fn db_cred_from_string(string: &str) -> Result<HashMap<String, String>, errors::UpdateDbError> {
    let capture_pattern = regex::Regex::new(r#"(?P<field_name>\w+)=(?P<value>[^\s]+)"#)?;
    let url_pattern = regex::Regex::new(r"\w+://.+")?;
    let cap = capture_pattern.captures_iter(string);
    let url_cap = url_pattern.captures(string).unwrap();
    let mut hash: HashMap<String, String> = HashMap::new();
    for caps in cap {
        // Contraption to remove ending " from sslmode value
        if &caps["field_name"] == "sslmode" {
            let value = caps["value"].to_owned();
            let value: Vec<&str> = value.split(r#"""#).collect();
            let value = value[0];
            let field_name = caps["field_name"].to_owned();
            hash.insert(field_name, value.to_owned());
            continue;
        }
        hash.insert(caps["field_name"].to_owned(), caps["value"].to_owned());
    };
    hash.insert("url".to_owned(), url_cap[0].to_owned());

    Ok(hash)
}

// Heroku domain
pub fn download_backup(app_name: &str) -> Result<(), errors::UpdateDbError> {
    // Todo: Add logs
    delete_backup_if_exists()?;
    // download new backup
    println!("Downloading new backup");

    let output = process::Command::new("heroku")
    .arg("pg:backups:download")
    .arg("-a")
    .arg(app_name)
    .output()?;

    if output.status.success() {
        println!("Backup sucessfully downloaded");
        return Ok(());
    } else {
        let err_message = String::from_utf8_lossy(&output.stderr);
        return Err(errors::UpdateDbError::new(&err_message.to_string(), errors::DbErrorSource::HerokuError))
    }
}


fn delete_backup_if_exists() -> Result<(), errors::UpdateDbError> {
    // Todo: Add logs
    println!("Checking if backups exist");

    let mut current_dir = env::current_dir()?;
    if backup_exists(&mut current_dir, None) {
        println!("Removing existing backup");
        fs::remove_file(current_dir)?;
    }

    Ok(())
}

pub fn backup_exists(dir: &mut path::PathBuf, file_name: Option<&str>) -> bool {
    match file_name {
        Some(f_name) => dir.push(f_name),
        None => dir.push("latest.dump")
    }
    dir.exists()
}

// pub fn backup_dowloaded() 


// External test module
// #[cfg(test)]
// mod tests;