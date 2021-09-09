use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("TESTING");
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let startup_message = format!("> Starting server on http://{}", address);
    let listener = TcpListener::bind(address).expect("Failed to bind random port.");

    println!("{}", startup_message);
    run(listener)?.await
}
