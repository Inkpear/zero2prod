use zero2prod::run;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let listener = std::net::TcpListener::bind("localhost:8000")?;
	
	run(listener)?.await
}