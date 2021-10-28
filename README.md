# Rust MongoDb Warp API Example

---

## Overview

This is an example project that uses Rust, Warp and MongoDb to build an api.

#### Database

Im using a "dockerized" MongoDb database so we just need to run `docker-compose up` to spinup the database and thats it.

#### Server

Once the app is started, conntect to the port `8080` => `http://localhost:8080/`

#### Routes

_Obs: There's also a Postman file in the root of the project with a collection of all routes documented._

##### ==== Tasks ====

```
/tasks
    - POST -> create new task

/tasks/all
    - GET -> list all tasks

/tasks/{id}
    - GET -> find task by id
    - PUT -> edit task
    - DELETE -> delete task

tasks/dangerously-delete-all-tasks
    - DELETE -> delete all tasks
```

##### ==== Projecs ====

```
/projects
    - POST -> create new project

/projects/all
    - GET -> list all projects

/projects/{id}
    - GET -> find project by id
    - PUT -> edit project
    - DELETE -> delete project


projects/dangerously-delete-all-projects
    - DELETE -> delete all projects
```
