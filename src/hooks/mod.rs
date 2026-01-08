use std::fmt::Display;

pub mod post_tool_use;
pub mod pre_tool_use;
pub mod stop;
pub mod user_prompt_submit;

pub use post_tool_use::{
    PostToolUseCallback, PostToolUseDecision, PostToolUseInput, PostToolUseOutput,
};
pub use pre_tool_use::{PreToolUseCallback, PreToolUseDecision, PreToolUseInput, PreToolUseOutput};
pub use stop::{StopCallback, StopDecision, StopInput, StopOutput};
pub use user_prompt_submit::{
    UserPromptSubmitCallback, UserPromptSubmitDecision, UserPromptSubmitInput,
    UserPromptSubmitOutput,
};

#[derive(Default, Clone)]
pub struct Hooks {
    pre_tool_use: Vec<(Option<String>, PreToolUseCallback)>,
    post_tool_use: Vec<(Option<String>, PostToolUseCallback)>,
    user_prompt_submit: Vec<UserPromptSubmitCallback>,
    stop: Vec<StopCallback>,
}

impl Hooks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_pre_tool_use<P, S>(mut self, pattern: P, callback: PreToolUseCallback) -> Self
    where
        P: Into<Option<S>>,
        S: Display,
    {
        let pattern = pattern.into().map(|s| s.to_string());
        self.pre_tool_use.push((pattern, callback));
        self
    }

    pub fn on_post_tool_use<P, S>(mut self, pattern: P, callback: PostToolUseCallback) -> Self
    where
        P: Into<Option<S>>,
        S: Display,
    {
        let pattern = pattern.into().map(|s| s.to_string());
        self.post_tool_use.push((pattern, callback));
        self
    }

    pub fn on_user_prompt_submit(mut self, callback: UserPromptSubmitCallback) -> Self {
        self.user_prompt_submit.push(callback);
        self
    }

    #[must_use]
    pub fn on_stop(mut self, callback: StopCallback) -> Self {
        self.stop.push(callback);
        self
    }

    pub fn add_pre_tool_use<P, S>(&mut self, pattern: P, callback: PreToolUseCallback)
    where
        P: Into<Option<S>>,
        S: Display,
    {
        let pattern = pattern.into().map(|s| s.to_string());
        self.pre_tool_use.push((pattern, callback));
    }

    pub fn add_post_tool_use<P, S>(&mut self, pattern: P, callback: PostToolUseCallback)
    where
        P: Into<Option<S>>,
        S: Display,
    {
        let pattern = pattern.into().map(|s| s.to_string());
        self.post_tool_use.push((pattern, callback));
    }

    pub fn add_user_prompt_submit(&mut self, callback: UserPromptSubmitCallback) {
        self.user_prompt_submit.push(callback);
    }

    pub fn add_stop(&mut self, callback: StopCallback) {
        self.stop.push(callback);
    }

    pub fn user_prompt_submit_hooks(
        &self,
    ) -> impl ExactSizeIterator<Item = &UserPromptSubmitCallback> {
        self.user_prompt_submit.iter()
    }

    pub fn post_tool_use_hooks(
        &self,
    ) -> impl ExactSizeIterator<Item = &(Option<String>, PostToolUseCallback)> {
        self.post_tool_use.iter()
    }

    pub fn pre_tool_use_hooks(
        &self,
    ) -> impl ExactSizeIterator<Item = &(Option<String>, PreToolUseCallback)> {
        self.pre_tool_use.iter()
    }

    pub fn stop_hooks(&self) -> impl ExactSizeIterator<Item = &StopCallback> {
        self.stop.iter()
    }

    pub fn has_pre_tool_use_hooks(&self) -> bool {
        !self.pre_tool_use.is_empty()
    }

    pub fn has_post_tool_use_hooks(&self) -> bool {
        !self.post_tool_use.is_empty()
    }

    pub fn has_user_prompt_submit_hooks(&self) -> bool {
        !self.user_prompt_submit.is_empty()
    }

    pub fn has_stop_hooks(&self) -> bool {
        !self.stop.is_empty()
    }
}
