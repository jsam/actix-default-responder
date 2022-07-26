# Actix Default Responder


[![Check Commit](https://github.com/jsam/actix-default-responder/actions/workflows/check-commit.yml/badge.svg)](https://github.com/jsam/actix-default-responder/actions/workflows/check-commit.yml)
[![](https://img.shields.io/crates/v/actix-default-responder.svg)](https://crates.io/crates/actix-default-responder)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Procedural macros for generating default `actix_web::Responder` implementation for a custom serialization.

Currently supported formats are:
- JSON
- Bincode
- XML


## Json Response

```rust
#[derive(Serialize, PartialEq, JsonResponder)]
struct JsonResponse {
    name: String,
}
```

## XML Response

```rust
#[derive(Debug, Serialize, PartialEq, XMLResponder)]
struct XMLResponse {
    name: String,
}
```

## Bincode Response

```rust
#[derive(Debug, Serialize, PartialEq, BincodeResponder)]
struct BincodeResponse {
    name: String,
}
```
