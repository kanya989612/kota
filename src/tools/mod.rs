use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileToolError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Path is not a file: {0}")]
    NotAFile(String),
}

mod create_directory;
mod delete_file;
mod edit_file;
mod execute_bash_command;
mod read_file;
mod scan_codebase;
mod write_file;

pub use create_directory::WrappedCreateDirectoryTool;
pub use delete_file::WrappedDeleteFileTool;
pub use edit_file::WrappedEditFileTool;
pub use execute_bash_command::WrappedExecuteBashCommandTool;
pub use read_file::WrappedReadFileTool;
pub use scan_codebase::WrappedScanCodebaseTool;
pub use write_file::WrappedWriteFileTool;
