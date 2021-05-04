#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to execute backend operation")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("failed to draw frame")]
    DrawFrame(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}
