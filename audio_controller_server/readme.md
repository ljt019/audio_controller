# Audio Controller API Documentation

## Endpoints

### Audio Playback Control

#### Play an audio file

- **Method:** POST
- **Path:** `/play_audio`
- **Parameters:**
  - `file_name` (query, required): Name of the audio file to play
- **Example:** `POST /play_audio?file_name=song.mp3`

#### Pause the currently playing audio

- **Method:** POST
- **Path:** `/pause_audio`
- **Example:** `POST /pause_audio`

#### Stop the currently playing audio

- **Method:** POST
- **Path:** `/stop_audio`
- **Example:** `POST /stop_audio`

### Volume Control

#### Change the volume of audio playback

- **Method:** POST
- **Path:** `/change_volume`
- **Parameters:**
  - `volume` (body, required): New volume level (0.0 to 1.0)
- **Example:** `POST /change_volume` with body `0.8`

#### Get the current volume level

- **Method:** GET
- **Path:** `/get_volume`
- **Example:** `GET /get_volume`

### Audio File Management

#### Get a list of all available audio files

- **Method:** GET
- **Path:** `/get_audio_files`
- **Example:** `GET /get_audio_files`

#### Upload a new audio file

- **Method:** POST
- **Path:** `/receive_audio_file`
- **Parameters:**
  - `file_name` (query, required): Name to give the uploaded audio file
  - `file` (body, required): The audio file content
- **Example:** `POST /receive_audio_file?file_name=new_song.mp3` with file content in body

#### Delete an audio file

- **Method:** DELETE
- **Path:** `/delete_audio_file`
- **Parameters:**
  - `file_name` (query, required): Name of the audio file to delete
- **Example:** `DELETE /delete_audio_file?file_name=old_song.mp3`

### Status

#### Get the current status of audio playback

- **Method:** GET
- **Path:** `/get_audio_status`
- **Example:** `GET /get_audio_status`

### Documentation

#### Get this API documentation

- **Method:** GET
- **Path:** `/api-docs`
- **Example:** `GET /api-docs`

## Usage Notes

- All POST and DELETE requests should include appropriate headers (e.g., `Content-Type: application/json` for JSON payloads).
- Ensure all file names are URL-encoded when used in query parameters.
- For file uploads, use multipart/form-data encoding.
