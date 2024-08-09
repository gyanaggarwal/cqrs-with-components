PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS Employees (
    Id VARCHAR(36) NOT NULL, 
    FirstName TEXT NOT NULL, 
    LastName TEXT NOT NULL,
    PRIMARY KEY (Id)
);

CREATE TABLE IF NOT EXISTS Addresses (
    EmployeeId VARCHAR(36) NOT NULL,
    Street VARCHAR(50) NOT NULL,
    Zip VARCHAR(10) NOT NULL,
    City VARCHAR(50) NOT NULL,
    FOREIGN KEY (EmployeeId) REFERENCES Employees (Id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Persons (
    Pid VARCHAR(36) NOT NULL, 
    FirstName TEXT NOT NULL, 
    LastName TEXT NOT NULL,
    Plid VARCHAR(36) NOT NULL,
    PRIMARY KEY (Pid),
    FOREIGN KEY (Plid) REFERENCES Locations (Lid)
);

CREATE TABLE IF NOT EXISTS Locations (
    Lid VARCHAR(36) NOT NULL,
    Street VARCHAR(50) NOT NULL,
    Zip VARCHAR(10) NOT NULL,
    City VARCHAR(50) NOT NULL,
    PRIMARY KEY (Lid)
);

INSERT INTO Employees(Id, FirstName, LastName)
SELECT '12a33c84-ee60-45a1-848d-428ad3259abc', 'John', 'Doe'
WHERE
NOT EXISTS (
SELECT Id FROM Employees WHERE Id = '12a33c84-ee60-45a1-848d-428ad3259abc');

INSERT INTO Addresses(EmployeeId, Street, Zip, City)
SELECT '12a33c84-ee60-45a1-848d-428ad3259abc', '1234 Main Street', '02112', 'Boston'
WHERE 
NOT EXISTS (
SELECT EmployeeId FROM Addresses WHERE EmployeeId = '12a33c84-ee60-45a1-848d-428ad3259abc');

