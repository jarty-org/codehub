use rocket::response::status;

#[get("/")]
pub fn not_found() ->  status::NotFound<&'static str> {
    return status::NotFound("404 page not found")
}

#[options("/<_..>")]
pub fn all_options() {
}
