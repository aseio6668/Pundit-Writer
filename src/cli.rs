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
    
    #[command(about = "Continue an existing document or series")]
    Continue {
        #[arg(short, long, help = "Path to existing document(s) to continue from")]
        source: Vec<PathBuf>,
        
        #[arg(short, long, help = "Type of content to generate")]
        content_type: Option<String>,
        
        #[arg(short, long, help = "Brief description of what to continue with")]
        premise: String,
        
        #[arg(short, long, help = "Target length for the new content")]
        target_length: Option<String>,
        
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
    
    #[command(about = "Write technical documentation")]
    TechnicalDoc {
        #[arg(short, long, help = "Type of technical document")]
        doc_type: TechnicalDocType,
        
        #[arg(short, long, help = "Target audience")]
        audience: String,
        
        #[arg(short, long, help = "Subject/topic of the documentation")]
        subject: String,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write white papers and research reports")]
    ResearchDoc {
        #[arg(short, long, help = "Type of research document")]
        doc_type: ResearchDocType,
        
        #[arg(short, long, help = "Research topic")]
        topic: String,
        
        #[arg(short, long, help = "Target length")]
        length: ResearchLength,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write poetry")]
    Poetry {
        #[arg(short, long, help = "Poetry style")]
        style: PoetryStyle,
        
        #[arg(short, long, help = "Theme or subject")]
        theme: String,
        
        #[arg(short, long, help = "Number of poems")]
        count: Option<usize>,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write interactive fiction")]
    InteractiveFiction {
        #[arg(short, long, help = "Fiction genre")]
        genre: Genre,
        
        #[arg(short, long, help = "Story premise")]
        premise: String,
        
        #[arg(short, long, help = "Number of chapters")]
        chapters: Option<usize>,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write personal journals and memoirs")]
    PersonalWriting {
        #[arg(short, long, help = "Type of personal writing")]
        writing_type: PersonalWritingType,
        
        #[arg(short, long, help = "Subject or time period")]
        subject: String,
        
        #[arg(short, long, help = "Target length")]
        length: PersonalLength,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write marketing content")]
    Marketing {
        #[arg(short, long, help = "Type of marketing content")]
        marketing_type: MarketingType,
        
        #[arg(short, long, help = "Product or service")]
        product: String,
        
        #[arg(short, long, help = "Target audience")]
        audience: String,
        
        #[arg(short, long, help = "Content length")]
        length: Option<MarketingLength>,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write blog posts and SEO articles")]
    BlogContent {
        #[arg(short, long, help = "Type of blog content")]
        content_type: BlogContentType,
        
        #[arg(short, long, help = "Topic or subject")]
        topic: String,
        
        #[arg(short, long, help = "Target keywords (for SEO)")]
        keywords: Option<String>,
        
        #[arg(short, long, help = "Target audience")]
        audience: Option<String>,
        
        #[arg(short, long, help = "Content length")]
        length: Option<BlogLength>,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write strategic plans and business documents")]
    StrategicDoc {
        #[arg(short, long, help = "Type of strategic document")]
        doc_type: StrategicDocType,
        
        #[arg(short, long, help = "Organization or project name")]
        organization: String,
        
        #[arg(short, long, help = "Time horizon (e.g., '1 year', '5 years')")]
        timeframe: String,
        
        #[arg(short, long, help = "Document length")]
        length: Option<StrategicLength>,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
    
    #[command(about = "Write meeting notes and summaries")]
    MeetingDoc {
        #[arg(short, long, help = "Type of meeting document")]
        doc_type: MeetingDocType,
        
        #[arg(short, long, help = "Meeting purpose or agenda")]
        purpose: String,
        
        #[arg(short, long, help = "Number of attendees")]
        attendees: Option<u32>,
        
        #[arg(short, long, help = "Meeting duration (e.g., '1 hour', '2 hours')")]
        duration: Option<String>,
        
        #[arg(short, long, help = "Document length")]
        length: Option<MeetingLength>,
        
        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Model to use", default_value = "gpt2")]
        model: String,
        
        #[arg(short = 'k', long, help = "Hugging Face API key")]
        api_key: Option<String>,
        
        #[arg(long, help = "Use local Ollama instead of HuggingFace API")]
        local: bool,
        
        #[arg(long, help = "Ollama server URL", default_value = "http://localhost:11434")]
        ollama_url: String,
    },
}

#[derive(Clone, ValueEnum, Debug)]
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
    
    #[value(name = "epic")]
    Epic,           // ~200,000-300,000 words (300-400 pages)
    
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
            BookSize::Epic => Some(250000),        // 300-400 pages
            BookSize::Unlimited => None,
        }
    }
    
    pub fn chapter_target(&self) -> usize {
        match self {
            BookSize::ShortStory => 1,              // 5,000 words/chapter
            BookSize::Short => 12,                  // ~2,900 words/chapter
            BookSize::Medium => 20,                 // ~3,250 words/chapter  
            BookSize::Large => 30,                  // ~3,300 words/chapter
            BookSize::VeryLarge => 45,              // ~3,500 words/chapter
            BookSize::Epic => 70,                   // ~3,500 words/chapter
            BookSize::Unlimited => 50,              // Initial target, can expand
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

#[derive(Clone, ValueEnum, Debug)]
pub enum TechnicalDocType {
    Manual,           // User manual
    #[value(name = "api-docs")]
    ApiDocs,          // API documentation
    #[value(name = "install-guide")]
    InstallGuide,     // Installation guide
    Tutorial,         // Tutorial/how-to
    #[value(name = "troubleshooting")]
    Troubleshooting,  // Troubleshooting guide
    #[value(name = "admin-guide")]
    AdminGuide,       // Administrator guide
}

#[derive(Clone, ValueEnum, Debug)]
pub enum ResearchDocType {
    #[value(name = "white-paper")]
    WhitePaper,       // White paper
    #[value(name = "research-report")]
    ResearchReport,   // Research report
    #[value(name = "case-study")]
    CaseStudy,        // Case study
    Analysis,         // Analysis document
    Survey,           // Survey report
    #[value(name = "feasibility-study")]
    FeasibilityStudy, // Feasibility study
}

#[derive(Clone, ValueEnum, Debug)]
pub enum ResearchLength {
    Brief,           // 5-15 pages
    Standard,        // 15-40 pages
    Comprehensive,   // 40-80 pages
    Extensive,       // 80+ pages
}

#[derive(Clone, ValueEnum, Debug)]
pub enum PoetryStyle {
    Sonnet,          // 14-line sonnet
    Haiku,           // Traditional 3-line haiku
    #[value(name = "free-verse")]
    FreeVerse,       // Free verse poetry
    Ballad,          // Narrative ballad
    Limerick,        // Humorous 5-line limerick
    Epic,            // Epic poetry
    Lyric,           // Lyrical poetry
    Acrostic,        // Acrostic poetry
}

#[derive(Clone, ValueEnum, Debug)]
pub enum PersonalWritingType {
    Journal,         // Personal journal
    Memoir,          // Memoir/autobiography
    Diary,           // Diary entries
    #[value(name = "travel-journal")]
    TravelJournal,   // Travel journal
    #[value(name = "creative-journal")]
    CreativeJournal, // Creative writing journal
    Reflection,      // Reflective writing
}

#[derive(Clone, ValueEnum)]
pub enum PersonalLength {
    Short,           // 5-10 entries/chapters
    Medium,          // 10-20 entries/chapters
    Long,            // 20-40 entries/chapters
    Extensive,       // 40+ entries/chapters
}

#[derive(Clone, ValueEnum, Debug)]
pub enum MarketingType {
    #[value(name = "social-ad")]
    SocialAd,        // Social media advertisement
    #[value(name = "display-ad")]
    DisplayAd,       // Display/banner advertisement
    #[value(name = "video-script")]
    VideoScript,     // Video advertisement script
    #[value(name = "press-release")]
    PressRelease,    // Press release
    #[value(name = "media-kit")]
    MediaKit,        // Media kit/press kit
    #[value(name = "product-description")]
    ProductDescription, // Product description
    #[value(name = "landing-page")]
    LandingPage,     // Landing page copy
    #[value(name = "email-campaign")]
    EmailCampaign,   // Email marketing campaign
    #[value(name = "brochure")]
    Brochure,        // Marketing brochure
}

#[derive(Clone, ValueEnum)]
pub enum MarketingLength {
    Brief,           // Short form (1-2 sections)
    Standard,        // Standard length (3-5 sections)
    Comprehensive,   // Comprehensive (6-10 sections)
}

#[derive(Clone, ValueEnum, Debug)]
pub enum BlogContentType {
    #[value(name = "blog-post")]
    BlogPost,        // General blog post
    #[value(name = "seo-article")]
    SeoArticle,      // SEO-optimized article
    #[value(name = "tutorial")]
    Tutorial,        // How-to tutorial
    #[value(name = "listicle")]
    Listicle,        // List-based article
    #[value(name = "review")]
    Review,          // Product/service review
    #[value(name = "news-article")]
    NewsArticle,     // News article
    #[value(name = "opinion")]
    Opinion,         // Opinion piece
    #[value(name = "interview")]
    Interview,       // Interview format
    #[value(name = "case-study")]
    CaseStudy,       // Case study article
}

#[derive(Clone, ValueEnum)]
pub enum BlogLength {
    Short,           // 500-800 words (2-3 sections)
    Medium,          // 800-1500 words (4-6 sections)
    Long,            // 1500-2500 words (7-10 sections)
    Epic,            // 2500+ words (10+ sections)
}

#[derive(Clone, ValueEnum)]
pub enum StrategicDocType {
    #[value(name = "strategic-plan")]
    StrategicPlan,   // Strategic planning document
    #[value(name = "business-plan")]
    BusinessPlan,    // Business plan
    #[value(name = "project-plan")]
    ProjectPlan,     // Project planning document
    #[value(name = "roadmap")]
    Roadmap,         // Product or business roadmap
    #[value(name = "vision-doc")]
    VisionDoc,       // Vision and mission document
    #[value(name = "governance")]
    Governance,      // Governance and policy document
    #[value(name = "risk-assessment")]
    RiskAssessment,  // Risk assessment document
    #[value(name = "budget-plan")]
    BudgetPlan,      // Budget planning document
}

#[derive(Clone, ValueEnum, Debug)]
pub enum StrategicLength {
    Brief,           // 3-8 pages (executive summary style)
    Standard,        // 8-20 pages (comprehensive)
    Comprehensive,   // 20-50 pages (detailed analysis)
    Extensive,       // 50+ pages (full strategic document)
}

#[derive(Clone, ValueEnum)]
pub enum MeetingDocType {
    #[value(name = "meeting-notes")]
    MeetingNotes,    // Meeting notes/minutes
    #[value(name = "action-items")]
    ActionItems,     // Action items and follow-ups
    #[value(name = "summary")]
    Summary,         // Meeting summary
    #[value(name = "transcript")]
    Transcript,      // Meeting transcript style
    #[value(name = "decision-log")]
    DecisionLog,     // Decision tracking document
    #[value(name = "agenda")]
    Agenda,          // Meeting agenda
}

#[derive(Clone, ValueEnum, Debug)]
pub enum MeetingLength {
    Brief,           // 1-2 pages (short meeting)
    Standard,        // 2-5 pages (standard meeting)
    Detailed,        // 5-10 pages (detailed meeting)
    Comprehensive,   // 10+ pages (all-day or multi-day)
}