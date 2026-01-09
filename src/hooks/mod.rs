use std::fmt::{Debug, Display};
use std::future::Future;
use std::sync::Arc;

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

    #[must_use]
    pub fn on_pre_tool_use<P, S, F, Fut>(mut self, pattern: P, callback: F) -> Self
    where
        P: Into<Option<S>>,
        S: Display,
        F: Fn(PreToolUseInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = PreToolUseOutput> + Send + 'static,
    {
        let pattern = pattern.into().map(|s| s.to_string());
        self.pre_tool_use
            .push((pattern, Arc::new(move |input| Box::pin(callback(input)))));
        self
    }

    #[must_use]
    pub fn on_post_tool_use<P, S, F, Fut>(mut self, pattern: P, callback: F) -> Self
    where
        P: Into<Option<S>>,
        S: Display,
        F: Fn(PostToolUseInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = PostToolUseOutput> + Send + 'static,
    {
        let pattern = pattern.into().map(|s| s.to_string());
        self.post_tool_use
            .push((pattern, Arc::new(move |input| Box::pin(callback(input)))));
        self
    }

    #[must_use]
    pub fn on_user_prompt_submit<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn(UserPromptSubmitInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = UserPromptSubmitOutput> + Send + 'static,
    {
        self.user_prompt_submit
            .push(Arc::new(move |input| Box::pin(callback(input))));
        self
    }

    #[must_use]
    pub fn on_stop<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn(StopInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = StopOutput> + Send + 'static,
    {
        self.stop
            .push(Arc::new(move |input| Box::pin(callback(input))));
        self
    }

    pub fn add_pre_tool_use<P, S, F, Fut>(&mut self, pattern: P, callback: F)
    where
        P: Into<Option<S>>,
        S: Display,
        F: Fn(PreToolUseInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = PreToolUseOutput> + Send + 'static,
    {
        let pattern = pattern.into().map(|s| s.to_string());
        self.pre_tool_use
            .push((pattern, Arc::new(move |input| Box::pin(callback(input)))));
    }

    pub fn add_post_tool_use<P, S, F, Fut>(&mut self, pattern: P, callback: F)
    where
        P: Into<Option<S>>,
        S: Display,
        F: Fn(PostToolUseInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = PostToolUseOutput> + Send + 'static,
    {
        let pattern = pattern.into().map(|s| s.to_string());
        self.post_tool_use
            .push((pattern, Arc::new(move |input| Box::pin(callback(input)))));
    }

    pub fn add_user_prompt_submit<F, Fut>(&mut self, callback: F)
    where
        F: Fn(UserPromptSubmitInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = UserPromptSubmitOutput> + Send + 'static,
    {
        self.user_prompt_submit
            .push(Arc::new(move |input| Box::pin(callback(input))));
    }

    pub fn add_stop<F, Fut>(&mut self, callback: F)
    where
        F: Fn(StopInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = StopOutput> + Send + 'static,
    {
        self.stop
            .push(Arc::new(move |input| Box::pin(callback(input))));
    }

    pub fn user_prompt_submit_hooks(
        &self,
    ) -> impl ExactSizeIterator<Item = &UserPromptSubmitCallback> {
        self.user_prompt_submit.iter()
    }

    pub fn get_user_prompt_submit_hook(&self, index: usize) -> Option<&UserPromptSubmitCallback> {
        self.user_prompt_submit.get(index)
    }

    pub fn post_tool_use_hooks(
        &self,
    ) -> impl ExactSizeIterator<Item = &(Option<String>, PostToolUseCallback)> {
        self.post_tool_use.iter()
    }

    pub fn get_post_tool_use_hook(
        &self,
        index: usize,
    ) -> Option<&(Option<String>, PostToolUseCallback)> {
        self.post_tool_use.get(index)
    }

    pub fn pre_tool_use_hooks(
        &self,
    ) -> impl ExactSizeIterator<Item = &(Option<String>, PreToolUseCallback)> {
        self.pre_tool_use.iter()
    }

    pub fn get_pre_tool_use_hook(
        &self,
        index: usize,
    ) -> Option<&(Option<String>, PreToolUseCallback)> {
        self.pre_tool_use.get(index)
    }

    pub fn stop_hooks(&self) -> impl ExactSizeIterator<Item = &StopCallback> {
        self.stop.iter()
    }

    pub fn get_stop_hook(&self, index: usize) -> Option<&StopCallback> {
        self.stop.get(index)
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

impl Debug for Hooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hooks")
            .field("pre_tool_use", &self.pre_tool_use.len())
            .field("post_tool_use", &self.post_tool_use.len())
            .field("user_prompt_submit", &self.user_prompt_submit.len())
            .field("stop", &self.stop.len())
            .finish()
    }
}

impl From<PostToolUseCallback> for Hooks {
    fn from(callback: PostToolUseCallback) -> Self {
        let mut hooks = Self::new();
        hooks.post_tool_use.push((None, callback));
        hooks
    }
}

impl From<PreToolUseCallback> for Hooks {
    fn from(callback: PreToolUseCallback) -> Self {
        let mut hooks = Self::new();
        hooks.pre_tool_use.push((None, callback));
        hooks
    }
}

impl From<UserPromptSubmitCallback> for Hooks {
    fn from(callback: UserPromptSubmitCallback) -> Self {
        let mut hooks = Self::new();
        hooks.user_prompt_submit.push(callback);
        hooks
    }
}

impl From<StopCallback> for Hooks {
    fn from(callback: StopCallback) -> Self {
        let mut hooks = Self::new();
        hooks.stop.push(callback);
        hooks
    }
}
