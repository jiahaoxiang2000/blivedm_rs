use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub connection: Option<ConnectionConfig>,
    #[serde(default)]
    pub tts: Option<TtsConfig>,
    #[serde(default)]
    pub auto_reply: Option<AutoReplyConfig>,
    #[serde(default)]
    pub debug: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionConfig {
    pub cookies: Option<String>,
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TtsConfig {
    pub server: Option<String>,
    pub voice: Option<String>,
    pub backend: Option<String>,
    pub quality: Option<String>,
    pub format: Option<String>,
    pub sample_rate: Option<u32>,
    pub volume: Option<f32>,
    pub command: Option<String>,
    pub args: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConfig {
    pub keywords: Vec<String>,
    pub responses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoReplyConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_cooldown")]
    pub cooldown_seconds: u64,
    #[serde(default)]
    pub triggers: Vec<TriggerConfig>,
}

impl Default for AutoReplyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cooldown_seconds: default_cooldown(),
            triggers: vec![],
        }
    }
}

fn default_cooldown() -> u64 {
    5
}

impl AutoReplyConfig {
    /// Convert to plugins::auto_reply::AutoReplyConfig
    pub fn to_plugin_config(&self) -> plugins::auto_reply::AutoReplyConfig {
        plugins::auto_reply::AutoReplyConfig {
            enabled: self.enabled,
            cooldown_seconds: self.cooldown_seconds,
            triggers: self.triggers.iter().map(|t| t.to_plugin_trigger()).collect(),
        }
    }
}

impl TriggerConfig {
    /// Convert to plugins::auto_reply::TriggerConfig
    pub fn to_plugin_trigger(&self) -> plugins::auto_reply::TriggerConfig {
        plugins::auto_reply::TriggerConfig {
            keywords: self.keywords.clone(),
            responses: self.responses.clone(),
        }
    }
}

impl Config {
    /// Load configuration from file with fallback locations
    pub fn load_from_file(config_path: Option<&Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let config_file = if let Some(path) = config_path {
            // Use provided path
            path.to_path_buf()
        } else {
            // Try current directory first
            let current_dir_config = PathBuf::from("config.toml");
            if current_dir_config.exists() {
                current_dir_config
            } else {
                // Try XDG config directory
                Self::get_default_config_path()?
            }
        };

        if !config_file.exists() {
            log::debug!("Config file {:?} not found", config_file);

            // Create config file if it doesn't exist and we're using default locations
            if config_path.is_none() {
                match Self::create_example_config(&config_file) {
                    Ok(()) => {
                        println!("Created configuration file: {:?}", config_file);
                        println!("You can customize it as needed.");
                    }
                    Err(e) => {
                        log::warn!("Failed to create config file: {}", e);
                        return Ok(Config::default());
                    }
                }
            } else {
                return Ok(Config::default());
            }
        }

        log::info!("Loading configuration from {:?}", config_file);
        let content = fs::read_to_string(&config_file)
            .map_err(|e| format!("Failed to read config file {:?}: {}", config_file, e))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file {:?}: {}", config_file, e))?;

        Ok(config)
    }

    /// Get the default configuration file path (~/.config/blivedm_rs/config.toml)
    fn get_default_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Unable to determine config directory")?
            .join("blivedm_rs");

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).map_err(|e| {
                format!("Failed to create config directory {:?}: {}", config_dir, e)
            })?;
        }

        Ok(config_dir.join("config.toml"))
    }

    /// Create an example configuration file
    pub fn create_example_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let example_config = Config {
            connection: None,
            tts: Some(TtsConfig {
                server: Some("http://localhost:8000".to_string()),
                voice: None,
                backend: None,
                quality: None,
                format: None,
                sample_rate: None,
                volume: None,
                command: None,
                args: None,
            }),
            auto_reply: Some(AutoReplyConfig {
                enabled: false,
                cooldown_seconds: 5,
                triggers: vec![
                    TriggerConfig {
                        keywords: vec!["你好".to_string(), "hello".to_string()],
                        responses: vec!["欢迎来到直播间！".to_string(), "Hello! Welcome!".to_string()],
                    },
                    TriggerConfig {
                        keywords: vec!["谢谢".to_string(), "thanks".to_string()],
                        responses: vec!["不客气～".to_string(), "You're welcome!".to_string()],
                    },
                ],
            }),
            debug: None,
        };

        let toml_string = toml::to_string_pretty(&example_config)
            .map_err(|e| format!("Failed to serialize example config: {}", e))?;

        fs::write(path, toml_string)
            .map_err(|e| format!("Failed to write example config to {:?}: {}", path, e))?;

        Ok(())
    }

    /// Print the effective configuration (for debugging)
    pub fn print_effective_config(
        cookies: &Option<String>,
        room_id: &str,
        tts_server: &Option<String>,
        tts_voice: &Option<String>,
        tts_backend: &Option<String>,
        tts_quality: &Option<String>,
        tts_format: &Option<String>,
        tts_sample_rate: &Option<u32>,
        tts_volume: &Option<f32>,
        tts_command: &Option<String>,
        tts_args: &Option<String>,
        auto_reply: &Option<AutoReplyConfig>,
        debug: bool,
    ) {
        println!("=== Effective Configuration ===");
        println!("Connection:");
        println!("  room_id: {}", room_id);
        if let Some(cookies_val) = cookies {
            println!(
                "  cookies: {}...",
                &cookies_val.chars().take(20).collect::<String>()
            );
        } else {
            println!("  cookies: None (will auto-detect)");
        }

        println!("TTS:");
        println!("  server: {:?}", tts_server);
        println!("  voice: {:?}", tts_voice);
        println!("  backend: {:?}", tts_backend);
        println!("  quality: {:?}", tts_quality);
        println!("  format: {:?}", tts_format);
        println!("  sample_rate: {:?}", tts_sample_rate);
        println!("  volume: {:?}", tts_volume);
        println!("  command: {:?}", tts_command);
        println!("  args: {:?}", tts_args);

        println!("Auto Reply:");
        if let Some(auto_reply_config) = auto_reply {
            println!("  enabled: {}", auto_reply_config.enabled);
            println!("  cooldown_seconds: {}", auto_reply_config.cooldown_seconds);
            println!("  triggers: {} configured", auto_reply_config.triggers.len());
        } else {
            println!("  enabled: false (not configured)");
        }

        println!("Debug: {}", debug);
        println!("===============================");
    }
}
