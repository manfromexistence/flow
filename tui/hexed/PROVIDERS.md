OK. Now I have the full picture of how codex-rs handles providers at every layer. Here's the complete breakdown of what you need to change and where.

---

# Complete Guide: Making Codex-RS Support Non-OpenAI Providers

## The Problem: Three Lock-In Points

Codex has **three distinct layers** that lock you to OpenAI. You must modify all three:

1. **Wire API Lock** — Only `Responses` API is supported now; `chat` was removed
2. **Built-in Provider Registry** — Only ships OpenAI + Ollama + LMStudio
3. **Authentication Lock** — Hardcoded ChatGPT OAuth + OpenAI API key flow

---

## Layer 1: The Wire API — The Core Bottleneck

### What It Is Now

The `WireApi` enum defines the wire protocol that the provider speaks. Currently it only has one variant — `Responses` — which maps to the OpenAI Responses API at `/v1/responses`. The `chat` variant has been removed: attempting to deserialize `"chat"` returns an error `CHAT_WIRE_API_REMOVED_ERROR`.

The error message states: `` `wire_api = "chat"` is no longer supported. How to fix: set `wire_api = "responses"` in your provider config. ``

This is the **#1 blocker**. Anthropic, Google, Mistral, Groq — none of them speak the OpenAI Responses API natively. They speak either their own native APIs or the OpenAI Chat Completions API (`/v1/chat/completions`).

### What The Request Pipeline Looks Like

Codex supports two wire protocols for LLM communication, managed through the `WireApi` enum. The modern OpenAI Responses API (`/v1/responses`) has a request path: `ModelClient::stream()` → `stream_responses_api()` → `ApiResponsesClient::stream_prompt()`, implemented in `codex-rs/core/src/client.rs` lines 194-287. The classic Chat Completions API (`/v1/chat/completions`) has a request path: `ModelClient::stream()` → `stream_chat_completions()` → `ApiChatClient::stream_prompt()`.

The key methods in `client.rs` are: `ModelClient::stream()` (main entry point, dispatches to wire API-specific methods at line 119-138), `stream_responses_api()` (handles `/v1/responses`, lines 198-287), `stream_chat_completions()` (handles `/v1/chat/completions`, lines 144-192), and `compact_conversation_history()` (unary compaction, lines 329-372).

### What You Must Change: `codex-rs/core/src/model_provider_info.rs`

**Step 1: Restore the `Chat` variant and add native provider variants**

```rust
// codex-rs/core/src/model_provider_info.rs

/// Wire protocol that the provider speaks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WireApi {
    /// The Responses API exposed by OpenAI at `/v1/responses`.
    #[default]
    Responses,

    /// The Chat Completions API at `/v1/chat/completions`.
    /// Supported by most third-party providers.
    Chat,

    /// Anthropic Messages API at `/v1/messages`.
    AnthropicMessages,

    /// Google Gemini API at `/v1beta/models/{model}:streamGenerateContent`.
    GeminiGenerateContent,
}

impl<'de> Deserialize<'de> for WireApi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "responses" => Ok(Self::Responses),
            "chat" => Ok(Self::Chat),
            "anthropic" | "anthropic_messages" => Ok(Self::AnthropicMessages),
            "gemini" | "gemini_generate_content" => Ok(Self::GeminiGenerateContent),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &["responses", "chat", "anthropic", "gemini"],
            )),
        }
    }
}
```

**Step 2: Remove the deprecation/removal guards**

In `codex-rs/core/src/codex.rs`, there's code that emits a deprecation notice when `wire_api == WireApi::Chat`:

The code checks `if config.model_provider.wire_api != WireApi::Chat { return; }` and pushes a `DeprecationNotice` event with `CHAT_WIRE_API_DEPRECATION_SUMMARY`.

**Remove** or gate this behind a feature flag. In your fork:

```rust
// codex-rs/core/src/codex.rs
// DELETE or comment out the entire emit_chat_wire_api_deprecation() function
// and its call site
```

---

## Layer 2: The ModelProviderInfo Struct

### What It Is Now

The `ModelProviderInfo` struct contains: a friendly display name, `base_url`, `env_key`, `env_key_instructions`, `experimental_bearer_token`, `wire_api: WireApi`, `query_params`, `http_headers`, `env_http_headers`, `request_max_retries`, `stream_max_retries`, and `stream_idle_timeout_ms`.

### What Built-In Providers Ship

The `built_in_model_providers()` function states: "We do not want to be in the business of adjudicating which third-party providers are bundled with Codex CLI, so we only include the OpenAI and open source ('oss') providers by default." It returns three entries: `("openai", P::create_openai_provider())`, `(OLLAMA_OSS_PROVIDER_ID, create_oss_provider(...))`, and `(LMSTUDIO_OSS_PROVIDER_ID, create_oss_provider(...))`.

### What You Must Change: Add built-in providers for every major AI company

```rust
// codex-rs/core/src/model_provider_info.rs

pub fn built_in_model_providers() -> HashMap<String, ModelProviderInfo> {
    use ModelProviderInfo as P;

    [
        // === EXISTING ===
        ("openai", P::create_openai_provider()),
        (
            OLLAMA_OSS_PROVIDER_ID,
            create_oss_provider(DEFAULT_OLLAMA_PORT, WireApi::Responses),
        ),
        (
            LMSTUDIO_OSS_PROVIDER_ID,
            create_oss_provider(DEFAULT_LMSTUDIO_PORT, WireApi::Responses),
        ),

        // === NEW: Anthropic ===
        ("anthropic", ModelProviderInfo {
            name: "Anthropic".into(),
            base_url: Some("https://api.anthropic.com/v1".into()),
            env_key: Some("ANTHROPIC_API_KEY".into()),
            env_key_instructions: Some(
                "Get your API key at https://console.anthropic.com/settings/keys".into()
            ),
            experimental_bearer_token: None,
            wire_api: WireApi::AnthropicMessages,
            query_params: None,
            http_headers: Some(
                [
                    ("anthropic-version".to_string(), "2023-06-01".to_string()),
                ]
                .into_iter()
                .collect(),
            ),
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),

        // === NEW: Google Gemini ===
        ("gemini", ModelProviderInfo {
            name: "Google Gemini".into(),
            base_url: Some("https://generativelanguage.googleapis.com".into()),
            env_key: Some("GEMINI_API_KEY".into()),
            env_key_instructions: Some(
                "Get your API key at https://aistudio.google.com/apikey".into()
            ),
            experimental_bearer_token: None,
            wire_api: WireApi::GeminiGenerateContent,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),

        // === NEW: OpenAI-Compatible Chat providers ===
        // These use wire_api = Chat because they implement
        // the /v1/chat/completions endpoint

        ("groq", ModelProviderInfo {
            name: "Groq".into(),
            base_url: Some("https://api.groq.com/openai/v1".into()),
            env_key: Some("GROQ_API_KEY".into()),
            env_key_instructions: Some(
                "Get your API key at https://console.groq.com/keys".into()
            ),
            experimental_bearer_token: None,
            wire_api: WireApi::Chat,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),

        ("mistral", ModelProviderInfo {
            name: "Mistral".into(),
            base_url: Some("https://api.mistral.ai/v1".into()),
            env_key: Some("MISTRAL_API_KEY".into()),
            env_key_instructions: Some(
                "Get your API key at https://console.mistral.ai/api-keys".into()
            ),
            experimental_bearer_token: None,
            wire_api: WireApi::Chat,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),

        ("together", ModelProviderInfo {
            name: "Together AI".into(),
            base_url: Some("https://api.together.xyz/v1".into()),
            env_key: Some("TOGETHER_API_KEY".into()),
            env_key_instructions: None,
            experimental_bearer_token: None,
            wire_api: WireApi::Chat,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),

        ("fireworks", ModelProviderInfo {
            name: "Fireworks AI".into(),
            base_url: Some("https://api.fireworks.ai/inference/v1".into()),
            env_key: Some("FIREWORKS_API_KEY".into()),
            env_key_instructions: None,
            experimental_bearer_token: None,
            wire_api: WireApi::Chat,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),

        ("deepseek", ModelProviderInfo {
            name: "DeepSeek".into(),
            base_url: Some("https://api.deepseek.com/v1".into()),
            env_key: Some("DEEPSEEK_API_KEY".into()),
            env_key_instructions: None,
            experimental_bearer_token: None,
            wire_api: WireApi::Chat,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),

        ("xai", ModelProviderInfo {
            name: "xAI (Grok)".into(),
            base_url: Some("https://api.x.ai/v1".into()),
            env_key: Some("XAI_API_KEY".into()),
            env_key_instructions: None,
            experimental_bearer_token: None,
            wire_api: WireApi::Chat,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),

        // === NEW: OpenRouter (access to everything) ===
        ("openrouter", ModelProviderInfo {
            name: "OpenRouter".into(),
            base_url: Some("https://openrouter.ai/api/v1".into()),
            env_key: Some("OPENROUTER_API_KEY".into()),
            env_key_instructions: Some(
                "Get your API key at https://openrouter.ai/keys".into()
            ),
            experimental_bearer_token: None,
            wire_api: WireApi::Chat,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(5),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}
```

---

## Layer 3: The Client — Where HTTP Requests Actually Happen

### The File: `codex-rs/core/src/client.rs`

This is the most complex change. Each provider is defined by a `ModelProviderInfo` struct that specifies connection parameters, authentication requirements, wire protocol type, and retry policies. The `ModelClient` struct consumes this configuration to create authenticated HTTP requests and handle streaming responses.

The `ModelClient::stream()` method is the dispatch point. Currently it looks like this:

```rust
// codex-rs/core/src/client.rs (CURRENT — simplified)
impl ModelClient {
    pub async fn stream(&self, prompt: Prompt) -> Result<ResponseStream> {
        match self.wire_api {
            WireApi::Responses => self.stream_responses_api(prompt).await,
            // Chat variant was here but removed
        }
    }
}
```

### What You Must Add

You need to implement four streaming paths:

```rust
// codex-rs/core/src/client.rs (YOUR FORK)
impl ModelClient {
    pub async fn stream(&self, prompt: Prompt) -> Result<ResponseStream> {
        match self.wire_api {
            WireApi::Responses => self.stream_responses_api(prompt).await,
            WireApi::Chat => self.stream_chat_completions(prompt).await,
            WireApi::AnthropicMessages => self.stream_anthropic(prompt).await,
            WireApi::GeminiGenerateContent => self.stream_gemini(prompt).await,
        }
    }
}
```

#### 3A: Restore `stream_chat_completions()`

The Chat Completions path was previously implemented. You can find it in older versions of the codebase (pre-removal). The key shape:

```rust
// codex-rs/core/src/client.rs
async fn stream_chat_completions(&self, prompt: Prompt) -> Result<ResponseStream> {
    let url = format!("{}/chat/completions", self.base_url);

    // Convert codex's internal Prompt into Chat Completions format
    let messages = prompt_to_chat_messages(&prompt);

    let body = serde_json::json!({
        "model": self.model,
        "messages": messages,
        "stream": true,
        "stream_options": { "include_usage": true },
        // Include tools if present
        "tools": prompt.tools_for_chat_format(),
    });

    let request = self.http_client
        .post(&url)
        .header("Authorization", format!("Bearer {}", self.api_key))
        .header("Content-Type", "application/json")
        .json(&body);

    // Add any custom headers from ModelProviderInfo
    let request = self.apply_provider_headers(request);

    let response = request.send().await?;
    let stream = response.bytes_stream();

    // Parse SSE stream and convert chat completion deltas
    // into codex's internal ResponseEvent format
    Ok(ResponseStream::from_chat_sse(stream))
}
```

**The critical translation**: Codex internally uses `ResponseEvent` as its unified event format. The Chat Completions SSE stream emits events like:

```
data: {"id":"chatcmpl-...","choices":[{"delta":{"content":"Hello"},"index":0}]}
```

You must convert these into codex's `ResponseEvent` variants (which normally come from the Responses API). Create a translator:

```rust
// codex-rs/core/src/client_chat_compat.rs (NEW FILE)

use crate::client_common::ResponseEvent;

/// Translates a Chat Completions SSE delta into codex's ResponseEvent.
pub fn chat_delta_to_response_event(delta: &ChatCompletionDelta) -> Vec<ResponseEvent> {
    let mut events = Vec::new();

    // Text content
    if let Some(content) = &delta.choices[0].delta.content {
        events.push(ResponseEvent::OutputTextDelta {
            text: content.clone(),
        });
    }

    // Tool calls
    if let Some(tool_calls) = &delta.choices[0].delta.tool_calls {
        for tc in tool_calls {
            if let Some(function) = &tc.function {
                events.push(ResponseEvent::FunctionCallDelta {
                    call_id: tc.id.clone().unwrap_or_default(),
                    name: function.name.clone(),
                    arguments_delta: function.arguments.clone().unwrap_or_default(),
                });
            }
        }
    }

    // Reasoning/thinking tokens (DeepSeek, QwQ, etc.)
    if let Some(reasoning) = &delta.choices[0].delta.reasoning_content {
        events.push(ResponseEvent::ReasoningDelta {
            text: reasoning.clone(),
        });
    }

    // Finish reason
    if let Some(finish) = &delta.choices[0].finish_reason {
        match finish.as_str() {
            "stop" => events.push(ResponseEvent::OutputTextDone),
            "tool_calls" => events.push(ResponseEvent::FunctionCallDone),
            _ => {}
        }
    }

    events
}
```

#### 3B: Implement `stream_anthropic()`

Anthropic's Messages API is NOT OpenAI-compatible. Different request format, different SSE events.

```rust
// codex-rs/core/src/client_anthropic.rs (NEW FILE)

use crate::client_common::{Prompt, ResponseEvent};

/// Request body for Anthropic's /v1/messages endpoint
#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking: Option<AnthropicThinking>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,  // "user" or "assistant"
    content: AnthropicContent,
}

#[derive(Serialize)]
#[serde(untagged)]
enum AnthropicContent {
    Text(String),
    Blocks(Vec<AnthropicContentBlock>),
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse { id: String, name: String, input: serde_json::Value },
    #[serde(rename = "tool_result")]
    ToolResult { tool_use_id: String, content: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String },
}

/// Convert codex's internal Prompt into Anthropic format
pub fn prompt_to_anthropic(prompt: &Prompt) -> AnthropicRequest {
    let mut system_text = None;
    let mut messages = Vec::new();

    for item in &prompt.items {
        match item {
            // System message → Anthropic's top-level `system` field
            ResponseInputItem::SystemMessage { content } => {
                system_text = Some(content.clone());
            }
            // User messages
            ResponseInputItem::UserMessage { content } => {
                messages.push(AnthropicMessage {
                    role: "user".into(),
                    content: AnthropicContent::Text(content.clone()),
                });
            }
            // Assistant messages
            ResponseInputItem::AssistantMessage { content } => {
                messages.push(AnthropicMessage {
                    role: "assistant".into(),
                    content: AnthropicContent::Text(content.clone()),
                });
            }
            // Tool results → tool_result content blocks in a user message
            ResponseInputItem::FunctionCallOutput { call_id, output } => {
                messages.push(AnthropicMessage {
                    role: "user".into(),
                    content: AnthropicContent::Blocks(vec![
                        AnthropicContentBlock::ToolResult {
                            tool_use_id: call_id.clone(),
                            content: output.clone(),
                        }
                    ]),
                });
            }
            _ => {}
        }
    }

    AnthropicRequest {
        model: prompt.model.clone(),
        max_tokens: prompt.max_output_tokens.unwrap_or(8192),
        messages,
        system: system_text,
        stream: true,
        tools: convert_tools_to_anthropic(&prompt.tools),
        thinking: if prompt.reasoning_effort.is_some() {
            Some(AnthropicThinking {
                r#type: "enabled".into(),
                budget_tokens: 10000,
            })
        } else {
            None
        },
    }
}

/// Parse Anthropic SSE events into codex ResponseEvents
///
/// Anthropic SSE event types:
///   message_start        → contains message metadata
///   content_block_start  → new content block (text, tool_use, thinking)
///   content_block_delta  → incremental content
///   content_block_stop   → block finished
///   message_delta        → stop_reason, usage
///   message_stop         → done
pub fn anthropic_sse_to_response_event(
    event_type: &str,
    data: &serde_json::Value,
) -> Vec<ResponseEvent> {
    let mut events = Vec::new();

    match event_type {
        "content_block_delta" => {
            if let Some(delta) = data.get("delta") {
                let delta_type = delta.get("type").and_then(|t| t.as_str());
                match delta_type {
                    Some("text_delta") => {
                        if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                            events.push(ResponseEvent::OutputTextDelta {
                                text: text.to_string(),
                            });
                        }
                    }
                    Some("input_json_delta") => {
                        // Tool use argument streaming
                        if let Some(json) = delta.get("partial_json").and_then(|t| t.as_str()) {
                            events.push(ResponseEvent::FunctionCallDelta {
                                call_id: String::new(), // filled from content_block_start
                                name: String::new(),
                                arguments_delta: json.to_string(),
                            });
                        }
                    }
                    Some("thinking_delta") => {
                        if let Some(text) = delta.get("thinking").and_then(|t| t.as_str()) {
                            events.push(ResponseEvent::ReasoningDelta {
                                text: text.to_string(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        "content_block_stop" => {
            // Could be end of text or end of tool_use
            events.push(ResponseEvent::OutputTextDone);
        }
        "message_delta" => {
            // Contains stop_reason and usage
            if let Some(usage) = data.get("usage") {
                events.push(ResponseEvent::UsageUpdate {
                    input_tokens: usage.get("input_tokens")
                        .and_then(|t| t.as_u64()).unwrap_or(0),
                    output_tokens: usage.get("output_tokens")
                        .and_then(|t| t.as_u64()).unwrap_or(0),
                });
            }
        }
        "message_stop" => {
            events.push(ResponseEvent::Done);
        }
        _ => {}
    }

    events
}
```

#### 3C: Implement `stream_gemini()`

```rust
// codex-rs/core/src/client_gemini.rs (NEW FILE)

/// Gemini uses a different streaming mechanism:
/// POST /v1beta/models/{model}:streamGenerateContent?alt=sse&key={key}
///
/// Request body uses `contents` array with `parts`

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<GeminiTool>>,
    generation_config: GeminiGenerationConfig,
}

#[derive(Serialize)]
struct GeminiContent {
    role: String,  // "user" or "model"
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum GeminiPart {
    Text { text: String },
    FunctionCall { function_call: GeminiFunctionCall },
    FunctionResponse { function_response: GeminiFunctionResponse },
}

pub fn prompt_to_gemini(prompt: &Prompt) -> GeminiRequest {
    let mut system = None;
    let mut contents = Vec::new();

    for item in &prompt.items {
        match item {
            ResponseInputItem::SystemMessage { content } => {
                system = Some(GeminiContent {
                    role: "user".into(),
                    parts: vec![GeminiPart::Text { text: content.clone() }],
                });
            }
            ResponseInputItem::UserMessage { content } => {
                contents.push(GeminiContent {
                    role: "user".into(),
                    parts: vec![GeminiPart::Text { text: content.clone() }],
                });
            }
            ResponseInputItem::AssistantMessage { content } => {
                contents.push(GeminiContent {
                    role: "model".into(),
                    parts: vec![GeminiPart::Text { text: content.clone() }],
                });
            }
            _ => {}
        }
    }

    GeminiRequest {
        contents,
        system_instruction: system,
        tools: convert_tools_to_gemini(&prompt.tools),
        generation_config: GeminiGenerationConfig {
            max_output_tokens: prompt.max_output_tokens,
        },
    }
}

/// Gemini SSE data contains `candidates[0].content.parts[0].text`
pub fn gemini_sse_to_response_event(data: &serde_json::Value) -> Vec<ResponseEvent> {
    let mut events = Vec::new();

    if let Some(candidates) = data.get("candidates").and_then(|c| c.as_array()) {
        if let Some(candidate) = candidates.first() {
            if let Some(content) = candidate.get("content") {
                if let Some(parts) = content.get("parts").and_then(|p| p.as_array()) {
                    for part in parts {
                        if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                            events.push(ResponseEvent::OutputTextDelta {
                                text: text.to_string(),
                            });
                        }
                        if let Some(fc) = part.get("functionCall") {
                            events.push(ResponseEvent::FunctionCallDelta {
                                call_id: uuid::Uuid::new_v4().to_string(),
                                name: fc.get("name")
                                    .and_then(|n| n.as_str())
                                    .unwrap_or("")
                                    .to_string(),
                                arguments_delta: fc.get("args")
                                    .map(|a| a.to_string())
                                    .unwrap_or_default(),
                            });
                        }
                    }
                }
            }
            // Check finish reason
            if let Some(reason) = candidate.get("finishReason").and_then(|r| r.as_str()) {
                if reason == "STOP" {
                    events.push(ResponseEvent::Done);
                }
            }
        }
    }

    // Usage metadata
    if let Some(usage) = data.get("usageMetadata") {
        events.push(ResponseEvent::UsageUpdate {
            input_tokens: usage.get("promptTokenCount")
                .and_then(|t| t.as_u64()).unwrap_or(0),
            output_tokens: usage.get("candidatesTokenCount")
                .and_then(|t| t.as_u64()).unwrap_or(0),
        });
    }

    events
}
```

#### 3D: Wire all paths into `ModelClient::stream()`

```rust
// codex-rs/core/src/client.rs (YOUR FORK — the stream dispatch)

mod client_chat_compat;
mod client_anthropic;
mod client_gemini;

impl ModelClient {
    pub async fn stream(&self, prompt: Prompt) -> Result<ResponseStream> {
        match self.wire_api {
            WireApi::Responses => {
                self.stream_responses_api(prompt).await
            }
            WireApi::Chat => {
                self.stream_chat_completions(prompt).await
            }
            WireApi::AnthropicMessages => {
                self.stream_anthropic_messages(prompt).await
            }
            WireApi::GeminiGenerateContent => {
                self.stream_gemini_content(prompt).await
            }
        }
    }

    async fn stream_anthropic_messages(&self, prompt: Prompt) -> Result<ResponseStream> {
        let url = format!("{}/messages", self.base_url);
        let body = client_anthropic::prompt_to_anthropic(&prompt);

        let api_key = self.resolve_api_key()?;
        let request = self.http_client
            .post(&url)
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body);

        let request = self.apply_provider_headers(request);
        let response = request.send().await?;
        let stream = response.bytes_stream();

        Ok(ResponseStream::from_anthropic_sse(
            stream,
            client_anthropic::anthropic_sse_to_response_event,
        ))
    }

    async fn stream_gemini_content(&self, prompt: Prompt) -> Result<ResponseStream> {
        let api_key = self.resolve_api_key()?;
        let url = format!(
            "{}/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            self.base_url, self.model, api_key
        );

        let body = client_gemini::prompt_to_gemini(&prompt);
        let request = self.http_client
            .post(&url)
            .header("content-type", "application/json")
            .json(&body);

        let request = self.apply_provider_headers(request);
        let response = request.send().await?;
        let stream = response.bytes_stream();

        Ok(ResponseStream::from_gemini_sse(
            stream,
            client_gemini::gemini_sse_to_response_event,
        ))
    }
}
```

---

## Layer 4: Authentication Bypass for Non-OpenAI Providers

The authentication system supports two primary modes: ChatGPT OAuth (uses OpenAI's OAuth flow for users with ChatGPT Plus, Pro, or Team plans) and API Key (uses traditional OpenAI API keys for usage-based billing). The system is built around the `CodexAuth` struct and `AuthManager`, with automatic token refresh, persistent storage in `auth.json`, and fallback to environment variables.

For non-OpenAI providers, you must **skip** the ChatGPT OAuth flow entirely. The `requires_openai_auth` field on `ModelProviderInfo` is the key:

```rust
// In your provider definitions, ensure:
requires_openai_auth: false,

// Then in the auth flow (codex-rs/login/src/lib.rs),
// ensure that when requires_openai_auth is false,
// the system only looks for the env_key environment variable
// and does NOT attempt ChatGPT OAuth.
```

The existing code already respects this for the OSS providers. Your new providers set `requires_openai_auth: false` so they'll use the `env_key` (e.g., `ANTHROPIC_API_KEY`) directly.

---

## Layer 5: The OSS Provider Validator — Remove the Restriction

The `set_default_oss_provider` function validates that the provider is one of the known OSS providers. It matches against `LMSTUDIO_OSS_PROVIDER_ID` and `OLLAMA_OSS_PROVIDER_ID` only, and returns an error for anything else: `"Invalid OSS provider '{provider}'. Must be one of: LMSTUDIO, OLLAMA"`.

You need to either remove this validation or extend it:

```rust
// codex-rs/core/src/config/mod.rs
pub fn set_default_oss_provider(codex_home: &Path, provider: &str) -> std::io::Result<()> {
    // REMOVE the strict validation, or expand it:
    // Accept any provider that exists in the model_providers map
    // instead of hardcoding a whitelist
    let known_providers = built_in_model_providers();
    if !known_providers.contains_key(provider) {
        // Still allow it — user might have defined it in config.toml
        tracing::warn!(
            "Provider '{}' is not a built-in provider. \
             Ensure it's defined in [model_providers] in config.toml.",
            provider
        );
    }

    use toml_edit::value;
    let edits = [ConfigEdit::SetPath {
        segments: vec!["oss_provider".to_string()],
        value: value(provider),
    }];
    // ... rest of function
}
```

---

## Layer 6: Tool Format Translation

This is subtle but critical. Codex defines tools (shell exec, apply patch, etc.) in OpenAI's tool format. Non-OpenAI providers need different tool schemas.

**OpenAI tool format:**
```json
{"type": "function", "function": {"name": "shell", "parameters": {...}}}
```

**Anthropic tool format:**
```json
{"name": "shell", "input_schema": {...}}
```

**Gemini tool format:**
```json
{"function_declarations": [{"name": "shell", "parameters": {...}}]}
```

Add a translator in each client module:

```rust
// codex-rs/core/src/client_anthropic.rs
pub fn convert_tools_to_anthropic(tools: &[Tool]) -> Option<Vec<AnthropicTool>> {
    if tools.is_empty() { return None; }

    Some(tools.iter().map(|t| AnthropicTool {
        name: t.function.name.clone(),
        description: t.function.description.clone(),
        input_schema: t.function.parameters.clone(),
    }).collect())
}

// codex-rs/core/src/client_gemini.rs
pub fn convert_tools_to_gemini(tools: &[Tool]) -> Option<Vec<GeminiTool>> {
    if tools.is_empty() { return None; }

    Some(vec![GeminiTool {
        function_declarations: tools.iter().map(|t| GeminiFunctionDeclaration {
            name: t.function.name.clone(),
            description: t.function.description.clone(),
            parameters: t.function.parameters.clone(),
        }).collect(),
    }])
}
```

---

## Summary: All Files You Must Touch

| File | What To Change |
|------|---------------|
| `core/src/model_provider_info.rs` | Add `Chat`, `AnthropicMessages`, `GeminiGenerateContent` to `WireApi`. Add providers to `built_in_model_providers()`. Remove chat deserialization error. |
| `core/src/client.rs` | Add match arms for new `WireApi` variants in `stream()`. Add `stream_chat_completions()`, `stream_anthropic_messages()`, `stream_gemini_content()` methods. |
| `core/src/client_chat_compat.rs` | **NEW FILE** — Chat Completions SSE → `ResponseEvent` translator |
| `core/src/client_anthropic.rs` | **NEW FILE** — Anthropic Messages request builder + SSE → `ResponseEvent` translator + tool format converter |
| `core/src/client_gemini.rs` | **NEW FILE** — Gemini request builder + SSE → `ResponseEvent` translator + tool format converter |
| `core/src/codex.rs` | Remove `emit_chat_wire_api_deprecation()` function and its call site |
| `core/src/config/mod.rs` | Remove or relax the strict OSS provider validation in `set_default_oss_provider()` |
| `core/Cargo.toml` | Add `uuid` dependency (for generating tool call IDs for providers that don't provide them) |

---

## Config Examples For Users

After your changes, users can do:

```toml
# ~/.codex/config.toml

# Use Anthropic Claude
model = "claude-sonnet-4-20250514"
model_provider = "anthropic"

# Use Google Gemini
# model = "gemini-2.5-pro"
# model_provider = "gemini"

# Use Groq (OpenAI-compatible, blazing fast)
# model = "llama-3.3-70b-versatile"
# model_provider = "groq"

# Use DeepSeek
# model = "deepseek-chat"
# model_provider = "deepseek"

# Use OpenRouter (access to everything)
# model = "anthropic/claude-sonnet-4-20250514"
# model_provider = "openrouter"

# Use xAI Grok
# model = "grok-3"
# model_provider = "xai"
```

A model provider defines how Codex connects to a model (base URL, wire API, and optional HTTP headers). Users can define additional providers and point `model_provider` at them. Your fork just makes this first-class with built-in definitions and proper wire API support for each provider's native protocol.


































































































































































































Let me search for the exact current state of codex-rust to see what was removed and how.Now I have the full picture. You're absolutely right. Let me lay out exactly what OpenAI removed and what you need to restore/add in your fork.

---

# Exact State of Codex-RS Provider Support (March 24, 2026) — And What You Must Change

## What OpenAI Did: The Full Removal Timeline

In early 2025, OpenAI introduced the Responses API — an interface optimized for reasoning models, multi-turn conversations, and tool-rich workflows. This API replaces the older chat/completions API, which originated in the GPT-3.5 era and was not designed for today's agentic coding and reasoning use cases. While Codex historically supported both APIs, the chat/completions API increasingly hampered their ability to improve Codex and deliver new features. Maintaining compatibility with this legacy protocol added complexity, introduced regressions, and increased support overhead. As a result, the Codex team deprecated support for the chat/completions API. Full removal was slated for early February 2026.

Codex feature requests related to the chat/completions API are now closed. During the deprecation period, bug reports involving the chat/completions API were triaged based on user impact.

And now it's a **hard error**. The codebase contains `CHAT_WIRE_API_REMOVED_ERROR: &str = "wire_api = \"chat\" is no longer supported. How to fix: set wire_api = \"responses\" in your provider config."` along with `LEGACY_OLLAMA_CHAT_PROVIDER_ID: &str = "ollama-chat"` and `OLLAMA_CHAT_PROVIDER_REMOVED_ERROR` with a similar removal message.

The community reaction was clear. One user stated: "So there is no way to use Codex now - models in lmstudio are too small and no providers support responses except OpenAI." Another reported: "vllm 15.1 does not work, llamacpp latest compiled does not work with responses api - what works?"

## What Exists Today in the Source Code

### `WireApi` Enum — Only ONE Variant

The current `WireApi` enum at lines 35-55 of `model_provider_info.rs` has only one variant: `Responses` (the default). The `Deserialize` impl returns `Ok(Self::Responses)` for `"responses"`, returns `Err` with `CHAT_WIRE_API_REMOVED_ERROR` for `"chat"`, and returns an `unknown_variant` error for anything else.

That's it. One variant. Everything else errors out.

### `ModelProviderInfo` Struct

The `ModelProviderInfo` struct at lines 58-62 contains a friendly display name and a base URL described as "Base URL for the provider's OpenAI-compatible API." Note the doc comment says "OpenAI-compatible" — the struct itself assumes OpenAI compatibility.

The struct also contains: `http_headers`, `env_http_headers`, `request_max_retries`, `stream_max_retries`, `stream_idle_timeout_ms`, `requires_openai_auth`, and `supports_websockets`.

### The API Layer — `codex-api` Crate

The actual HTTP calls go through a separate `codex-api` crate. The SSE response handling in `codex-rs/codex-api/src/sse/responses.rs` imports `ResponseStream`, `ApiError`, rate limit parsing, `SseTelemetry`, `ByteStream`, `StreamResponse`, `TransportError`, `ResponseItem`, `TokenUsage`, and uses `eventsource_stream::Eventsource` for SSE parsing.

The request building happens in `codex-rs/codex-api/src/requests/responses.rs`. This file works with `ResponseItem` types including `Reasoning`, `Message`, `WebSearchCall`, `FunctionCall`, `LocalShellCall`, and `CustomToolCall` — all Responses API-specific item types.

### Tools Are Responses API Format Only

The tool spec in `codex-rs/core/src/tools/spec.rs` has a function `create_tools_json_for_responses_api()` that returns JSON values compatible with Function Calling in the Responses API. It serializes `ToolSpec` objects directly to JSON.

There is **no** `create_tools_json_for_chat_api()` or any other format.

### Config Still *Documents* Custom Providers, But They Can't Use Chat

OpenAI's advanced config docs say: "A model provider defines how Codex connects to a model (base URL, wire API, and optional HTTP headers). Define additional providers and point model_provider at them" — and shows examples for `proxy`, `ollama`, and `mistral`.

But here's the trap: all those example providers like `[model_providers.mistral]` with `base_url = "https://api.mistral.ai/v1"` have **no `wire_api` specified**, which means they default to `Responses` — and Mistral doesn't support the Responses API. So this config example is effectively broken for real use.

---

## The Three Layers You Must Change In Your Fork

### Layer 1: `codex-rs/core/src/model_provider_info.rs`

This is the gatekeeper. You need to:

**A) Restore `Chat` variant and add native API variants to `WireApi`:**

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WireApi {
    /// OpenAI Responses API at `/v1/responses`
    #[default]
    Responses,

    /// OpenAI Chat Completions API at `/v1/chat/completions`
    /// Used by: Groq, Mistral, Together, DeepSeek, xAI, OpenRouter,
    /// Fireworks, and any OpenAI-compatible proxy
    Chat,

    /// Anthropic Messages API at `/v1/messages`
    Anthropic,

    /// Google Gemini at `/v1beta/models/{model}:streamGenerateContent`
    Gemini,
}
```

**B) Fix the `Deserialize` impl — remove the error for `"chat"`:**

```rust
impl<'de> Deserialize<'de> for WireApi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "responses" => Ok(Self::Responses),
            "chat" => Ok(Self::Chat),
            "anthropic" => Ok(Self::Anthropic),
            "gemini" => Ok(Self::Gemini),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &["responses", "chat", "anthropic", "gemini"],
            )),
        }
    }
}
```

**C) Remove the `CHAT_WIRE_API_REMOVED_ERROR` constant and `OLLAMA_CHAT_PROVIDER_REMOVED_ERROR`:**

Delete these lines entirely:
```rust
// DELETE THESE:
const CHAT_WIRE_API_REMOVED_ERROR: &str = ...;
pub(crate) const LEGACY_OLLAMA_CHAT_PROVIDER_ID: &str = "ollama-chat";
pub(crate) const OLLAMA_CHAT_PROVIDER_REMOVED_ERROR: &str = ...;
```

**D) Remove the test that asserts chat shows an error:**

There's a test `test_deserialize_chat_wire_api_shows_helpful_error` at line 430 that deserializes a TOML provider with `wire_api = "chat"` and asserts it contains `CHAT_WIRE_API_REMOVED_ERROR`.

Replace it with a test that asserts `"chat"` deserializes successfully.

**E) Add `ModelProviderInfo` fields for non-OpenAI auth:**

The current struct has `requires_openai_auth: bool`. You need an additional field:

```rust
pub struct ModelProviderInfo {
    // ... existing fields ...

    /// How authentication works for this provider.
    /// "bearer" = standard Bearer token from env_key
    /// "x-api-key" = Anthropic-style header
    /// "query" = API key as query parameter (Gemini)
    #[serde(default = "default_auth_style")]
    pub auth_style: AuthStyle,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AuthStyle {
    #[default]
    Bearer,      // Authorization: Bearer <key>
    XApiKey,     // x-api-key: <key> (Anthropic)
    QueryParam,  // ?key=<key> (Gemini)
}
```

### Layer 2: `codex-rs/codex-api/` Crate — The HTTP Layer

This is where the actual HTTP requests happen. Currently it only has `src/sse/responses.rs` and `src/requests/responses.rs`. You need to add parallel implementations.

**A) Add `src/requests/chat.rs` — Chat Completions request builder:**

This must translate codex's internal `ResponseItem` conversation history into the Chat Completions `messages` array format:

```rust
// codex-rs/codex-api/src/requests/chat.rs

use codex_protocol::models::ResponseItem;

/// Convert codex's ResponseItem list into Chat Completions messages
pub fn items_to_chat_messages(items: &[ResponseItem]) -> Vec<serde_json::Value> {
    let mut messages = Vec::new();
    for item in items {
        match item {
            ResponseItem::Message { role, content, .. } => {
                messages.push(serde_json::json!({
                    "role": role_to_chat_role(role),
                    "content": content_to_chat_content(content),
                }));
            }
            ResponseItem::FunctionCall { name, arguments, call_id, .. } => {
                // Append as assistant message with tool_calls
                // Or merge with previous assistant message
                append_tool_call(&mut messages, call_id, name, arguments);
            }
            ResponseItem::FunctionCallOutput { call_id, output, .. } => {
                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": call_id,
                    "content": output,
                }));
            }
            ResponseItem::Reasoning { .. } => {
                // Skip reasoning items — Chat API doesn't have these
                // (DeepSeek returns them IN the response, not as input)
            }
            _ => {}
        }
    }
    messages
}

/// Build the full Chat Completions request body
pub fn build_chat_request(
    model: &str,
    items: &[ResponseItem],
    tools: &[serde_json::Value],
    temperature: Option<f32>,
    max_tokens: Option<u32>,
) -> serde_json::Value {
    let mut body = serde_json::json!({
        "model": model,
        "messages": items_to_chat_messages(items),
        "stream": true,
        "stream_options": { "include_usage": true },
    });
    if !tools.is_empty() {
        body["tools"] = serde_json::json!(tools);
    }
    if let Some(t) = temperature {
        body["temperature"] = serde_json::json!(t);
    }
    if let Some(m) = max_tokens {
        body["max_tokens"] = serde_json::json!(m);
    }
    body
}
```

**B) Add `src/sse/chat.rs` — Chat Completions SSE parser:**

This must parse `data: {"choices":[{"delta":{"content":"..."}}]}` SSE events and convert them into the same `ResponseStream` events that the Responses API parser produces. This is the key — codex-core doesn't care which wire format the response came from, it only sees the unified stream.

```rust
// codex-rs/codex-api/src/sse/chat.rs

use crate::common::ResponseStream;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::TokenUsage;

/// Parse a Chat Completions SSE chunk and emit ResponseStream events
pub fn parse_chat_sse_event(data: &str) -> Vec<StreamEvent> {
    if data == "[DONE]" {
        return vec![StreamEvent::Done];
    }

    let chunk: ChatChunk = match serde_json::from_str(data) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut events = Vec::new();

    for choice in &chunk.choices {
        // Text content delta
        if let Some(content) = &choice.delta.content {
            events.push(StreamEvent::TextDelta(content.clone()));
        }

        // Reasoning content (DeepSeek, QwQ)
        if let Some(reasoning) = &choice.delta.reasoning_content {
            events.push(StreamEvent::ReasoningDelta(reasoning.clone()));
        }

        // Tool calls
        if let Some(tool_calls) = &choice.delta.tool_calls {
            for tc in tool_calls {
                if let Some(ref func) = tc.function {
                    if let Some(ref name) = func.name {
                        events.push(StreamEvent::FunctionCallStart {
                            call_id: tc.id.clone().unwrap_or_default(),
                            name: name.clone(),
                        });
                    }
                    if let Some(ref args) = func.arguments {
                        events.push(StreamEvent::FunctionCallArgDelta(args.clone()));
                    }
                }
            }
        }

        // Finish reason
        if let Some(ref reason) = choice.finish_reason {
            match reason.as_str() {
                "stop" => events.push(StreamEvent::MessageDone),
                "tool_calls" => events.push(StreamEvent::ToolCallsDone),
                _ => {}
            }
        }
    }

    // Usage (only in final chunk with stream_options.include_usage)
    if let Some(usage) = &chunk.usage {
        events.push(StreamEvent::Usage(TokenUsage {
            input_tokens: usage.prompt_tokens,
            output_tokens: usage.completion_tokens,
        }));
    }

    events
}

#[derive(Deserialize)]
struct ChatChunk {
    choices: Vec<ChatChoice>,
    usage: Option<ChatUsage>,
}

#[derive(Deserialize)]
struct ChatChoice {
    delta: ChatDelta,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ChatDelta {
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<ChatToolCallDelta>>,
}

#[derive(Deserialize)]
struct ChatToolCallDelta {
    id: Option<String>,
    function: Option<ChatFunctionDelta>,
}

#[derive(Deserialize)]
struct ChatFunctionDelta {
    name: Option<String>,
    arguments: Option<String>,
}

#[derive(Deserialize)]
struct ChatUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
}
```

**C) Add `src/requests/anthropic.rs` and `src/sse/anthropic.rs`:**

Anthropic's Messages API is structurally different. Key differences:
- System message is a top-level field, not in `messages`
- Auth uses `x-api-key` header, not `Authorization: Bearer`
- Requires `anthropic-version` header
- Tool use has `input_schema` not `parameters`
- Tool results are content blocks inside `user` messages
- SSE event types: `message_start`, `content_block_start`, `content_block_delta`, `content_block_stop`, `message_delta`, `message_stop`
- Thinking uses `type: "thinking"` content blocks with explicit budget

**D) Add `src/requests/gemini.rs` and `src/sse/gemini.rs`:**

Gemini is different again:
- Auth is a query parameter `?key=<key>`, not a header
- URL pattern: `/v1beta/models/{model}:streamGenerateContent?alt=sse`
- Uses `contents` with `parts` instead of `messages`
- Role is `"user"` or `"model"` (not `"assistant"`)
- System instruction is a separate top-level field
- Tool format uses `function_declarations` array

### Layer 3: The Dispatch Point

You need to find where `codex-api` dispatches the request based on `WireApi` and add the new paths. Based on the crate structure, the `codex-api` crate imports `Provider as ApiProvider` and `RetryConfig as ApiRetryConfig` from its own `codex_api::provider` module, so the dispatch likely lives in the `Provider` implementation.

Look for the method that currently does something like:

```rust
// CURRENT (conceptual — find the actual method)
match wire_api {
    WireApi::Responses => {
        // build responses request, stream SSE, parse with sse/responses.rs
    }
}
```

Add:

```rust
// YOUR FORK
match wire_api {
    WireApi::Responses => {
        // existing path
    }
    WireApi::Chat => {
        let url = format!("{}/chat/completions", base_url);
        let body = requests::chat::build_chat_request(model, items, tools, temp, max_tokens);
        let response = send_streaming_request(url, body, auth_headers).await?;
        sse::chat::parse_stream(response)
    }
    WireApi::Anthropic => {
        let url = format!("{}/messages", base_url);
        let body = requests::anthropic::build_request(model, items, tools, max_tokens);
        let response = send_streaming_request(url, body, anthropic_headers).await?;
        sse::anthropic::parse_stream(response)
    }
    WireApi::Gemini => {
        let url = format!(
            "{}/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            base_url, model, api_key
        );
        let body = requests::gemini::build_request(items, tools, max_tokens);
        let response = send_streaming_request(url, body, HashMap::new()).await?;
        sse::gemini::parse_stream(response)
    }
}
```

### Layer 3.5: Tool Format Translation

Currently `create_tools_json_for_responses_api()` is the only tool serialization function. You need parallel functions:

```rust
// codex-rs/core/src/tools/spec.rs — ADD THESE

/// Chat Completions tool format
pub fn create_tools_json_for_chat_api(tools: &[ToolSpec]) -> Result<Vec<Value>> {
    tools.iter().map(|t| {
        Ok(serde_json::json!({
            "type": "function",
            "function": {
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            }
        }))
    }).collect()
}

/// Anthropic tool format
pub fn create_tools_json_for_anthropic(tools: &[ToolSpec]) -> Result<Vec<Value>> {
    tools.iter().map(|t| {
        Ok(serde_json::json!({
            "name": t.name,
            "description": t.description,
            "input_schema": t.parameters,
        }))
    }).collect()
}

/// Gemini tool format
pub fn create_tools_json_for_gemini(tools: &[ToolSpec]) -> Result<Vec<Value>> {
    Ok(vec![serde_json::json!({
        "function_declarations": tools.iter().map(|t| {
            serde_json::json!({
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            })
        }).collect::<Vec<_>>()
    })])
}
```

---

## Every File You Must Touch — Complete List

| # | File | Action | What |
|---|------|--------|------|
| 1 | `core/src/model_provider_info.rs` | **MODIFY** | Add `Chat`, `Anthropic`, `Gemini` to `WireApi`. Remove error constants. Fix deserializer. Add `AuthStyle` enum. Fix test. |
| 2 | `codex-api/src/requests/chat.rs` | **NEW** | Chat Completions request builder (ResponseItem → messages format) |
| 3 | `codex-api/src/requests/anthropic.rs` | **NEW** | Anthropic Messages request builder |
| 4 | `codex-api/src/requests/gemini.rs` | **NEW** | Gemini GenerateContent request builder |
| 5 | `codex-api/src/sse/chat.rs` | **NEW** | Chat Completions SSE parser → unified StreamEvent |
| 6 | `codex-api/src/sse/anthropic.rs` | **NEW** | Anthropic SSE parser → unified StreamEvent |
| 7 | `codex-api/src/sse/gemini.rs` | **NEW** | Gemini SSE parser → unified StreamEvent |
| 8 | `codex-api/src/requests/mod.rs` | **MODIFY** | Export new request modules |
| 9 | `codex-api/src/sse/mod.rs` | **MODIFY** | Export new SSE modules |
| 10 | `codex-api/src/lib.rs` or provider dispatch | **MODIFY** | Add match arms for new `WireApi` variants in the streaming dispatch |
| 11 | `core/src/tools/spec.rs` | **MODIFY** | Add `create_tools_json_for_chat_api()`, `_anthropic()`, `_gemini()` |
| 12 | `core/src/codex.rs` | **MODIFY** | Remove `emit_chat_wire_api_deprecation()` and its call site |
| 13 | Wherever `LEGACY_OLLAMA_CHAT_PROVIDER_ID` is checked | **MODIFY** | Remove the error path, allow `ollama-chat` again or alias it |

---

## User-Facing Config After Your Fork

Once all the above is done, users of your DX CLI can do:

```toml
# ~/.codex/config.toml

# Anthropic Claude
model = "claude-sonnet-4-20250514"
model_provider = "anthropic"

[model_providers.anthropic]
name = "Anthropic"
base_url = "https://api.anthropic.com/v1"
env_key = "ANTHROPIC_API_KEY"
wire_api = "anthropic"
http_headers = { "anthropic-version" = "2023-06-01" }
requires_openai_auth = false

# Google Gemini
[model_providers.gemini]
name = "Google Gemini"
base_url = "https://generativelanguage.googleapis.com"
env_key = "GEMINI_API_KEY"
wire_api = "gemini"
requires_openai_auth = false

# Groq (OpenAI Chat Completions compatible)
[model_providers.groq]
name = "Groq"
base_url = "https://api.groq.com/openai/v1"
env_key = "GROQ_API_KEY"
wire_api = "chat"
requires_openai_auth = false

# DeepSeek (OpenAI Chat Completions compatible)
[model_providers.deepseek]
name = "DeepSeek"
base_url = "https://api.deepseek.com/v1"
env_key = "DEEPSEEK_API_KEY"
wire_api = "chat"
requires_openai_auth = false

# OpenRouter (access to everything via chat completions)
[model_providers.openrouter]
name = "OpenRouter"
base_url = "https://openrouter.ai/api/v1"
env_key = "OPENROUTER_API_KEY"
wire_api = "chat"
requires_openai_auth = false

# Mistral
[model_providers.mistral]
name = "Mistral"
base_url = "https://api.mistral.ai/v1"
env_key = "MISTRAL_API_KEY"
wire_api = "chat"
requires_openai_auth = false

# xAI Grok
[model_providers.xai]
name = "xAI"
base_url = "https://api.x.ai/v1"
env_key = "XAI_API_KEY"
wire_api = "chat"
requires_openai_auth = false
```

---

## Implementation Priority Order

1. **`wire_api = "chat"` first** — This unblocks Groq, Mistral, Together, DeepSeek, xAI, OpenRouter, Fireworks, and any OpenAI-compatible proxy. This is the biggest bang for the least code because the request/response formats are very similar to the Responses API.

2. **`wire_api = "anthropic"` second** — Anthropic (Claude) is the #1 non-OpenAI provider people want. Different request format, different SSE format, different auth, but well-documented.

3. **`wire_api = "gemini"` third** — Google Gemini has yet another format but is increasingly popular.

Start with Chat Completions. That alone will make your fork useful to the majority of people who currently feel "there is no way to use Codex" with non-OpenAI providers.
