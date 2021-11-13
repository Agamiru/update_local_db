# update_local_db

`update-local-db` is command line application for updating a local Postgres database using a
downloaded backup of the remote db from a Heroku app.

It basically uses pg_dump and pg_restore commands.

## Example
```bash
update-local-db --db-name my_local_db_name --username my_local_db_username --app my_heroku_app_name
```
Shorter version:
```bash
update-local-db -d my_local_db_name -u my_local_db_username -a my_heroku_app_name
```

## Assumptions
This application makes a few assumptions:
- You have Heroku command line application currently installed on your local machine.
- You Heroku app has a backed up db.
- You are currently authenticated to use the heroku commands for your heroku apps.
- You have Postgresql installed and running on your machine.

## Todo
- Change stderr color to red.
- Enable creation of heroku Postgres backups before downloading.
