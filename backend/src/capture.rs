use base64::Engine;
use base64::prelude::BASE64_STANDARD;

use chrono::Local;

use rocket::{Data, State};
use rocket::data::ToByteUnit;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;

use uuid::Uuid;

use std::collections::HashMap;
use std::sync::{Mutex, LazyLock};

use crate::EventChannels;
use crate::models::{CapturedRequest, HttpMethod};
use crate::models::CapturedRequestFetch;

static CAPTURED_REQUESTS: LazyLock<Mutex<Vec<CapturedRequest>>> = LazyLock::new(|| { Mutex::new(Vec::new()) });

// That is needed because the frontend does not know rocket::http::Method enum.
fn convert_http_method_to_model(method: rocket::http::Method) -> crate::models::HttpMethod {
    use rocket::http::Method;
    match method {
        Method::Get => HttpMethod::Get,
        Method::Head => HttpMethod::Head,
        Method::Options => HttpMethod::Options,
        Method::Trace => HttpMethod::Trace,
        Method::Put => HttpMethod::Put,
        Method::Delete => HttpMethod::Delete,
        Method::Post => HttpMethod::Post,
        Method::Patch => HttpMethod::Patch,
        Method::Connect => HttpMethod::Connect,
    }
}

/* 
 * Routes do not have access to their underlying Request.
 * We'll define here what data we capture, and how.
 */

// #[derive(Debug)]
// pub enum CaptureParsingError {  // Useless for now
//     UnknownError
// }

#[async_trait]
impl<'r> FromRequest<'r> for CapturedRequest {
    type Error = ();  // Might become CaptureParsingError 

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let headers = request.headers().iter().map(|h| {
            (h.name().to_string(), String::from(h.value()))
        }).collect::<HashMap<String, String>>();

        let query_params = request.query_fields()
            .map(|kv| {
                (kv.name.to_string(), kv.value.to_string())
            }).collect();
        
        let client_ip = if let Some(ip) = request.client_ip() { Some(ip.to_string()) } else { None };
        
        // "body" and "route" fields will be set by the route.
        Outcome::Success(CapturedRequest {
            uuid: Uuid::new_v4().simple().to_string(),
            method: convert_http_method_to_model(request.method()),
            route: None,
            headers: headers,
            query_parameters: query_params,
            timestamp: Local::now().format("%H:%M:%S %d/%m/%y").to_string(),
            client_ip: client_ip,
            body: None
        })
    }
}

// Complete and store CapturedRequest
pub async fn capture(req: CapturedRequest, route: &str, body: Data<'_>, channels: &State<EventChannels>) {
    // careful, Data::open consume the body bytes.
    let encoded_body = {
        match body.open(2.mebibytes()).into_bytes().await {
            Ok(b) => Some(BASE64_STANDARD.encode(b.into_inner())),
            _ => None
        }
    };
    let req = req.complete(route, encoded_body);

    let _ = channels.captured_reqs.send(serde_json::to_string(&req).unwrap());

    CAPTURED_REQUESTS.lock().unwrap().push(req);
}

pub fn get_current_requests_json() -> Json<CapturedRequestFetch> {
    // Returned copy of captured requests
    Json(CapturedRequestFetch {
        requests: CAPTURED_REQUESTS.lock().unwrap().clone()
    })
}