use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Router};
use serde_json::Value;
use std::sync::Arc;

use crate::{error::{chain_error, Error}, r#macro::{simple_get, simple_get_as}};

use super::{handler::HandlerVec, types::{Event, Message}};

#[derive(Clone)]
pub struct Manager {
    message_handler: Arc<HandlerVec<Message>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            message_handler: Arc::new(HandlerVec::new())
        }
    }

    /// Open web server to handle event from slack.
    pub fn open(&self) {
        let s = self.clone();
        tokio::spawn(async {
            let app = Router::new()
                .route("/", get(|| async { "This is JibagoBot." }))
                .route(
                    "/",
                    post(|r| async move { s.handle_request(r).await.unwrap() }),
                );

            let listener = tokio::net::TcpListener::bind("0.0.0.0:4420").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    }

    fn validate_event(&self, __request: &str) -> bool {
        // TODO
        // https://api.slack.com/authentication/verifying-requests-from-slack
        // somebody please implement this
        true
    }

    async fn handle_request(&self, request: String) -> Result<impl IntoResponse, Error> {
        if !self.validate_event(&request) {
            return Err(Error("bad request".into(), None))
        }

        println!("*** Received *** {}", request);

        let payload = serde_json::from_str::<Value>(&request)
            .map_err(chain_error!("not json"))?;
        let event_type = simple_get_as!(payload, "type", as_str)?;

        match event_type {
            "message" => {
                let message = serde_json::from_str::<Message>(&request)
                    .map_err(chain_error!("json parsing failed"))?;
                self.message_handler.broadcast(message).await;
            }
            _ => {},
        }

        Ok((StatusCode::OK, request.to_string()))
    }
}
