//! Chat message processing and tool execution
//!
//! This module handles the complex logic of processing chat messages,
//! including tool execution loops, confirmations, and response generation.

use anyhow::Result;
use mimir_dm_llm::{
    traits::ActionDescription, EditFileTool, FileToolsConfig, ListFilesTool, LlmProvider,
    ReadFileTool, TodoListTool, WriteFileTool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;
use tauri::Emitter;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::services::chat_logger::ChatTokenUsage;
use crate::services::tools::ToolRegistry;
use crate::services::llm::LlmService;

use super::REQUIRED_MODEL;

/// Helper macro for bifurcated logging - full content to file, truncated to console
macro_rules! debug_content {
    ($msg:expr, $full_content:expr, $truncate_at:expr) => {
        // Full content to file (debug level)
        debug!(target: "file_only", "{}: {}", $msg, $full_content);
        // Truncated content to console (debug level, but with default target)
        if $full_content.len() > $truncate_at {
            debug!(
                "{}: {}... [truncated from {} chars]",
                $msg,
                &$full_content
                    .chars()
                    .take($truncate_at)
                    .collect::<String>(),
                $full_content.len()
            );
        } else {
            debug!("{}: {}", $msg, $full_content);
        }
    };
}

/// Strip thinking blocks from content for logging (simple string replacement)
fn strip_thinking_blocks(content: &str) -> String {
    let mut result = content.to_string();

    // Remove <thinking> blocks (simple approach)
    while let (Some(start), Some(end)) = (result.find("<thinking>"), result.find("</thinking>")) {
        if start < end {
            result = format!("{}{}", &result[..start], &result[end + 12..]);
        } else {
            break;
        }
    }

    // Remove <think> blocks
    while let (Some(start), Some(end)) = (result.find("<think>"), result.find("</think>")) {
        if start < end {
            result = format!("{}{}", &result[..start], &result[end + 8..]);
        } else {
            break;
        }
    }

    result.trim().to_string()
}

/// Limit the size of thinking blocks to prevent token overflow
/// If thinking blocks exceed the limit, truncate them with a warning
fn limit_thinking_block_size(content: &str, max_thinking_chars: usize) -> String {
    if !content.contains("<thinking>") {
        return content.to_string();
    }

    let mut result = String::new();
    let mut remaining = content;
    let mut total_thinking_size = 0;

    while let Some(start_pos) = remaining.find("<thinking>") {
        // Add content before thinking block
        result.push_str(&remaining[..start_pos]);

        // Find the end of this thinking block
        let thinking_start = start_pos + "<thinking>".len();
        if let Some(end_pos) = remaining[thinking_start..].find("</thinking>") {
            let thinking_content = &remaining[thinking_start..thinking_start + end_pos];
            let thinking_size = thinking_content.len();

            total_thinking_size += thinking_size;

            if total_thinking_size <= max_thinking_chars {
                // Include full thinking block
                result.push_str("<thinking>");
                result.push_str(thinking_content);
                result.push_str("</thinking>");
            } else {
                // Truncate thinking block
                let available_space = max_thinking_chars - (total_thinking_size - thinking_size);
                if available_space > 100 {
                    result.push_str("<thinking>");
                    result.push_str(&thinking_content[..available_space]);
                    result.push_str("\n\n[THINKING TRUNCATED - too long for token limit]");
                    result.push_str("</thinking>");
                } else {
                    result.push_str(
                        "<thinking>[THINKING TRUNCATED - too long for token limit]</thinking>",
                    );
                }

                warn!(
                    "Truncated thinking block: {} chars -> {} chars (limit: {})",
                    thinking_size, available_space, max_thinking_chars
                );
            }

            // Move past this thinking block
            remaining = &remaining[thinking_start + end_pos + "</thinking>".len()..];
        } else {
            // Malformed thinking block, just add it as-is
            result.push_str(&remaining[start_pos..]);
            break;
        }
    }

    // Add any remaining content
    result.push_str(remaining);
    result
}

/// Intermediate message from LLM (during multi-turn tool execution)
#[derive(Clone, Serialize, Deserialize)]
pub struct IntermediateMessage {
    pub role: String,
    pub content: String,
    pub tool_calls: Vec<String>,
    pub iteration: usize,
    pub session_id: Option<String>,
}

/// Tool result message
#[derive(Clone, Serialize, Deserialize)]
pub struct ToolResultMessage {
    pub tool_name: String,
    pub result: String,
    pub success: bool,
    pub iteration: usize,
    pub session_id: Option<String>,
}

/// Chat response structure
pub struct ChatResponse {
    pub content: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Chat processor handles message processing and tool execution
pub struct ChatProcessor<'a> {
    llm: &'a LlmService,
}

impl<'a> ChatProcessor<'a> {
    pub fn new(llm: &'a LlmService) -> Self {
        Self { llm }
    }

    /// Process a chat message with optional tool support
    pub async fn process_chat(
        &self,
        mut provider_messages: Vec<mimir_dm_llm::Message>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
        enable_tools: bool,
        session_id: &str,
        ollama_url: Option<&str>,
        campaign_directory_path: Option<&str>,
        cancellation_token: CancellationToken,
    ) -> Result<ChatResponse, String> {
        // Create chat logger
        let chat_logger = match self.llm.get_chat_logger(session_id).await {
            Ok(logger) => {
                logger.log_session_info(
                    "chat_started",
                    json!({
                        "enable_tools": enable_tools,
                        "temperature": temperature,
                        "max_tokens": max_tokens,
                        "model": self.llm.model_name(),
                        "message_count": provider_messages.len()
                    }),
                );
                Some(logger)
            }
            Err(e) => {
                error!("Failed to create chat logger: {}", e);
                None
            }
        };

        // Log user message if available
        if let (Some(ref logger), Some(user_msg)) = (
            &chat_logger,
            provider_messages.iter().find(|m| m.role == "user"),
        ) {
            logger.log_user_message(&user_msg.content, None);
        }

        // Get tools if enabled
        let tools = if enable_tools {
            Some(self.get_tool_definitions(campaign_directory_path, session_id))
        } else {
            debug!("Tools disabled for this request");
            None
        };

        // Inject system rules if tools are enabled
        if tools.is_some() {
            self.inject_system_rules(
                &mut provider_messages,
                campaign_directory_path,
                session_id,
                &chat_logger,
            );
        }

        // Tool execution loop (max 20 iterations to prevent infinite loops)
        const MAX_TOOL_ITERATIONS: usize = 20;
        let mut tool_call_count = 0;
        let mut final_response = None;

        while tool_call_count < MAX_TOOL_ITERATIONS {
            // Check for cancellation
            if cancellation_token.is_cancelled() {
                info!("Cancellation detected, stopping LLM execution loop");
                return Err("Chat message was cancelled".to_string());
            }

            // Make LLM call
            let response = self
                .make_llm_call(
                    &provider_messages,
                    &tools,
                    temperature,
                    max_tokens,
                    ollama_url,
                    tool_call_count,
                    &chat_logger,
                    &cancellation_token,
                )
                .await?;

            // Check if there are tool calls
            if let Some(tool_calls) = &response.tool_calls {
                if !tool_calls.is_empty() {
                    tool_call_count += 1;
                    info!(
                        "Processing {} tool calls (iteration {})",
                        tool_calls.len(),
                        tool_call_count
                    );

                    // Emit intermediate message
                    self.emit_intermediate_message(&response, tool_call_count, session_id);

                    // Add assistant message with tool calls
                    provider_messages.push(mimir_dm_llm::Message {
                        role: "assistant".to_string(),
                        content: response.content.clone(),
                    });

                    // Execute tool calls
                    self.execute_tool_calls(
                        tool_calls,
                        &mut provider_messages,
                        campaign_directory_path,
                        session_id,
                        tool_call_count,
                        &chat_logger,
                    )
                    .await;

                    continue;
                }
            }

            // No tool calls, we have the final response
            info!("=== No tool calls found, ending loop ===");
            final_response = Some(response);
            break;
        }

        if tool_call_count >= MAX_TOOL_ITERATIONS {
            warn!("Reached maximum tool iterations ({})", MAX_TOOL_ITERATIONS);

            // Log warning to chat logger
            if let Some(ref logger) = chat_logger {
                logger.log_error(
                    "max_iterations_reached",
                    &format!("Reached maximum tool iterations: {}", MAX_TOOL_ITERATIONS),
                    "IterationLimitError",
                );
            }
        }

        let response = final_response
            .ok_or_else(|| "Maximum tool iterations reached".to_string())?;

        // Extract token usage
        let usage = response.usage.unwrap_or(mimir_dm_llm::Usage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        });

        // Apply thinking block size limit to prevent future token issues
        let limited_content = limit_thinking_block_size(&response.content, 12000); // ~3k tokens worth of thinking

        // Log completion to chat logger
        if let Some(ref logger) = chat_logger {
            logger.log_session_info(
                "chat_completed",
                json!({
                    "tool_iterations": tool_call_count,
                    "final_content_length": limited_content.len(),
                    "token_usage": {
                        "prompt": usage.prompt_tokens,
                        "completion": usage.completion_tokens,
                        "total": usage.total_tokens
                    }
                }),
            );
        }

        Ok(ChatResponse {
            content: limited_content,
            prompt_tokens: usage.prompt_tokens,
            completion_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
        })
    }

    /// Get tool definitions based on campaign directory
    fn get_tool_definitions(
        &self,
        campaign_directory_path: Option<&str>,
        _session_id: &str,
    ) -> Vec<mimir_dm_llm::Tool> {
        if let Some(campaign_dir) = campaign_directory_path {
            // Create campaign-specific file tools
            info!(
                "Configuring file tools for campaign directory: {}",
                campaign_dir
            );
            let campaign_file_config =
                Arc::new(FileToolsConfig::with_root(std::path::PathBuf::from(
                    campaign_dir,
                )));

            // Create a temporary tool registry with campaign-specific tools
            let mut campaign_tool_registry = ToolRegistry::new();
            campaign_tool_registry.register(Arc::new(ReadFileTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(WriteFileTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(ListFilesTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(EditFileTool::new(
                campaign_file_config.clone(),
            )));

            // Register TodoListTool with the same state manager as the main service
            let todo_tool = TodoListTool::new(self.llm.todo_state_manager.clone());
            campaign_tool_registry.register(Arc::new(todo_tool));

            campaign_tool_registry.get_tool_definitions()
        } else {
            // Use default tools from the main registry
            self.llm.tool_registry.get_tool_definitions()
        }
    }

    /// Inject system rules for tool guidance
    fn inject_system_rules(
        &self,
        provider_messages: &mut Vec<mimir_dm_llm::Message>,
        campaign_directory_path: Option<&str>,
        session_id: &str,
        chat_logger: &Option<Arc<crate::services::chat_logger::ChatLogger>>,
    ) {
        let system_rules = if let Some(campaign_dir) = campaign_directory_path {
            // Generate system rules for campaign-specific tools
            let campaign_file_config =
                Arc::new(FileToolsConfig::with_root(std::path::PathBuf::from(
                    campaign_dir,
                )));
            let mut campaign_tool_registry = ToolRegistry::new();
            campaign_tool_registry.register(Arc::new(ReadFileTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(WriteFileTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(ListFilesTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(EditFileTool::new(
                campaign_file_config.clone(),
            )));

            let todo_tool = TodoListTool::new(self.llm.todo_state_manager.clone());
            campaign_tool_registry.register(Arc::new(todo_tool));

            campaign_tool_registry
                .generate_system_rules_with_directory(Some(session_id), Some(campaign_dir))
        } else {
            self.llm
                .tool_registry
                .generate_system_rules(Some(session_id))
        };

        if !system_rules.is_empty() {
            let system_content = system_rules.join("\n\n");

            info!(
                "Generated {} system rules for LLM context",
                system_rules.len()
            );
            debug_content!("System rules content", system_content, 200);

            // If the first message is a system message, append contextual rules to the end
            if let Some(first_msg) = provider_messages.first_mut() {
                if first_msg.role == "system" {
                    // Put contextual info at the end after static prompt
                    let original_content = first_msg.content.clone();
                    first_msg.content = format!("{}\n\n{}", original_content, system_content);
                } else {
                    // Insert system message at the beginning
                    provider_messages.insert(
                        0,
                        mimir_dm_llm::Message {
                            role: "system".to_string(),
                            content: system_content.clone(),
                        },
                    );
                }
            } else {
                // No messages yet, add system message
                provider_messages.push(mimir_dm_llm::Message {
                    role: "system".to_string(),
                    content: system_content.clone(),
                });
            }

            info!(
                "Injected {} system rules for tool guidance",
                system_rules.len()
            );

            // Log the complete system prompt to chat logger
            if let Some(ref logger) = chat_logger {
                logger.log_system_prompt(&system_content, "tool_guidance_rules");
            }
        }
    }

    /// Make an LLM call
    #[allow(clippy::too_many_arguments)]
    async fn make_llm_call(
        &self,
        provider_messages: &[mimir_dm_llm::Message],
        tools: &Option<Vec<mimir_dm_llm::Tool>>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        ollama_url: Option<&str>,
        iteration: usize,
        chat_logger: &Option<Arc<crate::services::chat_logger::ChatLogger>>,
        cancellation_token: &CancellationToken,
    ) -> Result<mimir_dm_llm::ChatResponse, String> {
        // Log message flow before LLM call
        info!("=== LLM Call {} ===", iteration + 1);
        info!(
            "Sending {} messages to LLM ({})",
            provider_messages.len(),
            provider_messages
                .iter()
                .map(|m| m.role.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );

        // Log LLM call to chat logger
        if let Some(ref logger) = chat_logger {
            logger.log_llm_call(
                iteration + 1,
                provider_messages.len(),
                tools.is_some(),
                self.llm.model_name(),
            );

            // Log complete conversation context being sent to LLM
            logger.log_full_conversation_context(
                iteration + 1,
                provider_messages,
                temperature,
                max_tokens,
                tools.is_some(),
                tools.as_ref().map_or(0, |t| t.len()),
            );
        }

        self.log_request_details(provider_messages, tools, temperature, max_tokens);

        // Get the appropriate provider (with custom endpoint if specified)
        let provider = self
            .llm
            .get_provider_with_endpoint(ollama_url)
            .map_err(|e| {
                error!("Failed to get provider with endpoint: {}", e);
                error!(
                    "Endpoint: {}",
                    ollama_url.unwrap_or(super::OLLAMA_BASE_URL)
                );
                format!("Failed to get provider with endpoint: {}", e)
            })?;

        // Log the request details before making the call
        info!(
            "Making LLM request: endpoint={}, model={}, messages={}, tools={}",
            ollama_url.unwrap_or(super::OLLAMA_BASE_URL),
            REQUIRED_MODEL,
            provider_messages.len(),
            tools.as_ref().map_or(0, |t| t.len())
        );

        // Call the provider's chat method
        let response = provider
            .chat(
                provider_messages.to_vec(),
                tools.clone(),
                None,                      // n (number of completions)
                temperature.or(Some(0.3)), // temperature (default to 0.3 for more deterministic tool calling)
                max_tokens.or(Some(16384)), // max_tokens (default to 16384 for thinking models)
                None,                      // stop sequences
                None,                      // extra config
                Some(cancellation_token.clone()), // cancellation token
            )
            .await
            .map_err(|e| {
                error!("Chat request failed: {}", e);
                error!(
                    "Request details: endpoint={}, model={}, messages={}, tools={}",
                    ollama_url.unwrap_or(super::OLLAMA_BASE_URL),
                    REQUIRED_MODEL,
                    provider_messages.len(),
                    tools.as_ref().map_or(0, |t| t.len())
                );

                // Log error to chat logger
                if let Some(ref logger) = chat_logger {
                    logger.log_error(
                        "llm_request_failed",
                        &format!("Chat request failed: {}", e),
                        "RequestError",
                    );
                }

                format!("Chat request failed: {}", e)
            })?;

        // Log response structure
        info!(
            "LLM Response: content_length={}, tool_calls={}",
            response.content.len(),
            response.tool_calls.as_ref().map_or(0, |tc| tc.len())
        );

        // Log LLM response to chat logger
        if let Some(ref logger) = chat_logger {
            let token_usage = response.usage.as_ref().map(|u| ChatTokenUsage {
                prompt: u.prompt_tokens,
                completion: u.completion_tokens,
                total: u.total_tokens,
            });
            logger.log_llm_response(
                &response.content,
                token_usage,
                response.tool_calls.as_ref().map_or(0, |tc| tc.len()),
            );
        }

        self.log_response_details(&response);

        Ok(response)
    }

    /// Log request details
    fn log_request_details(
        &self,
        provider_messages: &[mimir_dm_llm::Message],
        tools: &Option<Vec<mimir_dm_llm::Tool>>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) {
        debug!("Request parameters:");
        debug!("  Temperature: {:?}", temperature.or(Some(0.3)));
        debug!("  Max tokens: {:?}", max_tokens.or(Some(16384)));
        debug!("  Tools provided: {}", tools.is_some());
        if let Some(ref tools) = tools {
            debug!(
                "  Tool names: [{}]",
                tools
                    .iter()
                    .map(|t| t.function.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        debug!("Message details:");
        for (i, msg) in provider_messages.iter().enumerate() {
            debug!(
                "  Message {}: role='{}' content_length={}",
                i + 1,
                msg.role,
                msg.content.len()
            );

            // Strip thinking blocks and show the actual content being sent
            let content_without_thinking = strip_thinking_blocks(&msg.content);
            if content_without_thinking.len() < 300 {
                debug!("    Content: {}", content_without_thinking);
            } else {
                // Safe UTF-8 truncation to avoid panics on character boundaries
                let truncated = content_without_thinking
                    .char_indices()
                    .take_while(|(i, _)| *i < 300)
                    .last()
                    .map(|(i, c)| i + c.len_utf8())
                    .unwrap_or(0);
                debug!(
                    "    Content preview: {}...",
                    &content_without_thinking[..truncated]
                );
            }

            // Show if thinking blocks were present
            if content_without_thinking.len() != msg.content.len() {
                debug!(
                    "    [Thinking blocks removed: {} chars -> {} chars]",
                    msg.content.len(),
                    content_without_thinking.len()
                );
            }
        }
    }

    /// Log response details
    fn log_response_details(&self, response: &mimir_dm_llm::ChatResponse) {
        debug!("Response details:");
        let response_without_thinking = strip_thinking_blocks(&response.content);
        debug_content!("Content preview", response_without_thinking, 150);
        if response_without_thinking.len() != response.content.len() {
            debug!(
                "  [Response thinking blocks removed: {} chars -> {} chars]",
                response.content.len(),
                response_without_thinking.len()
            );
        }
        debug!("  Tool calls present: {}", response.tool_calls.is_some());

        if let Some(tool_calls) = &response.tool_calls {
            debug!("  Tool calls count: {}", tool_calls.len());
            for (i, tool_call) in tool_calls.iter().enumerate() {
                debug!(
                    "    Tool call {}: function='{}' args_length={}",
                    i + 1,
                    tool_call.function.name,
                    serde_json::to_string(&tool_call.function.arguments).map_or(0, |s| s.len())
                );
                debug!(
                    "      Arguments: {}",
                    serde_json::to_string_pretty(&tool_call.function.arguments)
                        .unwrap_or_else(|_| "Invalid JSON".to_string())
                );
            }

            if !tool_calls.is_empty() {
                let tool_names: Vec<&str> = tool_calls
                    .iter()
                    .map(|tc| tc.function.name.as_str())
                    .collect();
                info!("Tool calls requested: [{}]", tool_names.join(", "));
            }
        } else {
            debug!("  No tool calls in response - final answer mode");
        }
    }

    /// Emit intermediate message to frontend
    fn emit_intermediate_message(
        &self,
        response: &mimir_dm_llm::ChatResponse,
        iteration: usize,
        session_id: &str,
    ) {
        if let Some(ref app) = self.llm.app_handle {
            if let Some(tool_calls) = &response.tool_calls {
                let tool_names: Vec<String> =
                    tool_calls.iter().map(|tc| tc.function.name.clone()).collect();

                let intermediate_msg = IntermediateMessage {
                    role: "assistant".to_string(),
                    content: response.content.clone(),
                    tool_calls: tool_names,
                    iteration,
                    session_id: Some(session_id.to_string()),
                };

                if let Err(e) = app.emit("llm-intermediate-message", &intermediate_msg) {
                    debug!("Failed to emit intermediate message: {}", e);
                }
            }
        }
    }

    /// Execute tool calls
    #[allow(clippy::too_many_arguments)]
    async fn execute_tool_calls(
        &self,
        tool_calls: &[mimir_dm_llm::ToolCall],
        provider_messages: &mut Vec<mimir_dm_llm::Message>,
        campaign_directory_path: Option<&str>,
        session_id: &str,
        iteration: usize,
        chat_logger: &Option<Arc<crate::services::chat_logger::ChatLogger>>,
    ) {
        info!("=== Processing {} tool calls ===", tool_calls.len());
        for (idx, tool_call) in tool_calls.iter().enumerate() {
            let tool_name = &tool_call.function.name;
            let mut tool_args = tool_call.function.arguments.clone();

            info!(
                "Processing tool call {}/{}: {}",
                idx + 1,
                tool_calls.len(),
                tool_name
            );
            let args_json = serde_json::to_string_pretty(&tool_args)
                .unwrap_or_else(|_| "Invalid JSON".to_string());
            debug_content!("Tool arguments", args_json, 300);

            // Inject session_id for todo_write tool if session_id is provided
            if tool_name == "todo_write" {
                tool_args.as_object_mut().unwrap().insert(
                    "session_id".to_string(),
                    serde_json::Value::String(session_id.to_string()),
                );
                debug!("Injected session_id '{}' into todo_write tool", session_id);
            }

            // Extract key parameters for logging
            let doc_type = tool_args
                .get("document_type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let campaign_id = tool_args
                .get("campaign_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(-1);

            info!(
                "Tool {}: {} (campaign: {}, doc: {})",
                idx + 1,
                tool_name,
                campaign_id,
                doc_type
            );

            // Check if tool requires confirmation
            let (requires_confirmation, action_desc) =
                self.check_tool_confirmation(tool_name, &tool_args, campaign_directory_path);

            if requires_confirmation {
                if let Some(action_desc) = action_desc {
                    info!("Tool {} requires confirmation, requesting from user", tool_name);

                    match self.llm.request_confirmation(action_desc, tool_name.clone()).await {
                        Ok(confirmed) => {
                            if !confirmed {
                                info!("User rejected tool {} execution", tool_name);
                                provider_messages.push(mimir_dm_llm::Message {
                                    role: "tool".to_string(),
                                    content: format!("Action cancelled by user: {}", tool_name),
                                });
                                continue;
                            }
                            info!("User confirmed tool {} execution", tool_name);
                        }
                        Err(e) => {
                            error!("Confirmation request failed: {}", e);
                            provider_messages.push(mimir_dm_llm::Message {
                                role: "tool".to_string(),
                                content: format!("Confirmation failed: {}", e),
                            });
                            continue;
                        }
                    }
                } else {
                    error!(
                        "Tool {} requires confirmation but provided no action description",
                        tool_name
                    );
                    provider_messages.push(mimir_dm_llm::Message {
                        role: "tool".to_string(),
                        content: "Tool configuration error: missing action description".to_string(),
                    });
                    continue;
                }
            }

            // Execute the tool
            let tool_result = self
                .execute_single_tool(
                    tool_name,
                    tool_args.clone(),
                    campaign_directory_path,
                    chat_logger,
                )
                .await;

            // Emit tool result
            self.emit_tool_result(
                tool_name,
                &tool_result,
                iteration,
                session_id,
            );

            // Add tool response to messages
            let is_error = tool_result.contains("Tool execution failed")
                || tool_result.contains("error");
            info!(
                "Adding tool result to conversation: {} (error: {})",
                tool_name, is_error
            );
            if is_error {
                warn!("Tool error being added to LLM context: {}", tool_result);
            }
            debug_content!("Tool result content", tool_result, 200);

            provider_messages.push(mimir_dm_llm::Message {
                role: "tool".to_string(),
                content: tool_result.clone(),
            });
        }

        info!("=== Continuing loop for next LLM call ===");
        info!(
            "Current conversation has {} messages",
            provider_messages.len()
        );
        info!(
            "Last message role: {}, content length: {} chars",
            provider_messages
                .last()
                .map(|m| m.role.as_str())
                .unwrap_or("none"),
            provider_messages.last().map(|m| m.content.len()).unwrap_or(0)
        );
    }

    /// Check if tool requires confirmation
    fn check_tool_confirmation(
        &self,
        tool_name: &str,
        tool_args: &serde_json::Value,
        campaign_directory_path: Option<&str>,
    ) -> (bool, Option<ActionDescription>) {
        if let Some(campaign_dir) = campaign_directory_path {
            let campaign_file_config =
                Arc::new(FileToolsConfig::with_root(std::path::PathBuf::from(
                    campaign_dir,
                )));
            let mut campaign_tool_registry = ToolRegistry::new();
            campaign_tool_registry.register(Arc::new(ReadFileTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(WriteFileTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(ListFilesTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(EditFileTool::new(
                campaign_file_config.clone(),
            )));

            let todo_tool = TodoListTool::new(self.llm.todo_state_manager.clone());
            campaign_tool_registry.register(Arc::new(todo_tool));

            (
                campaign_tool_registry.requires_confirmation(tool_name),
                campaign_tool_registry.get_action_description(tool_name, tool_args),
            )
        } else {
            (
                self.llm.tool_registry.requires_confirmation(tool_name),
                self.llm
                    .tool_registry
                    .get_action_description(tool_name, tool_args),
            )
        }
    }

    /// Execute a single tool
    async fn execute_single_tool(
        &self,
        tool_name: &str,
        tool_args: serde_json::Value,
        campaign_directory_path: Option<&str>,
        chat_logger: &Option<Arc<crate::services::chat_logger::ChatLogger>>,
    ) -> String {
        info!(
            "Executing tool: {} with {} bytes of arguments",
            tool_name,
            serde_json::to_string(&tool_args).unwrap_or_default().len()
        );

        let execution_start = Instant::now();

        let tool_result = if let Some(campaign_dir) = campaign_directory_path {
            // Use campaign-specific tools for execution
            let campaign_file_config =
                Arc::new(FileToolsConfig::with_root(std::path::PathBuf::from(
                    campaign_dir,
                )));
            let mut campaign_tool_registry = ToolRegistry::new();
            campaign_tool_registry.register(Arc::new(ReadFileTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(WriteFileTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(ListFilesTool::new(
                campaign_file_config.clone(),
            )));
            campaign_tool_registry.register(Arc::new(EditFileTool::new(
                campaign_file_config.clone(),
            )));

            let todo_tool = TodoListTool::new(self.llm.todo_state_manager.clone());
            campaign_tool_registry.register(Arc::new(todo_tool));

            match campaign_tool_registry
                .execute_tool(tool_name, tool_args.clone())
                .await
            {
                Ok(result) => {
                    info!(
                        "Tool {} succeeded - result length: {} chars",
                        tool_name,
                        result.len()
                    );
                    result
                }
                Err(e) => {
                    error!("Tool {} execution failed: {}", tool_name, e);
                    format!("Tool execution failed: {}", e)
                }
            }
        } else {
            // Use default tools
            match self
                .llm
                .tool_registry
                .execute_tool(tool_name, tool_args.clone())
                .await
            {
                Ok(result) => {
                    info!(
                        "Tool {} succeeded - result length: {} chars",
                        tool_name,
                        result.len()
                    );
                    result
                }
                Err(e) => {
                    error!("Tool {} execution failed: {}", tool_name, e);
                    format!("Tool execution failed: {}", e)
                }
            }
        };

        let execution_time_ms = execution_start.elapsed().as_millis() as u64;

        // Log tool execution to chat logger
        if let Some(ref logger) = chat_logger {
            let success = !tool_result.contains("Tool execution failed");
            logger.log_tool_call(
                tool_name,
                &tool_args,
                success,
                &tool_result,
                Some(execution_time_ms),
            );
        }

        info!("Tool {} result: {} chars", tool_name, tool_result.len());

        tool_result
    }

    /// Emit tool result to frontend
    fn emit_tool_result(
        &self,
        tool_name: &str,
        tool_result: &str,
        iteration: usize,
        session_id: &str,
    ) {
        if let Some(ref app) = self.llm.app_handle {
            let success = !tool_result.contains("Tool execution failed");
            let tool_result_msg = ToolResultMessage {
                tool_name: tool_name.to_string(),
                result: tool_result.to_string(),
                success,
                iteration,
                session_id: Some(session_id.to_string()),
            };

            if let Err(e) = app.emit("tool-result-message", &tool_result_msg) {
                debug!("Failed to emit tool result message: {}", e);
            }

            // If this was a todo_write tool and successful, emit todos update
            if tool_name == "todo_write" && success {
                let current_todos = self.llm.get_session_todos(session_id);
                if let Err(e) = app.emit(
                    "todos-updated",
                    &json!({
                        "session_id": session_id,
                        "todos": current_todos
                    }),
                ) {
                    debug!("Failed to emit todos update: {}", e);
                }
            }
        }
    }
}
