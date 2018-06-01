extern crate actix_web;
extern crate listenfd;
#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use actix_web::{server, App, HttpRequest};
use actix_web::middleware::Logger;
use listenfd::ListenFd;
use structopt::StructOpt;

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[derive(StructOpt)]
#[structopt(version_short = "v")]
struct Opts {
    /// Host on which to listen
    #[structopt(long = "host", short = "h", default_value = "127.0.0.1")]
    host: String,
    /// Port on which to listen
    #[structopt(long = "port", short = "p", default_value = "8080")]
    port: u64,
    /// Increase verbosity
    #[structopt(long = "verbose", short = "V")]
    verbosity: u64,
}

fn main() -> Result<(), std::io::Error> {
    let args = Opts::from_args();
    stderrlog::new()
        .module(module_path!())
        .verbosity(args.verbosity as usize)
        .init()
        .expect("Failed to initialize logger");
    let app = App::new()
        .middleware(Logger::default())
        .middleware(Logger::new("%a %{User-Agent}i"))
        .resource("/", |r| r.f(index))
        .finish();
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| app);
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(format!("{}:{}", args.host, args.port)).unwrap()
    };
    server.run();
    Ok(())
}
