use rust_extensions::StrOrString;

pub struct FlUrl {
    path: String,
    query: String,
}

impl FlUrl {
    pub fn new(path: &str) -> Self {
        let path = if path.starts_with("http") {
            path.to_string()
        } else {
            let mut full_path = super::super::GlobalAppSettings::new().origin;
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

        Self {
            path,
            query: Default::default(),
        }
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

    pub fn append_query_param<'n, 'v>(
        mut self,
        param_name: impl Into<StrOrString<'n>>,
        value: Option<impl Into<StrOrString<'v>>>,
    ) -> Self {
        let param_name = param_name.into();

        if self.query.len() == 0 {
            self.query.push('?');
        } else {
            self.query.push('&');
        }

        url_utils::encode_to_url_string_and_copy(&mut self.query, param_name.as_str());

        if let Some(value) = value {
            let value = value.into();
            self.query.push('=');
            url_utils::encode_to_url_string_and_copy(&mut self.query, value.as_str());
        }

        self
    }

    pub fn append_query_param_if_some<'n, 'v>(
        self,
        param_name: impl Into<StrOrString<'n>>,
        value: Option<impl Into<StrOrString<'v>>>,
    ) -> Self {
        if let Some(value) = value {
            return self.append_query_param(param_name, Some(value));
        }

        self
    }

    fn get_path_and_query<'s>(&'s self) -> StrOrString<'s> {
        if self.query.len() == 0 {
            return self.path.as_str().into();
        }

        format!("{}{}", self.path, self.query).into()
    }

    pub async fn get(&self) -> reqwest::Result<FlUrlResponse> {
        let path_and_query = self.get_path_and_query();
        crate::console_log(format!("[GET] {}", path_and_query.as_str()));
        let result = reqwest::get(path_and_query.as_str()).await?;

        Ok(FlUrlResponse { result })
    }

    pub async fn post(
        &self,
        body: impl Into<super::super::HttpRequestBody>,
    ) -> reqwest::Result<FlUrlResponse> {
        let body = body.into();
        let as_vec = body.into_vec();

        let client = reqwest::Client::new();

        let path_and_query = self.get_path_and_query();
        crate::console_log(format!("[POST] {}", path_and_query.as_str()));
        let result = client
            .post(path_and_query.as_str())
            .body(as_vec)
            .send()
            .await?;
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
