# Actix Default Responder

Procedural macros for generating default `actix_web::Responder` implementation for a custom serialization.

Currently supported formats are:

    1. JSON

    2. Bincode

    3. XML


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