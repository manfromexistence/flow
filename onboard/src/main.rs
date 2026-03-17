//! DX Onboarding - Interactive Setup Experience
#![allow(dead_code)]

mod prompts;
mod effects;
mod splash;

use anyhow::Result;
use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Local;
use rand::thread_rng;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use prompts::PromptInteraction;
use effects::RainbowEffect;
use splash::{render_dx_logo, render_train_animation};

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
    // Basic Info
    name: String,
    email: String,
    website: String,
    phone: String,
    birth_date: String,
    preferred_time: String,
    bio: String,
    
    // Experience & Skills
    experience_years: i64,
    satisfaction_rating: usize,
    productivity_level: i64,
    work_hours: (i64, i64),
    programming_languages: Vec<String>,
    favorite_language: String,
    framework: String,
    project_type: String,
    selected_skills: Vec<String>,
    
    // System & Environment
    runtime_environment: String,
    selected_components: Vec<String>,
    selected_providers: Vec<String>,
    
    // Preferences
    preferences: OnboardingPreferences,
    
    // Visual & Design
    brand_color: String,
    accent_color: String,
    favorite_emoji: String,
    
    // Project Info
    project_deadline: String,
    code_snippet: CodeSnippetData,
    config_file_path: String,
    
    // Workflow Data
    todo_list_items: usize,
    team_table_rows: usize,
    kanban_tasks: usize,
    wizard_completed_steps: usize,
    
    // Metadata
    timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
struct CodeSnippetData {
    name: String,
    language: String,
    code: String,
    description: String,
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
    // Initialize rainbow effect
    let rainbow = RainbowEffect::new();
    
    // Clear screen and show animated splash
    print!("\x1B[2J\x1B[H"); // Clear screen and move cursor to top
    
    // Show train animation for a few frames
    for frame in 0..10 {
        print!("\x1B[H"); // Move cursor to top
        render_train_animation(&rainbow, frame)?;
        thread::sleep(Duration::from_millis(200));
    }
    
    // Clear and show DX logo
    print!("\x1B[2J\x1B[H"); // Clear screen
    render_dx_logo(&rainbow)?;
    println!();
    println!("Enhanced Development Experience");
    println!();
    thread::sleep(Duration::from_millis(1000));

    // Welcome
    prompts::intro("🚀 DX Onboarding - Complete Prompts Showcase")?;
    
    prompts::section_with_width("Welcome to DX", 80, |lines| {
        lines.push("This onboarding showcases ALL available prompt types!".to_string());
        lines.push(format!("Detected runtime: {}", detect_runtime_environment().label()));
        lines.push(format!("Runtime hint: {}", detect_runtime_environment().hint()));
        lines.push("".to_string());
        lines.push("Let's explore every single prompt component available.".to_string());
    })?;

    let runtime_env = detect_runtime_environment();
    prompts::log::info(format!("Runtime Environment: {}", runtime_env.label()))?;
    prompts::log::info(format!("Workspace root: {}", find_workspace_root().display()))?;

    // 1. Basic Input
    let name = prompts::input::input("What's your name?")
        .placeholder("Developer")
        .interact()?;
    
    // 2. Email Input with validation
    let email = prompts::email::email("What's your email?")
        .initial_value("dev@example.com")
        .interact()?;

    prompts::log::success(format!("Hello, {}! ({})", name, email))?;

    // 3. Password Input
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

    // 4. URL Input
    let website = prompts::url::url("What's your website or portfolio URL?")
        .interact()?;
    prompts::log::info(format!("Website: {}", website))?;

    // 5. Phone Input
    let phone = prompts::phone_input::phone_input("What's your phone number?")
        .interact()?;
    prompts::log::info(format!("Phone: {}", phone))?;

    // 6. Credit Card Input (demo)
    let use_payment = prompts::confirm("Would you like to add payment info? (demo only)")
        .initial_value(false)
        .interact()?;

    if use_payment {
        let _card = prompts::credit_card::credit_card("Enter credit card number (demo)")
            .interact()?;
        prompts::log::info("Credit card info collected (demo only)")?;
    }

    // 7. Date Picker
    let birth_date = prompts::date_picker::date_picker("What's your birth date?")
        .interact()?;
    prompts::log::info(format!("Birth date: {}", birth_date))?;

    // 8. Time Picker
    let preferred_time = prompts::time_picker::time_picker("What's your preferred work start time?")
        .interact()?;
    prompts::log::info(format!("Preferred time: {}", preferred_time))?;

    // 9. Number Input
    let experience_years = prompts::number::number("How many years of coding experience?")
        .min(0)
        .max(50)
        .interact()?;
    prompts::log::info(format!("Experience: {} years", experience_years))?;

    // 10. Rating
    let satisfaction = prompts::rating::rating("Rate your current dev setup satisfaction")
        .max(5)
        .interact()?;
    prompts::log::info(format!("Current setup satisfaction: {}/5 stars", satisfaction))?;

    // 11. Slider
    let productivity = prompts::slider::slider("Rate your productivity level (0-100)", 0, 100)
        .initial_value(75)
        .interact()?;
    prompts::log::info(format!("Productivity level: {}%", productivity))?;

    // 12. Range Slider
    let work_hours = prompts::range_slider::range_slider("Select your preferred work hours", 0, 24)
        .initial_range(9, 17)
        .interact()?;
    prompts::log::info(format!("Work hours: {}:00 - {}:00", work_hours.0, work_hours.1))?;

    // 13. Toggle switches
    let notifications = prompts::toggle::toggle("Enable desktop notifications")
        .initial_value(true)
        .interact()?;

    let auto_updates = prompts::toggle::toggle("Enable automatic updates")
        .initial_value(false)
        .interact()?;

    let telemetry = prompts::toggle::toggle("Share anonymous usage data")
        .initial_value(false)
        .interact()?;

    // 14. Single Select
    let theme = prompts::select("Choose your preferred theme")
        .item("dark", "Dark Theme", "Perfect for late-night coding")
        .item("light", "Light Theme", "Easy on the eyes during the day")
        .item("auto", "Auto Theme", "Follows system preference")
        .item("cyberpunk", "Cyberpunk", "Neon colors and futuristic vibes")
        .interact()?;
    prompts::log::info(format!("Selected theme: {}", theme))?;

    // 15. Editor preference
    let editor = prompts::select("What's your preferred code editor?")
        .item("vscode", "Visual Studio Code", "Most popular choice")
        .item("neovim", "Neovim", "Modal editing powerhouse")
        .item("vim", "Vim", "The classic")
        .item("emacs", "Emacs", "Extensible and customizable")
        .item("sublime", "Sublime Text", "Fast and lightweight")
        .item("atom", "Atom", "Hackable text editor")
        .interact()?;

    // 16. Shell preference
    let shell = prompts::select("What's your preferred shell?")
        .item("bash", "Bash", "The standard shell")
        .item("zsh", "Zsh", "Feature-rich with great plugins")
        .item("fish", "Fish", "User-friendly with smart defaults")
        .item("powershell", "PowerShell", "Cross-platform automation")
        .item("cmd", "Command Prompt", "Windows classic")
        .interact()?;

    // 17. Multi-select for components
    let components = build_component_targets(runtime_env);
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

    // 18. Multi-select for providers
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

    // 19. Tags input for programming languages
    let languages = prompts::tags::tags("What programming languages do you use?")
        .placeholder("Type languages and press Enter")
        .interact()?;
    prompts::log::info(format!("Programming languages: {}", languages.join(", ")))?;

    // 20. Autocomplete
    let favorite_language = prompts::autocomplete::autocomplete("What's your favorite programming language?")
        .item("rust", "Rust")
        .item("javascript", "JavaScript")
        .item("typescript", "TypeScript")
        .item("python", "Python")
        .item("go", "Go")
        .item("java", "Java")
        .item("cpp", "C++")
        .item("csharp", "C#")
        .interact()?;
    prompts::log::info(format!("Favorite language: {}", favorite_language))?;

    // 21. Search Filter
    let framework = prompts::search_filter::search_filter("Choose your preferred web framework")
        .item("React", "React", vec!["frontend".to_string(), "javascript".to_string()])
        .item("Vue.js", "Vue.js", vec!["frontend".to_string(), "javascript".to_string()])
        .item("Angular", "Angular", vec!["frontend".to_string(), "typescript".to_string()])
        .item("Svelte", "Svelte", vec!["frontend".to_string(), "javascript".to_string()])
        .item("Next.js", "Next.js", vec!["fullstack".to_string(), "react".to_string()])
        .item("Express.js", "Express.js", vec!["backend".to_string(), "javascript".to_string()])
        .interact()?;
    prompts::log::info(format!("Preferred framework: {}", framework))?;

    // 22. Tree Select - simplified for now
    let project_type = prompts::select("What type of project are you working on?")
        .item("web_frontend", "Web Frontend", "React, Vue, Angular applications")
        .item("web_backend", "Web Backend", "APIs and server applications")
        .item("mobile", "Mobile Development", "iOS, Android, Cross-platform")
        .item("desktop", "Desktop Applications", "Native or Electron apps")
        .item("systems", "Systems Programming", "OS, embedded, low-level")
        .interact()?;
    prompts::log::info(format!("Project type: {}", project_type))?;

    // 23. Matrix Select - simplified for now
    let skills = prompts::multiselect("Rate your skills in different areas")
        .item("frontend", "Frontend Development", "HTML, CSS, JavaScript")
        .item("backend", "Backend Development", "APIs, databases, servers")
        .item("devops", "DevOps", "CI/CD, containers, cloud")
        .item("mobile", "Mobile Development", "iOS, Android apps")
        .item("aiml", "AI/ML", "Machine learning, data science")
        .interact()?;
    prompts::log::info(format!("Skills selected: {} areas", skills.len()))?;

    // 24. File Browser
    let config_file = prompts::file_browser::file_browser("Select a configuration file")
        .interact()?;
    prompts::log::info(format!("Selected file: {}", config_file.display()))?;

    // 25. Color Picker
    let brand_color = prompts::color_picker::color_picker("Choose your brand color")
        .interact()?;
    prompts::log::info(format!("Brand color: {}", brand_color))?;

    // 26. Advanced Color Picker
    let accent_color = prompts::color_picker_advanced::color_picker_advanced("Choose accent color")
        .mode(prompts::ColorMode::HEX)
        .interact()?;
    prompts::log::info(format!("Accent color: {}", accent_color))?;

    // 27. Emoji Picker
    let favorite_emoji = prompts::emoji_picker::emoji_picker("Pick your favorite emoji")
        .interact()?;
    prompts::log::info(format!("Favorite emoji: {}", favorite_emoji))?;

    // 28. Calendar View
    let project_deadline = prompts::calendar::calendar("When is your project deadline?")
        .interact()?;
    prompts::log::info(format!("Project deadline: {}", project_deadline))?;

    // 29. Code Snippet - simplified
    let code_example = prompts::code_snippet::code_snippet("Paste a code snippet you're proud of")
        .snippet(prompts::CodeSnippet {
            name: "Hello World".to_string(),
            language: "rust".to_string(),
            code: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
            description: "A simple Rust hello world program".to_string(),
        })
        .interact()?;
    prompts::log::info(format!("Code snippet length: {} characters", code_example.code.len()))?;

    // 30. JSON Editor - simplified
    let _config_json = prompts::json_editor::json_editor("Edit your configuration")
        .interact()?;
    prompts::log::info("JSON configuration updated")?;

    // 31. Markdown Editor - simplified
    let readme_content = prompts::markdown_editor::markdown_editor("Write your project README")
        .interact()?;
    prompts::log::info(format!("README length: {} characters", readme_content.len()))?;

    // 32. Table Editor
    let team_info = prompts::table_editor::table_editor("Edit team information")
        .headers(vec!["Name".to_string(), "Role".to_string(), "Experience".to_string()])
        .interact()?;
    prompts::log::info(format!("Team table has {} rows", team_info.len()))?;

    // 33. List Editor
    let todo_list = prompts::list_editor("Create your TODO list")
        .interact()?;
    prompts::log::info(format!("TODO list has {} items", todo_list.len()))?;

    // 34. Kanban Board
    let project_board = prompts::kanban::kanban("Organize your project tasks")
        .column("TODO")
        .column("In Progress")
        .column("Done")
        .interact()?;
    prompts::log::info(format!("Kanban board has {} tasks", project_board.len()))?;

    // 35. Progress Bar Demo
    prompts::log::info("Simulating setup progress...")?;
    let mut progress = prompts::progress::ProgressBar::new("Setting up environment", 100);
    progress.start()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    progress.set(25)?;
    progress.set_message("Installing dependencies...")?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    progress.set(50)?;
    progress.set_message("Configuring settings...")?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    progress.set(75)?;
    progress.set_message("Finalizing setup...")?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    progress.finish("Setup complete!")?;

    // 36. Spinner Demo
    let mut spinner = prompts::spinner::spinner("Processing your configuration...");
    spinner.start()?;
    std::thread::sleep(std::time::Duration::from_millis(2000));
    spinner.stop("Configuration processed successfully!")?;

    // 37. Text Area
    let bio = prompts::text::text("Tell us about yourself")
        .placeholder("Write a short bio...")
        .interact()?;
    prompts::log::info(format!("Bio length: {} characters", bio.len()))?;

    // 38. Wizard (multi-step process)
    let wizard_result = prompts::wizard::wizard("Complete Project Setup")
        .step("Project Basics", "Set up basic project information")
        .step("Advanced Settings", "Configure advanced options")
        .step("Review & Confirm", "Review your settings")
        .interact()?;
    prompts::log::info(format!("Wizard completed: {}", wizard_result))?;

    // Final confirmation
    let proceed = prompts::confirm("Ready to complete the setup with all these amazing prompts?")
        .initial_value(true)
        .interact()?;

    if !proceed {
        prompts::log::warning("Setup cancelled by user")?;
        return Err(anyhow::anyhow!("Setup cancelled"));
    }

    // Create result with ALL collected data
    let preferences = OnboardingPreferences {
        theme: theme.to_string(),
        editor: editor.to_string(),
        shell: shell.to_string(),
        notifications,
        auto_updates,
        telemetry,
    };

    let code_snippet_data = CodeSnippetData {
        name: code_example.name.clone(),
        language: code_example.language.clone(),
        code: code_example.code.clone(),
        description: code_example.description.clone(),
    };

    let result = OnboardingResult {
        // Basic Info
        name,
        email,
        website: website.clone(),
        phone: phone.clone(),
        birth_date: birth_date.clone(),
        preferred_time: preferred_time.clone(),
        bio,
        
        // Experience & Skills
        experience_years,
        satisfaction_rating: satisfaction,
        productivity_level: productivity,
        work_hours,
        programming_languages: languages.clone(),
        favorite_language: favorite_language.to_string(),
        framework: framework.to_string(),
        project_type: project_type.to_string(),
        selected_skills: skills.iter().map(|s| s.to_string()).collect(),
        
        // System & Environment
        runtime_environment: runtime_env.as_str().to_string(),
        selected_components,
        selected_providers,
        
        // Preferences
        preferences,
        
        // Visual & Design
        brand_color: brand_color.clone(),
        accent_color: accent_color.clone(),
        favorite_emoji: favorite_emoji.clone(),
        
        // Project Info
        project_deadline,
        code_snippet: code_snippet_data,
        config_file_path: config_file.display().to_string(),
        
        // Workflow Data
        todo_list_items: todo_list.len(),
        team_table_rows: team_info.len(),
        kanban_tasks: project_board.len(),
        wizard_completed_steps: wizard_result,
        
        // Metadata
        timestamp: Local::now().to_rfc3339(),
    };

    // Final summary
    prompts::section_with_width("🎉 Complete Setup Summary", 80, |lines| {
        lines.push(format!("Name: {}", result.name));
        lines.push(format!("Email: {}", result.email));
        lines.push(format!("Website: {}", website));
        lines.push(format!("Phone: {}", phone));
        lines.push(format!("Birth Date: {}", birth_date));
        lines.push(format!("Preferred Time: {}", preferred_time));
        lines.push(format!("Experience: {} years", experience_years));
        lines.push(format!("Satisfaction: {}/5 stars", satisfaction));
        lines.push(format!("Productivity: {}%", productivity));
        lines.push(format!("Work Hours: {}:00-{}:00", work_hours.0, work_hours.1));
        lines.push(format!("Runtime: {}", runtime_env.label()));
        lines.push(format!("Theme: {}", result.preferences.theme));
        lines.push(format!("Editor: {}", result.preferences.editor));
        lines.push(format!("Shell: {}", result.preferences.shell));
        lines.push(format!("Favorite Language: {}", favorite_language));
        lines.push(format!("Framework: {}", framework));
        lines.push(format!("Project Type: {}", project_type));
        lines.push(format!("Brand Color: {}", brand_color));
        lines.push(format!("Accent Color: {}", accent_color));
        lines.push(format!("Favorite Emoji: {}", favorite_emoji));
        lines.push(format!("Components: {}", result.selected_components.len()));
        lines.push(format!("Providers: {}", result.selected_providers.len()));
        lines.push(format!("Languages: {}", languages.join(", ")));
        lines.push("".to_string());
        lines.push("🚀 ALL 38 PROMPT TYPES DEMONSTRATED! 🚀".to_string());
        lines.push("Your DX environment is fully configured!".to_string());
    })?;

    // Save configuration
    let config_json = serde_json::to_string_pretty(&result)?;
    let config_path = find_workspace_root().join("dx.json");
    fs::write(&config_path, config_json)?;
    prompts::log::success(format!("Configuration saved to: {}", config_path.display()))?;

    prompts::outro("🎉 Complete onboarding with ALL prompts finished!")?;
    
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