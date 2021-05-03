#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to draw to terminal")]
    Draw(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("failed to flush")]
    Flush(#[source] std::io::Error),

    #[error("failed to get terminal size")]
    GetTerminalSize(#[source] std::io::Error),

    #[error("failed to draw background")]
    DrawBackground(#[source] std::fmt::Error),

    #[error("failed to draw foreground")]
    DrawForeground(#[source] std::fmt::Error),

    #[error("failed to construct modifier diff")]
    ModifierDiff(#[source] std::fmt::Error),

    #[error("failed to move cursor to position: {1:?}")]
    MoveCursor(
        #[source] Box<dyn std::error::Error + Send + Sync + 'static>,
        (u16, u16),
    ),

    #[error("failed to get cursor position")]
    GetCursosPos(#[source] std::io::Error),

    #[error("failed to show cursor")]
    ShowCursor(#[source] std::io::Error),

    #[error("failed to hide cursor")]
    HideCursor(#[source] std::io::Error),

    #[error("failed to clear terminal")]
    Clear(#[source] std::io::Error),
}
