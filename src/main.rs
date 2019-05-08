#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate env_logger;

use actix_web::{web, guard, HttpResponse, App, HttpServer};
use actix_web::client::Client;
use log::Level;
use colored::*;

mod handlers;
use crate::handlers::ClientParameters;
use crate::handlers::index_handler;
use crate::handlers::hello_handler;
use crate::handlers::customer_accounts_handler;
use crate::handlers::customer_account_handler;
use crate::handlers::customer_account_detail_handler;

// Defines the default port
const DEFAULT_PORT: u16          = 9296;

// Defines the workers used by server
const DEFAULT_WORKERS: usize     = 2;

// Defines the url for the called service (calculator)
const DEFAULT_CLIENT_URL: &str   = "http://127.0.0.1:9596"; 

#[derive(Deserialize, Debug)]
struct ConfigPort {
    port: u16,
}

#[derive(Deserialize, Debug)]
struct ConfigWorkers {
    workers: usize,
}

#[derive(Deserialize, Debug)]
struct ConfigCalledService {
    client_url: String,
}

fn config_port() -> u16 {
    match envy::prefixed("FE_").from_env::<ConfigPort>() {
      Ok(config) => {
          info!("Port set to: {}", config.port);
          config.port
      },
      Err(error) => {
          error!("Error with env var PORT {}", error);
          info!("Port set to {} - default value", DEFAULT_PORT);
          DEFAULT_PORT
      }
   }
}

fn config_workers() -> usize {
    match envy::prefixed("FE_").from_env::<ConfigWorkers>() {
      Ok(config) => {
          info!("Workers set to: {}", config.workers);
          config.workers
      },
      Err(error) => {
          error!("Error with env var WORKERS {}", error);
          info!("Workers set to {} - default value", DEFAULT_WORKERS);
          DEFAULT_WORKERS
      }
   }
}

fn config_called_service() -> String {
    match envy::prefixed("FE_").from_env::<ConfigCalledService>() {
      Ok(config) => {
          info!("Client URL set to: {}", config.client_url.to_string());
          config.client_url
      },
      Err(error) => {
          error!("Error with env var CLIENT_URL {}", error);
          info!("Client URL set to {} - default value", DEFAULT_CLIENT_URL.to_string());
          DEFAULT_CLIENT_URL.to_string()
      }
   }
}

// Displays intro banner
fn intro() {
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "                    Front End v 0.1.0".yellow().bold());
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "   Please use env variables for configuration:".yellow().bold());
    println!("{}", "       FE_PORT=port number".yellow().bold());
    println!("{}", "       FE_WORKERS=workers for server".yellow().bold());
    println!("{}", "       FE_CLIENT_URL=url of called service".yellow().bold());
    println!("{}", "-----------------------------------------------------------");
    println!("Starting configuration......\n");
}


fn main() -> std::io::Result<()> {
    env_logger::init();
    /*Builder::new()
        .parse(&env::var("BANK_LOG").unwrap_or_default())
        .init();*/

    intro();

    let port = config_port();
    let workers = config_workers();
    let endpoint = config_called_service();    

    println!("{}", "-----------------------------------------------------------");
    println!("Starting server.... Press Ctrl-C to stop it.");

    if log_enabled!(Level::Info) {
        debug!("Starting server");
    }    
    
    HttpServer::new(
        move || App::new()
            .data(ClientParameters{
                client: Client::default(), 
                c_endpoint: endpoint.clone(),
            })
            .service(
                web::resource("/")
                    .route(web::get().to(index_handler))
            ) // end service
            .service(
                web::resource("/hello")
                .route(web::get().to_async(hello_handler))
            ) // end hello service
            .service(
                web::resource("/customer/accounts")
                .route(web::get().to_async(customer_accounts_handler))
            ) // end customer accounts
            .service(
                web::resource("/customer/account")
                .route(web::get().to_async(customer_account_handler))
            ) // end customer account 
            .service(
                web::resource("/customer/account/detail")
                .route(web::get().to_async(customer_account_detail_handler))
            ) // end customer account details                                   
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get()
                      //.to(p404)
                      .to(|| {
                          debug!("Response for wrong url");
                          HttpResponse::NotFound()
                      }))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
            
    ) // end http server
    .workers(workers)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
}
