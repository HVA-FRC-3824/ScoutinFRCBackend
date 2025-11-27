use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use scoutin_frc_backend::lib::{DatabaseInterface, DebugDatabase, MatchData, PitData};

// Global database
static DATABASE: DebugDatabase = DebugDatabase{};

// POST
#[post("/api/set_match_data")]
async fn set_match_data(data: web::Json<MatchData>) -> impl Responder // UPDATE IDENTIFIER
{
    // Unwrap is OK because both Result types are serializable
    HttpResponse::Ok().json(DATABASE.set_match_data(data.0).unwrap())
}


#[post("/api/set_pit_data")]
async fn set_pit_data(data: web::Json<PitData>) -> impl Responder // UPDATE IDENTIFIER
{
    // Unwrap is OK because both Result types are serializable
    HttpResponse::Ok().json(DATABASE.set_pit_data(data.0).unwrap())
}

// GET
#[get("/api/get_all_match_data")]
async fn get_all_match_data() -> impl Responder // UPDATE IDENTIFIER
{
    // Unwrap is OK because both Result types are serializable
    HttpResponse::Ok().json(DATABASE.get_all_pit_data().unwrap())
}

#[get("/api/get_all_pit_data")]
async fn get_all_pit_data() -> impl Responder // UPDATE IDENTIFIER
{
    // Unwrap is OK because both Result types are serializable
    HttpResponse::Ok().json(DATABASE.get_all_pit_data().unwrap())
}

#[get("/api/get_match_data")]
async fn get_match_data(identifier: web::Json<u32>) -> impl Responder // UPDATE IDENTIFIER
{
    // Unwrap is OK because both Result types are serializable
    HttpResponse::Ok().json(DATABASE.get_match_data(identifier.to_owned()).unwrap())
}


#[get("/api/get_pit_data")]
async fn get_pit_data(identifier: web::Json<u32>) -> impl Responder // UPDATE IDENTIFIER
{
    // Unwrap is OK because both Result types are serializable
    HttpResponse::Ok().json(DATABASE.get_pit_data(identifier.to_owned()).unwrap())
}

#[get("/api/index")]
async fn index() -> impl Responder 
{
    HttpResponse::Ok().json(DATABASE.get_server_info())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> 
{
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}