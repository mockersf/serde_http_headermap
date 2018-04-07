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

    assert!(serde_http_headermap::from_headermap::<Basic>(&headers).is_ok());
}

#[test]
fn can_serialize_struct() {
    #[derive(Serialize)]
    struct Basic {
        host: String,
        content_length: Option<String>,
    }

    let my_struct = Basic {
        host: "localhost".to_string(),
        content_length: None,
    };

    assert!(serde_http_headermap::to_headermap(&my_struct).is_ok());
}
