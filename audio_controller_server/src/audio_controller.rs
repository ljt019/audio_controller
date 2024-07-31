use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct AudioController {
    pub volume: f32,
    pub audio_status: Arc<Mutex<String>>,
    pub currently_playing_audio_file: Arc<Mutex<Option<String>>>,
    sink: Arc<Mutex<Option<Sink>>>,
    _stream: Arc<OutputStream>,
    stream_handle: OutputStreamHandle,
}

impl AudioController {
    pub fn new() -> AudioController {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioController {
            volume: 0.5,
            audio_status: Arc::new(Mutex::new("stopped".to_string())),
            currently_playing_audio_file: Arc::new(Mutex::new(None)),
            sink: Arc::new(Mutex::new(Some(sink))),
            _stream: Arc::new(stream),
            stream_handle,
        }
    }

    pub async fn play_audio(&mut self, file_path: &str) -> Result<(), String> {
        println!("Attempting to play audio: {}", file_path);

        let path = Path::new(file_path);
        let extension = path
            .extension()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("");
        println!("File extension: {}", extension);

        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("Failed to open file: {}", e)),
        };

        let buffer = BufReader::new(file);

        let source = match Decoder::new(buffer) {
            Ok(source) => source,
            Err(e) => {
                println!("Failed to decode with default decoder: {}", e);
                if extension.eq_ignore_ascii_case("wav") {
                    println!("Attempting to use WAV-specific decoder");
                    let file = match File::open(file_path) {
                        Ok(file) => file,
                        Err(e) => return Err(format!("Failed to open WAV file: {}", e)),
                    };
                    match rodio::Decoder::new_wav(BufReader::new(file)) {
                        Ok(source) => {
                            println!("Successfully decoded WAV file");
                            source
                        }
                        Err(e) => {
                            println!("Failed to decode WAV file: {}", e);
                            return Err(format!("Failed to decode WAV file: {}", e));
                        }
                    }
                } else {
                    return Err(format!("Failed to decode audio file: {}", e));
                }
            }
        };

        let mut sink_guard = self.sink.lock().await;
        if sink_guard.is_none() {
            println!("Creating new Sink");
            *sink_guard = Some(Sink::try_new(&self.stream_handle).unwrap());
        }

        if let Some(sink) = sink_guard.as_mut() {
            sink.append(source);
            sink.set_volume(self.volume);
            sink.play();

            {
                let mut status = self.audio_status.lock().await;
                *status = "playing".to_string();
            }
            {
                let mut current_file = self.currently_playing_audio_file.lock().await;
                *current_file = Some(file_path.to_string());
            }
            println!("Audio playback started successfully");

            // Start a background task to update status when playback ends
            let sink_clone = Arc::clone(&self.sink);
            let status_clone = Arc::clone(&self.audio_status);
            let current_file_clone = Arc::clone(&self.currently_playing_audio_file);
            tokio::spawn(async move {
                loop {
                    sleep(Duration::from_millis(100)).await;
                    let sink_guard = sink_clone.lock().await;
                    if let Some(sink) = &*sink_guard {
                        if sink.empty() {
                            println!("Audio playback finished");
                            let mut status = status_clone.lock().await;
                            *status = "stopped".to_string();
                            let mut current_file = current_file_clone.lock().await;
                            *current_file = None;
                            break;
                        }
                    }
                }
            });

            Ok(())
        } else {
            Err("Failed to create audio sink".to_string())
        }
    }

    pub async fn pause_audio(&mut self) {
        if let Some(sink) = &*self.sink.lock().await {
            sink.pause();
            let mut status = self.audio_status.lock().await;
            *status = "paused".to_string();
        }
    }

    pub async fn stop_audio(&mut self) {
        if let Some(sink) = &*self.sink.lock().await {
            sink.stop();
            let mut status = self.audio_status.lock().await;
            *status = "stopped".to_string();
            let mut current_file = self.currently_playing_audio_file.lock().await;
            *current_file = None;
        }
    }

    pub async fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        if let Some(sink) = &*self.sink.lock().await {
            sink.set_volume(volume);
        }
    }

    pub async fn resume_audio(&mut self) {
        if let Some(sink) = &*self.sink.lock().await {
            sink.play();
            let mut status = self.audio_status.lock().await;
            *status = "playing".to_string();
        }
    }

    pub async fn get_current_audio(&self) -> Option<String> {
        self.currently_playing_audio_file.lock().await.clone()
    }

    pub async fn get_audio_status(&self) -> String {
        self.audio_status.lock().await.clone()
    }
}

// Implement Send and Sync for AudioController
unsafe impl Send for AudioController {}
unsafe impl Sync for AudioController {}
