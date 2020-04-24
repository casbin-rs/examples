# ntex FileAdapter ACL

Basic integration of [Casbin-RS](https://github.com/casbin/casbin-rs) with `FileAdapter` for [ntex](https://github.com/ntex-rs/ntex).

This example uses the [ACL](https://en.wikipedia.org/wiki/Access_control_list) model.

## Usage

```sh
cd examples/ntex_fileadapter_acl
```

Modify the files in the `acl` directory and the code in the `src` directory as required.

## Running Server

```sh
cd examples/ntex_fileadapter_acl
cargo run (or ``cargo watch -x run``)

# Started http server: 127.0.0.1:8080
```

In this example, you can get the the result at `http://localhost:8080/auth/{name}/{action}`,
please use `alice` or `bob` instead of `{name}`, use `read` or `write` instead of `{action}`.
