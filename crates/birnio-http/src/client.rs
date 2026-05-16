use std::time::Duration;

#[derive(Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> crate::HttpResult<Self> {
        let inner = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(Duration::from_secs(60))
            .build()?;

        Ok(Self { inner })
    }

    pub(crate) fn inner(&self) -> &reqwest::Client {
        &self.inner
    }
}
