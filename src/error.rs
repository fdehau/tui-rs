#[derive(Debug, thiserror::Error)]
pub enum Error<B, D>
where
    B: std::error::Error + Send + Sync + 'static,
    D: std::error::Error + Send + Sync + 'static,
{
    #[error("failed to execute backend operation")]
    Backend(#[source] B),

    #[error("failed to draw frame")]
    DrawFrame(#[source] D),
}
