extern crate iron;
extern crate router;
extern crate logger;

use iron::prelude::*;
use iron::status;
use router::Router;
use logger::Logger;

enum Role {
    ADMIN,
    USER
}

struct User {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
    roles: Vec<Role>,
    enabled: bool,
    locked: bool,
    validation: String
}

struct Login {
    email: String,
    password: String
}


fn main() {
    let (logger_before, logger_after) = Logger::new(None);

    let mut router = Router::new();
    router.get("/users", get_users_handler, "get_users");
    router.post("/users", post_users_handler, "post_users");
    router.get("/users/:id", get_users_id_handler, "get_users_id");
    router.patch("/users/:id", patch_users_id_handler, "patch_users_id");
    router.delete("/users/:id", delete_users_id_handler, "delete_users_id");
    router.post("/users/login", login_handler, "login");
    router.get("/users/logout", logout_handler, "logout");
    router.get("/users/validate", validate_handler, "validate");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain)
        .http("localhost:3000")
        .unwrap();

    fn get_users_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }

    fn post_users_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }

    fn get_users_id_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }

    fn patch_users_id_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }

    fn delete_users_id_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }

    fn login_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }

    fn logout_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }

    fn validate_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }
}
