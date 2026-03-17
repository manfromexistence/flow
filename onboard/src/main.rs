//! DX Onboarding - Interactive Setup Experience
#![allow(dead_code)]

mod prompts;

use anyhow::Result;
use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Local;
use rand::thread_rng;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use prompts::PromptInteraction;

#[derive(Debug, Clone, Copy)]
enum RuntimeEnvironment {
    RealOs,
    Vps,
    Container,
    Restricted,
}

impl RuntimeEnvironment {
    fn as_str(self) -> &'static str {
        match self {
            RuntimeEnvironment::RealOs => "real_os",
            RuntimeEnvironment::Vps => "vps",
            RuntimeEnvironment::Container => "container",
            RuntimeEnvironment::Restricted => "restricted",
        }
    }

    fn label(self) -> &'static str {
        match self {
            RuntimeEnvironment::RealOs => "Real OS workstation",
            RuntimeEnvironment::Vps => "VPS / Cloud VM",
            RuntimeEnvironment::Container => "Docker / Container",
            RuntimeEnvironment::Restricted => "Restricted / CI runner",
        }
    }

    fn hint(self) -> &'static str {
        match self {
            RuntimeEnvironment::RealOs => "Best for desktop app + extension onboarding",
            RuntimeEnvironment::Vps => "Best for remote gateway + channel bridge",
            RuntimeEnvironment::Container => "Best for ephemeral test/deploy environments",
            RuntimeEnvironment::Restricted => "Best for non-interactive automation",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct OnboardingResult {
    name: String,
    email: String,
    runtime_environment: String,
    selected_components: Vec<String>,
    selected_providers: Vec<String>,
    preferences: OnboardingPreferences,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
struct OnboardingPreferences {
    theme: String,
    editor: String,
    shell: String,
    notifications: bool,
    auto_updates: bool,
    telemetry: bool,
}

fn detect_runtime_environment() -> RuntimeEnvironment {
    let ci = env::var("CI")
        .map(|value| {
            let normalized = value.to_ascii_lowercase();
            normalized == "1" || normalized == "true"
        })
        .unwrap_or(false);
    if ci {
        return RuntimeEnvironment::Restricted;
    }

    let container_detected = Path::new("/.dockerenv").exists()
        || env::var("KUBERNETES_SERVICE_HOST").is_ok()
        || env::var("DOCKER_CONTAINER").is_ok()
        || fs::read_to_string("/proc/1/cgroup")
            .map(|content| {
                let lowered = content.to_ascii_lowercase();
                lowered.contains("docker")
                    || lowered.contains("containerd")
                    || lowered.contains("kubepods")
                    || lowered.contains("podman")
            })
            .unwrap_or(false);
    if container_detected {
        return RuntimeEnvironment::Container;
    }

    let cloud_hint = env::var("VERCEL")
        .or_else(|_| env::var("RAILWAY_ENVIRONMENT"))
        .or_else(|_| env::var("FLY_APP_NAME"))
        .or_else(|_| env::var("HEROKU_APP_NAME"))
        .or_else(|_| env::var("DIGITALOCEAN_APP_ID"))
        .or_else(|_| env::var("AWS_EXECUTION_ENV"))
        .or_else(|_| env::var("GCP_PROJECT"))
        .or_else(|_| env::var("AZURE_HTTP_USER_AGENT"))
        .is_ok();
    let virtualization_hint =
        Path::new("/proc/vz").exists() || Path::new("/proc/user_beancounters").exists();

    if cloud_hint || virtualization_hint {
        return RuntimeEnvironment::Vps;
    }

    RuntimeEnvironment::RealOs
}

fn find_workspace_root() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    for ancestor in cwd.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if cargo_toml.exists()
            && let Ok(content) = fs::read_to_string(&cargo_toml)
            && content.contains("[workspace]")
        {
            return ancestor.to_path_buf();
        }
    }
    cwd
}

fn build_component_targets(runtime: RuntimeEnvironment) -> Vec<String> {
    match runtime {
        RuntimeEnvironment::RealOs => vec![
            "desktop_app".to_string(),
            "tui".to_string(),
            "ide_extension".to_string(),
            "browser_extension".to_string(),
            "local_website".to_string(),
        ],
        RuntimeEnvironment::Vps
        | RuntimeEnvironment::Container
        | RuntimeEnvironment::Restricted => {
            vec!["tui".to_string(), "local_website".to_string()]
        }
    }
}

fn hash_password(password: &str) -> Result<String> {
    let salt = argon2::password_hash::SaltString::generate(&mut thread_rng());
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| anyhow::anyhow!("password hashing failed: {}", err))?;
    Ok(hash.to_string())
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed = match PasswordHash::new(password_hash) {
        Ok(value) => value,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

fn run_onboarding() -> Result<OnboardingResult> {
    // Welcome
    prompts::intro("🚀 DX Onboarding")?;
    
    prompts::section_with_width("Welcome to DX", 80, |lines| {
        lines.push("Environment-aware onboarding + auth + provider/channel setup".to_string());
        lines.push(format!("Detected runtime: {}", detect_runtime_environment().label()));
        lines.push(format!("Runtime hint: {}", detect_runtime_environment().hint()));
        lines.push("".to_string());
        lines.push("This onboarding will help you set up your DX environment.".to_string());
    })?;

    // Environment detection
    let runtime_env = detect_runtime_environment();
    prompts::log::info(format!("Runtime Environment: {}", runtime_env.label()))?;
    prompts::log::info(format!("Workspace root: {}", find_workspace_root().display()))?;

    // User information
    let name = prompts::input::input("What's your name?")
        .placeholder("Developer")
        .interact()?;
    
    let email = prompts::email::email("What's your email?")
        .initial_value("dev@example.com")
        .interact()?;

    prompts::log::success(format!("Hello, {}! ({})", name, email))?;

    // Password setup
    let use_password = prompts::confirm("Would you like to set up a password?")
        .initial_value(true)
        .interact()?;

    if use_password {
        let password = prompts::password::password("Enter a password")
            .interact()?;
        
        match hash_password(&password) {
            Ok(hash) => {
                prompts::log::success("Password hashed successfully")?;
                if verify_password(&password, &hash) {
                    prompts::log::success("Password verification working")?;
                } else {
                    prompts::log::warning("Password verification failed")?;
                }
            }
            Err(e) => prompts::log::warning(format!("Password hashing failed: {}", e))?,
        }
    }

    // Theme selection
    let theme = prompts::select("Choose your preferred theme")
        .item("dark", "Dark Theme", "Perfect for late-night coding")
        .item("light", "Light Theme", "Easy on the eyes during the day")
        .item("auto", "Auto Theme", "Follows system preference")
        .item("cyberpunk", "Cyberpunk", "Neon colors and futuristic vibes")
        .interact()?;

    prompts::log::info(format!("Selected theme: {}", theme))?;

    // Editor preference
    let editor = prompts::select("What's your preferred code editor?")
        .item("vscode", "Visual Studio Code", "Most popular choice")
        .item("neovim", "Neovim", "Modal editing powerhouse")
        .item("vim", "Vim", "The classic")
        .item("emacs", "Emacs", "Extensible and customizable")
        .item("sublime", "Sublime Text", "Fast and lightweight")
        .item("atom", "Atom", "Hackable text editor")
        .interact()?;

    // Shell preference
    let shell = prompts::select("What's your preferred shell?")
        .item("bash", "Bash", "The standard shell")
        .item("zsh", "Zsh", "Feature-rich with great plugins")
        .item("fish", "Fish", "User-friendly with smart defaults")
        .item("powershell", "PowerShell", "Cross-platform automation")
        .item("cmd", "Command Prompt", "Windows classic")
        .interact()?;

    // Number input for experience
    let experience_years = prompts::number::number("How many years of coding experience?")
        .min(0)
        .max(50)
        .interact()?;

    prompts::log::info(format!("Experience: {} years", experience_years))?;

    // Rating for current setup satisfaction
    let satisfaction = prompts::rating::rating("Rate your current dev setup satisfaction")
        .max(5)
        .interact()?;

    prompts::log::info(format!("Current setup satisfaction: {}/5 stars", satisfaction))?;

    // Toggle preferences
    let notifications = prompts::toggle::toggle("Enable desktop notifications")
        .initial_value(true)
        .interact()?;

    let auto_updates = prompts::toggle::toggle("Enable automatic updates")
        .initial_value(false)
        .interact()?;

    let telemetry = prompts::toggle::toggle("Share anonymous usage data")
        .initial_value(false)
        .interact()?;

    // Component selection
    let components = build_component_targets(runtime_env);
    prompts::section_with_width("Available Components", 80, |lines| {
        lines.push("Select components to install based on your environment:".to_string());
        for component in &components {
            lines.push(format!("• {}", component));
        }
    })?;

    let mut component_multiselect = prompts::multiselect("Which components would you like to install?");
    for component in &components {
        component_multiselect = component_multiselect.item(component.clone(), component.clone(), "Available component");
    }
    let selected_components = component_multiselect.interact()?;

    if !selected_components.is_empty() {
        prompts::log::info("Selected components:")?;
        for component in &selected_components {
            prompts::log::step(component)?;
        }
    }

    // Provider selection
    let providers = vec![
        ("openai", "OpenAI"),
        ("anthropic", "Anthropic"), 
        ("google", "Google Gemini"),
        ("github_copilot", "GitHub Copilot"),
        ("ollama", "Ollama (Local)"),
        ("huggingface", "Hugging Face"),
    ];

    let mut provider_multiselect = prompts::multiselect("Which AI providers would you like to configure?")
        .required(false);
    for (id, name) in &providers {
        provider_multiselect = provider_multiselect.item(id.to_string(), name.to_string(), "AI Provider");
    }
    let selected_providers = provider_multiselect.interact()?;

    if !selected_providers.is_empty() {
        prompts::log::info("Selected providers:")?;
        for provider in &selected_providers {
            prompts::log::step(provider)?;
        }
    }

    // Programming languages (tags input)
    let languages = prompts::tags::tags("What programming languages do you use?")
        .placeholder("Type languages and press Enter")
        .interact()?;

    prompts::log::info(format!("Programming languages: {}", languages.join(", ")))?;

    // Final confirmation
    let proceed = prompts::confirm("Ready to complete the setup?")
        .initial_value(true)
        .interact()?;

    if !proceed {
        prompts::log::warning("Setup cancelled by user")?;
        return Err(anyhow::anyhow!("Setup cancelled"));
    }

    // Create result
    let preferences = OnboardingPreferences {
        theme: theme.to_string(),
        editor: editor.to_string(),
        shell: shell.to_string(),
        notifications,
        auto_updates,
        telemetry,
    };

    let result = OnboardingResult {
        name,
        email,
        runtime_environment: runtime_env.as_str().to_string(),
        selected_components,
        selected_providers,
        preferences,
        timestamp: Local::now().to_rfc3339(),
    };

    // Final summary
    prompts::section_with_width("Setup Complete", 80, |lines| {
        lines.push(format!("Name: {}", result.name));
        lines.push(format!("Email: {}", result.email));
        lines.push(format!("Runtime: {}", runtime_env.label()));
        lines.push(format!("Theme: {}", result.preferences.theme));
        lines.push(format!("Editor: {}", result.preferences.editor));
        lines.push(format!("Shell: {}", result.preferences.shell));
        lines.push(format!("Components: {}", result.selected_components.len()));
        lines.push(format!("Providers: {}", result.selected_providers.len()));
        lines.push(format!("Languages: {}", languages.join(", ")));
        lines.push("".to_string());
        lines.push("Your DX environment is ready to use!".to_string());
    })?;

    // Save configuration
    let config_json = serde_json::to_string_pretty(&result)?;
    let config_path = find_workspace_root().join("dx-config.json");
    fs::write(&config_path, config_json)?;
    prompts::log::success(format!("Configuration saved to: {}", config_path.display()))?;

    prompts::outro("🎉 Onboarding completed successfully!")?;
    
    Ok(result)
}

fn async_main() -> Result<()> {
    match run_onboarding() {
        Ok(result) => {
            println!("\n✨ Setup completed successfully!");
            println!("Welcome to DX, {}! 🚀", result.name);
        }
        Err(e) => {
            eprintln!("❌ Setup failed: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    async_main()
}