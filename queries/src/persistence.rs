use anyhow::anyhow;
use spin_sdk::sqlite::{Connection, Value};
use spin_sdk::http::{IntoResponse, Params, Response};

use crate::models::{AddressDetailsModel, EmployeeDetailsModel, EmployeeListModel,
                    LocationDetailsModel, PersonDetailsModel, PersonListModel};

const QUERY_ALL_EMPLOYEE_COMMAND: &str =
    "SELECT Employees.Id, Employees.LastName || ', ' || Employees.FirstName Name, Addresses.City FROM Employees INNER JOIN Addresses ON Employees.Id = Addresses.EmployeeId ORDER BY NAME ASC";
const QUERY_SINGLE_EMPLOYEE_COMMAND: &str = 
    "SELECT Employees.Id, Employees.FirstName, Employees.LastName, Addresses.Street, Addresses.Zip, Addresses.City FROM Employees INNER JOIN Addresses ON Employees.Id = Addresses.EmployeeId WHERE Employees.Id = ?";
const QUERY_ALL_PERSON_COMMAND: &str =
    "SELECT Persons.Pid, Persons.LastName || ', ' || Persons.FirstName Name, Locations.City FROM Locations INNER JOIN Persons ON Locations.Lid = Persons.Plid ORDER BY NAME ASC";
const QUERY_SINGLE_PERSON_COMMAND: &str = 
    "SELECT Persons.Pid, Persons.FirstName, Persons.LastName, Locations.Lid, Locations.Street, Locations.Zip, Locations.City FROM Locations INNER JOIN Persons ON Locations.Lid = Persons.Plid WHERE Persons.Pid = ?";
const QUERY_SINGLE_LOCATION_COMMAND: &str = 
    "SELECT Lid, Street, Zip, City FROM Locations WHERE Lid = ?";
const QUERY_ALL_LOCATION_COMMAND: &str = 
    "SELECT Lid, Street, Zip, City FROM Locations ORDER BY City";

pub fn pall_employees() -> anyhow::Result<impl IntoResponse> {
    let con = Connection::open_default()?;
    let query_result = con.execute(QUERY_ALL_EMPLOYEE_COMMAND, &[])?;
 
    let products: Vec<_> = query_result
        .rows()
        .map(|row| {
            let id = String::from(
                row.get::<&str>("Id")
                    .ok_or_else(|| anyhow!("Employees.Id not present"))?,
            );
            let name = String::from(
                row.get::<&str>("Name")
                    .ok_or_else(|| anyhow!("Name not present"))?,
            );
            let city = String::from(
                row.get::<&str>("City")
                    .ok_or_else(|| anyhow!("Addresses.City not present"))?,
            );
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

pub fn pemployee_by_id(params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };

    let con = Connection::open_default()?;
    let id = [Value::Text(id.to_string())];
    let query_result = con.execute(QUERY_SINGLE_EMPLOYEE_COMMAND, &id)?;

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

pub fn pall_locations() -> anyhow::Result<impl IntoResponse> {
    let con = Connection::open_default()?;
    let query_result = con.execute(QUERY_ALL_LOCATION_COMMAND, &[])?;
 
    let products: Vec<_> = query_result
        .rows()
        .map(|row| {
            let lid = String::from(
                row.get::<&str>("Lid")
                   .ok_or_else(|| anyhow!("Lid not present"))?,
            );
            let street = String::from(
                row.get::<&str>("Street")
                    .ok_or_else(|| anyhow!("Street not present"))?,
            );
            let zip = String::from(
                row.get::<&str>("Zip")
                   .ok_or_else(|| anyhow!("Zip not present"))?,
            );
            let city = String::from(
                row.get::<&str>("City")
                   .ok_or_else(|| anyhow!("City not present"))?,
            );
            anyhow::Ok(LocationDetailsModel { lid, street, zip, city })
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

pub fn plocation_by_id(params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(lid) = params.get("lid") else {
        return Ok(Response::new(400, ()));
    };

    let con = Connection::open_default()?;
    let lid = [Value::Text(lid.to_string())];
    let query_result = con.execute(QUERY_SINGLE_LOCATION_COMMAND, &lid)?;

    let products: Vec<_> = query_result
        .rows()
        .map(|row| {
            let lid = String::from(
                row.get::<&str>("Lid")
                    .ok_or_else(|| anyhow!("Lid not present"))?,
            );
            let street = String::from(
                row.get::<&str>("Street")
                    .ok_or_else(|| anyhow!("Street not present"))?,
            );
            let zip = String::from(
                row.get::<&str>("Zip")
                    .ok_or_else(|| anyhow!("Zip not present"))?,
            );
            let city = String::from(
                row.get::<&str>("City")
                    .ok_or_else(|| anyhow!("City not present"))?,
            );
            anyhow::Ok(LocationDetailsModel{lid, street, zip, city})
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

pub fn pperson_by_id(params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(pid) = params.get("pid") else {
        return Ok(Response::new(400, ()));
    };

    let con = Connection::open_default()?;
    let pid = [Value::Text(pid.to_string())];
    let query_result = con.execute(QUERY_SINGLE_PERSON_COMMAND, &pid)?;
  
    let products: Vec<_> = query_result
        .rows()
        .map(|row| {
            let pid = String::from(
                row.get::<&str>("Pid")
                    .ok_or_else(|| anyhow!("Pid not present"))?,
            );
            let first_name = String::from(
                row.get::<&str>("FirstName")
                    .ok_or_else(|| anyhow!("FirstName not present"))?,
            );
            let last_name = String::from(
                row.get::<&str>("LastName")
                    .ok_or_else(|| anyhow!("LastName not present"))?,
            );
            let lid = String::from(
                row.get::<&str>("Lid")
                    .ok_or_else(|| anyhow!("Lid not present"))?,
            );
            let street = String::from(
                row.get::<&str>("Street")
                    .ok_or_else(|| anyhow!("Street not present"))?,
            );
            let zip = String::from(
                row.get::<&str>("Zip")
                    .ok_or_else(|| anyhow!("Zip not present"))?,
            );   
            let city = String::from(
                row.get::<&str>("City")
                    .ok_or_else(|| anyhow!("Locations.City not present"))?,
            );
            anyhow::Ok(PersonDetailsModel{
                                pid, 
                                first_name, 
                                last_name,
                                address: LocationDetailsModel{
                                    lid, 
                                    street, 
                                    zip, city }})
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

pub fn pall_persons() -> anyhow::Result<impl IntoResponse> {
    let con = Connection::open_default()?;
    let query_result = con.execute(QUERY_ALL_PERSON_COMMAND, &[])?;
    let products: Vec<_> = query_result
        .rows()
        .map(|row| {
            let pid = String::from(
                row.get::<&str>("Pid")
                    .ok_or_else(|| anyhow!("Pid not present"))?,
            );
            let name = String::from(
                row.get::<&str>("Name")
                    .ok_or_else(|| anyhow!("Name not present"))?,
            );
            let city = String::from(
                row.get::<&str>("City")
                    .ok_or_else(|| anyhow!("Locations.City not present"))?,
            );
            anyhow::Ok(PersonListModel { pid, name, city })
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

