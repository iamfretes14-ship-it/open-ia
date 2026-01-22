use std::path::PathBuf;
use std::sync::Arc;

use codex_protocol::ThreadId;
use futures::future::BoxFuture;
use serde::Serialize;

pub(crate) type HookFn =
    Arc<dyn for<'a> Fn(&'a HookPayload) -> BoxFuture<'a, HookOutcome> + Send + Sync>;

#[derive(Clone)]
pub(crate) struct Hook {
    pub(crate) func: HookFn,
}

impl Default for Hook {
    fn default() -> Self {
        Self {
            func: Arc::new(|_| Box::pin(async { HookOutcome::Continue })),
        }
    }
}

impl Hook {
    pub(super) async fn execute(&self, payload: &HookPayload) -> HookOutcome {
        (self.func)(payload).await
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) struct HookPayload {
    pub(crate) session_id: ThreadId,
    pub(crate) cwd: PathBuf,
    pub(crate) triggered_at: String,
    pub(crate) hook_event: HookEvent,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct HookEventAfterAgent {
    pub thread_id: String,
    pub turn_id: String,
    pub input_messages: Vec<String>,
    pub last_assistant_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub(crate) enum HookEvent {
    AfterAgent {
        #[serde(flatten)]
        event: HookEventAfterAgent,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HookOutcome {
    Continue,
    #[allow(dead_code)]
    Stop,
}
