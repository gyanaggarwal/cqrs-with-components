mod models;
mod persistence;

use anyhow::Result;
use models::{CreateEmployeeModel, UpdateEmployeeModel, 
             CreateLocationModel, UpdateLocationModel,
             CreatePersonModel,   UpdatePersonModel};
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[tracing::instrument(name="handle_commands", skip_all)]
#[http_component]
fn handle_commands(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();

    router.post("/create_employee",      create_employee);
    router.post("/update_employee/:id",  update_employee);
    router.post("/delete_employee/:id",  delete_employee);
    router.post("/create_location",      create_location);
    router.post("/create_person",        create_person);
    router.post("/update_location/:lid", update_location);
    router.post("/update_person/:pid",   update_person);
    router.post("/delete_person/:pid",   delete_person);
    router.any("*", fallback);
    Ok(router.handle(req))
}

#[tracing::instrument(name="create_employee", skip_all)]
fn create_employee(req: Request, _: Params) -> Result<impl IntoResponse> {
    let model: CreateEmployeeModel = serde_json::from_slice(req.body())?;

    let created = persistence::create_employee(model)?;
    let b = serde_json::to_vec(&created)?;
    Ok(ResponseBuilder::new(201)
        .header("Content-Type", "application/json")
        .body(b)
        .build())
}

#[tracing::instrument(name="update_employee", skip_all)]
fn update_employee(req: Request, params: Params) -> Result<impl IntoResponse> {
    let model: UpdateEmployeeModel = serde_json::from_slice(req.body())?;

    let updated = match params.get("id") {
        Some(id) => persistence::update_employee_by_id(id, model)?,
        None => return Ok(Response::new(400, ())),
    };
    match updated {
        Some(u) => {
            let b = serde_json::to_vec(&u)?;
            Ok(ResponseBuilder::new(200)
                .header("Content-Type", "application/json")
                .body(b)
                .build())
        }
        None => Ok(Response::new(404, "Not Found")),
    }
}

#[tracing::instrument(name="delete_employee", skip_all)]
fn delete_employee(_req: Request, params: Params) -> Result<impl IntoResponse> {
    match params.get("id") {
        Some(id) => match persistence::delete_employee_by_id(id)? {
            true => Ok(Response::new(204, ())),
            false => Ok(Response::new(404, ())),
        },
        None => Ok(Response::new(400, "Bad Request")),
    }
}

fn create_location(req: Request, _: Params) -> Result<impl IntoResponse> {
    let model: CreateLocationModel = serde_json::from_slice(req.body())?;

    let created = persistence::create_location(model)?;
    let b = serde_json::to_vec(&created)?;
    Ok(ResponseBuilder::new(201)
        .header("Content-Type", "application/json")
        .body(b)
        .build())
}

fn create_person(req: Request, _: Params) -> Result<impl IntoResponse> {
    let model: CreatePersonModel = serde_json::from_slice(req.body())?;

    let created = persistence::create_person(model)?;
    let b = serde_json::to_vec(&created)?;
    Ok(ResponseBuilder::new(201)
        .header("Content-Type", "application/json")
        .body(b)
        .build())
}

fn update_location(req: Request, params: Params) -> Result<impl IntoResponse> {
    let model: UpdateLocationModel = serde_json::from_slice(req.body())?;

    let updated = match params.get("lid") {
        Some(lid) => persistence::update_location_by_id(lid, model)?,
        None => return Ok(Response::new(400, ())),
    };
    match updated {
        Some(u) => {
            let b = serde_json::to_vec(&u)?;
            Ok(ResponseBuilder::new(200)
                .header("Content-Type", "application/json")
                .body(b)
                .build())
        }
        None => Ok(Response::new(404, "Not Found")),
    }
}

fn update_person(req: Request, params: Params) -> Result<impl IntoResponse> {
    let model: UpdatePersonModel = serde_json::from_slice(req.body())?;

    let updated = match params.get("pid") {
        Some(pid) => persistence::update_person_by_id(pid, model)?,
        None => return Ok(Response::new(400, ())),
    };
    match updated {
        Some(u) => {
            let b = serde_json::to_vec(&u)?;
            Ok(ResponseBuilder::new(200)
                .header("Content-Type", "application/json")
                .body(b)
                .build())
        }
        None => Ok(Response::new(404, "Not Found")),
    }
}

fn delete_person(_req: Request, params: Params) -> Result<impl IntoResponse> {
    match params.get("pid") {
        Some(pid) => match persistence::delete_person_by_id(pid)? {
            true => Ok(Response::new(204, ())),
            false => Ok(Response::new(404, ())),
        },
        None => Ok(Response::new(400, "Bad Request")),
    }
}

#[tracing::instrument(name="fallback", skip_all)]
fn fallback(req: Request, _: Params) -> Result<impl IntoResponse> {
    println!("commands:fallback {}:{}", req.method(), req.uri());
    Ok(Response::new(404, ()))
}
