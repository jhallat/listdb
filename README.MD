# ListDB

_A simple database for **small** data_

The goal of ListDB is to provide a lightweight database that executes CRUD operations on small lists. There are three implementations in progress: Rust, Crystal, and Ballerina. The purpose of the three implementations is to act as a comparison of the three languages to determine their suitability as a language for developing a database.

The first phase of the database is a simple command line interface. In the second phase, the project will be separated into a CLI and server.

## Currently Implemented Functions in CLI

### CREATE _\<target\> \<id\>_

**target** := The object type to be created. Currently only "TOPIC" is supported.  
**id** := a unique name for the object to be created.

### LIST

### STATUS

### OPEN _\<target\> \<id\>_

### EXIT
