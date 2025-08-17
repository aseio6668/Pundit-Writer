use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default_model: String,
    pub api_key: Option<String>,
    pub output_directory: PathBuf,
    pub default_author: String,
    pub generation_settings: GenerationSettings,
    pub auto_save: bool,
    pub save_frequency: u32, // Save every N chapters
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationSettings {
    pub temperature: f32,
    pub context_window: usize,
    pub words_per_chapter: usize,
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_model: "gpt2".to_string(),
            api_key: None,
            output_directory: get_default_output_dir(),
            default_author: "Pundit AI".to_string(),
            generation_settings: GenerationSettings {
                temperature: 0.8,
                context_window: 3,
                words_per_chapter: 2500,
                max_retries: 3,
                retry_delay_seconds: 5,
            },
            auto_save: true,
            save_frequency: 1,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        
        if config_path.exists() {
            let config_content = fs::read_to_string(&config_path)
                .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
            
            let config: Config = serde_json::from_str(&config_content)
                .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
            
            Ok(config)
        } else {
            let default_config = Config::default();
            default_config.save()?;
            Ok(default_config)
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| anyhow!("Failed to create config directory: {}", e))?;
        }
        
        let config_content = serde_json::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;
        
        fs::write(&config_path, config_content)
            .map_err(|e| anyhow!("Failed to write config file: {}", e))?;
        
        Ok(())
    }
    
    pub fn update_api_key(&mut self, api_key: String) -> Result<()> {
        self.api_key = Some(api_key);
        self.save()
    }
    
    pub fn get_effective_api_key(&self) -> Option<String> {
        // Check environment variable first, then config
        std::env::var("HF_API_KEY").ok()
            .or_else(|| self.api_key.clone())
    }
}

fn get_config_path() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "pundit", "pundit-writer")
        .ok_or_else(|| anyhow!("Failed to determine config directory"))?;
    
    Ok(project_dirs.config_dir().join("config.json"))
}

fn get_default_output_dir() -> PathBuf {
    if let Some(project_dirs) = ProjectDirs::from("com", "pundit", "pundit-writer") {
        project_dirs.data_dir().join("books")
    } else {
        PathBuf::from("./books")
    }
}

pub fn get_books_dir() -> Result<PathBuf> {
    let config = Config::load()?;
    let books_dir = config.output_directory;
    
    if !books_dir.exists() {
        fs::create_dir_all(&books_dir)
            .map_err(|e| anyhow!("Failed to create books directory: {}", e))?;
    }
    
    Ok(books_dir)
}

pub fn save_book_state(content: &crate::content::Content) -> Result<()> {
    let books_dir = get_books_dir()?;
    let content_file = books_dir.join(format!("{}.json", content.id));
    
    let content_json = serde_json::to_string_pretty(content)
        .map_err(|e| anyhow!("Failed to serialize content: {}", e))?;
    
    fs::write(&content_file, content_json)
        .map_err(|e| anyhow!("Failed to save content state: {}", e))?;
    
    Ok(())
}

pub fn load_book_state(content_id: &str) -> Result<crate::content::Content> {
    let books_dir = get_books_dir()?;
    let content_file = books_dir.join(format!("{}.json", content_id));
    
    if !content_file.exists() {
        return Err(anyhow!("Content state file not found"));
    }
    
    let content_data = fs::read_to_string(&content_file)
        .map_err(|e| anyhow!("Failed to read content state: {}", e))?;
    
    let content: crate::content::Content = serde_json::from_str(&content_data)
        .map_err(|e| anyhow!("Failed to parse content state: {}", e))?;
    
    Ok(content)
}

pub fn list_saved_books() -> Result<Vec<String>> {
    let books_dir = get_books_dir()?;
    
    if !books_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut book_ids = Vec::new();
    
    for entry in fs::read_dir(&books_dir)
        .map_err(|e| anyhow!("Failed to read books directory: {}", e))? {
        let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                book_ids.push(stem.to_string());
            }
        }
    }
    
    Ok(book_ids)
}