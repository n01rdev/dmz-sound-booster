#![no_std]
#![no_main]

use crate::audio::audio_service::AudioService;
use crate::csr8645::csr8645::Csr8645Error;

/// `AudioController` is a struct that controls the audio services.
///
/// It uses an instance of a type that implements the `AudioService` trait to handle audio operations.
pub struct AudioController<'a, T: AudioService + 'a> {
    /// An instance of a type that implements the `AudioService` trait.
    audio_service: T,
}

impl<'a, T: AudioService> AudioController<'a, T> {
    /// Creates a new instance of `AudioController`.
    ///
    /// # Arguments
    ///
    /// * `audio_service` - An instance of a type that implements the `AudioService` trait.
    ///
    /// # Returns
    ///
    /// * `Self` - The new `AudioController` instance.
    pub fn new(audio_service: T) -> Self {
        Self { audio_service }
    }

    /// Handles the transmission of audio data.
    ///
    /// This method receives audio data from a mobile device and plays it on a speaker.
    ///
    /// # Arguments
    ///
    /// * `data` - The audio data to be transmitted.
    ///
    /// # Returns
    ///
    /// * `Result<(), Csr8645Error>` - The result of the audio transmission operation.
    pub async fn handle_audio_transmission(&self, data: &[u8]) -> Result<(), Csr8645Error> {
        let mut buffer = [0u8; 1024]; // Adjust the buffer size according to your needs

        // Receive audio data from the mobile device
        self.audio_service.receive_audio(&mut buffer)?;

        // Play the audio data on the speaker
        self.audio_service.play_audio(&buffer)?;

        Ok(())
    }
}