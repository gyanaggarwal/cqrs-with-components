mod models;
mod persistence;

use spin_sdk::http::{IntoResponse, Params, Request, Router};
use spin_sdk::http_component;

#[tracing::instrument(name="handle_queries", skip_all)]
#[http_component]
fn handle_queries(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();

    // register routes for queries
    router.get("/employees",      all_employees);
    router.get("/employees/:id",  employee_by_id);
    router.get("/locations",      all_locations);
    router.get("/persons",        all_persons);
    router.get("/locations/:lid", location_by_id);
    router.get("/persons/:pid",   person_by_id);
 
    // handle all the requests
    Ok(router.handle(req))
}

fn all_employees(_req: Request, _param: Params) -> anyhow::Result<impl IntoResponse> {
    persistence::pall_employees()
}

fn employee_by_id(_req:Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    persistence::pemployee_by_id(params)
}

fn all_locations(_req: Request, _param: Params) -> anyhow::Result<impl IntoResponse> {
    persistence::pall_locations()
}

fn all_persons(_req: Request, _param: Params) -> anyhow::Result<impl IntoResponse> {
    persistence::pall_persons()
}

fn location_by_id(_req:Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    persistence::plocation_by_id(params)
}

fn person_by_id(_req:Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    persistence::pperson_by_id(params)
}