struct KHttpResponse {
    pub code: int32,
    pub message: String,
}

pub struct AuthRequest<'a> {
    api_token: &'a str,
}

impl<'a> AuthenticatedRequest<'a> {
    pub fn new(api_token: &'a str) -> Self {
        Self { api_token }
    }
}

