use reqwest::{
    header::{HeaderMap, HeaderValue},
    blocking::Client,
};

use serde_json::Value as Json;

pub struct ElasticSearchDocumentIterator<C: HttpClient> {
    host: String,
    index: String,
    client: Option<C>,
    scroll: Option<String>,
    documents: Vec<Json>,
    at_end: bool,
}

pub const BUFFER_FETCH_CAPACITY: usize = 100;

pub trait HttpClient {
    fn new() -> Self;

    fn start_scroll(&self, host: &str, index: &str) -> Json;

    fn next_scroll(&self, host: &str, scroll: &str) -> Json;
}

struct Scrollable(String, Vec<Json>);

impl From<Json> for Scrollable {
    fn from(mut data: Json) -> Self {
        let scroll = match data["_scroll_id"].take() {
            Json::String(string) => string,
            _ => panic!("expected scroll id")
        };

        let documents = match data["hits"]["hits"].take() {
            Json::Array(docs) => docs,
            _ => panic!("missing hits->hits")
        };

        Self(scroll, documents)
    }
}

impl HttpClient for Client {
    fn new() -> Self {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        Self::builder()
            .default_headers(headers)
            .build()
            .expect("A valid client")
    }

    fn start_scroll(&self, host: &str, index: &str) -> Json {
        self
            .post([host, "/", index, "/_search?scroll=1m"].join(""))
            .body(r#"{"size":100}"#)
            .send()
            .expect("A valid http response")
            .json::<Json>()
            .expect("valid json")
    }

    fn next_scroll(&self, host: &str, scroll: &str) -> Json {
        let body = format!(r#"{{"scroll":"1m","scroll_id":"{scroll}"}}"#);

        self
            .post([host, "/_search/scroll"].join(""))
            .body(body)
            .send()
            .expect("A valid http response")
            .json::<Json>()
            .expect("valid json")
    }
}

impl<C: HttpClient> ElasticSearchDocumentIterator<C> {
    pub fn new<S: ToString>(host: S, index: S) -> Self {
        Self {
            host: host.to_string(),
            index: index.to_string(),
            client: None,
            scroll: None,
            documents: vec![],
            at_end: false,
        }
    }
}

impl<C: HttpClient> Iterator for ElasticSearchDocumentIterator<C> {
    type Item = Json;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end {
            return None;
        }

        if self.client.is_none() {
            let client = C::new();
            let Scrollable(scroll, mut documents) = client
                .start_scroll(&self.host, &self.index)
                .into();

            if documents.is_empty() {
                self.at_end = true;
                return None;
            }

            self.scroll = Some(scroll);
            self.client = Some(client);
            documents.reverse();
            std::mem::swap(&mut self.documents, &mut documents);
            return self.documents.pop();
        }

        if self.documents.is_empty() {
            let Scrollable(scroll, mut documents) = self
                .client
                .as_ref()
                .unwrap()
                .next_scroll(&self.host, self.scroll.as_ref().unwrap())
                .into();

            if documents.is_empty() {
                self.at_end = true;
                return None;
            }

            documents.reverse();
            std::mem::swap(&mut self.documents, &mut documents);
            self.scroll = Some(scroll);
            return self.documents.pop();
        }

        self.documents.pop()
    }
}
