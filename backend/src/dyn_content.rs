// File to manage dynamically added content
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::LazyLock;

use rocket::http::Status;

use crate::models::*;


static PAGES: LazyLock<Mutex<HashMap<String, RouteDefinition>>> = LazyLock::new(|| { Mutex::new(HashMap::new()) });


pub fn get_content(path: &str) -> Option<RouteDefinition> {
    // TODO refine response. For now, simply ignore RouteDefinition other fields.
    PAGES.lock().unwrap().get(path).cloned()
}

fn is_illegal_route(route: &String) -> bool {
    // Whether this route is a right dynamic route and the user should be able to add, modify or delete.
    !route.starts_with("/") || route.starts_with("/backapi") || route.starts_with("/front")
}

pub fn add_dyn_route(UpdateRouteRequest {target_route, definition: def}: &UpdateRouteRequest) -> Result<bool, Status> {
    // Returns true if added, false if updated.

    let route_change = if let Some(target) = target_route {
        *target != def.route
    } else { false };

    // Both the target and the "new" route should be legal
    let is_illegal = match route_change {
        // Both the target and the "new" route should be legal
        true => is_illegal_route(&def.route) || is_illegal_route(target_route.as_ref().unwrap()),

        // No route change: no need to check twice for the same value.
        false => is_illegal_route(&def.route)
    };

    if is_illegal {
        Err(Status::BadRequest)
    } else {
        let mut map = PAGES.lock().unwrap();
        if route_change { // Route change
            map.remove(target_route.as_ref().unwrap());
        }
        Ok(map.insert(def.route.clone(), def.clone()) == None)
    }
}

pub fn delete_dyn_route(req: &DeleteRouteRequest) -> Status {
    if is_illegal_route(&req.route) {
        Status::BadRequest
    } else {
        PAGES.lock().unwrap().remove(&req.route);
        Status::Ok
    }
}

pub fn get_dyn_routes() -> RouteListRequest {
    RouteListRequest {
        routes: PAGES.lock().unwrap().iter().map(|(_, def)| {
            def.clone()
        }).collect()
    }
}