extern crate iron;
extern crate router;

extern crate iron_diesel;
extern crate diesel;
use self::iron_diesel::*;
use self::iron_diesel::models::*;
use self::diesel::prelude::*;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use router::Router;

fn index_handler(req: &mut Request) -> IronResult<Response> {

    Ok(Response::with((status::Ok, "Welcome to the index page, visit /monkeys to see the monkeys")))

}

fn get_monkeys_handler(req: &mut Request) -> IronResult<Response> {

    use iron_diesel::schema::monkeys::dsl::*;

    let connection = db_connection();
    let results = monkeys.limit(20)
    .load::<Monkey>(&connection)
    .expect("Error loading monkeys");

    println!("The results: {:?}", results);

    let mut monkeys_formatted = "[".to_string();

    for monkey in results {

        let m = format!("{{ \"name\": \"{}\", \"color\": {:?} }}", monkey.name, monkey.color.unwrap_or("".to_string()));

        monkeys_formatted += &m;

    }

    monkeys_formatted += "]";

    println!("Formatted monkeys are {}", monkeys_formatted);

    Ok(Response::with((ContentType::json().0, status:: Ok, monkeys_formatted)))

}

fn main() {

    let mut router = Router::new();

    router.get("/", index_handler, "index");
    router.get("/monkeys", get_monkeys_handler, "monkeys");

    Iron::new(router).http("localhost:3000").unwrap();

}
