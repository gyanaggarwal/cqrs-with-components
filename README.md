# cqrs-with-components
CQRS with WASM components using Spin

It has 3 components

1. gateway -> interface to the system

2. commands -> executes user update Commnads

3. queries -> executes user Queries


                            http request
                                  |
                                  |
                                  |
                                  V
                ---------------------------------------
                                 gateway 
                ---------------------------------------
                    |                            |
                    |                            |
                    |                            |
                    V                            V
       ----------------------            ----------------------
             queries                           commnads
       ----------------------            ----------------------

CREATE TABLE Persons (
    Pid VARCHAR(36) NOT NULL, 
    FirstName TEXT NOT NULL, 
    LastName TEXT NOT NULL,
    Plid VARCHAR(36) NOT NULL,
    PRIMARY KEY (Pid),
    FOREIGN KEY (Plid) REFERENCES Locations (Lid)
);

CREATE TABLE Locations (
    Lid VARCHAR(36) NOT NULL,
    Street VARCHAR(50) NOT NULL,
    Zip VARCHAR(10) NOT NULL,
    City VARCHAR(50) NOT NULL,
    PRIMARY KEY (Lid)
);

There is many-to-one relationship between Persons and Locations

CREATE TABLE Employees (
    Id VARCHAR(36) NOT NULL, 
    FirstName TEXT NOT NULL, 
    LastName TEXT NOT NULL,
    PRIMARY KEY (Id)
);

CREATE TABLE Addresses (
    EmployeeId VARCHAR(36) NOT NULL,
    Street VARCHAR(50) NOT NULL,
    Zip VARCHAR(10) NOT NULL,
    City VARCHAR(50) NOT NULL,
    FOREIGN KEY (EmployeeId) REFERENCES Employees (Id)
        ON DELETE CASCADE
);

There is one-to-one relationship between Employees and Addresses