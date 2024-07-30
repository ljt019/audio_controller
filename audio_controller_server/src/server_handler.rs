use crate::audio_controller::AudioController;
use crate::audio_file_controller::AudioFileController;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

pub struct ServerHandler {
    pub audio_controller: Arc<Mutex<AudioController>>,
    pub file_controller: Arc<AudioFileController>,
}

impl ServerHandler {
    pub fn new(
        audio_controller: AudioController,
        audio_folder_path: &str,
    ) -> Result<ServerHandler, std::io::Error> {
        let file_controller = AudioFileController::new(audio_folder_path)?;
        Ok(ServerHandler {
            audio_controller: Arc::new(Mutex::new(audio_controller)),
            file_controller: Arc::new(file_controller),
        })
    }

    pub fn routes(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let audio_controller = self.audio_controller.clone();
        let file_controller = self.file_controller.clone();

        let play_audio = warp::path("play_audio")
            .and(warp::post())
            .and(warp::query::<HashMap<String, String>>())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_play_audio);

        let pause_audio = warp::path("pause_audio")
            .and(warp::post())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_pause_audio);

        let stop_audio = warp::path("stop_audio")
            .and(warp::post())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_stop_audio);

        let change_volume = warp::path("change_volume")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_change_volume);

        let get_audio_files = warp::path("get_audio_files")
            .and(warp::get())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_get_audio_files);

        let get_volume = warp::path("get_volume")
            .and(warp::get())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_get_volume);

        let get_audio_status = warp::path("get_audio_status")
            .and(warp::get())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_get_audio_status);

        let receive_audio_file = warp::path("receive_audio_file")
            .and(warp::post())
            .and(warp::body::bytes())
            .and(warp::query::<HashMap<String, String>>())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_receive_audio_file);

        let delete_audio_file = warp::path("delete_audio_file")
            .and(warp::delete())
            .and(warp::query::<HashMap<String, String>>())
            .and(with_controllers(
                audio_controller.clone(),
                file_controller.clone(),
            ))
            .and_then(Self::handle_delete_audio_file);

        let api_docs = warp::path("api-docs")
            .and(warp::get())
            .and_then(Self::handle_api_docs);

        play_audio
            .or(pause_audio)
            .or(stop_audio)
            .or(change_volume)
            .or(get_audio_files)
            .or(get_volume)
            .or(get_audio_status)
            .or(receive_audio_file)
            .or(delete_audio_file)
            .or(api_docs)
    }

    async fn handle_play_audio(
        params: HashMap<String, String>,
        (audio_controller, file_controller): (
            Arc<Mutex<AudioController>>,
            Arc<AudioFileController>,
        ),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(file_name) = params.get("file_name") {
            if file_controller.file_exists(file_name) {
                let file_path = file_controller.get_file_path(file_name);
                let mut controller = audio_controller.lock().await;
                controller.play_audio(file_path.to_str().unwrap()).await;
                Ok(warp::reply::json(&format!("Playing audio: {}", file_name)))
            } else {
                Err(warp::reject::custom(FileNotFound))
            }
        } else {
            Err(warp::reject::custom(BadRequest))
        }
    }

    async fn handle_pause_audio(
        (audio_controller, _): (Arc<Mutex<AudioController>>, Arc<AudioFileController>),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut controller = audio_controller.lock().await;
        controller.pause_audio().await;
        Ok(warp::reply::json(&"Pausing audio"))
    }

    async fn handle_stop_audio(
        (audio_controller, _): (Arc<Mutex<AudioController>>, Arc<AudioFileController>),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut controller = audio_controller.lock().await;
        controller.stop_audio().await;
        Ok(warp::reply::json(&"Stopping audio"))
    }

    async fn handle_change_volume(
        volume: f32,
        (audio_controller, _): (Arc<Mutex<AudioController>>, Arc<AudioFileController>),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut controller = audio_controller.lock().await;
        controller.set_volume(volume).await;
        Ok(warp::reply::json(&format!("Changing volume to {}", volume)))
    }

    async fn handle_get_audio_files(
        (_, file_controller): (Arc<Mutex<AudioController>>, Arc<AudioFileController>),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        match file_controller.get_audio_files() {
            Ok(files) => Ok(warp::reply::json(&files)),
            Err(_) => Err(warp::reject::custom(InternalError)),
        }
    }

    async fn handle_get_volume(
        (audio_controller, _): (Arc<Mutex<AudioController>>, Arc<AudioFileController>),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let controller = audio_controller.lock().await;
        Ok(warp::reply::json(&controller.volume))
    }

    async fn handle_get_audio_status(
        (audio_controller, _): (Arc<Mutex<AudioController>>, Arc<AudioFileController>),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let controller = audio_controller.lock().await;
        Ok(warp::reply::json(&controller.audio_status))
    }

    async fn handle_receive_audio_file(
        file_content: bytes::Bytes,
        params: HashMap<String, String>,
        (_, file_controller): (Arc<Mutex<AudioController>>, Arc<AudioFileController>),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(file_name) = params.get("file_name") {
            match file_controller.receive_audio_file(file_name, &file_content) {
                Ok(_) => Ok(warp::reply::json(&format!(
                    "Received audio file: {}",
                    file_name
                ))),
                Err(_) => Err(warp::reject::custom(InternalError)),
            }
        } else {
            Err(warp::reject::custom(BadRequest))
        }
    }

    async fn handle_delete_audio_file(
        params: HashMap<String, String>,
        (_, file_controller): (Arc<Mutex<AudioController>>, Arc<AudioFileController>),
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(file_name) = params.get("file_name") {
            match file_controller.delete_audio_file(file_name) {
                Ok(_) => Ok(warp::reply::json(&format!(
                    "Deleted audio file: {}",
                    file_name
                ))),
                Err(_) => Err(warp::reject::custom(InternalError)),
            }
        } else {
            Err(warp::reject::custom(BadRequest))
        }
    }

    async fn handle_api_docs() -> Result<impl warp::Reply, warp::Rejection> {
        let content = include_str!("../../templates/docs.html");
        Ok(warp::reply::html(content))
    }
}

fn with_controllers(
    audio_controller: Arc<Mutex<AudioController>>,
    file_controller: Arc<AudioFileController>,
) -> impl Filter<
    Extract = ((Arc<Mutex<AudioController>>, Arc<AudioFileController>),),
    Error = std::convert::Infallible,
> + Clone {
    warp::any().map(move || (audio_controller.clone(), file_controller.clone()))
}

#[derive(Debug)]
struct FileNotFound;
impl warp::reject::Reject for FileNotFound {}

#[derive(Debug)]
struct InternalError;
impl warp::reject::Reject for InternalError {}

#[derive(Debug)]
struct BadRequest;
impl warp::reject::Reject for BadRequest {}
