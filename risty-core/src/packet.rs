use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarshalError {
    #[error("failed to bit pack struct")]
    StructPackedFailure(#[from] packed_struct::PackingError),
}

pub trait Marshal {
    fn marshal(&self, buf: &mut [u8]) -> Result<usize, MarshalError>;

    /// Size in bytes
    fn marshal_size(&self) -> usize;
}
