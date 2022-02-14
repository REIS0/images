use gif::EncodingError;

#[derive(Debug)]
pub enum GifGenerationError {
    FileCreationError(std::io::Error),
    EncondingError(EncodingError)
}