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
    
    #[command(about = "Write a movie screenplay")]
    Screenplay {
        #[arg(short, long, help = "Genre of the screenplay")]
        genre: Genre,
        
        #[arg(short, long, help = "Writing style")]
        style: WritingStyle,
        
        #[arg(short = 'z', long, help = "Screenplay length", default_value = "feature")]
        length: ScreenplayLength,
        
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
    
    #[command(about = "Write a stage play script")]
    Play {
        #[arg(short, long, help = "Genre of the play")]
        genre: Genre,
        
        #[arg(short, long, help = "Writing style")]
        style: WritingStyle,
        
        #[arg(short = 'z', long, help = "Play length", default_value = "full")]
        length: PlayLength,
        
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
    
    #[command(about = "Write TV show scripts")]
    TvScript {
        #[arg(short, long, help = "Type of TV show")]
        show_type: TvShowType,
        
        #[arg(short, long, help = "Genre of the show")]
        genre: Genre,
        
        #[arg(short, long, help = "Writing style")]
        style: WritingStyle,
        
        #[arg(short = 'e', long, help = "Number of episodes", default_value = "1")]
        episodes: u32,
        
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
    
    #[command(about = "Write audio drama or podcast script")]
    AudioScript {
        #[arg(short, long, help = "Type of audio content")]
        audio_type: AudioType,
        
        #[arg(short, long, help = "Genre")]
        genre: Genre,
        
        #[arg(short, long, help = "Writing style")]
        style: WritingStyle,
        
        #[arg(short = 'd', long, help = "Target duration in minutes", default_value = "30")]
        duration: u32,
        
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
    
    #[command(about = "Write video game script with branching dialogue")]
    GameScript {
        #[arg(short, long, help = "Game genre")]
        genre: GameGenre,
        
        #[arg(short, long, help = "Writing style")]
        style: WritingStyle,
        
        #[arg(short = 'c', long, help = "Number of character interactions", default_value = "5")]
        characters: u32,
        
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
    
    #[command(about = "Write business or technical documents")]
    Document {
        #[arg(short, long, help = "Type of document")]
        doc_type: DocumentType,
        
        #[arg(short, long, help = "Writing style")]
        style: WritingStyle,
        
        #[arg(short, long, help = "Target length")]
        length: DocumentLength,
        
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
    
    #[command(about = "Start interactive mode to choose content type")]
    Interactive,
}

#[derive(Clone, ValueEnum)]
pub enum Genre {
    Fiction,
    NonFiction,
    Mystery,
    Romance,
    SciFi,
    Fantasy,
    Horror,
    Thriller,
    Biography,
    History,
    SelfHelp,
    Technical,
    Poetry,
    Drama,
    Comedy,
    Adventure,
    Crime,
    Dystopian,
    Historical,
    Memoir,
    Philosophy,
    Science,
    Travel,
    #[value(name = "young-adult")]
    YoungAdult,
    Children,
}

#[derive(Clone, ValueEnum)]
pub enum WritingStyle {
    Formal,
    Casual,
    Academic,
    Creative,
    Journalistic,
    Poetic,
    Conversational,
    Descriptive,
    Narrative,
    Persuasive,
    Expository,
    Technical,
    Humorous,
    Dramatic,
    Minimalist,
    Verbose,
    #[value(name = "stream-of-consciousness")]
    StreamOfConsciousness,
    Epistolary,
    #[value(name = "first-person")]
    FirstPerson,
    #[value(name = "third-person")]
    ThirdPerson,
    Omniscient,
}

#[derive(Clone, ValueEnum, Debug, PartialEq)]
pub enum BookSize {
    #[value(name = "short-story")]
    ShortStory,     // ~1,000-7,500 words
    
    #[value(name = "short")]
    Short,          // ~20,000-50,000 words
    
    #[value(name = "medium")]
    Medium,         // ~50,000-80,000 words
    
    #[value(name = "large")]
    Large,          // ~80,000-120,000 words
    
    #[value(name = "very-large")]
    VeryLarge,      // ~120,000-200,000 words
    
    #[value(name = "unlimited")]
    Unlimited,      // No limit, writes until natural conclusion
}

impl BookSize {
    pub fn word_target(&self) -> Option<usize> {
        match self {
            BookSize::ShortStory => Some(5000),
            BookSize::Short => Some(35000),
            BookSize::Medium => Some(65000),
            BookSize::Large => Some(100000),
            BookSize::VeryLarge => Some(160000),
            BookSize::Unlimited => None,
        }
    }
    
    pub fn chapter_target(&self) -> usize {
        match self {
            BookSize::ShortStory => 1,
            BookSize::Short => 8,
            BookSize::Medium => 15,
            BookSize::Large => 25,
            BookSize::VeryLarge => 40,
            BookSize::Unlimited => 50, // Initial target, can expand
        }
    }
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
            Genre::SelfHelp => write!(f, "Self-Help"),
            Genre::Technical => write!(f, "Technical"),
            Genre::Poetry => write!(f, "Poetry"),
            Genre::Drama => write!(f, "Drama"),
            Genre::Comedy => write!(f, "Comedy"),
            Genre::Adventure => write!(f, "Adventure"),
            Genre::Crime => write!(f, "Crime"),
            Genre::Dystopian => write!(f, "Dystopian"),
            Genre::Historical => write!(f, "Historical Fiction"),
            Genre::Memoir => write!(f, "Memoir"),
            Genre::Philosophy => write!(f, "Philosophy"),
            Genre::Science => write!(f, "Science"),
            Genre::Travel => write!(f, "Travel"),
            Genre::YoungAdult => write!(f, "Young Adult"),
            Genre::Children => write!(f, "Children's"),
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum ScreenplayLength {
    #[value(name = "short")]
    Short,        // 5-30 pages (short film)
    
    #[value(name = "feature")]
    Feature,      // 90-120 pages (feature film)
    
    #[value(name = "epic")]
    Epic,         // 120+ pages (epic film)
}

#[derive(Clone, ValueEnum)]
pub enum PlayLength {
    #[value(name = "one-act")]
    OneAct,       // 10-30 minutes
    
    #[value(name = "full")]
    Full,         // 90-120 minutes
    
    #[value(name = "musical")]
    Musical,      // 2-3 hours with songs
}

#[derive(Clone, ValueEnum)]
pub enum TvShowType {
    Sitcom,       // 22-30 minutes
    Drama,        // 42-60 minutes
    #[value(name = "mini-series")]
    MiniSeries,   // Limited series
    Anthology,    // Standalone episodes
}

#[derive(Clone, ValueEnum)]
pub enum AudioType {
    #[value(name = "audio-drama")]
    AudioDrama,   // Narrative audio drama
    Podcast,      // Interview/discussion format
    #[value(name = "radio-play")]
    RadioPlay,    // Traditional radio drama
    Documentary,  // Documentary format
}

#[derive(Clone, ValueEnum)]
pub enum GameGenre {
    RPG,          // Role-playing game
    Adventure,    // Adventure game
    #[value(name = "visual-novel")]
    VisualNovel,  // Visual novel
    Action,       // Action game
    Strategy,     // Strategy game
    Horror,       // Horror game
    #[value(name = "sci-fi")]
    SciFi,        // Science fiction
    Fantasy,      // Fantasy setting
}

#[derive(Clone, ValueEnum)]
pub enum DocumentType {
    #[value(name = "business-plan")]
    BusinessPlan,
    
    #[value(name = "technical-manual")]
    TechnicalManual,
    
    #[value(name = "user-guide")]
    UserGuide,
    
    Report,
    Proposal,
    
    #[value(name = "marketing-copy")]
    MarketingCopy,
    
    #[value(name = "legal-template")]
    LegalTemplate,
    
    #[value(name = "lesson-plan")]
    LessonPlan,
    
    #[value(name = "course-outline")]
    CourseOutline,
}

#[derive(Clone, ValueEnum)]
pub enum DocumentLength {
    Brief,        // 1-5 pages
    Standard,     // 5-20 pages
    Comprehensive, // 20-50 pages
    Extensive,    // 50+ pages
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
            WritingStyle::Narrative => write!(f, "Narrative"),
            WritingStyle::Persuasive => write!(f, "Persuasive"),
            WritingStyle::Expository => write!(f, "Expository"),
            WritingStyle::Technical => write!(f, "Technical"),
            WritingStyle::Humorous => write!(f, "Humorous"),
            WritingStyle::Dramatic => write!(f, "Dramatic"),
            WritingStyle::Minimalist => write!(f, "Minimalist"),
            WritingStyle::Verbose => write!(f, "Verbose"),
            WritingStyle::StreamOfConsciousness => write!(f, "Stream of Consciousness"),
            WritingStyle::Epistolary => write!(f, "Epistolary"),
            WritingStyle::FirstPerson => write!(f, "First Person"),
            WritingStyle::ThirdPerson => write!(f, "Third Person"),
            WritingStyle::Omniscient => write!(f, "Omniscient"),
        }
    }
}