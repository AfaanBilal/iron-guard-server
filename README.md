📦 Iron Guard Server
=====================

> Iron Guard Server is an inventory management system server written in Rust.

**Author**: [Afaan Bilal](https://afaan.dev)

---

## Configuration
The following environment variables configure the server.

| Environment Variable   | Default value | Description                          |
| :--------------------- | :------------ | :----------------------------------- |
| IRON_GUARD_SECRET      | `test`        | The JWT signing secret. Must be set. |
| IRON_GUARD_DB_TYPE     | `mysql`       | Database Type. Options: `mysql`.     |
| IRON_GUARD_DB_HOST     | `localhost`   | Database Host                        |
| IRON_GUARD_DB_PORT     | `3306`        | Database Port                        |
| IRON_GUARD_DB_USERNAME | `root`        | Database Username                    |
| IRON_GUARD_DB_PASSWORD | `[blank]`     | Database Password                    |
| IRON_GUARD_DB_DATABASE | `iron_guard`  | Database Name                        |
| ROCKET_ADDRESS         | `0.0.0.0`     | HTTP Server Bind Address             |
| ROCKET_PORT            | `8000`        | HTTP Server Port                     |

---

## Run
````
cargo run
````

---

## API

| Method | Path                 | Auth? | Description                                    |
| :----- | :------------------- | :---- | :--------------------------------------------- |
| GET    | /                    | ⬜     | Index. Returns `Iron Guard`.                   |
| POST   | /auth/sign-in        | ⬜     | Returns a JWT on success.                      |
| GET    | /categories          | ✅     | Get a list of categories.                      |
| POST   | /categories          | ✅     | Create a category.                             |
| GET    | /categories/`{uuid}` | ✅     | Get a category with matching the `uuid`.       |
| PUT    | /categories/`{uuid}` | ✅     | Update the category matching the `uuid`.       |
| DELETE | /categories/`{uuid}` | ✅     | Delete the category matching the `uuid`.       |
| GET    | /items               | ✅     | Get a list of items.                           |
| POST   | /items               | ✅     | Create a item.                                 |
| GET    | /items/`{uuid}`      | ✅     | Get a item with matching the `uuid`.           |
| PUT    | /items/`{uuid}`      | ✅     | Update the item matching the `uuid`.           |
| DELETE | /items/`{uuid}`      | ✅     | Delete the item matching the `uuid`.           |
| GET    | /users               | ✅     | `[admin]` Get a list of users.                 |
| POST   | /users               | ✅     | `[admin]` Create a user.                       |
| GET    | /users/`{uuid}`      | ✅     | `[admin]` Get a user with matching the `uuid`. |
| PUT    | /users/`{uuid}`      | ✅     | `[admin]` Update the user matching the `uuid`. |
| DELETE | /users/`{uuid}`      | ✅     | `[admin]` Delete the user matching the `uuid`. |

### Authentication
- **All auth required requests**: Add header `token` with the JWT as the value.
- **Token lifetime**: 4 hours.

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
