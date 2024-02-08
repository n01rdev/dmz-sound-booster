#![no_std]
#![no_main]

use crate::csr8645::csr8645::Csr8645;
use crate::csr8645::csr8645::Csr8645Error;

/// `AudioService` is a trait that defines the necessary methods for audio services.
pub trait AudioService {
    /// Plays the provided audio data.
    ///
    /// # Arguments
    ///
    /// * `data` - The audio data to be played.
    ///
    /// # Returns
    ///
    /// * `Result<(), Csr8645Error>` - The result of the audio playback operation.
    fn play_audio(&self, data: &[u8]) -> Result<(), Csr8645Error>;

    /// Receives audio data into the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer where the received audio data will be stored.
    ///
    /// # Returns
    ///
    /// * `Result<(), Csr8645Error>` - The result of the audio receiving operation.
    fn receive_audio(&self, buffer: &mut [u8]) -> Result<(), Csr8645Error>;
}

/// `AudioServiceImpl` is a struct that implements the `AudioService` trait.
pub struct AudioServiceImpl<'a> {
    /// A reference to a `Csr8645` instance.
    csr8645: &'a Csr8645<'a>,
}

impl<'a> AudioServiceImpl<'a> {
    /// Creates a new instance of `AudioServiceImpl`.
    ///
    /// # Arguments
    ///
    /// * `csr8645` - A reference to a `Csr8645` instance.
    ///
    /// # Returns
    ///
    /// * `Self` - The new `AudioServiceImpl` instance.
    pub fn new(csr8645: &'a Csr8645<'a>) -> Self {
        Self { csr8645 }
    }
}

impl<'a> AudioService for AudioServiceImpl<'a> {
    fn play_audio(&self, data: &[u8]) -> Result<(), Csr8645Error> {
        self.csr8645.play_audio(data)
    }

    fn receive_audio(&self, buffer: &mut [u8]) -> Result<(), Csr8645Error> {
        self.csr8645.receive_audio(buffer)
    }
}