use crate::modal::AnimatedSuggestionList;
use crate::theme::ChatTheme;
use anyhow::Result;
// use reqwest::Client;
// use serde_json::Value;

/// Autocomplete suggestion source
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionSource {
	Local,
	// Remote,
}

/// A single autocomplete suggestion
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Suggestion {
	pub text: String,
	#[allow(dead_code)]
	pub source: SuggestionSource,
	pub description: String,
}

/// Local CLI commands for autocomplete
#[allow(dead_code)]
const CLI_COMMANDS: &[&str] = &[
	// Interactive TUI
	"dx",
	// Exec mode
	"dx exec",
	"dx exec resume",
	"dx exec fork",
	"dx exec review",
	// Resume
	"dx resume",
	"dx resume --last",
	"dx resume --last --all",
	// Fork
	"dx fork",
	"dx fork --last",
	// Review
	"dx review",
	"dx review --uncommitted",
	"dx review --base",
	"dx review --commit",
	// Apply
	"dx apply",
	// Cloud
	"dx cloud",
	"dx cloud exec",
	"dx cloud status",
	"dx cloud diff",
	"dx cloud list",
	"dx cloud apply",
	// App
	"dx app",
	"dx app-server",
	// Auth
	"dx login",
	"dx login --device",
	"dx login --with-api-key",
	"dx login --provider",
	"dx login status",
	"dx login list",
	"dx logout",
	"dx logout --all",
	// Features
	"dx features list",
	"dx features enable",
	"dx features disable",
	// MCP
	"dx mcp list",
	"dx mcp add",
	"dx mcp show",
	"dx mcp remove",
	"dx mcp login",
	"dx mcp logout",
	"dx mcp-server",
	// Skills
	"dx skills list",
	"dx skills info",
	"dx skills check",
	"dx skills install",
	"dx skills uninstall",
	"dx skills search",
	"dx skills publish",
	"dx skills browse",
	// Models
	"dx models list",
	"dx models status",
	"dx models set",
	"dx models set-image",
	"dx models scan",
	"dx models pull",
	"dx models benchmark",
	"dx models aliases list",
	"dx models aliases add",
	"dx models aliases remove",
	"dx models fallbacks list",
	"dx models fallbacks add",
	"dx models fallbacks remove",
	"dx models fallbacks clear",
	"dx models auth add",
	"dx models auth setup-token",
	"dx models auth paste-token",
	"dx models auth order get",
	"dx models auth order set",
	// Gateway
	"dx gateway",
	"dx gateway start",
	"dx gateway stop",
	"dx gateway restart",
	"dx gateway install",
	"dx gateway uninstall",
	"dx gateway status",
	"dx gateway run",
	"dx gateway logs",
	// Channels
	"dx channels list",
	"dx channels status",
	"dx channels logs",
	"dx channels add",
	"dx channels remove",
	"dx channels login",
	// Message
	"dx message send",
	"dx message edit",
	"dx message delete",
	"dx message search",
	"dx message react",
	"dx message pin",
	"dx message unpin",
	"dx message poll",
	// Agents
	"dx agents list",
	"dx agents add",
	"dx agents delete",
	"dx agents modify",
	"dx agents status",
	"dx agents bind",
	"dx agents unbind",
	"dx agents route list",
	"dx agent --deliver",
	// Sessions
	"dx sessions list",
	"dx sessions show",
	"dx sessions delete",
	"dx sessions export",
	"dx sessions share",
	// Cron
	"dx cron list",
	"dx cron status",
	"dx cron add",
	"dx cron edit",
	"dx cron rm",
	"dx cron enable",
	"dx cron disable",
	"dx cron run",
	"dx cron runs",
	// Browser
	"dx browser status",
	"dx browser start",
	"dx browser stop",
	"dx browser reset-profile",
	"dx browser tabs",
	"dx browser screenshot",
	"dx browser snapshot",
	"dx browser navigate",
	"dx browser profiles create-profile",
	"dx browser profiles delete-profile",
	// Nodes
	"dx nodes list",
	"dx nodes status",
	"dx nodes add",
	"dx nodes remove",
	"dx node run",
	"dx node status",
	"dx node install",
	"dx node uninstall",
	"dx node start",
	"dx node stop",
	"dx node restart",
	"dx devices list",
	// Memory
	"dx memory status",
	"dx memory index",
	"dx memory search",
	"dx memory clear",
	"dx memory export",
	"dx memory import",
	// Plugins
	"dx plugins list",
	"dx plugins info",
	"dx plugins install",
	"dx plugins uninstall",
	"dx plugins enable",
	"dx plugins disable",
	"dx plugins doctor",
	// Config
	"dx config",
	"dx config get",
	"dx config set",
	"dx config unset",
	"dx config file",
	"dx config validate",
	"dx config reset",
	"dx config export",
	"dx config import",
	// Security
	"dx security audit",
	"dx execpolicy check",
	"dx approvals get",
	"dx approvals set",
	"dx approvals allowlist add",
	"dx approvals allowlist remove",
	// Sandbox
	"dx sandbox list",
	"dx sandbox recreate",
	"dx sandbox explain",
	"dx sandbox run",
	// Setup
	"dx setup",
	"dx onboard",
	"dx configure",
	"dx doctor",
	// System
	"dx update",
	"dx reset",
	"dx uninstall",
	"dx completion bash",
	"dx completion zsh",
	"dx completion fish",
	"dx completion powershell",
	"dx status",
	"dx health",
	"dx logs",
	"dx dashboard",
	// Debug
	"dx debug app-server",
	"dx debug clear-memories",
	"dx debug sessions",
	"dx debug providers",
	"dx debug tokens",
	// Generate
	"dx generate code",
	"dx generate chart",
	"dx generate image",
	"dx generate video",
	"dx generate audio",
	"dx generate 3d",
	"dx generate research",
	// Tokens
	"dx tokens status",
	"dx tokens history",
	"dx tokens compare",
	"dx tokens budget set",
	"dx tokens budget status",
	// Voice
	"dx voice start",
	"dx voice stop",
	"dx voice devices",
	"dx voice transcribe",
	// Workspace
	"dx workspace init",
	"dx workspace wizard",
	"dx workspace status",
	"dx workspace validate",
	// ACP
	"dx acp",
	"dx acp status",
	"dx acp health",
	// Heartbeat
	"dx heartbeat last",
	"dx heartbeat enable",
	"dx heartbeat disable",
	"dx heartbeat status",
	// Presence
	"dx presence status",
	"dx presence set",
	// Slash commands (in-TUI)
	"/model",
	"/plan",
	"/review",
	"/fork",
	"/resume",
	"/clear",
	"/compact",
	"/copy",
	"/status",
	"/statusline",
	"/config",
	"/permissions",
	"/personality",
	"/experimental",
	"/agent",
	"/mcp",
	"/apps",
	"/theme",
	"/quit",
	"/exit",
	"/feedback",
	"/sandbox-add-read-dir",
	"/provider",
	"/tokens",
	"/undo",
	"/skills",
	"/generate",
	"/voice",
	"/offline",
	"/route",
	"/budget",
	"/share",
	"/export",
];

/// Autocomplete manager with animated suggestions
#[allow(dead_code)]
pub struct Autocomplete {
	suggestion_list: AnimatedSuggestionList,
	// http_client: Client,
}

#[allow(dead_code)]
impl Autocomplete {
	pub fn new(theme: ChatTheme) -> Self {
		Self {
			suggestion_list: AnimatedSuggestionList::new(theme),
			// http_client: Client::builder()
			//     .timeout(std::time::Duration::from_secs(3))
			//     .build()
			//     .unwrap_or_default(),
		}
	}

	/// Get animated suggestion list component
	pub fn suggestion_list_mut(&mut self) -> &mut AnimatedSuggestionList {
		&mut self.suggestion_list
	}

	/// Get animated suggestion list component (read-only)
	pub fn suggestion_list(&self) -> &AnimatedSuggestionList {
		&self.suggestion_list
	}

	/// Get autocomplete suggestions for the given query
	pub async fn get_suggestions(&self, query: &str) -> Result<Vec<Suggestion>> {
		let query = query.trim();

		if query.is_empty() {
			return Ok(Vec::new());
		}

		// Get local CLI command matches with fuzzy matching
		let all_suggestions = self.get_local_suggestions(query);

		// Remote suggestions commented out
		// if let Ok(remote_suggestions) = self.get_remote_suggestions(query).await {
		//     all_suggestions.extend(remote_suggestions);
		// }

		Ok(all_suggestions)
	}

	/// Get local CLI command suggestions with fuzzy matching
	fn get_local_suggestions(&self, query: &str) -> Vec<Suggestion> {
		let query_lower = query.to_lowercase();

		// Fuzzy match: check if all characters in query appear in order in the command
		let fuzzy_match = |cmd: &str| -> bool {
			let cmd_lower = cmd.to_lowercase();
			let mut query_chars = query_lower.chars();
			let mut current_char = query_chars.next();

			if current_char.is_none() {
				return true;
			}

			for cmd_char in cmd_lower.chars() {
				if let Some(qc) = current_char
					&& cmd_char == qc
				{
					current_char = query_chars.next();
					if current_char.is_none() {
						return true;
					}
				}
			}

			current_char.is_none()
		};

		CLI_COMMANDS
			.iter()
			.filter(|cmd| fuzzy_match(cmd))
			.map(|cmd| {
				let description = match *cmd {
					"dx" => "Interactive TUI mode",
					"dx exec" => "Execute a task",
					"dx resume" => "Resume previous session",
					"dx fork" => "Fork current session",
					"dx review" => "Review code changes",
					"dx apply" => "Apply changes",
					"dx cloud" => "Cloud operations",
					"dx login" => "Authenticate with provider",
					"dx logout" => "Sign out",
					"dx models list" => "List available models",
					"dx config" => "Manage configuration",
					"/model" => "Switch AI model",
					"/plan" => "Plan mode",
					"/clear" => "Clear chat history",
					"/quit" | "/exit" => "Exit application",
					_ => "CLI command",
				};

				Suggestion {
					text: cmd.to_string(),
					source: SuggestionSource::Local,
					description: description.to_string(),
				}
			})
			.collect() // Show all matching suggestions
	}

	// /// Fetch remote suggestions from Google's Firefox autocomplete API
	// async fn get_remote_suggestions(&self, query: &str) -> Result<Vec<Suggestion>> {
	//     let url = reqwest::Url::parse_with_params(
	//         "https://suggestqueries.google.com/complete/search",
	//         &[
	//             ("output", "firefox"),
	//             ("client", "firefox"),
	//             ("hl", "en-US"),
	//             ("q", query),
	//         ],
	//     )?;

	//     let body = self
	//         .http_client
	//         .get(url)
	//         .header(
	//             "User-Agent",
	//             "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
	//         )
	//         .send()
	//         .await?
	//         .text()
	//         .await?;

	//     // Response format: ["query", ["suggestion1", "suggestion2", ...]]
	//     let parsed: Vec<Value> = serde_json::from_str(&body)?;
	//     let suggestions = parsed
	//         .into_iter()
	//         .nth(1)
	//         .and_then(|v| v.as_array().cloned())
	//         .unwrap_or_default()
	//         .into_iter()
	//         .filter_map(|v| v.as_str().map(|s| s.to_string()))
	//         .take(10)
	//         .map(|text| Suggestion {
	//             text: text.clone(),
	//             source: SuggestionSource::Remote,
	//             description: "Search suggestion".to_string(),
	//         })
	//         .collect();

	//     Ok(suggestions)
	// }
}

impl Default for Autocomplete {
	fn default() -> Self {
		Self::new(ChatTheme::dark_fallback())
	}
}
