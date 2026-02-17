pub struct FlUrl {
    path: String,
}

impl FlUrl {
    pub fn new(path: &str) -> Self {
        let path = if path.starts_with("http") {
            path.to_string()
        } else {
            let mut full_path = super::GlobalAppSettings::new().origin;
            if !full_path.ends_with("/") {
                full_path.push('/');
            }

            if path.starts_with('/') {
                full_path.push_str(&path[1..]);
            } else {
                full_path.push_str(path);
            }

            full_path
        };

        Self { path }
    }

    pub fn append_path_segment(mut self, path_segment: &str) -> Self {
        if !self.path.ends_with('/') {
            self.path.push('/');
        }
        if path_segment.starts_with('/') {
            self.path.push_str(&path_segment[1..]);
        } else {
            self.path.push_str(path_segment);
        }

        self
    }

    pub async fn get(&self) -> reqwest::Result<FlUrlResponse> {
        crate::console_log(format!("Doing request to {}", self.path));
        let result = reqwest::get(self.path.as_str()).await?;

        Ok(FlUrlResponse { result })
    }

    pub async fn post(
        &self,
        body: impl Into<super::HttpRequestBody>,
    ) -> reqwest::Result<FlUrlResponse> {
        let body = body.into();
        let as_vec = body.into_vec();

        let client = reqwest::Client::new();

        let result = client.post(self.path.as_str()).body(as_vec).send().await?;
        Ok(FlUrlResponse { result })
    }
}

pub struct FlUrlResponse {
    pub result: reqwest::Response,
}

impl FlUrlResponse {
    pub fn get_status_code(&self) -> u16 {
        self.result.status().as_u16()
    }

    pub async fn get_body_as_text(self) -> reqwest::Result<String> {
        self.result.text().await
    }
}
