use anyhow::Result;
use spin_sdk::http::{
    send, HeaderValue, IntoResponse, Params, Request, RequestBuilder, Response, ResponseBuilder,
    Router,
};
use spin_sdk::http_component;

const QUERY_ROOT_URL: &str = "https://queries.spin.internal";
const COMMAND_ROOT_URL: &str = "https://commands.spin.internal";

#[tracing::instrument(name="execute_command", skip_all)]
async fn execute_command(url: String,
                         content_type: Option<&HeaderValue>,
                         payload: Option<Vec<u8>>) -> Result<Response> {
    let req: Request = match content_type {
        Some(ct) => RequestBuilder::new(spin_sdk::http::Method::Post, url)
            .header("Accept", "application/json")
            .header("Content-Type", ct.as_str().unwrap())
            .body(payload)
            .build(),
        None => RequestBuilder::new(spin_sdk::http::Method::Post, url)
            .header("Accept", "application/json")
            .body(())
            .build(),
    };

    let res: Response = send(req).await?;
    parse_result(res)
}

#[tracing::instrument(name="execute_query", skip_all)]
async fn execute_query(url: &str) -> Result<Response> {
    let req: Request = RequestBuilder::new(spin_sdk::http::Method::Get, url)
        .header("Accept", "application/json")
        .build();
    let res: Response = send(req).await?;
    parse_result(res)
}

#[tracing::instrument(name="parse_result", skip_all)]
fn parse_result(res: Response) -> Result<Response> {
    match res.status() {
        300..=499 => Ok(Response::new(*res.status(), ())),
        500..=599 => {
            println!("{}", String::from_utf8_lossy(res.body()));
            Ok(Response::new(500, "Internal Server Error"))
        }
        200 | 201 | 204 => Ok(ResponseBuilder::new(*res.status())
            .header("Content-Type", "application/json")
            .body(res.into_body())
            .build()),
        _ => {
            println!("{}", String::from_utf8_lossy(res.body()));
            Ok(Response::new(*res.status(), ()))
        }
    }
}

#[tracing::instrument(name="create_employee", skip_all)]
async fn create_employee(req: Request, _: Params) -> Result<impl IntoResponse> {
    let url = format!("{}/create_employee", COMMAND_ROOT_URL);
    execute_command(url, req.header("content-type"), Some(req.body().to_vec())).await
}

async fn create_location(req: Request, _: Params) -> Result<impl IntoResponse> {
    let url = format!("{}/create_location", COMMAND_ROOT_URL);
    execute_command(url, req.header("content-type"), Some(req.body().to_vec())).await
}

async fn create_person(req: Request, _: Params) -> Result<impl IntoResponse> {
    let url = format!("{}/create_person", COMMAND_ROOT_URL);
    execute_command(url, req.header("content-type"), Some(req.body().to_vec())).await
}

#[tracing::instrument(name="update_employee_by_id", skip_all)]
async fn update_employee_by_id(req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };
    let url = format!("{}/update_employee/{}", COMMAND_ROOT_URL, id);
    let ct = req.header("content-type");
    execute_command(url, ct, Some(req.body().to_vec())).await
}

async fn update_location_by_id(req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("lid") else {
        return Ok(Response::new(400, ()));
    };
    let url = format!("{}/update_location/{}", COMMAND_ROOT_URL, id);
    let ct = req.header("content-type");
    execute_command(url, ct, Some(req.body().to_vec())).await
}

async fn update_person_by_id(req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("pid") else {
        return Ok(Response::new(400, ()));
    };
    let url = format!("{}/update_person/{}", COMMAND_ROOT_URL, id);
    let ct = req.header("content-type");
    execute_command(url, ct, Some(req.body().to_vec())).await
}

#[tracing::instrument(name="delete_employee_by_id", skip_all)]
async fn delete_employee_by_id(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };
    let url = format!("{}/delete_employee/{}", COMMAND_ROOT_URL, id);
    execute_command(url, None, None).await
}

async fn delete_person_by_id(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("pid") else {
        return Ok(Response::new(400, ()));
    };
    let url = format!("{}/delete_person/{}", COMMAND_ROOT_URL, id);
    execute_command(url, None, None).await
}

#[tracing::instrument(name="get_employee_by_id", skip_all)]
async fn get_employee_by_id(_req: Request, params: Params) -> Result<impl IntoResponse> {
    match params.get("id") {
        Some(id) => {
            let url = format!("{}/employees/{}", QUERY_ROOT_URL, id);
            execute_query(url.as_str()).await
        }
        None => Ok(Response::new(200, ())),
    }
}

#[tracing::instrument(name="get_employees", skip_all)]
async fn get_employees(_req: Request, _: Params) -> Result<impl IntoResponse> {
    let url = format!("{}/employees", QUERY_ROOT_URL);
    execute_query(url.as_str()).await
}

#[tracing::instrument(name="get_locations", skip_all)]
async fn get_locations(_req: Request, _: Params) -> Result<impl IntoResponse> {
    let url = format!("{}/locations", QUERY_ROOT_URL);
    execute_query(url.as_str()).await
}

#[tracing::instrument(name="get_persons", skip_all)]
async fn get_persons(_req: Request, _: Params) -> Result<impl IntoResponse> {
    let url = format!("{}/persons", QUERY_ROOT_URL);
    execute_query(url.as_str()).await
}

#[tracing::instrument(name="get_location_by_id", skip_all)]
async fn get_location_by_id(_req: Request, params: Params) -> Result<impl IntoResponse> {
    match params.get("lid") {
        Some(lid) => {
            let url = format!("{}/locations/{}", QUERY_ROOT_URL, lid);
            execute_query(url.as_str()).await
        }
        None => Ok(Response::new(200, ())),
    }
}

#[tracing::instrument(name="get_person_by_id", skip_all)]
async fn get_person_by_id(_req: Request, params: Params) -> Result<impl IntoResponse> {
    match params.get("pid") {
        Some(pid) => {
            let url = format!("{}/persons/{}", QUERY_ROOT_URL, pid);
            execute_query(url.as_str()).await
        }
        None => Ok(Response::new(200, ())),
    }
}

#[tracing::instrument(name="handle_gateway", skip_all)]
#[http_component]
fn handle_gateway(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();

    router.get_async("/employees",        get_employees);
    router.get_async("/employees/:id",    get_employee_by_id);
    router.get_async("/locations",        get_locations);
    router.get_async("/locations/:lid",   get_location_by_id);
    router.get_async("/persons",          get_persons);
    router.get_async("/persons/:pid",     get_person_by_id);
 
    router.post_async("/employees",       create_employee);
    router.put_async("/employees/:id",    update_employee_by_id);
    router.delete_async("/employees/:id", delete_employee_by_id);

    router.post_async("/locations",       create_location);
    router.put_async("/locations/:lid",   update_location_by_id);

    router.post_async("/persons",         create_person);
    router.put_async("/persons/:pid",     update_person_by_id);
    router.delete_async("/persons/:pid",  delete_person_by_id);

    Ok(router.handle(req))
}