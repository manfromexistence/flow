// Providers submenu
pub fn get_submenu() -> Vec<(&'static str, &'static str)> {
	vec![
		("1. OpenAI Provider", ""),
		("2. Anthropic Provider", ""),
		("3. Local LLM Provider", ""),
		("4. Custom Provider", ""),
		("5. Provider Priority", ""),
		("6. API Key Management", ""),
		("7. Model Selection", ""),
		("8. Token Limits", ""),
		("9. Rate Limiting", ""),
	]
}
