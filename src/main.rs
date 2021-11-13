
//! ## update-local-db
//!
//! `update-local-db` is command line application for updating your local Postgres database using a
//! downloaded backup of the remote db from a Heroku app.
//! 
//! It basically uses pg_dump and pg_restore commands.
//! 
//! ## Example
//! ```bash
//! update-local-db --db-name my_local_db_name --username my_local_db_username --app my_heroku_app_name
//! ```
//! Shorter version:
//! ```bash
//! update-local-db -d my_local_db_name -u my_local_db_username -a my_heroku_app_name
//! ```
//! 
//! ## Assumptions
//! This executable makes a few assumptions:
//! - You have Heroku command line application currently installed on your local machine.
//! - You Heroku app has backed up db.
//! - You are currently authenticated to use the heroku commands for your heroku apps.
//! - You have Postgresql installed and running on your machine.
//! 
//! ## Todo
//! - Change stderr color to red.
//! - Enable creation of heroku Postgres backups before downloading.


use update_local_db::{download_backup, pg_restore, UpdateDbCli};


fn main() {
    let matches = UpdateDbCli::new("update-local-db").get_matches();

    // These assignments should panic.
    let local_db_name = matches.value_of("local_db_name").unwrap();
    let user_name = matches.value_of("db_username").unwrap();
    let app_name = matches.value_of("app_name").unwrap();

    // Exit if error.
    download_backup(app_name).unwrap_or_else(|err| err.exit());
    pg_restore(user_name, local_db_name).unwrap_or_else(|err| err.exit())

}
