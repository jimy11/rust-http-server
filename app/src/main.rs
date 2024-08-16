use serde::Deserialize;
use server::protocol::{ContentType, HttpError, HttpMethod, HttpRequest, HttpResponse};

fn main() {
    server::start("127.0.0.1", 8080, router_to_handle_request);
}

fn router_to_handle_request(request: &HttpRequest) -> Result<HttpResponse, HttpError> {
    println!("------handle_http------");
    if request.path == "/" {
        match request.method {
            HttpMethod::POST => {
                return Ok(HttpResponse::new(
                    405,
                    "Method Not Allowed",
                    ContentType::Plain,
                ));
            }
            HttpMethod::GET => {
                return Ok(HttpResponse::new(200, "Hello World", ContentType::Plain));
            }
        }
    } else if request.path.starts_with("/login") {
        match request.method {
            HttpMethod::POST => {
                return Ok(HttpResponse::new(
                    405,
                    "Method Not Allowed",
                    ContentType::Plain,
                ));
            }
            HttpMethod::GET => {
                println!("login-------------request.path: {}", request.path);
                let params_str = request.path.replace("/login?", "");
                let params: Vec<&str> = params_str.split("&").collect();
                println!("{:?}", params);
                let mut content = String::new();
                for p in params {
                    let ps: Vec<&str> = p.split("=").collect();
                    if "name" == ps[0] {
                        content.push_str(ps[1]);
                        break;
                    }
                }
                return Ok(HttpResponse::new(
                    200,
                    format!("Hi {}, Login Success", content).as_str(),
                    ContentType::Plain,
                ));
            }
        }
    } else if request.path == "/insert" {
        match request.method {
            HttpMethod::POST => {
                let u: Result<User, serde_json::Error> = serde_json::from_str(&request.body);
                match u {
                    Ok(user) => {
                        return Ok(HttpResponse::new(
                            200,
                            format!("Welcome {}", user.name).as_str(),
                            ContentType::Json,
                        ));
                    }
                    Err(_) => {
                        return Ok(HttpResponse::new(200, "system error", ContentType::Json));
                    }
                };
            }
            HttpMethod::GET => {
                return Ok(HttpResponse::new(
                    405,
                    "Method Not Allowed",
                    ContentType::Plain,
                ));
            }
        }
    }

    return Ok(HttpResponse::new(404, "unsupport", ContentType::Plain));
}

#[derive(Deserialize)]
pub struct User {
    name: String,
    age: u8,
}
