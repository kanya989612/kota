use rig::agent::StreamingPromptHook;
use rig::completion::CompletionModel;

/// Session-aware hook that logs tool calls and completions with session context
#[derive(Clone)]
pub struct SessionIdHook {
    pub session_id: String,
}

impl SessionIdHook {
    pub fn new(session_id: String) -> Self {
        Self { session_id }
    }
}

impl<M: CompletionModel> StreamingPromptHook<M> for SessionIdHook {
   
}
