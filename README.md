[🚧] Iron Guard Server
=======================

> Iron Guard Server is an inventory management system server written in Rust.

**Author**: [Afaan Bilal](https://afaan.dev)

---

## Configuration
The following environment variables configure the server.

| Environment Variable    | Default value | Description
| :---------------------- | :------------ | :-----------
| IRON_GUARD_SECRET       | `test`        | Set the JWT signing secret. Must be set.
| IRON_GUARD_DB_TYPE      | `mysql`       | Set the database type. Options: `mysql`.
| IRON_GUARD_DB_HOST      | `0.0.0.0`     | Server Bind Host.
| IRON_GUARD_DB_PORT      | `3306`        | Server Port.
| IRON_GUARD_DB_USERNAME  | `root`        | Set the username. Must be set if authentication is enabled.
| IRON_GUARD_DB_PASSWORD  | `[blank]`     | Set the password. Must be set if authentication is enabled.
| IRON_GUARD_DB_DATABASE  | `iron_guard`  | Set the password. Must be set if authentication is enabled.

---

## Run
````
cargo run
````

---

## Test
````
cargo test
````

---

## Entity Generation
````
sea-orm-cli generate entity -o src/entities -u mysql://root:@localhost:3306/iron_guard
````

---

## Contributing
All contributions are welcome. Please create an issue first for any feature request
or bug. Then fork the repository, create a branch and make any changes to fix the bug
or add the feature and create a pull request. That's it!
Thanks!

---

## License
**Iron Guard Server** is released under the MIT License.
Check out the full license [here](LICENSE).
