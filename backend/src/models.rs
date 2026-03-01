/* 
 * Request models.
 * This file content should be the same as the frontend models.rs one,
 * except for the dependencies.
 */

use std::collections::HashMap;
use serde::{Serialize, Deserialize};  // serde crate is needed anyway by the backend: standardize dependency.


#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Head,
    Options,
    Trace,
    Put,
    Delete,
    Post,
    Patch,
    Connect,
}


#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct RouteDefinition {
    pub route: String,
    pub description: String,

    pub status_code: u16,
    pub content_type: String,

    pub response_body: String,
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CapturedRequest {
    // See backend/src/routes.rs for impl of rocket::FromRequest trait
    pub uuid: String,
    pub method: HttpMethod,
    pub route: Option<String>,
    pub headers: HashMap<String, String>,
    pub query_parameters: HashMap<String, String>,
    pub timestamp: String,
    pub client_ip: Option<String>,
    pub body: Option<String>, // base64-encoded body ; might be null
}

#[allow(dead_code)] // Only used by the backend
impl CapturedRequest {
    pub fn complete(self: Self, route: &str, body: Option<String>) -> Self {
        CapturedRequest {
            uuid: self.uuid,
            method: self.method,
            route: Some(String::from(route)),
            headers: self.headers,
            query_parameters: self.query_parameters,
            timestamp: self.timestamp, 
            client_ip: self.client_ip,
            body: body
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateRouteRequest {
    pub target_route: Option<String>,
    pub definition: RouteDefinition,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteRouteRequest {
    pub route: String,
}

#[derive(Serialize, Deserialize)]
pub struct CapturedRequestFetch {
    pub requests: Vec<CapturedRequest>,
}

#[derive(Serialize, Deserialize)]
pub struct RouteListRequest {
    pub routes: Vec<RouteDefinition>,
}