use gloo_net::http::{Request, Headers};
use thiserror::Error;
use url::form_urlencoded::byte_serialize;
use crate::Config;
use crate::ListBucketResult;
use crate::Contents;

const USER_AGENT: &str = "wasm-fetch";

#[derive(Clone, Copy)]
pub struct S3Api {

}

impl S3Api {

    pub const fn new() -> Self {
        Self{}
    }

    pub fn bucket_name(&self) -> String {
        let config = Config::new();
        config.s3_bucket_name.expect("no bucket configured.")
    }

    pub async fn get_comment(&self, id: String) -> Option<String> {
        let url = format!("https://{}.s3.amazonaws.com/comments/{}", self.bucket_name(), id);
        let request = Request::get(&url).headers(self.get_headers());
        let response = request.send().await.unwrap();
        if response.ok() {
        let text_result = response.text().await;
            match text_result {
                Ok(s) => Some(s),
                _ => None,
            }
        } else {
            None
        }
    } 

    fn get_headers(&self) -> Headers {
        let headers = Headers::new();
        headers.append("X-Amz-Content-Sha256", "UNSIGNED-PAYLOAD");
        headers.append("X-Amz-User-Agent", USER_AGENT);
        headers
    }

    async fn send_list_request(&self, url: String) -> ListBucketResult {
        let request = Request::get(&url).headers(self.get_headers());
        let response = request.send().await.unwrap();
        let xml = response.text().await.unwrap();
        let result: ListBucketResult = quick_xml::de::from_str(&xml).unwrap();
        result
    }

    pub async fn list_images(&self) -> Result<Vec<Contents>> {
        let mut contents: Vec<Contents> = vec![];
        let url = {format!("https://{}.s3.amazonaws.com/?list-type=2&prefix=images%2F&start-after=images%2F", 
            self.bucket_name())};
        let mut r = self.send_list_request(url).await;
        contents.append(&mut r.contents);
        while r.is_truncated {
            if r.next_continuation_token.is_some() {
                let continuation_token = String::from(r.next_continuation_token.clone().unwrap());
                let url = format!("https://{}.s3.amazonaws.com/?list-type=2&continuation-token={}&prefix=images%2F&start-after=images%2F",
                self.bucket_name(), byte_serialize(continuation_token.as_bytes()).collect::<String>());
                r = self.send_list_request(url).await;
                contents.append(&mut r.contents);
                log::info!("is_truncated {}", r.is_truncated);
            }
        }
        log::info!("{} images loaded.", contents.len());
        contents.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
        Ok(contents)
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(crate::Error),
}

impl From<crate::Error> for Error {
    fn from(e: crate::Error) -> Self {
        Self::Api(e)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn s3_api_test() {
        //let s3_api = S3Api::new();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
