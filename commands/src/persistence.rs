use anyhow::Result;
use spin_sdk::sqlite::{Connection, Value};
use uuid::Uuid;

use crate::models::{
    AddressCreatedModel, AddressUpdatedModel, CreateEmployeeModel, EmployeeCreatedModel,
    EmployeeUpdatedModel, UpdateEmployeeModel,
    CreateLocationModel, UpdateLocationModel, LocationCreatedModel, LocationUpdatedModel,
    CreatePersonModel, UpdatePersonModel, PersonCreatedModel, PersonUpdatedModel
};

const COMMAND_CREATE_EMPLOYEE: &str =
    "INSERT INTO Employees (Id, FirstName, LastName) VALUES (?,?,?);";
const COMMAND_CREATE_ADDRESS: &str =
    "INSERT INTO Addresses (EmployeeId, Street, Zip, City) VALUES (?,?,?,?);";
const COMMAND_CREATE_PERSON: &str =
    "INSERT INTO Persons (Pid, FirstName, LastName, Plid) VALUES (?, ?, ?, ?);";
const COMMAND_CREATE_LOCATION: &str =
    "INSERT INTO Locations (Lid, Street, Zip, City) VALUES (?, ?, ?, ?);";

const COMMAND_UPDATE_EMPLOYEE: &str =
    "UPDATE Employees SET FirstName = ?, LastName = ? WHERE Id = ?; RETURNING Id;";
const COMMAND_UPDATE_ADDRESS: &str =
    "UPDATE Addresses SET Street = ?, Zip = ?, City = ? WHERE EmployeeId = ? RETURNING EmployeeId";
const COMMAND_UPDATE_PERSON: &str =
    "UPDATE Persons SET FirstName = ?, LastName = ?, Plid = ? WHERE Pid = ? RETURNING Pid";
const COMMAND_UPDATE_LOCATION: &str =
    "UPDATE Locations SET Street = ?, Zip = ?, City = ? WHERE Lid = ? RETURNING Lid";

const COMMAND_DELETE_EMPLOYEE: &str = 
    "DELETE FROM Employees WHERE Id = ? RETURNING Id";
const COMMAND_DELETE_PERSON: &str = 
    "DELETE FROM Persons WHERE Pid = ? RETURNING Pid";

pub(crate) fn create_employee(model: CreateEmployeeModel) -> Result<EmployeeCreatedModel> {
    let con = Connection::open_default()?;
    let id = Uuid::new_v4();
    let employee_params = [
        Value::Text(id.to_string()),
        Value::Text(model.first_name.clone()),
        Value::Text(model.last_name.clone()),
    ];
    let address_params = [
        Value::Text(id.to_string()),
        Value::Text(model.address.street.clone()),
        Value::Text(model.address.zip.clone()),
        Value::Text(model.address.city.clone()),
    ];
    let _ = con.execute("BEGIN TRANSACTION;", &[]);
    let _ = con.execute(COMMAND_CREATE_EMPLOYEE, &employee_params)?;
    let _ = con.execute(COMMAND_CREATE_ADDRESS, &address_params);
    let _ = con.execute("END TRANSACTION;", &[]);
    Ok(EmployeeCreatedModel {
        id: id.to_string(),
        first_name: model.first_name,
        last_name: model.last_name,
        address: AddressCreatedModel {
            id: id.to_string(),
            street: model.address.street,
            zip: model.address.zip,
            city: model.address.city,
        },
    })
}

pub(crate) fn delete_employee_by_id(id: &str) -> Result<bool> {
    let con = Connection::open_default()?;
    let params = [Value::Text(id.to_string())];
    let query_result = con.execute(COMMAND_DELETE_EMPLOYEE, &params)?;
    let count = query_result.rows().count();
    Ok(count > 0)
}

pub(crate) fn update_employee_by_id(id: &str,
                                    model: UpdateEmployeeModel) -> Result<Option<EmployeeUpdatedModel>> {
    let con = Connection::open_default()?;
    let employee_params = [
        Value::Text(model.first_name.clone()),
        Value::Text(model.last_name.clone()),
        Value::Text(id.to_string()),
    ];
    let address_params = [
        Value::Text(model.address.street.clone()),
        Value::Text(model.address.zip.clone()),
        Value::Text(model.address.city.clone()),
        Value::Text(id.to_string()),
    ];
    let _ = con.execute("BEGIN TRANSACTION;", &[]);
    let _ = con.execute(COMMAND_UPDATE_EMPLOYEE, &employee_params)?;
    let _ = con.execute(COMMAND_UPDATE_ADDRESS, &address_params)?;
    let _ = con.execute("END TRANSACTION;", &[])?;
    Ok(Some(EmployeeUpdatedModel {
        id: id.to_string(),
        first_name: model.first_name,
        last_name: model.last_name,
        address: AddressUpdatedModel {
            id: id.to_string(),
            street: model.address.street,
            zip: model.address.zip,
            city: model.address.city,
        },
    }))
}

pub(crate) fn create_location(model: CreateLocationModel) -> Result<LocationCreatedModel> {
    let con = Connection::open_default()?;
    let lid = Uuid::new_v4();
    let params = [
        Value::Text(lid.to_string()),
        Value::Text(model.street.clone()),
        Value::Text(model.zip.clone()),
        Value::Text(model.city.clone()),
    ]; 

    let _ = con.execute(COMMAND_CREATE_LOCATION, &params)?;

    Ok(LocationCreatedModel{
        lid: lid.to_string(),
        street: model.street,
        zip: model.zip,
        city: model.city
    })
}

pub(crate) fn create_person(model: CreatePersonModel) -> Result<PersonCreatedModel> {
    let con = Connection::open_default()?;
    let pid = Uuid::new_v4();
    let params = [
        Value::Text(pid.to_string()),
        Value::Text(model.first_name.clone()),
        Value::Text(model.last_name.clone()),
        Value::Text(model.plid.clone())
    ];

    let _ = con.execute(COMMAND_CREATE_PERSON, &params)?;

    Ok(PersonCreatedModel{
        pid: pid.to_string(),
        first_name: model.first_name,
        last_name: model.last_name,
        plid: model.plid

    })
}

pub(crate) fn update_location_by_id(lid: &str,
                                    model: UpdateLocationModel) -> Result<Option<LocationUpdatedModel>> {
    let con = Connection::open_default()?;
    let params = [
        Value::Text(model.street.clone()),
        Value::Text(model.zip.clone()),
        Value::Text(model.city.clone()),
        Value::Text(lid.to_string().clone())
    ];

    let _ = con.execute(COMMAND_UPDATE_LOCATION, &params)?;

    Ok(Some(LocationUpdatedModel{
        lid: lid.to_string(),
        street: model.street,
        zip: model.zip,
        city: model.city
    }))                                       
}

pub(crate) fn update_person_by_id(pid: &str,
                                  model: UpdatePersonModel) -> Result<Option<PersonUpdatedModel>> {
    let con = Connection::open_default()?;
    let params = [
        Value::Text(model.first_name.clone()),
        Value::Text(model.last_name.clone()),
        Value::Text(model.plid.clone()),
        Value::Text(pid.to_string().clone())
    ];
                                
    let _ = con.execute(COMMAND_UPDATE_PERSON, &params)?;
                                
    Ok(Some(PersonUpdatedModel{
        pid: pid.to_string(),
        first_name: model.first_name,
        last_name: model.last_name,
        plid: model.plid
    }))                                                       
}

pub(crate) fn delete_person_by_id(pid: &str) -> Result<bool> {
    let con = Connection::open_default()?;
    let params = [Value::Text(pid.to_string())];
    let query_result = con.execute(COMMAND_DELETE_PERSON, &params)?;
    let count = query_result.rows().count();
    Ok(count > 0)
}
