use actix_web::client::Client;
use actix_web::web::Query;
use actix_web::{web, HttpResponse};
use futures::{Future};
use std::str;


pub struct ClientParameters {
    pub client: Client,
    pub c_endpoint: String,
}

#[derive(Deserialize)]
pub struct Parameters {
    #[serde(rename = "accountId")]
    account_id: String,
}

#[derive(Deserialize)]
pub struct SortedParameters {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(default = "default_numeric")]
    sort: usize,
    #[serde(default = "default_numeric")]
    asc: usize,
}

#[derive(Deserialize)]
pub struct TopSortedParameters {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "totalElements", default = "default_elements")]
    total_elements: usize,
    #[serde(default = "default_numeric")]
    asc: usize,
}

// Will return default value for numeric parameters
fn default_numeric() -> usize {
    0
}

// Will return default value for total elements parameter
fn default_elements() -> usize {
    0
}

/// Handle index route
pub fn index_handler() -> &'static str {
    "Hello world!\r\n"
}  

/// hello handler.
///   Send an empty call to backendr endpoint
///   Receives a UUID
pub fn hello_handler(c_parameters: web::Data<ClientParameters>) -> 
    impl Future<Item = HttpResponse, Error = ()> {

    let mut endpoint = c_parameters.c_endpoint.to_string();
    endpoint.push_str(&"/hello".to_string());
    debug!("Calling endpoint: {}", endpoint);

    c_parameters.client.get(endpoint)   // <- Create request builder
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

/// Returns a list of customer accounts as Json.
/// Receives no parameter.
pub fn customer_accounts_handler(c_parameters: web::Data<ClientParameters>) -> 
    impl Future<Item = HttpResponse, Error = ()> {
    
    let mut endpoint = c_parameters.c_endpoint.to_string();
    endpoint.push_str(&"/customer/accounts".to_string());
    
    debug!("Calling endpoint: {}", endpoint);

    c_parameters.client.get(endpoint)   // <- Create request builder
            .header("User-Agent", "Actix-web")
            .send()                               // <- Send http request
            .map_err(|_| ())
            //.map_err(Error::from)
            .and_then(|mut response| {
                    response.body().and_then( |body| {
                        debug!("Received from endpoint: {}", str::from_utf8(&body).unwrap());
                        Ok(HttpResponse::Ok().body(body))
                    }).map_err(|_| ())
                }).map_err(|_| ())
}

/// Return data of requested account.
/// Receives accountId.
pub fn customer_account_handler(c_parameters: web::Data<ClientParameters>, 
    msg: Query<Parameters>) -> 
    impl Future<Item = HttpResponse, Error = ()> {

    let account_id = msg.account_id.clone();
    let mut endpoint = c_parameters.c_endpoint.to_string();
    endpoint.push_str(&"/customer/account?accountId=".to_string());
    endpoint.push_str(&account_id);
    
    debug!("Calling endpoint: {}", endpoint);

    c_parameters.client.get(endpoint)   // <- Create request builder
            .header("User-Agent", "Actix-web")
            .send()                               // <- Send http request
            .map_err(|_| ())
            //.map_err(Error::from)
            .and_then(|mut response| {
                    response.body().and_then( |body| {
                        debug!("Received from endpoint: {}", str::from_utf8(&body).unwrap());
                        Ok(HttpResponse::Ok().body(body))
                    }).map_err(|_| ())
                }).map_err(|_| ())
}