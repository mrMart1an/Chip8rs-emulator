use rodio::{OutputStream, Sink, OutputStreamHandle};
use rodio::source::{SineWave, Source};

/*
*
*   Rodio based sound system
*
*/

pub struct RodioSound {
    _stream_handle: OutputStreamHandle,
    _stream: OutputStream,

    /// Control the sine wave source stream
    sink: Sink,
}

// Implement constructor and methods for rodio sound
impl RodioSound {
    /// Create a new sound system given frequency and volume of the bell
    pub fn new(frequency: f32, volume: f32) -> Self {
        // Create the audio handler and sink
        let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&_stream_handle).unwrap();
        
        // Create a sine wave source and give it to a sink
        let source = SineWave::new(frequency).amplify(volume);
        sink.append(source);
        sink.pause();

        // Create the sound system object
        Self { 
            _stream_handle,
            _stream,

            sink,
        }
    }

    /// Update the current bell status to the given input
    pub fn update_bell(&self, bell_status: bool) {
        if bell_status {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }
}
