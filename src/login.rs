use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use serde::Deserialize;
use url::Url;
use urlencoding::encode;
use webbrowser;
use tokio::task::LocalSet;

#[derive(Deserialize)]
struct SsoQuery {
    sso: String,
    sig: String,
}

// In-memory store for nonce and user login state
pub struct AppState {
    pub nonce: Mutex<String>,
    pub login: Mutex<Option<HashMap<String, String>>>,
}

async fn index(req: HttpRequest, data: web::Data<Arc<AppState>>) -> impl Responder {
    let sso_secret = ""; 
    let sso_url = "https://boykisser.cc/session/sso_provider";
    let return_url = format!("http://{}/sso_callback", req.connection_info().host());

    let nonce = generate_nonce();
    {
        let mut nonce_guard = data.nonce.lock().unwrap();
        *nonce_guard = nonce.clone();
    }

    let login_url = generate_sso_url(&sso_secret, sso_url, &return_url, &nonce);
    HttpResponse::Found().append_header(("Location", login_url)).finish()
}

async fn sso_callback(query: web::Query<SsoQuery>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let sso_secret = ""; 

    if let Some(params) = validate_sso_response(&sso_secret, &query.sso, &query.sig) {
        let nonce_guard = data.nonce.lock().unwrap();
        if params.get("nonce") != Some(&*nonce_guard) {
            return HttpResponse::BadRequest().body("Invalid nonce.");
        }

        {
            let mut login_guard = data.login.lock().unwrap();
            *login_guard = Some(params.clone());
        }

        if let Some(username) = params.get("username") {
            return HttpResponse::Ok().body(format!("We logged you in, you can close this window and continue playing. Username: {}", username));
        } else {
            return HttpResponse::BadRequest().body("Username not found in SSO response.");
        }
    }

    HttpResponse::BadRequest().body("Invalid SSO response.")
}

fn generate_sso_url(sso_secret: &str, sso_url: &str, return_url: &str, nonce: &str) -> String {
    let payload = format!("nonce={}&return_sso_url={}", nonce, encode(return_url));
    let base64_payload = general_purpose::STANDARD.encode(&payload);

    let mut mac = Hmac::<Sha256>::new_from_slice(sso_secret.as_bytes()).unwrap();
    mac.update(base64_payload.as_bytes());
    let signature = mac.finalize().into_bytes();

    let url = Url::parse_with_params(sso_url, &[("sso", base64_payload), ("sig", format!("{:x}", signature))]).unwrap();
    url.to_string()
}

fn validate_sso_response(sso_secret: &str, sso_payload: &str, sig: &str) -> Option<HashMap<String, String>> {
    let mut mac = Hmac::<Sha256>::new_from_slice(sso_secret.as_bytes()).unwrap();
    mac.update(sso_payload.as_bytes());
    let expected_signature = mac.finalize().into_bytes();

    if format!("{:x}", expected_signature) != sig {
        return None;
    }

    let decoded_payload = general_purpose::STANDARD.decode(sso_payload).ok()?;
    let decoded_str = String::from_utf8(decoded_payload).ok()?;
    let params: HashMap<String, String> = decoded_str.split('&')
        .map(|s| s.split_once('='))
        .filter_map(|pair| pair)
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    Some(params)
}

fn generate_nonce() -> String {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    format!("{:?}", now)
}

async fn run_server(app_state: Arc<AppState>) -> std::io::Result<()> {
    let data = web::Data::new(app_state.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/sso_callback", web::get().to(sso_callback))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn open_browser() {
    if webbrowser::open("http://localhost:8080").is_ok() {
        println!("Please open the browser and log in. If you are stuck on this screen, please visit http://localhost:8080 in your browser.");
    } else {
        println!("Failed to open browser");
    }
}

pub async fn login() -> std::io::Result<()> {
    let app_state = Arc::new(AppState {
        nonce: Mutex::new(String::new()),
        login: Mutex::new(None),
    });

    let local_set = LocalSet::new();

    open_browser();

    local_set.spawn_local(run_server(app_state.clone()));

    local_set.run_until(async {
        loop {
            {
                let login_guard = app_state.login.lock().unwrap();
                if let Some(ref login) = *login_guard {
                    if let Some(username) = login.get("username") {
                        println!("Welcome, {}!", username);
                        break;
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        Ok(())
    }).await
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    login().await
}
