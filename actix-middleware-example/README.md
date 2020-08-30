# Actix-web example with Casbin Middleware and Actor

![Check Code](https://github.com/casbin-rs/examples/workflows/Check%20Code/badge.svg)

A simple Anonynous Forum app using Actix-web, Casbin and Diesel, with JWT support.
This example uses [Casbin Actix Middleware](https://github.com/casbin-rs/actix-casbin-auth) and [Casbin Actix Actor](https://github.com/casbin-rs/actix-casbin)

## Require

- [Rust Stable](https://rustup.rs)
- [Postgres](https://www.postgresql.org/)

## Running Server

- Rename `secret.key.sample` to `secret.key` or create your own key by running `head -c16 /dev/urandom > secret.key` in command line  and copy to `/src` folder
- Create a database in postgres.
- Rename `.env.sample` to `.env`.
- Update `DATABASE_URL` with your custom configuration.
- You may modify `APP_HOST`, `APP_PORT`, `POOL_SIZE` with your custom configuration.
- **WARNING**: Please do not change `HASH_ROUNDS`, if you insist to do so, please modify the root default password with bcrypt encryption manually (/migrations/2020-07-15-061549_add_root_user/up.sql).
- Run with release profile: `cargo run --release`

## Explanation

- Default root username: `root`, root password: `casbin`
- After running the server you may register `admin` user with password, then modify the `user` table in database, alter the `role` column into `1`. Then you have `admin permission` with user `admin`.
- `preset.csv` contains the casbin policies which you want add in the begining, you can alter it to meet your needs.
- `casbin.conf` is the casbin configuration file, you can alter it to meet your needs.
- **WARNING**: If you want to run this example, suggest not modify `preset.csv` and `casbin.conf`.