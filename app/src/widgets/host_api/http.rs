use crate::widgets::running::runtime::{
    http::{self, Response},
    PluginState,
};

impl http::Host for PluginState {
    fn request(
        &mut self,
        method: http::Method,
        url: String,
        body: Option<Vec<u8>>,
    ) -> wasmtime::Result<Result<Response, ()>> {
        let client = reqwest::blocking::Client::new();
        let response = match method {
            http::Method::Get => client.get(url).send(),
            http::Method::Head => client.head(url).send(),
            http::Method::Post => {
                let mut req = client.post(url);
                if let Some(body) = body {
                    req = req.body(body);
                }
                req.send()
            }
            http::Method::Put => client.put(url).send(),
            http::Method::Delete => client.delete(url).send(),
        };
        match response {
            Ok(response) => Ok(Ok(Response {
                content_length: response.content_length(),
                status: response.status().as_u16(),
                bytes: response.bytes()?.to_vec(),
            })),
            Err(_) => Ok(Err(())),
        }
    }
}
