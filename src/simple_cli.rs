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