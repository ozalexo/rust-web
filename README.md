# rust-web

Challenge: programming in a new language (Rust) without learning

## MacOS: Rust installation

```sh
brew install rustup
rustup-init
```

## VSCode

Recommended extensions:
- rust-analyzer
- Dev Containers

## Build project

```sh
cargo build
```

## Start project

```sh
cargo run
```
URL: http://127.0.0.1:8000

📬 Routes:
   >> (index) GET /
   >> (refund) POST /refund application/json
   >> (authorize) POST /authorize application/json


# Task specifications

## Software stack
- rocket / tokio (async futures executor)
- postgres / sqlx (not diesel)
- clap (command line arguments processing)
- lldb (also console lldb client)
- Yew (webassembly)

## DB requirements
- table actions_queue

## HTTPS endpoints

1. authorize

- HTTP method: POST

- Request schema:
| *Key* | *Type* | *Description* |
| account	| string | User ID |
| amount | number | Funds amount |

- Response schema:
| *Key* | *HTTP response code* | *Type* | *Description* |
| status | 200	| string | "success" |
| status | 500	| string | "insuficient funds" |

2. refund

- HTTP method: POST

- Request schema:
| *Key* | *Type* | *Description* |
| account	| string | User ID |
| amount | number | Funds amount | 

- Response schema:
| *Key* | *HTTP response code* | *Type* | *Description* |
| status | 200	| string | "success" |
| status | 500	| string | "invalid refund" | 

3. queue

- HTTP method: GET

- Available parameters:
page: Filter by the given page number.
limit: Amount of records on a page
last_page: Total amount of pages available

## Web UI requirments
Request to /queue endpoing and display list of records (table format).
Table controls:
- next/prev page
- records amount

## Next steps
- blockchain
- logging