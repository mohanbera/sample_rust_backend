use actix_cors::Cors;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::collections::HashMap;

use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

#[derive(Serialize)]
struct User {
    id: i32,
    user_name: String,
    session_id: String,
    join_time: i32,
    room_id: String,
    is_captain: bool,
}

#[derive(Serialize)]
struct Room {
    room_id: String,
    users: Vec<User>,
    create_time: i32,
}

impl Room {
    fn remove_user_by_session_id(&mut self, session_id: String) {
        let mut index = -1 as i8;
        for user in self.users.iter() {
            index += 1;
            if user.session_id == session_id {
                break;
            }
        }

        if index != -1 {
            self.users.remove(index as usize);
        }
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
}

struct Rooms {
    rooms: HashMap<String, Room>,
}

impl Rooms {
    fn get_room_by_id(&self, room_id: &String) -> Option<&Room> {
        self.rooms.get(room_id)
    }

    fn add_room(&mut self, room_id: String, room: Room) -> bool {
        if self.rooms.contains_key(&room_id) {
            return false;
        }
        self.rooms.insert(room_id, room);
        true
    }

    fn remove_room_by_id(&mut self, room_id: &String) {
        self.rooms.remove(room_id);
    }

    fn check_if_room_exists(&self, room_id: &String) -> bool {
        self.rooms.contains_key(room_id)
    }
}

#[get("/api")]
async fn connect() -> impl Responder {
    "Hello!"
}

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
      

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("{:?}", msg);
                ctx.pong(&msg);
            }
            Ok(ws::Message::Text(text)) => {
                println!("{:?}", text);
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // Allow requests from any origin
                    .allow_any_method() // Allow any HTTP method (GET, POST, etc.)
                    .allow_any_header() // Allow any headers
                    .max_age(3600), // Cache the CORS response for 1 hour
            )
            .service(connect).route("/ws/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8082))?
    .run()
    .await
}
