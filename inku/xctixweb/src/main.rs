#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_static_files;
use anyhow::Result;
use chrono;
use handlebars::Handlebars;
use rand::Rng;
use rcgen::{Certificate, CertificateParams, DnType};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::collections::HashMap;
use std::fs::File;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::{BufReader, Cursor};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[cfg(target_os = "linux")]
use webterm::WebTermExt;

// https://docs.rs/actix-web-static-files/0.2.4/actix_web_static_files/fn.resource_dir.html
// Generate resources for dir with file name generated.rs
// stored in path defined by OUT_DIR environment variable.
// Function name is 'generate'
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(StructOpt, Debug)]
#[structopt(name = "xctixweb")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// xctixweb https server listen address
    #[structopt(short = "l", long = "listen", default_value = "0.0.0.0:8443")]
    listen: SocketAddr,

    /// TLS cert to use
    #[structopt(long = "cert", requires = "tls_key")]
    tls_cert: Option<PathBuf>,

    /// TLS key to use
    #[structopt(long = "key", requires = "tls_cert")]
    tls_key: Option<PathBuf>,

    /// The command to execute
    #[structopt(short, long, default_value = "/bin/bash")]
    command: String,
}

lazy_static! {
    static ref OPT: Opt = Opt::from_args();
}

const BACKEND_TPL: &str = include_str!("./backend_tpl.html");
// const CERT: &str = include_str!("./server.crt");
// const KEY: &str = include_str!("./server.key");

fn load_certs(filename: &PathBuf) -> std::io::Result<Vec<rustls::Certificate>> {
    let certfile = File::open(filename)?;
    let mut reader = BufReader::new(certfile);
    certs(&mut reader)
        .map_err(|_| IoError::new(IoErrorKind::Other, "File contains an invalid certificate"))
}

fn load_const_cert(cert: &str) -> std::io::Result<Vec<rustls::Certificate>> {
    certs(&mut BufReader::new(Cursor::new(cert)))
        .map_err(|_| IoError::new(IoErrorKind::Other, "App contains an invalid certificate"))
}

fn load_const_private_key(key_str: &str) -> std::io::Result<rustls::PrivateKey> {
    let pkcs8_keys = {
        let mut reader = BufReader::new(Cursor::new(key_str));
        rustls::internal::pemfile::pkcs8_private_keys(&mut reader).map_err(|_| {
            IoError::new(
                IoErrorKind::Other,
                "File contains invalid pkcs8 private key (encrypted keys not supported)",
            )
        })?
    };

    assert!(!pkcs8_keys.is_empty());
    // prefer to load pkcs8 keys
    Ok(pkcs8_keys[0].clone())
}

fn load_private_key(filename: &PathBuf) -> std::io::Result<rustls::PrivateKey> {
    let rsa_keys = {
        let keyfile = File::open(filename)?;
        let mut reader = BufReader::new(keyfile);
        rsa_private_keys(&mut reader).map_err(|_| {
            IoError::new(IoErrorKind::Other, "File contains invalid RSA private key")
        })?
    };

    let pkcs8_keys = {
        let keyfile = File::open(filename)?;
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::pkcs8_private_keys(&mut reader).map_err(|_| {
            IoError::new(
                IoErrorKind::Other,
                "File contains invalid pkcs8 private key (encrypted keys not supported)",
            )
        })?
    };

    // prefer to load pkcs8 keys
    if !pkcs8_keys.is_empty() {
        Ok(pkcs8_keys[0].clone())
    } else {
        assert!(!rsa_keys.is_empty());
        Ok(rsa_keys[0].clone())
    }
}

#[get("/backend")]
fn backend(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "name": "XiDunKu",
        "server_time":  chrono::offset::Local::now().to_string(),
    });
    let body = hb.render("backend_tpl", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/")]
fn indexfile() -> HttpResponse {
    HttpResponse::MovedPermanently()
        .header(actix_web::http::header::LOCATION, "/index.html")
        .finish()
}

fn gen_server_cert_key() -> (String, String) {
    let mut params = CertificateParams::new(vec!["hello-dev.xidunku".to_string()]);
    params
        .distinguished_name
        .push(DnType::OrganizationName, "XIDUNKU");
    params
        .distinguished_name
        .push(DnType::CommonName, "DevHost");
    params.serial_number = Some(rand::thread_rng().gen());
    let cert = Certificate::from_params(params).unwrap();
    (
        cert.serialize_pem().unwrap(),
        cert.serialize_private_key_pem(),
    )
}

// TODO
// web interface types
// #[wasmtime_rust::wasmtime]
// trait Webfoo {
//     fn geopattern_gen_base64_svg_string(&mut self, input: &str) -> String;
//}

fn main() -> Result<(), anyhow::Error> {

    pretty_env_logger::init();
    
    let opt = Opt::from_args();
    if opt.debug {
        std::env::set_var("RUST_LOG", "actix_web=debug,actix=debug");
    }
    
    let (srv_cert, srv_key) = gen_server_cert_key();
    
    // TODO CPU 100%
    // web interface types
    //
    // let mut webfoo = Webfoo::load_file("htdocs/wasm/webfoo/webfoo_lib.wasm")?;
    // let x = webfoo.geopattern_gen_base64_svg_string("Hello, Rust!");
    // println!("Load webfoo_lib.wasm");
    // println!("{}", x);
    
    let mut handlebars = Handlebars::new();
    
    //handlebars.register_templates_directory(".html", "./htdocs").unwrap();
    handlebars.register_template_string("backend_tpl", BACKEND_TPL)?;
    let handlebars_ref = web::Data::new(handlebars);
    
    let mut config = ServerConfig::new(NoClientAuth::new());
    if opt.tls_cert.is_some() && opt.tls_key.is_some() {
        let tls_cert = opt.tls_cert.unwrap();
        let tls_key = opt.tls_key.unwrap();
        let cert_file = load_certs(&tls_cert)?;
        let key_file = load_private_key(&tls_key)?;
        config
        .set_single_cert(cert_file, key_file)
        .map_err(|e| IoError::new(IoErrorKind::Other, e.to_string()))?;
    } else {
        let cert_file = load_const_cert(&srv_cert)?;
        let key_file = load_const_private_key(&srv_key)?;
        config
        .set_single_cert(cert_file, key_file)
        .map_err(|e| IoError::new(IoErrorKind::Other, e.to_string()))?;
    }
    
    info!("xctixweb start at https://{}", opt.listen.to_string());
    let server = HttpServer::new(move || {
        // // https://docs.rs/actix-web-static-files/0.2.4/actix_web_static_files/fn.resource_dir.html
        // Generate resources for dir with file name OUT_DIR/generated.rs
        // Function name is 'generate'
        let generated = generate();
        App::new()
        .register_data(handlebars_ref.clone())
        .service(indexfile)
        .webterm_socket("/websocket", |_req| {
            let mut cmd = Command::new(OPT.command.clone());
            cmd.env("TERM", "xterm");
            println!("/websocke req cmd {:?}",cmd);
            cmd
        })
            .service(backend)
            .service(actix_web_static_files::ResourceFiles::new("/", generated))
    });

    server
        .bind_rustls(opt.listen, config)?
        .system_exit()
        .run()
        .map_err(|e| anyhow::Error::new(e))
}
