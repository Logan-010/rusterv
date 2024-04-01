use actix_files::Files;
use actix_web::{middleware::Compress, App, HttpServer};
use std::{env, process::exit};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("USAGE: {} <PORT> <DIR>", &args[0]);
        exit(0);
    }

    let port: u16 = args[1]
        .trim()
        .parse()
        .expect("Make sure <PORT> argument is a port number!");

    println!("Listening on:\n\thttp://127.0.0.1:{port}\n\thttp://[::1]:{port}\n\t");

    HttpServer::new(move || {
        App::new().wrap(Compress::default()).service(
            Files::new("/", &args[2])
                .show_files_listing()
                .index_file("index.html"),
        )
    })
    .bind(("localhost", port))?
    .run()
    .await
}
