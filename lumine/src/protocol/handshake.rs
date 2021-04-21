use log::debug;

use tokio_tungstenite::tungstenite::handshake::server::{
    Callback, ErrorResponse, Request, Response,
};

pub(crate) struct HandshakeCallback {
    access_token: &'static str,
    entry_point: &'static str,
}

impl HandshakeCallback {
    pub(crate) fn new(access_token: &'static str, entry_point: &'static str) -> Self {
        Self {
            access_token,
            entry_point,
        }
    }
}

impl Callback for HandshakeCallback {
    fn on_request(self, request: &Request, response: Response) -> Result<Response, ErrorResponse> {
        if request.uri().path().as_bytes() != self.entry_point.as_bytes() {
            return Err(Response::builder()
                .status(404)
                .body(Some("No such entry point".to_owned()))
                .unwrap());
        }

        let headers = request.headers();
        debug!("Handshake headers: {:?}", headers);

        let user_access_token = match headers.get("Authorization") {
            Some(token) => token.as_bytes().strip_prefix(b"Bearer ").unwrap_or(b""),
            None => b"",
        };

        if user_access_token == self.access_token.as_bytes() {
            Ok(response)
        } else {
            Err(Response::builder()
                .status(403)
                .body(Some("Authorization header incorrect".to_owned()))
                .unwrap())
        }
    }
}
