extern crate http;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_http_headermap;

#[test]
fn can_deserialize_struct() {
    #[derive(Deserialize)]
    struct Basic {
        host: String,
        content_length: Option<String>,
    }
    let mut headers = http::HeaderMap::new();

    headers.insert(http::header::HOST, "example.com".parse().unwrap());
    headers.insert(http::header::CONTENT_LENGTH, "123".parse().unwrap());

    assert!(serde_http_headermap::from_headermap::<Basic>(&headers).is_ok())
}
