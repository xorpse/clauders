//! High-level conversation API for multi-turn interactions with Claude.
//!
//! This module provides a builder-style API for managing multi-turn conversations
//! with streaming support and response callbacks.
//!
//! # Example
//!
//! ```no_run
//! use clauders::{Client, Options};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), clauders::Error> {
//!     let client = Client::new(Options::new()).await?;
//!     let mut conv = client.conversation();
//!
//!     // Simple multi-turn conversation
//!     let response1 = conv.say("What is Rust?").await?;
//!     let response2 = conv.say("What about its ownership model?").await?;
//!
//!     // With streaming callback
//!     let text = conv
//!         .turn("Explain async/await")
//!         .on_text(|chunk| print!("{}", chunk))
//!         .send_text()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

use futures::StreamExt;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

use crate::client::Client;
use crate::error::Error;
use crate::response::{Responses, ToolUseResponse};

/// A multi-turn conversation session with builder configuration.
///
/// Tracks conversation history on the client side while the CLI manages
/// the actual session state. History is provided for user convenience
/// to inspect previous turns.
pub struct Conversation<'a> {
    client: &'a Client,
    history: Vec<Turn>,
}

/// A single turn in the conversation.
///
/// Contains the prompt that was sent and all responses received.
#[derive(Debug, Clone)]
pub struct Turn {
    /// The prompt that was sent for this turn
    pub prompt: String,
    /// All responses received for this turn
    pub responses: Responses,
}

impl Turn {
    /// Returns the concatenated text content from this turn's responses.
    pub fn text(&self) -> String {
        self.responses.text_content()
    }
}

type TextCallback<'a> = Box<dyn FnMut(&str) + Send + 'a>;
type ToolUseCallback<'a> = Box<dyn FnMut(&ToolUseResponse) + Send + 'a>;

/// Builder for configuring and executing a single conversation turn.
///
/// Created by [`Conversation::turn`] and provides methods for:
/// - Setting up streaming callbacks for text, thinking, and tool use events
/// - Controlling whether responses are collected
/// - Executing the turn with various return types
pub struct TurnBuilder<'a, 'c> {
    conversation: &'a mut Conversation<'c>,
    prompt: String,
    on_text: Option<TextCallback<'a>>,
    on_thinking: Option<TextCallback<'a>>,
    on_tool_use: Option<ToolUseCallback<'a>>,
    collect: bool,
}

impl<'a> Conversation<'a> {
    /// Creates a new conversation session.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            history: Vec::new(),
        }
    }

    /// Starts building a new turn with the given prompt.
    ///
    /// Returns a [`TurnBuilder`] that can be configured with callbacks
    /// before executing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clauders::{Client, Options};
    /// # async fn example() -> Result<(), clauders::Error> {
    /// # let client = Client::new(Options::new()).await?;
    /// let mut conv = client.conversation();
    ///
    /// let responses = conv
    ///     .turn("Explain async/await")
    ///     .on_text(|chunk| print!("{}", chunk))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn turn(&mut self, prompt: impl Into<String>) -> TurnBuilder<'_, 'a> {
        TurnBuilder {
            conversation: self,
            prompt: prompt.into(),
            on_text: None,
            on_thinking: None,
            on_tool_use: None,
            collect: true,
        }
    }

    /// Sends a simple text query and returns the text response.
    ///
    /// This is a convenience method equivalent to:
    /// ```ignore
    /// conv.turn(prompt).send_text().await
    /// ```
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clauders::{Client, Options};
    /// # async fn example() -> Result<(), clauders::Error> {
    /// # let client = Client::new(Options::new()).await?;
    /// let mut conv = client.conversation();
    ///
    /// let response1 = conv.say("What is Rust?").await?;
    /// let response2 = conv.say("What about its ownership model?").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn say(&mut self, prompt: &str) -> Result<String, Error> {
        self.turn(prompt).send_text().await
    }

    /// Returns the conversation history.
    ///
    /// Each entry represents a single turn (prompt + responses).
    pub fn history(&self) -> &[Turn] {
        &self.history
    }

    /// Returns the last turn in the conversation, if any.
    pub fn last(&self) -> Option<&Turn> {
        self.history.last()
    }

    /// Clears the client-side history.
    ///
    /// Note: The CLI session persists; this only clears the local record
    /// of previous turns.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
}

impl<'a, 'c> TurnBuilder<'a, 'c> {
    /// Sets a callback for text content as it streams.
    ///
    /// The callback is called for each text chunk received.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clauders::{Client, Options};
    /// # async fn example() -> Result<(), clauders::Error> {
    /// # let client = Client::new(Options::new()).await?;
    /// # let mut conv = client.conversation();
    /// conv.turn("Tell me a story")
    ///     .on_text(|chunk| print!("{}", chunk))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn on_text<F>(mut self, f: F) -> Self
    where
        F: FnMut(&str) + Send + 'a,
    {
        self.on_text = Some(Box::new(f));
        self
    }

    /// Sets a callback for thinking content as it streams.
    ///
    /// The callback is called for each thinking chunk received.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clauders::{Client, Options};
    /// # async fn example() -> Result<(), clauders::Error> {
    /// # let client = Client::new(Options::new()).await?;
    /// # let mut conv = client.conversation();
    /// conv.turn("Solve this problem")
    ///     .on_thinking(|thought| eprintln!("[thinking] {}", thought))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn on_thinking<F>(mut self, f: F) -> Self
    where
        F: FnMut(&str) + Send + 'a,
    {
        self.on_thinking = Some(Box::new(f));
        self
    }

    /// Sets a callback for tool use events.
    ///
    /// The callback is called when Claude uses a tool.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clauders::{Client, Options};
    /// # async fn example() -> Result<(), clauders::Error> {
    /// # let client = Client::new(Options::new()).await?;
    /// # let mut conv = client.conversation();
    /// conv.turn("Analyze this code")
    ///     .on_tool_use(|tool| println!("[tool: {}]", tool.name()))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn on_tool_use<F>(mut self, f: F) -> Self
    where
        F: FnMut(&ToolUseResponse) + Send + 'a,
    {
        self.on_tool_use = Some(Box::new(f));
        self
    }

    /// Controls whether responses are collected.
    ///
    /// When set to `false`, responses are not stored in the turn's response
    /// collection, which can reduce memory usage for large responses.
    /// Callbacks are still invoked regardless of this setting.
    ///
    /// Default is `true`.
    pub fn collect(mut self, collect: bool) -> Self {
        self.collect = collect;
        self
    }

    /// Executes the turn and returns the full response collection.
    ///
    /// This method:
    /// 1. Sends the query to Claude
    /// 2. Streams responses, invoking any configured callbacks
    /// 3. Collects responses (if enabled)
    /// 4. Adds the turn to conversation history
    /// 5. Returns the collected responses
    pub async fn send(self) -> Result<Responses, Error> {
        let TurnBuilder {
            conversation,
            prompt,
            mut on_text,
            mut on_thinking,
            mut on_tool_use,
            collect,
        } = self;

        conversation.client.query(&prompt).await?;

        let mut responses = Responses::new();
        let mut stream = std::pin::pin!(conversation.client.receive());

        while let Some(result) = stream.next().await {
            let response = result?;

            if let Some(text) = response.as_text()
                && let Some(ref mut cb) = on_text
            {
                cb(text.content());
            }
            if let Some(thinking) = response.as_thinking()
                && let Some(ref mut cb) = on_thinking
            {
                cb(thinking.content());
            }
            if let Some(tool_use) = response.as_tool_use()
                && let Some(ref mut cb) = on_tool_use
            {
                cb(tool_use);
            }

            if collect {
                responses.push(response);
            }
        }

        conversation.history.push(Turn {
            prompt,
            responses: responses.clone(),
        });

        Ok(responses)
    }

    /// Executes the turn and returns just the text content.
    ///
    /// This is equivalent to calling [`send`](Self::send) and then
    /// extracting the text content from the responses.
    pub async fn send_text(self) -> Result<String, Error> {
        let responses = self.send().await?;
        Ok(responses.text_content())
    }

    /// Executes the turn and deserializes the structured output.
    ///
    /// Requires that the client was created with a JSON schema matching
    /// the type `T`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use clauders::{Client, Options};
    /// use schemars::JsonSchema;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize, JsonSchema)]
    /// struct Analysis {
    ///     summary: String,
    ///     score: i32,
    /// }
    ///
    /// # async fn example() -> Result<(), clauders::Error> {
    /// let client = Client::new(
    ///     Options::new().with_json_schema::<Analysis>()
    /// ).await?;
    ///
    /// let mut conv = client.conversation();
    /// let analysis: Analysis = conv
    ///     .turn("Analyze this text")
    ///     .send_as()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_as<T>(self) -> Result<T, Error>
    where
        T: DeserializeOwned + JsonSchema,
    {
        let responses = self.send().await?;

        let completion = responses
            .completion()
            .ok_or_else(|| Error::ProtocolError("no completion response".to_owned()))?;

        let structured_output = completion
            .structured_output()
            .ok_or_else(|| Error::ProtocolError("no structured output in response".to_owned()))?;

        let result = serde_json::from_value::<T>(structured_output.clone())?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require mocking or integration with Claude CLI
    // For now, we just test the basic structure

    #[test]
    fn test_turn_text() {
        let turn = Turn {
            prompt: "Hello".to_string(),
            responses: Responses::new(),
        };
        assert_eq!(turn.text(), "");
        assert_eq!(turn.prompt, "Hello");
    }
}
