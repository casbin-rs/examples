# Actix FileAdapter RBAC

Basic integration of [Casbin-RS](https://github.com/casbin/casbin-rs) with `FileAdapter` for Actix web.

This example uses the [RABC](https://en.wikipedia.org/wiki/Role-based_access_control) model.

## Usage

```sh
cd examples/actix_fileadapter_rbac
```

Modify the files in the `rbac` directory and the code in the `src` directory as required.

## Running Server

```sh
cd examples/actix_fileadapter_rbac
cargo run (or ``cargo watch -x run``)

# Started http server: 127.0.0.1:8080
```

In this example, you can get the the successful result at `http://localhost:8080/success` (accessible) and the failed result at `http://localhost:8080/fail` (inaccessible, `ERR_EMPTY_RESPONSE`).
