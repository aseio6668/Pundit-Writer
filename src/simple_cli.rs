use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use crate::cli_types::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "pundit")]
#[command(about = "A cross-platform CLI tool for generating entire books using AI models")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short = 'q', long = "quiet", help = "Quiet mode - suppress AI enhancement messages", global = true)]
    pub quiet: bool,
    
    #[arg(short = 'Q', long = "very-quiet", help = "Very quiet mode - only show critical messages", global = true)]
    pub very_quiet: bool,
    
    #[arg(short = 'l', long = "language", help = "Language for content generation (default: English)", global = true)]
    pub language: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Write a book with specified parameters")]
    Book {
        #[arg(short, long, help = "Genre of the book (e.g., fiction, mystery, romance)")]
        genre: String,
        
        #[arg(short, long, help = "Writing style (e.g., formal, casual, creative)")]
        style: String,
        
        #[arg(short = 'z', long, help = "Book size (short-story, short, medium, large, epic)", default_value = "medium")]
        size: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write a movie screenplay")]
    Screenplay {
        #[arg(short, long, help = "Genre of the screenplay")]
        genre: String,
        
        #[arg(short, long, help = "Writing style")]
        style: String,
        
        #[arg(short = 'z', long, help = "Screenplay length (short, feature, epic)", default_value = "feature")]
        length: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write a stage play script")]
    Play {
        #[arg(short, long, help = "Genre of the play")]
        genre: String,
        
        #[arg(short, long, help = "Writing style")]
        style: String,
        
        #[arg(short = 'z', long, help = "Play length (one-act, full, epic, musical)", default_value = "full")]
        length: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Start interactive mode")]
    Interactive,
    
    #[command(about = "Non-stop writing mode with automatic error recovery")]
    NonStop {
        #[arg(short, long, help = "Genre of the content")]
        genre: String,
        
        #[arg(short, long, help = "Writing style")]
        style: String,
        
        #[arg(short = 'z', long, help = "Content size", default_value = "medium")]
        size: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
        
        #[arg(long, help = "Target number of sections", default_value = "10")]
        sections: usize,
        
        #[arg(long, help = "Buffer size in MB", default_value = "50")]
        buffer_size: usize,
        
        #[arg(long, help = "Enable auto-continue without user input")]
        auto_continue: bool,
    },
    
    #[command(about = "Enhanced intelligent writing with advanced AI systems")]
    Enhanced {
        #[arg(short, long, help = "Genre of the content")]
        genre: String,
        
        #[arg(short, long, help = "Writing style")]
        style: String,
        
        #[arg(short = 'z', long, help = "Content size", default_value = "medium")]
        size: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
        
        #[arg(long, help = "Target number of chapters", default_value = "12")]
        chapters: usize,
        
        #[arg(long, help = "Buffer size in MB", default_value = "100")]
        buffer_size: usize,
        
        #[arg(long, help = "Enable auto-continue without user input")]
        auto_continue: bool,
        
        #[arg(long, help = "Enable Chapter 3 interruption recovery mode")]
        interruption_recovery: bool,
    },
    
    #[command(about = "Write in the style of historical literary masters")]
    Persona {
        #[arg(short, long, help = "Genre of the content")]
        genre: String,
        
        #[arg(short, long, help = "Writing style")]
        style: String,
        
        #[arg(short = 'z', long, help = "Content size", default_value = "medium")]
        size: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
        
        #[arg(long, help = "Target number of chapters", default_value = "8")]
        chapters: usize,
        
        #[arg(long, help = "Historical writer to emulate")]
        writer: Option<String>,
        
        #[arg(long, help = "Literary era to emulate", value_parser = ["classical", "medieval", "renaissance", "enlightenment", "romantic", "victorian", "modernist", "contemporary"])]
        era: Option<String>,
        
        #[arg(long, help = "Enable auto-continue without user input")]
        auto_continue: bool,
        
        #[arg(long, help = "Language enhancement level", value_parser = ["none", "subtle", "moderate", "strong", "authentic"], default_value = "moderate")]
        language_enhancement: String,
    },
    
    #[command(about = "Super-intelligent writing with master AI system")]
    Superintelligent {
        #[arg(short, long, help = "Genre of the content")]
        genre: String,
        
        #[arg(short, long, help = "Writing style")]
        style: String,
        
        #[arg(short = 'z', long, help = "Content size", default_value = "medium")]
        size: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
        
        #[arg(long, help = "Target number of chapters", default_value = "10")]
        chapters: usize,
        
        #[arg(long, help = "Intelligence level (0.1-1.0)", default_value = "0.8")]
        intelligence_level: f32,
        
        #[arg(long, help = "Learning acceleration factor", default_value = "1.5")]
        learning_acceleration: f32,
        
        #[arg(long, help = "Creativity enhancement factor", default_value = "1.3")]
        creativity_enhancement: f32,
        
        #[arg(long, help = "Enable autonomous improvement")]
        autonomous_improvement: bool,
        
        #[arg(long, help = "Enable meta-cognition")]
        meta_cognition: bool,
        
        #[arg(long, help = "Consciousness simulation level (0.1-1.0)", default_value = "0.6")]
        consciousness_level: f32,
        
        #[arg(long, help = "Enable continuous learning")]
        continuous_learning: bool,
    },
    
    #[command(about = "Learning optimization and intelligence enhancement session")]
    LearnOptimize {
        #[arg(long, help = "Focus area for learning")]
        focus: Option<String>,
        
        #[arg(long, help = "Target improvement percentage", default_value = "15")]
        target_improvement: f32,
        
        #[arg(long, help = "Learning session duration in minutes", default_value = "30")]
        duration: u32,
        
        #[arg(long, help = "Enable experimental learning modes")]
        experimental: bool,
        
        #[arg(long, help = "Generate comprehensive learning report")]
        detailed_report: bool,
    },
    
    #[command(about = "Soul Memory - Persistent AI learning system")]
    SoulMemory {
        #[command(subcommand)]
        command: crate::soul_memory_cli::SoulMemoryCommand,
    },
    
    #[command(about = "Emotional Writing - Write with genuine creative consciousness")]
    EmotionalWrite {
        #[arg(short, long, help = "Genre of the work")]
        genre: String,
        
        #[arg(short, long, help = "Writing style")]
        style: String,
        
        #[arg(short, long, help = "Content type (book, chapter, scene)", default_value = "chapter")]
        content_type: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<std::path::PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(long, help = "Project description or theme")]
        theme: Option<String>,
        
        #[arg(long, help = "Enable detailed emotional journey logging")]
        show_journey: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Encyclopedia - Create comprehensive knowledge references")]
    Encyclopedia {
        #[arg(short, long, help = "Main topic or subject area")]
        topic: String,
        
        #[arg(short, long, help = "Scope of coverage (comprehensive, specialized, concise)", default_value = "comprehensive")]
        scope: String,
        
        #[arg(short, long, help = "Number of encyclopedia entries", default_value = "20")]
        entries: usize,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<std::path::PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },

    #[command(about = "Contemplative Writing - Create content with deep reflection and reduced chatter")]
    Contemplative {
        #[arg(short = 't', long = "type", help = "Content type (book, poetry, screenplay, etc.)", default_value = "book")]
        content_type: String,
        
        #[arg(short, long, help = "Genre of the content")]
        genre: String,
        
        #[arg(short, long, help = "Writing style")]
        style: String,
        
        #[arg(long, help = "Content size/length", default_value = "medium")]
        size: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<std::path::PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "llama3.2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
        
        #[arg(long, help = "Contemplation depth level (0.1 to 1.0)", default_value = "0.7")]
        depth: f32,
        
        #[arg(long, help = "Number of sections/chapters", default_value = "5")]
        sections: u32,
        
        #[arg(long, help = "Enable detailed meditation state logging")]
        show_meditation: bool,
    },
}

// String to enum parsing functions
pub fn parse_genre(s: &str) -> Result<Genre> {
    match s.to_lowercase().as_str() {
        "fiction" => Ok(Genre::Fiction),
        "non-fiction" | "nonfiction" => Ok(Genre::NonFiction),
        "mystery" => Ok(Genre::Mystery),
        "romance" => Ok(Genre::Romance),
        "sci-fi" | "scifi" | "science-fiction" => Ok(Genre::SciFi),
        "fantasy" => Ok(Genre::Fantasy),
        "horror" => Ok(Genre::Horror),
        "thriller" => Ok(Genre::Thriller),
        "biography" => Ok(Genre::Biography),
        "history" => Ok(Genre::History),
        "science" => Ok(Genre::Science),
        "technology" | "tech" => Ok(Genre::Technology),
        "business" => Ok(Genre::Business),
        "self-help" | "selfhelp" => Ok(Genre::SelfHelp),
        "adventure" => Ok(Genre::Adventure),
        "comedy" => Ok(Genre::Comedy),
        "drama" => Ok(Genre::Drama),
        "action" => Ok(Genre::Action),
        "western" => Ok(Genre::Western),
        "crime" => Ok(Genre::Crime),
        "psychological" => Ok(Genre::Psychological),
        "supernatural" => Ok(Genre::Supernatural),
        "dystopian" => Ok(Genre::Dystopian),
        "historical" => Ok(Genre::Historical),
        "contemporary" => Ok(Genre::Contemporary),
        "literary" => Ok(Genre::Literary),
        "experimental" => Ok(Genre::Experimental),
        "philosophy" => Ok(Genre::Philosophy),
        "religion" => Ok(Genre::Religion),
        "health" => Ok(Genre::Health),
        "fitness" => Ok(Genre::Fitness),
        "cooking" => Ok(Genre::Cooking),
        "travel" => Ok(Genre::Travel),
        "politics" => Ok(Genre::Politics),
        "economics" => Ok(Genre::Economics),
        "sociology" => Ok(Genre::Sociology),
        "psychology" => Ok(Genre::Psychology),
        "education" => Ok(Genre::Education),
        "parenting" => Ok(Genre::Parenting),
        "relationships" => Ok(Genre::Relationships),
        "career" => Ok(Genre::Career),
        "finance" => Ok(Genre::Finance),
        "environment" => Ok(Genre::Environment),
        "nature" => Ok(Genre::Nature),
        "sports" => Ok(Genre::Sports),
        "art" => Ok(Genre::Art),
        "music" => Ok(Genre::Music),
        "photography" => Ok(Genre::Photography),
        "crafts" => Ok(Genre::Crafts),
        "hobbies" => Ok(Genre::Hobbies),
        "memoir" => Ok(Genre::Memoir),
        "autobiography" => Ok(Genre::Autobiography),
        "reference" => Ok(Genre::Reference),
        "encyclopedia" => Ok(Genre::Encyclopedia),
        "dictionary" => Ok(Genre::Dictionary),
        "guide" => Ok(Genre::Guide),
        "manual" => Ok(Genre::Manual),
        "tutorial" => Ok(Genre::Tutorial),
        "young-adult" | "ya" => Ok(Genre::YoungAdult),
        "children" => Ok(Genre::Children),
        "technical" => Ok(Genre::Technical),
        "poetry" => Ok(Genre::Poetry),
        _ => Err(anyhow!("Unknown genre: {}. Use --help to see available options", s)),
    }
}

pub fn parse_writing_style(s: &str) -> Result<WritingStyle> {
    match s.to_lowercase().as_str() {
        "formal" => Ok(WritingStyle::Formal),
        "casual" => Ok(WritingStyle::Casual),
        "academic" => Ok(WritingStyle::Academic),
        "creative" => Ok(WritingStyle::Creative),
        "journalistic" => Ok(WritingStyle::Journalistic),
        "poetic" => Ok(WritingStyle::Poetic),
        "conversational" => Ok(WritingStyle::Conversational),
        "descriptive" => Ok(WritingStyle::Descriptive),
        "narrative" => Ok(WritingStyle::Narrative),
        "persuasive" => Ok(WritingStyle::Persuasive),
        "expository" => Ok(WritingStyle::Expository),
        "technical" => Ok(WritingStyle::Technical),
        "humorous" => Ok(WritingStyle::Humorous),
        "dramatic" => Ok(WritingStyle::Dramatic),
        "minimalist" => Ok(WritingStyle::Minimalist),
        "verbose" => Ok(WritingStyle::Verbose),
        "stream-of-consciousness" | "stream" => Ok(WritingStyle::StreamOfConsciousness),
        "epistolary" => Ok(WritingStyle::Epistolary),
        "first-person" | "first" => Ok(WritingStyle::FirstPerson),
        "third-person" | "third" => Ok(WritingStyle::ThirdPerson),
        "omniscient" => Ok(WritingStyle::Omniscient),
        _ => Err(anyhow!("Unknown writing style: {}. Use --help to see available options", s)),
    }
}

pub fn parse_book_size(s: &str) -> Result<BookSize> {
    match s.to_lowercase().as_str() {
        "short-story" | "shortstory" => Ok(BookSize::ShortStory),
        "short" => Ok(BookSize::Short),
        "medium" => Ok(BookSize::Medium),
        "large" => Ok(BookSize::Large),
        "very-large" | "verylarge" => Ok(BookSize::VeryLarge),
        "epic" => Ok(BookSize::Epic),
        "unlimited" => Ok(BookSize::Unlimited),
        _ => Err(anyhow!("Unknown book size: {}. Use --help to see available options", s)),
    }
}

pub fn parse_screenplay_length(s: &str) -> Result<ScreenplayLength> {
    match s.to_lowercase().as_str() {
        "short" => Ok(ScreenplayLength::Short),
        "feature" => Ok(ScreenplayLength::Feature),
        "epic" => Ok(ScreenplayLength::Epic),
        _ => Err(anyhow!("Unknown screenplay length: {}. Use --help to see available options", s)),
    }
}

pub fn parse_play_length(s: &str) -> Result<PlayLength> {
    match s.to_lowercase().as_str() {
        "one-act" | "oneact" => Ok(PlayLength::OneAct),
        "full" => Ok(PlayLength::Full),
        "epic" => Ok(PlayLength::Epic),
        "musical" => Ok(PlayLength::Musical),
        _ => Err(anyhow!("Unknown play length: {}. Use --help to see available options", s)),
    }
}