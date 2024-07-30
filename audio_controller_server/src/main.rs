mod audio_controller;
mod audio_file_controller;
mod server_handler;

use audio_controller::AudioController;
use server_handler::ServerHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_controller = AudioController::new();
    let server_handler = ServerHandler::new(audio_controller, "audio_files")?;

    let routes = server_handler.routes();

    println!("Server starting on http://127.0.0.1:3030");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    Ok(())
}
