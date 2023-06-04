# nERD

> The nerdy Entity Relationship Diagram powered by Rust

## Goal

- [ ] Parse SQL and get AST
- [ ] Show ERD like below

| SQL                                                                                        | ERD  |
| ------------------------------------------------------------------------------------------ | ---- |
| CREATE TABLE Dept (id INT, name TEXT);                                                     | Dept |
| CREATE TABLE Emp (id INT, name TEXT, deptId INT, FOREIGN KEY (deptId) REFERENCES Dept(id); | Emp  |
