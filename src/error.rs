#[derive(Debug, thiserror::Error)]
pub enum Error<B>
where
    B: std::error::Error + Send + Sync + 'static,
{
    #[error("failed to execute backend operation")]
    Backend(#[source] B),

    #[error("failed to draw frame")]
    DrawFrame(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}
