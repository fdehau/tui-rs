#[derive(Debug, thiserror::Error)]
pub enum Error<E>
where
    E: std::error::Error + Sync + Sync + 'static,
{
    #[error("failed to execute backend operation")]
    Backend(#[source] E),

    #[error("failed to draw frame")]
    DrawFrame(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}
