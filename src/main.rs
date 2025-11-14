use zero2prod::{configuration::get_configuration, run};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let configuration = get_configuration().expect("Failed to read configuration.");
	
	let address = format!("localhost:{}", configuration.application_port);
	let listener = std::net::TcpListener::bind(address)?;

	run(listener)?.await
}
