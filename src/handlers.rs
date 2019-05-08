use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use futures::{Future};
use actix_web::web::Path;
use std::str;

pub struct Parameters {
    pub client: Client,
    pub c_endpoint: String,
}

/// Handle index route
pub fn index_handler() -> &'static str {
    "Hello world!\r\n"
}  

/// hello handler.
///   Send an empty call to backendr endpoint
///   Receives a UUID
pub fn hello_handler(parameters: web::Data<Parameters>) -> 
    impl Future<Item = HttpResponse, Error = ()> {

    let mut endpoint = parameters.c_endpoint.to_string();
    endpoint.push_str(&"/hello".to_string());
    debug!("Calling endpoint: {}", endpoint);

    parameters.client.get(endpoint)   // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|err| {
                error!("error 1 = {:?}", err);
            })
            //.map_err(Error::from)
            .and_then(|mut response| {
                    response.body().and_then( |body| {
                        debug!("Received from endpoint: {}", str::from_utf8(&body).unwrap());
                        Ok(HttpResponse::Ok().body(body))
                    }).map_err(|err| {
                        error!("error 2 = {:?}", err);
                    })
                //}).map_err(|_| ())
            }).map_err(|err| {
                 error!("error 3 = {:?}", err);
            })
} 