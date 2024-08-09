use serde::{Deserialize, Serialize};

/// API Model for creating a new Employee
#[derive(Debug, Deserialize)]
pub struct CreateEmployeeModel {
    /// Employee first name
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Employee last name
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Employee address
    pub address: CreateAddressModel,
}

/// API Model for creating a new address
#[derive(Debug, Deserialize)]
pub struct CreateAddressModel {
    /// street
    pub street: String,
    /// zip code
    pub zip: String,
    /// city
    pub city: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePersonModel {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub plid: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLocationModel {
    pub street: String,
    pub zip: String,
    pub city: String,
}

/// API Model for updating an Employee
#[derive(Debug, Deserialize)]
pub struct UpdateEmployeeModel {
    /// first name
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// last name
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// address
    pub address: UpdateAddressModel,
}

/// API Model for updating an Address
#[derive(Debug, Deserialize)]
pub struct UpdateAddressModel {
    /// street
    pub street: String,

    /// zip code
    pub zip: String,

    /// city
    pub city: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePersonModel {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub plid: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLocationModel {
    pub street: String,
    pub zip: String,
    pub city: String,
}

/// Response Model for a newly created Employee
#[derive(Debug, Serialize)]
pub struct EmployeeCreatedModel {
    /// unique identifier
    pub id: String,

    /// first name
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// last name
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// address
    pub address: AddressCreatedModel,
}

/// API model for a newly created address
#[derive(Debug, Serialize)]
pub struct AddressCreatedModel {
    /// identifier
    pub id: String,

    /// street
    pub street: String,

    /// zip code
    pub zip: String,

    /// city
    pub city: String,
}

/// API model for an updated employee
#[derive(Debug, Serialize)]
pub struct EmployeeUpdatedModel {
    /// identifier
    pub id: String,
    #[serde(rename = "firstName")]

    /// first name
    pub first_name: String,
    #[serde(rename = "lastName")]

    /// last name
    pub last_name: String,

    /// address
    pub address: AddressUpdatedModel,
}

/// API model for an updated Address
#[derive(Debug, Serialize)]
pub struct AddressUpdatedModel {
    /// identifier
    pub id: String,

    /// street
    pub street: String,

    /// zip code
    pub zip: String,

    /// city
    pub city: String,
}

#[derive(Debug, Serialize)]
pub struct PersonCreatedModel {
    pub pid: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub plid: String    
}

#[derive(Debug, Serialize)]
pub struct LocationCreatedModel {
    pub lid: String,
    pub street: String,
    pub zip: String,
    pub city: String
}

#[derive(Debug, Serialize)]
pub struct PersonUpdatedModel {
    pub pid: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub plid: String 
}

#[derive(Debug, Serialize)]
pub struct LocationUpdatedModel {
    pub lid: String,
    pub street: String,
    pub zip: String,
    pub city: String
}