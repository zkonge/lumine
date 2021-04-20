use log::debug;

use tokio_tungstenite::tungstenite::handshake::server::{Callback, ErrorResponse, Request, Response};

pub(crate) struct HandshakeCallback {
    access_token: &'static str,
}

impl HandshakeCallback {
    pub(crate) fn new(access_token: &'static str) -> Self {
        Self {
            access_token: access_token,
        }
    }
}

impl Callback for HandshakeCallback {
    fn on_request(self, request: &Request, response: Response) -> Result<Response, ErrorResponse> {
        let headers = request.headers();
        debug!("Handshake headers: {:?}", headers);

        let user_access_token = headers
            .get("Authorization")
            .map_or(b"".as_ref(), |x| x.as_bytes());

        let user_access_token = user_access_token
            .strip_prefix(b"Bearer ")
            .unwrap_or(user_access_token);

        if user_access_token == self.access_token.as_bytes() {
            Ok(response)
        } else {
            Err(Response::builder()
                .status(403)
                .body(Some("Authorization head incorrect".to_owned()))
                .unwrap())
        }
    }
}
