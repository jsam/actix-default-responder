#[macro_use]
extern crate actix_default_responder;

use actix_web::body::to_bytes;
use actix_web::{test, HttpResponse};
use serde::Serialize;

#[derive(Serialize, PartialEq, JsonResponder)]
struct JsonResponse {
    name: String,
}

#[derive(Debug, Serialize, PartialEq, XMLResponder)]
struct XMLResponse {
    name: String,
}

#[derive(Debug, Serialize, PartialEq, BincodeResponder)]
struct BincodeResponse {
    name: String,
}

#[actix_web::test]
async fn json_responder() {
    use actix_web::Responder;

    let req = test::TestRequest::default().to_http_request();
    let test_value = JsonResponse {
        name: "Test".to_string(),
    };

    let res: HttpResponse = test_value.respond_to(&req);
    let res = to_bytes(res.into_body()).await.unwrap();

    assert_eq!(res, &b"{\"name\":\"Test\"}"[..]);
}

#[actix_web::test]
async fn xml_responder() {
    use actix_web::Responder;

    let req = test::TestRequest::default().to_http_request();
    let test_value = XMLResponse {
        name: "Test".to_string(),
    };

    let res: HttpResponse = test_value.respond_to(&req);
    let res = to_bytes(res.into_body()).await.unwrap();

    assert_eq!(
        res,
        &b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><XMLResponse><name>Test</name></XMLResponse>"
            [..]
    );
}

#[actix_web::test]
async fn bincode_responder() {
    use actix_web::Responder;

    let req = test::TestRequest::default().to_http_request();
    let test_value = BincodeResponse {
        name: "Test".to_string(),
    };

    let res: HttpResponse = test_value.respond_to(&req);
    let res = to_bytes(res.into_body()).await.unwrap();

    assert_eq!(res, &b"\x04\0\0\0\0\0\0\0Test"[..]);
}
