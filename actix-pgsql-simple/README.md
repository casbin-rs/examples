# Casbin with Actix & PgSQL

A simple example, using Actix web, diesel-adapter and Postgresql.

## Prerequisite

You need to have `docker` and `docker-compose` commands installed.

## Run

Run `make` to setup and run the application.

Then open http://127.0.0.1:1080/?name=casbin, it should say `OK` which can mean you have access!

If you change the `name` to anything else, it would return a `403` response with `Forbidden` message. 
