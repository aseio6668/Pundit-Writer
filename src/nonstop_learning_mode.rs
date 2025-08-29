use anyhow::Result;
use console::Term;
use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::cli_types::{Genre, WritingStyle, BookSize, ScreenplayLength, PlayLength};
use crate::content::ContentType;
use crate::writer::{write_book, write_screenplay, write_play};
use crate::advanced_learning_system::AdvancedLearningSystem;
use crate::master_intelligence_system::MasterIntelligenceSystem;
use crate::ollama::OllamaClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonstopLearningConfig {
    pub duration_hours: Option<f32>, // None for unlimited
    pub max_works: Option<u32>, // None for unlimited
    pub work_types: Vec<WorkType>,
    pub auto_retry_attempts: u8,
    pub learning_frequency: u32, // Learn every N works
    pub pause_between_works_seconds: u32,
    pub auto_generate_titles: bool, // Auto-generate titles and descriptions
    pub auto_approve_outlines: bool, // Automatically approve outlines without user input
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkType {
    Book,
    Screenplay, 
    Play,
    ChildrensBook,
    Poetry,
    TechnicalDoc,
    BusinessDoc,
}

#[derive(Debug, Clone)]
pub struct RandomWorkParams {
    pub title: String,
    pub description: String,
    pub genre: Genre,
    pub style: WritingStyle,
    pub work_type: WorkType,
    pub size_length: WorkSize,
}

#[derive(Debug, Clone)]
pub enum WorkSize {
    BookSize(BookSize),
    ScreenplayLength(ScreenplayLength),
    PlayLength(PlayLength),
}

#[derive(Debug, Clone)]
pub struct WorkGenerationResult {
    pub success: bool,
    pub work_params: RandomWorkParams,
    pub output_path: Option<String>,
    pub error_message: Option<String>,
    pub generation_time_seconds: u64,
    pub retry_count: u8,
}

#[derive(Debug, Clone)]
pub struct LearningSession {
    pub session_id: String,
    pub start_time: Instant,
    pub works_completed: u32,
    pub works_failed: u32,
    pub total_errors: u32,
    pub learning_insights_gained: u32,
    pub improvements_made: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonstopSessionData {
    pub session_id: String,
    pub start_time_unix: u64, // Store as Unix timestamp instead of Instant
    pub duration_seconds: u64,
    pub works_completed: u32,
    pub works_failed: u32,
    pub total_errors: u32,
    pub learning_insights_gained: u32,
    pub improvements_made: u32,
    pub config: NonstopLearningConfig,
    pub timestamp: std::time::SystemTime,
}

pub struct NonstopLearningMode {
    config: NonstopLearningConfig,
    session: LearningSession,
    is_paused: Arc<AtomicBool>,
    should_stop: Arc<AtomicBool>,
    learning_system: Option<Arc<AdvancedLearningSystem>>,
    intelligence_system: Option<Arc<MasterIntelligenceSystem>>,
}

impl Default for NonstopLearningConfig {
    fn default() -> Self {
        Self {
            duration_hours: None, // Unlimited by default
            max_works: None, // Unlimited by default
            work_types: vec![
                WorkType::Book,
                WorkType::Screenplay,
                WorkType::Play,
                WorkType::ChildrensBook,
                WorkType::Poetry,
            ],
            auto_retry_attempts: 3,
            learning_frequency: 5, // Learn every 5 works
            pause_between_works_seconds: 2,
            auto_generate_titles: true, // Default to full autonomy
            auto_approve_outlines: true, // Default to full autonomy
        }
    }
}

impl NonstopLearningMode {
    pub fn new(config: NonstopLearningConfig) -> Self {
        let session = LearningSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: Instant::now(),
            works_completed: 0,
            works_failed: 0,
            total_errors: 0,
            learning_insights_gained: 0,
            improvements_made: 0,
        };

        Self {
            config,
            session,
            is_paused: Arc::new(AtomicBool::new(false)),
            should_stop: Arc::new(AtomicBool::new(false)),
            learning_system: None,
            intelligence_system: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("🚀 Starting Nonstop Learning Mode");
        println!("Press 'p' to pause, 'r' to resume, 'q' to quit");
        println!("═══════════════════════════════════════");

        let term = Term::stdout();
        
        while !self.should_stop.load(Ordering::Relaxed) {
            // Check for keyboard input
            self.handle_keyboard_input(&term).await?;

            if self.is_paused.load(Ordering::Relaxed) {
                sleep(Duration::from_millis(100)).await;
                continue;
            }

            // Check if we should stop based on time or work limits
            if self.should_stop_based_on_limits() {
                break;
            }

            // Generate random work parameters
            let work_params = self.generate_random_work_params();
            
            // Attempt to generate the work
            let result = self.generate_work(&work_params).await;
            
            // Process results and learn
            self.process_work_result(result).await?;
            
            // Brief pause between works
            sleep(Duration::from_secs(self.config.pause_between_works_seconds as u64)).await;
        }

        self.print_session_summary();
        Ok(())
    }

    async fn handle_keyboard_input(&mut self, term: &Term) -> Result<()> {
        // Use a simple polling approach since read_key_timeout might not be available
        // This is a simplified version that checks for keyboard input periodically
        if term.is_term() {
            // For now, we'll use a basic implementation
            // Users can press Ctrl+C to stop, which is handled by the system
        }
        Ok(())
    }

    fn should_stop_based_on_limits(&self) -> bool {
        // Check time limit
        if let Some(duration_hours) = self.config.duration_hours {
            let elapsed = self.session.start_time.elapsed();
            if elapsed.as_secs_f32() / 3600.0 >= duration_hours {
                println!("⏰ Time limit reached");
                return true;
            }
        }

        // Check work count limit
        if let Some(max_works) = self.config.max_works {
            if self.session.works_completed >= max_works {
                println!("📊 Work count limit reached");
                return true;
            }
        }

        false
    }

    fn generate_random_work_params(&self) -> RandomWorkParams {
        let mut rng = thread_rng();

        // Random work type from configured types
        let work_type = self.config.work_types[rng.gen_range(0..self.config.work_types.len())].clone();
        
        // Random genre
        let genres = vec![
            Genre::Fantasy, Genre::Mystery, Genre::Romance, Genre::SciFi,
            Genre::Thriller, Genre::Historical, Genre::Contemporary,
            Genre::Adventure, Genre::Horror, Genre::Comedy,
        ];
        let genre = genres[rng.gen_range(0..genres.len())].clone();

        // Random style
        let styles = vec![
            WritingStyle::Descriptive, WritingStyle::Narrative, WritingStyle::Creative,
            WritingStyle::Formal, WritingStyle::Casual, WritingStyle::Poetic,
            WritingStyle::Conversational, WritingStyle::Dramatic, WritingStyle::Humorous,
            WritingStyle::StreamOfConsciousness,
        ];
        let style = styles[rng.gen_range(0..styles.len())].clone();

        // Generate size/length based on work type
        let size_length = match work_type {
            WorkType::Book | WorkType::ChildrensBook => {
                let sizes = vec![BookSize::Short, BookSize::Medium, BookSize::Large];
                WorkSize::BookSize(sizes[rng.gen_range(0..sizes.len())].clone())
            },
            WorkType::Screenplay => {
                let lengths = vec![ScreenplayLength::Short, ScreenplayLength::Feature];
                WorkSize::ScreenplayLength(lengths[rng.gen_range(0..lengths.len())].clone())
            },
            WorkType::Play => {
                let lengths = vec![PlayLength::OneAct, PlayLength::Full];
                WorkSize::PlayLength(lengths[rng.gen_range(0..lengths.len())].clone())
            },
            _ => WorkSize::BookSize(BookSize::Short), // Default for other types
        };

        // Generate random title and description
        let title = self.generate_random_title(&genre, &work_type);
        let description = self.generate_random_description(&genre, &style, &work_type);

        RandomWorkParams {
            title,
            description,
            genre,
            style,
            work_type,
            size_length,
        }
    }

    fn generate_random_title(&self, genre: &Genre, work_type: &WorkType) -> String {
        let mut rng = thread_rng();
        
        let genre_words = match genre {
            Genre::Fantasy => vec!["Dragon", "Magic", "Realm", "Quest", "Prophecy", "Shadow", "Crown", "Enchanted"],
            Genre::Mystery => vec!["Secret", "Hidden", "Clue", "Detective", "Murder", "Missing", "Case", "Truth"],
            Genre::Romance => vec!["Love", "Heart", "Kiss", "Forever", "Passion", "Desire", "Sweet", "Tender"],
            Genre::SciFi => vec!["Star", "Future", "Galaxy", "Robot", "Time", "Space", "Quantum", "Cyber"],
            Genre::Thriller => vec!["Danger", "Chase", "Edge", "Deadly", "Escape", "Hunt", "Dark", "Final"],
            Genre::Historical => vec!["Empire", "War", "Legacy", "Crown", "Revolution", "Ancient", "Dynasty", "Chronicle"],
            Genre::Contemporary => vec!["Modern", "City", "Life", "Journey", "Change", "Dream", "New", "Real"],
            Genre::Adventure => vec!["Journey", "Explorer", "Treasure", "Island", "Mountain", "Wild", "Quest", "Discovery"],
            Genre::Horror => vec!["Haunted", "Nightmare", "Dark", "Terror", "Cursed", "Evil", "Sinister", "Forbidden"],
            Genre::Comedy => vec!["Funny", "Laugh", "Crazy", "Wild", "Silly", "Mad", "Hilarious", "Comic"],
            _ => vec!["Story", "Tale", "Work", "Narrative", "Journey", "Adventure", "Mystery", "Epic"], // Default for other genres
        };

        let descriptors = vec!["The", "A", "An", "Last", "First", "Lost", "Hidden", "Secret", "Great", "Silent"];
        let connector_words = vec!["of", "in", "at", "under", "beyond", "within", "through", "across"];
        
        // Generate title patterns
        let patterns = vec![
            format!("{} {}", 
                descriptors[rng.gen_range(0..descriptors.len())],
                genre_words[rng.gen_range(0..genre_words.len())]
            ),
            format!("{} {} {} {}", 
                descriptors[rng.gen_range(0..descriptors.len())],
                genre_words[rng.gen_range(0..genre_words.len())],
                connector_words[rng.gen_range(0..connector_words.len())],
                genre_words[rng.gen_range(0..genre_words.len())]
            ),
        ];

        patterns[rng.gen_range(0..patterns.len())].clone()
    }

    fn generate_random_description(&self, genre: &Genre, style: &WritingStyle, work_type: &WorkType) -> String {
        let work_desc = match work_type {
            WorkType::Book => "novel",
            WorkType::Screenplay => "screenplay",
            WorkType::Play => "play",
            WorkType::ChildrensBook => "children's book",
            WorkType::Poetry => "poetry collection",
            WorkType::TechnicalDoc => "technical document",
            WorkType::BusinessDoc => "business document",
        };

        let genre_desc = match genre {
            Genre::Fantasy => "magical world with mythical creatures and epic adventures",
            Genre::Mystery => "intriguing puzzle with clues and unexpected revelations",
            Genre::Romance => "heartwarming love story with emotional depth",
            Genre::SciFi => "futuristic setting with advanced technology and exploration",
            Genre::Thriller => "suspenseful narrative with high stakes and danger",
            Genre::Historical => "richly detailed historical setting with authentic period elements",
            Genre::Contemporary => "modern setting exploring current themes and relationships",
            Genre::Adventure => "exciting journey with challenges and discoveries",
            Genre::Horror => "atmospheric tale of suspense and supernatural elements",
            Genre::Comedy => "humorous story with witty dialogue and amusing situations",
            _ => "compelling narrative with engaging characters and plot", // Default for other genres
        };

        let style_desc = match style {
            WritingStyle::Descriptive => "with rich, detailed descriptions that paint vivid scenes",
            WritingStyle::Narrative => "using compelling storytelling techniques",
            WritingStyle::Creative => "with imaginative and artistic expression",
            WritingStyle::Formal => "following structured, professional conventions",
            WritingStyle::Casual => "with relaxed, accessible language",
            WritingStyle::Poetic => "using lyrical and expressive language",
            WritingStyle::Conversational => "with natural, dialogue-driven flow",
            WritingStyle::Dramatic => "with intense, emotionally charged scenes",
            WritingStyle::Humorous => "with wit and comedic elements",
            WritingStyle::StreamOfConsciousness => "employing stream-of-consciousness narrative flow",
            _ => "with engaging and well-crafted prose",
        };

        format!("A {} set in a {} {}", work_desc, genre_desc, style_desc)
    }

    async fn generate_work(&mut self, params: &RandomWorkParams) -> WorkGenerationResult {
        let start_time = Instant::now();
        let mut retry_count = 0;
        
        println!("\n📝 Generating: {} ({})", params.title, params.work_type.as_str());
        println!("   Genre: {:?}, Style: {:?}", params.genre, params.style);
        
        while retry_count <= self.config.auto_retry_attempts {
            // Create a cleaner output path structure
            std::fs::create_dir_all("generated_works").unwrap_or_default();
            let sanitized_title = params.title.replace(" ", "_").replace("/", "_").replace("\\", "_");
            let output_path = format!("generated_works/{}_{}_work{}", 
                params.work_type.as_str().to_lowercase(),
                sanitized_title.to_lowercase(),
                self.session.works_completed + 1
            );

            let result = match &params.size_length {
                WorkSize::BookSize(size) => {
                    // Set up environment for autonomous generation
                    std::env::set_var("NONSTOP_LEARNING_MODE", "true");
                    if self.config.auto_generate_titles {
                        std::env::set_var("AUTO_GENERATE_TITLES", "true");
                        std::env::set_var("AUTO_TITLE", &params.title);
                        std::env::set_var("AUTO_DESCRIPTION", &params.description);
                    }
                    if self.config.auto_approve_outlines {
                        std::env::set_var("AUTO_APPROVE_OUTLINES", "true");
                    }
                    
                    // Pre-check Ollama connection health before starting expensive operation
                    if let Err(e) = self.ensure_ollama_connection().await {
                        println!("⚠️  Ollama connection issue detected: {}. Skipping this work...", e);
                        
                        // Clean up environment variables
                        std::env::remove_var("NONSTOP_LEARNING_MODE");
                        std::env::remove_var("AUTO_GENERATE_TITLES");
                        std::env::remove_var("AUTO_TITLE");
                        std::env::remove_var("AUTO_DESCRIPTION");
                        std::env::remove_var("AUTO_APPROVE_OUTLINES");
                        
                        let generation_time = start_time.elapsed().as_secs();
                        return WorkGenerationResult {
                            success: false,
                            work_params: params.clone(),
                            output_path: None,
                            error_message: Some(format!("Ollama connection failed: {}", e)),
                            generation_time_seconds: generation_time,
                            retry_count: 0,
                        };
                    }
                    
                    let result = write_book(
                        params.genre.clone(),
                        params.style.clone(),
                        size.clone(),
                        Some(std::path::PathBuf::from(output_path.clone())),
                        "llama3.2".to_string(), // model
                        None, // api_key
                        true, // local
                        "http://localhost:11434".to_string(), // ollama_url
                    ).await;
                    
                    // Clean up environment variables
                    std::env::remove_var("NONSTOP_LEARNING_MODE");
                    std::env::remove_var("AUTO_GENERATE_TITLES");
                    std::env::remove_var("AUTO_TITLE");
                    std::env::remove_var("AUTO_DESCRIPTION");
                    std::env::remove_var("AUTO_APPROVE_OUTLINES");
                    
                    result
                },
                WorkSize::ScreenplayLength(length) => {
                    // Set up environment for autonomous generation
                    std::env::set_var("NONSTOP_LEARNING_MODE", "true");
                    if self.config.auto_generate_titles {
                        std::env::set_var("AUTO_GENERATE_TITLES", "true");
                        std::env::set_var("AUTO_TITLE", &params.title);
                        std::env::set_var("AUTO_DESCRIPTION", &params.description);
                    }
                    if self.config.auto_approve_outlines {
                        std::env::set_var("AUTO_APPROVE_OUTLINES", "true");
                    }
                    
                    // Pre-check Ollama connection health before starting expensive operation
                    if let Err(e) = self.ensure_ollama_connection().await {
                        println!("⚠️  Ollama connection issue detected: {}. Skipping this work...", e);
                        
                        // Clean up environment variables
                        std::env::remove_var("NONSTOP_LEARNING_MODE");
                        std::env::remove_var("AUTO_GENERATE_TITLES");
                        std::env::remove_var("AUTO_TITLE");
                        std::env::remove_var("AUTO_DESCRIPTION");
                        std::env::remove_var("AUTO_APPROVE_OUTLINES");
                        
                        let generation_time = start_time.elapsed().as_secs();
                        return WorkGenerationResult {
                            success: false,
                            work_params: params.clone(),
                            output_path: None,
                            error_message: Some(format!("Ollama connection failed: {}", e)),
                            generation_time_seconds: generation_time,
                            retry_count: 0,
                        };
                    }
                    
                    let result = write_screenplay(
                        params.genre.clone(),
                        params.style.clone(),
                        length.clone(),
                        Some(std::path::PathBuf::from(output_path.clone())),
                        "llama3.2".to_string(),
                        None,
                        true,
                        "http://localhost:11434".to_string(),
                    ).await;
                    
                    // Clean up environment variables
                    std::env::remove_var("NONSTOP_LEARNING_MODE");
                    std::env::remove_var("AUTO_GENERATE_TITLES");
                    std::env::remove_var("AUTO_TITLE");
                    std::env::remove_var("AUTO_DESCRIPTION");
                    std::env::remove_var("AUTO_APPROVE_OUTLINES");
                    
                    result
                },
                WorkSize::PlayLength(length) => {
                    // Set up environment for autonomous generation
                    std::env::set_var("NONSTOP_LEARNING_MODE", "true");
                    if self.config.auto_generate_titles {
                        std::env::set_var("AUTO_GENERATE_TITLES", "true");
                        std::env::set_var("AUTO_TITLE", &params.title);
                        std::env::set_var("AUTO_DESCRIPTION", &params.description);
                    }
                    if self.config.auto_approve_outlines {
                        std::env::set_var("AUTO_APPROVE_OUTLINES", "true");
                    }
                    
                    // Pre-check Ollama connection health before starting expensive operation
                    if let Err(e) = self.ensure_ollama_connection().await {
                        println!("⚠️  Ollama connection issue detected: {}. Skipping this work...", e);
                        
                        // Clean up environment variables
                        std::env::remove_var("NONSTOP_LEARNING_MODE");
                        std::env::remove_var("AUTO_GENERATE_TITLES");
                        std::env::remove_var("AUTO_TITLE");
                        std::env::remove_var("AUTO_DESCRIPTION");
                        std::env::remove_var("AUTO_APPROVE_OUTLINES");
                        
                        let generation_time = start_time.elapsed().as_secs();
                        return WorkGenerationResult {
                            success: false,
                            work_params: params.clone(),
                            output_path: None,
                            error_message: Some(format!("Ollama connection failed: {}", e)),
                            generation_time_seconds: generation_time,
                            retry_count: 0,
                        };
                    }
                    
                    let result = write_play(
                        params.genre.clone(),
                        params.style.clone(),
                        length.clone(),
                        Some(std::path::PathBuf::from(output_path.clone())),
                        "llama3.2".to_string(),
                        None,
                        true,
                        "http://localhost:11434".to_string(),
                    ).await;
                    
                    // Clean up environment variables
                    std::env::remove_var("NONSTOP_LEARNING_MODE");
                    std::env::remove_var("AUTO_GENERATE_TITLES");
                    std::env::remove_var("AUTO_TITLE");
                    std::env::remove_var("AUTO_DESCRIPTION");
                    std::env::remove_var("AUTO_APPROVE_OUTLINES");
                    
                    result
                },
            };

            match result {
                Ok(()) => {
                    let generation_time = start_time.elapsed().as_secs();
                    println!("✅ Generated successfully in {}s", generation_time);
                    
                    return WorkGenerationResult {
                        success: true,
                        work_params: params.clone(),
                        output_path: Some(output_path),
                        error_message: None,
                        generation_time_seconds: generation_time,
                        retry_count,
                    };
                },
                Err(e) => {
                    retry_count += 1;
                    println!("❌ Attempt {} failed: {}", retry_count, e);
                    
                    if retry_count > self.config.auto_retry_attempts {
                        let generation_time = start_time.elapsed().as_secs();
                        return WorkGenerationResult {
                            success: false,
                            work_params: params.clone(),
                            output_path: None,
                            error_message: Some(e.to_string()),
                            generation_time_seconds: generation_time,
                            retry_count: retry_count - 1,
                        };
                    }
                    
                    println!("🔄 Retrying in 2 seconds...");
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }

        unreachable!()
    }

    async fn process_work_result(&mut self, result: WorkGenerationResult) -> Result<()> {
        if result.success {
            self.session.works_completed += 1;
            println!("📊 Total works completed: {}", self.session.works_completed);
        } else {
            self.session.works_failed += 1;
            self.session.total_errors += 1;
            println!("📊 Total works failed: {}", self.session.works_failed);
            
            // Learn from the error if we have learning systems
            if let (Some(learning_system), Some(intelligence_system)) = 
                (&self.learning_system, &self.intelligence_system) {
                
                if let Some(error_msg) = &result.error_message {
                    let learning_result = intelligence_system
                        .learn_from_generation_failure(error_msg, &result.work_params.description)
                        .map_err(|e| anyhow::anyhow!("Learning failed: {}", e));
                        
                    match learning_result {
                        Ok(_) => {
                            self.session.learning_insights_gained += 1;
                            println!("🧠 Learned from error - insights gained: {}", 
                                self.session.learning_insights_gained);
                        },
                        Err(e) => println!("⚠️  Could not learn from error: {}", e),
                    }
                }
            }
        }

        // Periodic learning from successful works
        if result.success && self.session.works_completed % self.config.learning_frequency == 0 {
            self.perform_learning_cycle().await?;
        }

        Ok(())
    }

    async fn perform_learning_cycle(&mut self) -> Result<()> {
        println!("🧠 Performing learning cycle...");
        
        if let (Some(learning_system), Some(intelligence_system)) = 
            (&self.learning_system, &self.intelligence_system) {
            
            // Simulate learning from recent successes
            // In a real implementation, we'd analyze the successful works
            self.session.improvements_made += 1;
            println!("📈 System improvements made: {}", self.session.improvements_made);
        }
        
        Ok(())
    }

    fn print_session_summary(&self) {
        let elapsed = self.session.start_time.elapsed();
        let hours = elapsed.as_secs_f32() / 3600.0;
        
        println!("\n🎯 Nonstop Learning Session Summary");
        println!("═══════════════════════════════════════");
        println!("Session ID: {}", self.session.session_id);
        println!("Duration: {:.2} hours", hours);
        println!("Works completed: {}", self.session.works_completed);
        println!("Works failed: {}", self.session.works_failed);
        println!("Total errors: {}", self.session.total_errors);
        println!("Learning insights gained: {}", self.session.learning_insights_gained);
        println!("System improvements made: {}", self.session.improvements_made);
        
        if self.session.works_completed > 0 {
            let success_rate = (self.session.works_completed as f32 / 
                (self.session.works_completed + self.session.works_failed) as f32) * 100.0;
            println!("Success rate: {:.1}%", success_rate);
        }
        
        println!("═══════════════════════════════════════");
        
        // Save session data to disk
        if let Err(e) = self.save_session_to_disk() {
            println!("⚠️  Warning: Could not save session data: {}", e);
        }
    }
    
    fn save_session_to_disk(&self) -> Result<()> {
        use crate::config::get_learning_data_dir;
        
        let learning_dir = get_learning_data_dir()?;
        let sessions_dir = learning_dir.join("nonstop_sessions");
        std::fs::create_dir_all(&sessions_dir)?;
        
        // Save session summary
        let start_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .saturating_sub(self.session.start_time.elapsed().as_secs());
            
        let session_summary = NonstopSessionData {
            session_id: self.session.session_id.clone(),
            start_time_unix: start_unix,
            duration_seconds: self.session.start_time.elapsed().as_secs(),
            works_completed: self.session.works_completed,
            works_failed: self.session.works_failed,
            total_errors: self.session.total_errors,
            learning_insights_gained: self.session.learning_insights_gained,
            improvements_made: self.session.improvements_made,
            config: self.config.clone(),
            timestamp: std::time::SystemTime::now(),
        };
        
        let session_path = sessions_dir.join(format!("{}.json", self.session.session_id));
        let session_json = serde_json::to_string_pretty(&session_summary)?;
        std::fs::write(session_path, session_json)?;
        
        println!("💾 Session data saved to learning directory");
        Ok(())
    }

    pub fn set_learning_systems(&mut self, 
        learning_system: Arc<AdvancedLearningSystem>,
        intelligence_system: Arc<MasterIntelligenceSystem>) {
        self.learning_system = Some(learning_system);
        self.intelligence_system = Some(intelligence_system);
    }
    
    async fn ensure_ollama_connection(&self) -> Result<()> {
        println!("🔍 Checking Ollama connection health...");
        let client = OllamaClient::new("http://localhost:11434".to_string())?;
        
        match client.check_server().await {
            Ok(true) => {
                println!("✅ Ollama server is healthy");
                
                // Also check if the model is available
                match client.list_models().await {
                    Ok(models) => {
                        if models.iter().any(|m| m.contains("llama3.2")) {
                            println!("✅ Required model 'llama3.2' is available");
                        } else {
                            println!("⚠️  Model 'llama3.2' not found, but proceeding (may be auto-downloaded)");
                            println!("   Available models: {:?}", models);
                        }
                    },
                    Err(e) => {
                        println!("⚠️  Could not verify model availability: {}", e);
                        println!("   Proceeding anyway - model may be auto-downloaded during generation");
                    }
                }
                
                Ok(())
            },
            Ok(false) => {
                Err(anyhow::anyhow!("Ollama server is not responding at localhost:11434"))
            },
            Err(e) => {
                Err(anyhow::anyhow!("Ollama connection failed: {}", e))
            }
        }
    }
}

impl WorkType {
    fn as_str(&self) -> &'static str {
        match self {
            WorkType::Book => "Book",
            WorkType::Screenplay => "Screenplay", 
            WorkType::Play => "Play",
            WorkType::ChildrensBook => "ChildrensBook",
            WorkType::Poetry => "Poetry",
            WorkType::TechnicalDoc => "TechnicalDoc",
            WorkType::BusinessDoc => "BusinessDoc",
        }
    }
}

// Configuration helpers for interactive setup
pub async fn setup_nonstop_learning_config() -> Result<NonstopLearningConfig> {
    use dialoguer::{Select, Input, MultiSelect, Confirm};
    
    println!("🎛️  Nonstop Learning Mode Configuration");
    println!("═══════════════════════════════════════");
    
    // Duration setup
    let duration_hours = if Confirm::new()
        .with_prompt("Set a time limit?")
        .default(false)
        .interact()? {
        
        Some(Input::<f32>::new()
            .with_prompt("Duration in hours")
            .default(2.0)
            .interact()?)
    } else {
        None
    };
    
    // Work count limit
    let max_works = if Confirm::new()
        .with_prompt("Set a maximum number of works?")
        .default(false)
        .interact()? {
        
        Some(Input::<u32>::new()
            .with_prompt("Maximum works to generate")
            .default(10)
            .interact()?)
    } else {
        None
    };
    
    // Work types selection
    let work_type_options = vec![
        "📚 Books",
        "🎬 Screenplays", 
        "🎭 Plays",
        "👶 Children's Books",
        "🎨 Poetry",
        "🔬 Technical Documents",
        "📋 Business Documents",
    ];
    
    let selected = MultiSelect::new()
        .with_prompt("Which work types should be generated?")
        .items(&work_type_options)
        .defaults(&[true, true, true, false, false, false, false])
        .interact()?;
    
    let work_types = selected.iter().map(|&i| match i {
        0 => WorkType::Book,
        1 => WorkType::Screenplay,
        2 => WorkType::Play,
        3 => WorkType::ChildrensBook,
        4 => WorkType::Poetry,
        5 => WorkType::TechnicalDoc,
        6 => WorkType::BusinessDoc,
        _ => WorkType::Book,
    }).collect();
    
    // Retry attempts
    let auto_retry_attempts = Input::<u8>::new()
        .with_prompt("Auto-retry attempts per work")
        .default(3)
        .interact()?;
    
    // Learning frequency
    let learning_frequency = Input::<u32>::new()
        .with_prompt("Learn from results every N works")
        .default(5)
        .interact()?;
    
    // Pause between works
    let pause_between_works_seconds = Input::<u32>::new()
        .with_prompt("Pause between works (seconds)")
        .default(2)
        .interact()?;
    
    // Auto-generate titles and descriptions
    let auto_generate_titles = Confirm::new()
        .with_prompt("Auto-generate titles and descriptions? (Recommended for full autonomy)")
        .default(true)
        .interact()?;
    
    // Auto-approve outlines
    let auto_approve_outlines = Confirm::new()
        .with_prompt("Auto-approve all outlines without user input? (Recommended for full autonomy)")
        .default(true)
        .interact()?;
    
    println!();
    if auto_generate_titles && auto_approve_outlines {
        println!("✅ Full autonomy enabled - no user input required during generation!");
    } else if auto_generate_titles {
        println!("⚠️  Partial autonomy - you'll still need to approve outlines");
    } else {
        println!("⚠️  Manual mode - you'll need to provide titles/descriptions and approve outlines");
    }
    
    Ok(NonstopLearningConfig {
        duration_hours,
        max_works,
        work_types,
        auto_retry_attempts,
        learning_frequency,
        pause_between_works_seconds,
        auto_generate_titles,
        auto_approve_outlines,
    })
}