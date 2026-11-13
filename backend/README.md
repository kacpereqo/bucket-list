# Bucket List — backend

REST API for the Bucket List app: users own **bucket lists**, which contain
**todo lists**, which contain **todo items**. Bucket lists can be shared with
other users.

Stack: [actix-web](https://actix.rs) · [sqlx](https://github.com/launchbadge/sqlx) + SQLite ·
[utoipa](https://github.com/juhaku/utoipa) (OpenAPI + Swagger UI).

---

## Quick start

```bash
cd backend
cargo run
```

The server listens on `http://127.0.0.1:8080`. On startup it creates
`database.db` if needed and applies all migrations from `migrations/`.

- Swagger UI: <http://127.0.0.1:8080/docs/>
- OpenAPI spec: <http://127.0.0.1:8080/api-docs/openapi.json>

Quick check:

```bash
curl http://127.0.0.1:8080/api/health/db
# {"epoch":1763100000}
```

### Configuration

| Variable       | Default                          | Used by                                   |
|----------------|----------------------------------|-------------------------------------------|
| `DATABASE_URL` | `sqlite:database.db?mode=rwc`    | the server at runtime, and the `sqlx` macros at **compile time** (read from `.env`) |

Logging is set to `info` in `main.rs` (it overrides `RUST_LOG` from the shell).

---

## Project structure

```
backend/
├── migrations/          SQL schema, applied on startup (sqlx migrate)
├── src/
│   ├── main.rs          starts the HTTP server (thin: calls build_app)
│   ├── lib.rs           init_db() + build_app(): middleware, routes, Swagger
│   ├── router.rs        URL scopes: /api/me, /api/buckets, /api/lists, …
│   ├── auth.rs          CurrentUser extractor (temporary X-User-Id header)
│   ├── state.rs         AppState { db: SqlitePool }
│   ├── models/          structs: rows, request bodies (*Create/*Update), nested responses
│   ├── repository/      SQL queries, one file per table, no HTTP or permission logic
│   └── services/        HTTP handlers, one file per URL scope
└── tests/               integration tests (real app + fresh DB per test)
```

How a request flows:

```
HTTP request
  → router.rs        picks the scope (/api/items → services::todo_items)
  → services/*.rs    extracts path/body/CurrentUser, calls the repository,
                     maps the result to an HTTP status
  → repository/*.rs  runs the SQL, returns Result<T, sqlx::Error>
  → models/*.rs      types shared by both layers
```

Conventions:

- **Request bodies** (`*Create`, `*Update`) derive `Deserialize`;
  **responses** (`TodoItem`, `BucketList`, …) derive `Serialize`. All derive `ToSchema` for Swagger.
- **Parent ids come from the URL, never from the body** — e.g. `POST /api/lists/{id}/items`
  takes the todo list id from the path; `POST /api/buckets` takes the owner from `CurrentUser`.
- **Repository functions return `sqlx::Error::RowNotFound`** when an update/delete matched no row;
  services turn that into `404`.

---

## Authentication (temporary)

There is no login yet. Endpoints that need to know the user take a
`CurrentUser` argument, which reads the user id from a header:

```
X-User-Id: 1
```

Missing or non-numeric header → **`401 Unauthorized`**.

> ⚠️ This is **not** security: any client can send any id. When real login
> (JWT / session) is added, only `CurrentUser::from_request` in `src/auth.rs`
> changes — handlers stay the same.

In Swagger UI the header shows up as a parameter on the endpoints that use it.

---

## Endpoints

All paths are under `/api`. Bodies are JSON (`Content-Type: application/json`).

| Method | Path                         | Auth          | Body               | Success                    | Errors           |
|--------|------------------------------|---------------|--------------------|----------------------------|------------------|
| GET    | `/health/db`                 | —             | —                  | `200` `{"epoch": <int>}`   | —                |
| GET    | `/me/buckets`                | `X-User-Id`   | —                  | `200` `BucketList[]`       | 401              |
| GET    | `/me/overview`               | `X-User-Id`   | —                  | `200` `BucketListWithLists[]` | 401           |
| POST   | `/buckets`                   | `X-User-Id`   | `BucketListCreate` | `201` new id               | 400, 401         |
| PUT    | `/buckets/{id}`              | `X-User-Id`, owner only | `BucketListUpdate` | `204`            | 400, 401, 404    |
| GET    | `/lists/{id}/items`          | —             | —                  | `200` `TodoItem[]`         | —                |
| POST   | `/lists/{id}/items`          | —             | `TodoItemCreate`   | `201` new id               | 400              |
| PUT    | `/items/{id}`                | —             | `TodoItemUpdate`   | `204`                      | 400, 404         |
| PATCH  | `/items/{id}/toggle`         | —             | —                  | `204`                      | 404              |
| DELETE | `/items/{id}`                | —             | —                  | `204`                      | 404              |

Every endpoint can also return `500` on a database error (details are logged
on the server, never sent to the client).

### Me

**`GET /api/me/buckets`** — bucket lists the user owns **or** that were shared
with them. Owned lists are included because the `add_owner_to_shared` trigger
adds the owner to `shared_bucket_lists` when a bucket list is created.

**`GET /api/me/overview`** — the same bucket lists, with all their todo lists
and items, in **one request**. Meant for the app's start screen. Built with a
single SQL query (`LEFT JOIN`s), so empty buckets and empty lists are included:

```json
[
  {
    "id": 1, "owner_id": 1, "name": "Trips", "created_at": 1763100000,
    "todo_lists": [
      {
        "id": 4, "bucket_list_id": 1, "name": "Japan", "created_at": 1763100000,
        "items": [
          { "id": 9, "todo_list_id": 4, "title": "Buy tickets", "description": null,
            "is_done": false, "created_at": 1763100000 }
        ]
      }
    ]
  },
  { "id": 2, "owner_id": 1, "name": "Books", "created_at": 1763100000, "todo_lists": [] }
]
```

### Bucket lists

**`POST /api/buckets`** — `{"name": "Trips"}`. The owner is the `CurrentUser`.
Returns `201` with the new id.

**`PUT /api/buckets/{id}`** — `{"name": "New name"}`. Only the **owner** can
rename; for anyone else (including users it is shared with) the response is
`404`, so the API doesn't reveal that the list exists.

### Todo lists

**`GET /api/lists/{id}/items`** — items of todo list `{id}`; `[]` if it has none.

**`POST /api/lists/{id}/items`** — `{"title": "Buy tickets", "description": null}`.
`description` is optional. New items start with `is_done = false`. Returns `201`
with the new id.

### Todo items

**`PUT /api/items/{id}`** — replaces `title` and `description`
(`{"title": "...", "description": "..." | null}`). Does not touch `is_done`.

**`PATCH /api/items/{id}/toggle`** — flips `is_done`.

**`DELETE /api/items/{id}`** — deletes the item.

### Data types

| Type | Fields |
|---|---|
| `BucketList` | `id`, `owner_id`, `name`, `created_at` |
| `TodoList` | `id`, `bucket_list_id`, `name`, `created_at` |
| `TodoItem` | `id`, `todo_list_id`, `title`, `description` (nullable), `is_done`, `created_at` |
| `BucketListWithLists` | `BucketList` fields + `todo_lists: TodoListWithItems[]` |
| `TodoListWithItems` | `TodoList` fields + `items: TodoItem[]` |
| `BucketListCreate` / `BucketListUpdate` | `name` |
| `TodoItemCreate` / `TodoItemUpdate` | `title`, `description` (optional) |

`created_at` is a Unix timestamp in seconds (UTC), set by the database.

### Compression

Responses are compressed (gzip / brotli / zstd) when the client sends
`Accept-Encoding` — browsers and `fetch` do this automatically.

---

## Database

Schema: `migrations/` (diagram: [`../docs/models/database.puml`](../docs/models/database.puml)).

```
users ──< bucket_lists ──< todo_lists ──< todo_items ──< comments
  └──< shared_bucket_lists >──┘
```

- Deleting a row cascades to its children (`ON DELETE CASCADE`).
- Trigger `add_owner_to_shared`: inserting a bucket list also inserts
  `(owner_id, id)` into `shared_bucket_lists`.

### Changing the schema

**Never edit a migration that has already been applied** — sqlx stores a
checksum and the server will refuse to start with `VersionMismatch`.
Add a new file instead:

```bash
sqlx migrate add <name>      # needs: cargo install sqlx-cli
```

While developing locally you can also delete `database.db` and restart.

### Compile-time checked SQL

`sqlx::query!` / `query_as!` check SQL against the database in `DATABASE_URL`
**while compiling**. That database must exist and have the migrations applied
(start the server once), otherwise the build fails.

---

## Tests

```bash
cargo test                      # everything
cargo test --test todo_items    # one file
cargo test overview             # tests whose name contains "overview"
```

- Every test gets a **fresh SQLite database** with all migrations applied
  (`#[sqlx::test]`), so tests are independent and never touch `database.db`.
- Requests go through the **real app** (`backend::build_app`, the same one
  `main.rs` runs) using `actix_web::test` — no server or port needed.
- Shared helpers live in `tests/common/mod.rs`: `app(&pool)` and seed functions
  (`insert_user`, `insert_bucket_list`, `insert_todo_list`, `insert_todo_item`,
  `seed_todo_list`, `seed_todo_item`).

| File | Covers |
|---|---|
| `tests/app.rs` | compression, OpenAPI spec lists every endpoint, Swagger UI, health |
| `tests/me.rs` | `/me/buckets`, `/me/overview`, 401 handling |
| `tests/bucket_lists.rs` | create/rename bucket lists, owner checks, overview repository functions |
| `tests/todo_lists.rs` | listing and creating items in a list |
| `tests/todo_items.rs` | update, toggle, delete items |

Template for a new test file:

```rust
mod common;

use actix_web::{http::StatusCode, test};
use sqlx::SqlitePool;

#[sqlx::test]
async fn my_endpoint_works(pool: SqlitePool) {
    let item_id = common::seed_todo_item(&pool).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::get().uri(&format!("/api/...{item_id}")).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}
```

---

## Adding an endpoint

1. **Model** (`src/models/<table>.rs`) — request body with `Deserialize, ToSchema`,
   response with `Serialize, ToSchema`.
2. **Query** (`src/repository/<table>.rs`) — `pub async fn ...(pool: &SqlitePool, ...) -> Result<T, sqlx::Error>`.
3. **Handler** (`src/services/<scope>.rs`) — needs **both** `#[utoipa::path(...)]`
   and the route attribute (`#[get(...)]`, …); without `#[utoipa::path]`
   `cfg.service(...)` won't compile. Map `RowNotFound` → `404`, other errors → log + `500`.
4. **Register** it in that file's `configure`: `cfg.service(handler);`.
   A new scope also goes into `src/router.rs`.
5. **Test** it in `tests/`, and add it to the list in `tests/app.rs::openapi_spec_lists_all_endpoints`.

---

## Known gaps

- **No permission checks on todo lists and items — intentional for now.**
  `/api/lists/...` and `/api/items/...` don't use `CurrentUser`, so any client
  can read or change any list or item by id. This is expected at this stage;
  checks will come later (look up the item's bucket list and check the user's
  access — owner / shared — in the service before calling the repository).
- **Authentication is a placeholder** (`X-User-Id`, see above).
- **Not implemented yet:** users, comments and sharing endpoints
  (`services/users.rs`, `comments.rs`, `shared_bucket_lists.rs` are empty),
  deleting bucket lists, renaming/deleting todo lists.
- **Swagger title** shows the library default (`utoipa-actix-web`) instead of
  "Bucket List API".
- `../docs/models/rest-api.puml` describes an earlier plan (`/api/v1/...`),
  not the current routes — the table above is the source of truth.
