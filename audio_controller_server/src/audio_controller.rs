use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct AudioController {
    pub volume: f32,
    pub audio_status: String,
    pub currently_playing_audio_file: Option<String>,
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
            audio_status: "stopped".to_string(),
            currently_playing_audio_file: None,
            sink: Arc::new(Mutex::new(Some(sink))),
            _stream: Arc::new(stream),
            stream_handle,
        }
    }

    pub async fn play_audio(&mut self, file_path: &str) {
        println!("Attempting to play audio: {}", file_path);

        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();

        let mut sink_guard = self.sink.lock().await;
        if sink_guard.is_none() {
            *sink_guard = Some(Sink::try_new(&self.stream_handle).unwrap());
        }

        if let Some(sink) = sink_guard.as_mut() {
            sink.append(source);
            sink.set_volume(self.volume);
            sink.play();

            self.audio_status = "playing".to_string();
            self.currently_playing_audio_file = Some(file_path.to_string());
            println!("Audio playback started successfully");

            // Start a background task to update status when playback ends
            let sink_clone = Arc::clone(&self.sink);
            tokio::spawn(async move {
                loop {
                    sleep(Duration::from_millis(100)).await;
                    let sink_guard = sink_clone.lock().await;
                    if let Some(sink) = &*sink_guard {
                        if sink.empty() {
                            println!("Audio playback finished");
                            break;
                        }
                    }
                }
            });
        }
    }

    pub async fn pause_audio(&mut self) {
        if let Some(sink) = &*self.sink.lock().await {
            sink.pause();
            self.audio_status = "paused".to_string();
        }
    }

    pub async fn stop_audio(&mut self) {
        if let Some(sink) = &*self.sink.lock().await {
            sink.stop();
            self.audio_status = "stopped".to_string();
            self.currently_playing_audio_file = None;
        }
    }

    pub async fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        if let Some(sink) = &*self.sink.lock().await {
            sink.set_volume(volume);
        }
    }
}

// Implement Send and Sync for AudioController
unsafe impl Send for AudioController {}
unsafe impl Sync for AudioController {}
