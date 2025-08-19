use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "pundit")]
#[command(about = "A cross-platform CLI tool for generating entire books using Hugging Face models")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Write a book with specified parameters")]
    Book {
        #[arg(short, long, help = "Genre of the book")]
        genre: Genre,
        
        #[arg(short, long, help = "Writing style")]
        style: WritingStyle,
        
        #[arg(short = 'z', long, help = "Book size/length", default_value = "medium")]
        size: BookSize,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use (HuggingFace or Ollama model)", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Interactive mode")]
    Interactive,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Genre {
    Fiction,
    #[value(name = "non-fiction")]
    NonFiction,
    Mystery,
    Romance,
    #[value(name = "sci-fi")]
    SciFi,
    Fantasy,
    Horror,
    Thriller,
    Biography,
    History,
    Science,
    Technology,
    Business,
    #[value(name = "self-help")]
    SelfHelp,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum WritingStyle {
    Formal,
    Casual,
    Academic,
    Creative,
    Journalistic,
    Poetic,
    Conversational,
    Descriptive,
    #[value(name = "first-person")]
    FirstPerson,
    #[value(name = "third-person")]
    ThirdPerson,
    Omniscient,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum BookSize {
    #[value(name = "short-story")]
    ShortStory,    // 5,000-15,000 words (~20-60 pages)
    Short,         // 40,000-60,000 words (~160-240 pages)
    Medium,        // 70,000-90,000 words (~280-360 pages)
    Large,         // 90,000-120,000 words (~360-480 pages)
    #[value(name = "very-large")]
    VeryLarge,     // 120,000-200,000 words (~480-800 pages)
    Epic,          // 200,000-250,000 words (~800-1000 pages)
    Unlimited,     // No limit, keeps generating until user stops
}

impl std::fmt::Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Genre::Fiction => write!(f, "Fiction"),
            Genre::NonFiction => write!(f, "Non-Fiction"),
            Genre::Mystery => write!(f, "Mystery"),
            Genre::Romance => write!(f, "Romance"),
            Genre::SciFi => write!(f, "Science Fiction"),
            Genre::Fantasy => write!(f, "Fantasy"),
            Genre::Horror => write!(f, "Horror"),
            Genre::Thriller => write!(f, "Thriller"),
            Genre::Biography => write!(f, "Biography"),
            Genre::History => write!(f, "History"),
            Genre::Science => write!(f, "Science"),
            Genre::Technology => write!(f, "Technology"),
            Genre::Business => write!(f, "Business"),
            Genre::SelfHelp => write!(f, "Self-Help"),
        }
    }
}

impl std::fmt::Display for WritingStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WritingStyle::Formal => write!(f, "Formal"),
            WritingStyle::Casual => write!(f, "Casual"),
            WritingStyle::Academic => write!(f, "Academic"),
            WritingStyle::Creative => write!(f, "Creative"),
            WritingStyle::Journalistic => write!(f, "Journalistic"),
            WritingStyle::Poetic => write!(f, "Poetic"),
            WritingStyle::Conversational => write!(f, "Conversational"),
            WritingStyle::Descriptive => write!(f, "Descriptive"),
            WritingStyle::FirstPerson => write!(f, "First Person"),
            WritingStyle::ThirdPerson => write!(f, "Third Person"),
            WritingStyle::Omniscient => write!(f, "Omniscient"),
        }
    }
}