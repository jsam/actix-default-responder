extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;

enum ResponderType {
    Json,
    Xml,
    Bincode,
}

#[proc_macro_derive(JsonResponder)]
pub fn json_responder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_responder(&ast, ResponderType::Json)
}

#[proc_macro_derive(XMLResponder)]
pub fn xml_responder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_responder(&ast, ResponderType::Xml)
}

#[proc_macro_derive(BincodeResponder)]
pub fn bincode_responder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_responder(&ast, ResponderType::Bincode)
}

fn impl_responder(ast: &syn::DeriveInput, responder: ResponderType) -> TokenStream {
    let name = &ast.ident;

    let body = match responder {
        ResponderType::Json => quote! {
            match serde_json::to_string(&self) {
                Ok(_body) => actix_web::HttpResponse::Ok().content_type("application/json").body(_body),
                Err(_err) => actix_web::HttpResponse::build(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                ).body(_err.to_string()),
            }
        },
        ResponderType::Xml => quote! {
            match serde_xml_rs::to_string(&self) {
                Ok(_body) => actix_web::HttpResponse::Ok().content_type("application/xml").body(_body),
                Err(_err) => actix_web::HttpResponse::build(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                ).body(_err.to_string()),
            }
        },
        ResponderType::Bincode => quote! {
            match bincode::serialize(&self) {
                Ok(_body) => actix_web::HttpResponse::Ok().content_type("multipart/form-data").body(_body),
                Err(_err) => actix_web::HttpResponse::build(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                ).body(_err.to_string()),
            }
        },
    };

    let gen = quote! {
        impl actix_web::Responder for #name {
            type Body = actix_web::body::BoxBody;

            #[inline]
            fn respond_to(mut self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                #body
            }

        }
    };

    gen.into()
}
