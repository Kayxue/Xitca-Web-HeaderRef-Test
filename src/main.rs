#![allow(incomplete_features)]
#![feature(unsized_const_params)]

use xitca_web::{
    error::Error,
    handler::{handler_service, FromRequest},
    http::{HeaderName, HeaderValue, StatusCode},
    App, WebContext,
};

/// extractor type with string literal as const generic
struct HeaderRef<'a, const NAME: &'static str>(&'a HeaderValue);

/// extract header value based on given string as header name
impl<'a, 'r, C, B, const NAME: &'static str> FromRequest<'a, WebContext<'r, C, B>>
    for HeaderRef<'a, NAME>
{
    type Type<'b> = HeaderRef<'b, NAME>;
    type Error = Error;

    async fn from_request(ctx: &'a WebContext<'r, C, B>) -> Result<Self, Self::Error> {
        ctx.req()
            .headers()
            .get(&HeaderName::from_static(NAME))
            .map(HeaderRef)
            .ok_or_else(|| todo!())
    }
}

fn main() -> std::io::Result<()> {
    App::new()
        .at(
            "/",
            handler_service(async |HeaderRef(val): HeaderRef<'_, "user-agent">| {
                println!("user agent header value: {:?}", val.to_str().unwrap());
                StatusCode::OK
            }),
        )
        .serve()
        .bind("localhost:8080")?
        .run()
        .wait()
}