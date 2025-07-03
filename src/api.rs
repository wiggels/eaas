use actix_web::{HttpResponse, get, web};
use rand::{Rng, rng};
use serde::Deserialize;

#[derive(Deserialize)]
struct ErrorQuery {
    chance: Option<u8>,
}

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Errors-as-a-Service",
        description = "Where failure is always an option!",
        contact(
            name = "Hunter Wigelsworth",
            email = "wiggels@gmail.com",
            url = "https://wiggels.dev"
        ),
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
    paths(any, client, host),
    components()
)]
pub struct ApiDoc;

const CLIENT_ERRORS: &[u16] = &[400, 401, 403, 404, 418, 429];
const SERVER_ERRORS: &[u16] = &[500, 502, 503, 504];

enum ErrorType {
    Client,
    Host,
    Any,
}

impl ErrorType {
    fn get_error(&self) -> u16 {
        let mut rng = rng();
        let codes: Vec<u16> = match self {
            ErrorType::Client => CLIENT_ERRORS.to_vec(),
            ErrorType::Host => SERVER_ERRORS.to_vec(),
            ErrorType::Any => {
                let mut all = CLIENT_ERRORS.to_vec();
                all.extend(SERVER_ERRORS);
                all
            }
        };
        codes[rng.random_range(0..codes.len())]
    }

    fn decide_fate(&self, chance: u8) -> bool {
        let mut rng = rng();
        let roll: u8 = rng.random_range(0..=100);
        roll <= chance
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(any).service(client).service(host);
}

#[utoipa::path(
    get,
    path = "/api/host",
    tag = "api",
    description = "Possibly get a host error",
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    params(
        ("chance" = Option<u8>, Query, description = "Percentage chance to get an error (defaults to 50)")
    )
)]
#[get("/host")]
pub async fn host(query: web::Query<ErrorQuery>) -> HttpResponse {
    let error = ErrorType::Host;
    if error.decide_fate(query.chance.unwrap_or(50)) {
        let code = actix_web::http::StatusCode::from_u16(error.get_error()).unwrap();
        return HttpResponse::build(code)
            .content_type("text/plain")
            .body(format!(
                "{} {}",
                code.as_u16(),
                code.canonical_reason().unwrap().to_uppercase()
            ));
    }
    HttpResponse::Ok().content_type("text/plain").body("200 OK")
}

#[utoipa::path(
    get,
    path = "/api/client",
    tag = "api",
    description = "Possibly get a client error",
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
        (status = 418, description = "I'm a teapot"),
        (status = 429, description = "Too Many Requests"),
    ),
    params(
        ("chance" = Option<u8>, Query, description = "Percentage chance to get an error (defaults to 50)")
    )
)]
#[get("/client")]
pub async fn client(query: web::Query<ErrorQuery>) -> HttpResponse {
    let error = ErrorType::Client;
    if error.decide_fate(query.chance.unwrap_or(50)) {
        let code = actix_web::http::StatusCode::from_u16(error.get_error()).unwrap();
        return HttpResponse::build(code)
            .content_type("text/plain")
            .body(format!(
                "{} {}",
                code.as_u16(),
                code.canonical_reason().unwrap().to_uppercase()
            ));
    }
    HttpResponse::Ok().content_type("text/plain").body("200 OK")
}

#[utoipa::path(
    get,
    path = "/api/any",
    tag = "api",
    description = "Possibly get any error",
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
        (status = 418, description = "I'm a Teapot"),
        (status = 429, description = "Too Many Requests"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    params(
        ("chance" = Option<u8>, Query, description = "Percentage chance to get an error (defaults to 50)")
    )
)]
#[get("/any")]
pub async fn any(query: web::Query<ErrorQuery>) -> HttpResponse {
    let error = ErrorType::Any;
    if error.decide_fate(query.chance.unwrap_or(50)) {
        let code = actix_web::http::StatusCode::from_u16(error.get_error()).unwrap();
        return HttpResponse::build(code)
            .content_type("text/plain")
            .body(format!(
                "{} {}",
                code.as_u16(),
                code.canonical_reason().unwrap().to_uppercase()
            ));
    }
    HttpResponse::Ok().content_type("text/plain").body("200 OK")
}
