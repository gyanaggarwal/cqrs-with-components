mod models;

use anyhow::anyhow;
use spin_sdk::sqlite::{Connection, Value};
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

use models::{AddressDetailsModel, EmployeeDetailsModel, EmployeeListModel};
const QUERY_ALL_COMMAND: &str =
    "SELECT Employees.Id, Employees.LastName || ', ' || Employees.FirstName Name, Addresses.City FROM Employees INNER JOIN Addresses ON Employees.Id = Addresses.EmployeeId ORDER BY NAME ASC";
const QUERY_SINGLE_COMMAND: &str = "SELECT Employees.Id, Employees.FirstName, Employees.LastName, Addresses.Street, Addresses.Zip, Addresses.City FROM Employees INNER JOIN Addresses ON Employees.Id = Addresses.EmployeeId WHERE Employees.Id = ?";

#[tracing::instrument(name="handle_queries", skip_all)]
#[http_component]
fn handle_queries(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();

    // register routes for queries
    router.get("/employees", all_employees);
    router.get("/employees/:id", employee_by_id);

    // handle all the requests
    Ok(router.handle(req))
}

#[tracing::instrument(name="all_employees", skip_all)]
fn all_employees(_req: Request, _param: Params) -> anyhow::Result<impl IntoResponse> {
    let con = Connection::open_default()?;
    let query_result = con.execute(QUERY_ALL_COMMAND, &[])?;
 
    let products: Vec<_> = query_result
        .rows()
        .map(|row| {
            let id = String::from(
                row.get::<&str>("Id")
                    .ok_or_else(|| anyhow!("Employees.Id not present"))?,
            );
            println!("id {}", id);
            let name = String::from(
                row.get::<&str>("Name")
                    .ok_or_else(|| anyhow!("Name not present"))?,
            );
            println!("name {}", name);
            let city = String::from(
                row.get::<&str>("City")
                    .ok_or_else(|| anyhow!("Addresses.City not present"))?,
            );
            println!("city {}", city);
            anyhow::Ok(EmployeeListModel { id, name, city })
        })
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .collect();

    let payload = serde_json::to_vec(&products)?;
    Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(payload)
            .build())
    
}

#[tracing::instrument(name="employee_by_id", skip_all)]
fn employee_by_id(_req:Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };

    let con = Connection::open_default()?;
    let id = [Value::Text(id.to_string())];
    let query_result = con.execute(QUERY_SINGLE_COMMAND, &id)?;

    let products: Vec<_> = query_result
                            .rows()
                            .map(|row| {
                                let id = String::from(
                                    row.get::<&str>("Id")
                                        .ok_or_else(|| anyhow!("Employees.Id not present"))?,
                                );
                                let first_name = String::from(
                                    row.get::<&str>("FirstName")
                                        .ok_or_else(|| anyhow!("Employees.FirstName not present"))?,
                                );
                                let last_name = String::from(
                                    row.get::<&str>("LastName")
                                        .ok_or_else(|| anyhow!("Employees.LastName not present"))?,
                                );
                                let street = String::from(
                                    row.get::<&str>("Street")
                                        .ok_or_else(|| anyhow!("Addresses.Street not present"))?,
                                );
                                let zip = String::from(
                                    row.get::<&str>("Zip")
                                        .ok_or_else(|| anyhow!("Addresses.Zip not present"))?,
                                );
                                let city = String::from(
                                    row.get::<&str>("City")
                                        .ok_or_else(|| anyhow!("Addresses.City not present"))?,
                                );
                                anyhow::Ok(EmployeeDetailsModel {
                                    id: id.clone(),
                                    first_name,
                                    last_name,
                                    address: AddressDetailsModel {
                                        id: id.clone(),
                                        street,
                                        zip,
                                        city,
                                    },
                                })
                            })
                .filter(|item| item.is_ok())
                .map(|item| item.unwrap())
                .collect();

    let payload = serde_json::to_vec(&products)?;

    Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(payload)
            .build())
}
