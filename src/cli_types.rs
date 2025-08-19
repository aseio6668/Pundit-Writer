// CLI types without clap derives to avoid stack overflow

#[derive(Clone, Debug)]
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
    Science,
    Technology,
    Business,
    SelfHelp,
    Adventure,
    Comedy,
    Drama,
    Action,
    Western,
    Crime,
    Psychological,
    Supernatural,
    Dystopian,
    Historical,
    Contemporary,
    Literary,
    Experimental,
    Philosophy,
    Religion,
    Health,
    Fitness,
    Cooking,
    Travel,
    Politics,
    Economics,
    Sociology,
    Psychology,
    Education,
    Parenting,
    Relationships,
    Career,
    Finance,
    Environment,
    Nature,
    Sports,
    Art,
    Music,
    Photography,
    Crafts,
    Hobbies,
    Memoir,
    Autobiography,
    Reference,
    Encyclopedia,
    Dictionary,
    Guide,
    Manual,
    Tutorial,
    YoungAdult,
    Children,
    Technical,
    Poetry,
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
            Genre::Adventure => write!(f, "Adventure"),
            Genre::Comedy => write!(f, "Comedy"),
            Genre::Drama => write!(f, "Drama"),
            Genre::Action => write!(f, "Action"),
            Genre::Western => write!(f, "Western"),
            Genre::Crime => write!(f, "Crime"),
            Genre::Psychological => write!(f, "Psychological"),
            Genre::Supernatural => write!(f, "Supernatural"),
            Genre::Dystopian => write!(f, "Dystopian"),
            Genre::Historical => write!(f, "Historical"),
            Genre::Contemporary => write!(f, "Contemporary"),
            Genre::Literary => write!(f, "Literary"),
            Genre::Experimental => write!(f, "Experimental"),
            Genre::Philosophy => write!(f, "Philosophy"),
            Genre::Religion => write!(f, "Religion"),
            Genre::Health => write!(f, "Health"),
            Genre::Fitness => write!(f, "Fitness"),
            Genre::Cooking => write!(f, "Cooking"),
            Genre::Travel => write!(f, "Travel"),
            Genre::Politics => write!(f, "Politics"),
            Genre::Economics => write!(f, "Economics"),
            Genre::Sociology => write!(f, "Sociology"),
            Genre::Psychology => write!(f, "Psychology"),
            Genre::Education => write!(f, "Education"),
            Genre::Parenting => write!(f, "Parenting"),
            Genre::Relationships => write!(f, "Relationships"),
            Genre::Career => write!(f, "Career"),
            Genre::Finance => write!(f, "Finance"),
            Genre::Environment => write!(f, "Environment"),
            Genre::Nature => write!(f, "Nature"),
            Genre::Sports => write!(f, "Sports"),
            Genre::Art => write!(f, "Art"),
            Genre::Music => write!(f, "Music"),
            Genre::Photography => write!(f, "Photography"),
            Genre::Crafts => write!(f, "Crafts"),
            Genre::Hobbies => write!(f, "Hobbies"),
            Genre::Memoir => write!(f, "Memoir"),
            Genre::Autobiography => write!(f, "Autobiography"),
            Genre::Reference => write!(f, "Reference"),
            Genre::Encyclopedia => write!(f, "Encyclopedia"),
            Genre::Dictionary => write!(f, "Dictionary"),
            Genre::Guide => write!(f, "Guide"),
            Genre::Manual => write!(f, "Manual"),
            Genre::Tutorial => write!(f, "Tutorial"),
            Genre::YoungAdult => write!(f, "Young Adult"),
            Genre::Children => write!(f, "Children"),
            Genre::Technical => write!(f, "Technical"),
            Genre::Poetry => write!(f, "Poetry"),
        }
    }
}

#[derive(Clone, Debug)]
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
    StreamOfConsciousness,
    Epistolary,
    FirstPerson,
    ThirdPerson,
    Omniscient,
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

#[derive(Clone, Debug, PartialEq)]
pub enum BookSize {
    ShortStory,    // ~1,000-7,500 words
    Short,         // ~20,000-50,000 words
    Medium,        // ~50,000-80,000 words
    Large,         // ~80,000-120,000 words
    VeryLarge,     // ~120,000-200,000 words
    Epic,          // ~200,000-250,000 words
    Unlimited,     // No limit, keeps generating until user stops
}

impl BookSize {
    pub fn word_target(&self) -> Option<usize> {
        match self {
            BookSize::ShortStory => Some(5000),      // Aimed at short story length
            BookSize::Short => Some(35000),          // Short book/novella
            BookSize::Medium => Some(65000),         // Standard novel
            BookSize::Large => Some(100000),         // Large novel
            BookSize::VeryLarge => Some(160000),     // Very large novel
            BookSize::Epic => Some(225000),          // Epic length
            BookSize::Unlimited => None,             // Unlimited has no target
        }
    }

    pub fn chapter_target(&self) -> usize {
        match self {
            BookSize::ShortStory => 3,    // Short stories typically have 3-5 sections
            BookSize::Short => 12,        // Short books ~12 chapters
            BookSize::Medium => 20,       // Standard novels ~20 chapters
            BookSize::Large => 25,        // Large novels ~25 chapters
            BookSize::VeryLarge => 40,    // Very large novels ~40 chapters
            BookSize::Epic => 50,         // Epic novels ~50 chapters
            BookSize::Unlimited => 30,    // Start with reasonable number for unlimited
        }
    }
}

impl std::fmt::Display for BookSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookSize::ShortStory => write!(f, "short story"),
            BookSize::Short => write!(f, "short"),
            BookSize::Medium => write!(f, "medium"),
            BookSize::Large => write!(f, "large"),
            BookSize::VeryLarge => write!(f, "very large"),
            BookSize::Epic => write!(f, "epic"),
            BookSize::Unlimited => write!(f, "unlimited"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ScreenplayLength {
    Short,         // 15-30 pages
    Feature,       // 90-120 pages
    Epic,          // 120+ pages
}

#[derive(Clone, Debug)]
pub enum PlayLength {
    OneAct,        // 10-30 pages
    Full,          // 60-120 pages
    Epic,          // 120+ pages
    Musical,       // Musical theatre length
}

#[derive(Clone, Debug)]
pub enum TvShowType {
    Comedy,        // 22-24 minutes
    Drama,         // 42-45 minutes
    Miniseries,    // Multiple episodes
    Special,       // Variable length
    Sitcom,        // Situation comedy
    MiniSeries,    // Alternative spelling
    Anthology,     // Anthology series
    Reality,       // Reality TV
    Documentary,   // Documentary series
    News,          // News program
    Talk,          // Talk show
    Variety,       // Variety show
}

#[derive(Clone, Debug)]
pub enum AudioType {
    Podcast,       // 15-60 minutes
    RadioDrama,    // 30-60 minutes
    Audiobook,     // Multiple hours
    Commercial,    // 30-60 seconds
    AudioDrama,    // Narrative audio drama
    RadioPlay,     // Traditional radio drama
    Documentary,   // Documentary format
}

#[derive(Clone, Debug)]
pub enum GameGenre {
    Adventure,
    RPG,
    Mystery,
    Horror,
    SciFi,
    Fantasy,
    Historical,
    Contemporary,
    VisualNovel,
    Action,
    Strategy,
}

#[derive(Clone, Debug)]
pub enum DocumentType {
    Business,
    Academic,
    Technical,
    Legal,
    Creative,
    BusinessPlan,
    TechnicalManual,
    UserGuide,
    Report,
    Proposal,
    MarketingCopy,
    LegalTemplate,
    LessonPlan,
    CourseOutline,
}

#[derive(Clone, Debug)]
pub enum DocumentLength {
    Brief,         // 1-5 pages
    Standard,      // 5-20 pages
    Comprehensive, // 20-50 pages
    Extensive,     // 50+ pages
}

// Additional types for all the new content types
#[derive(Clone, Debug)]
pub enum TechnicalDocType {
    Manual,
    ApiDocs,
    InstallGuide,
    Tutorial,
    Troubleshooting,
    AdminGuide,
}

#[derive(Clone, Debug)]
pub enum ResearchDocType {
    WhitePaper,
    ResearchReport,
    CaseStudy,
    Analysis,
    Survey,
    FeasibilityStudy,
}

#[derive(Clone, Debug)]
pub enum ResearchLength {
    Brief,           // 5-15 pages
    Standard,        // 15-40 pages
    Comprehensive,   // 40-80 pages
    Extensive,       // 80+ pages
}

#[derive(Clone, Debug)]
pub enum PoetryStyle {
    Sonnet,
    Haiku,
    FreeVerse,
    Ballad,
    Limerick,
    Epic,
    Lyric,
    Acrostic,
}

#[derive(Clone, Debug)]
pub enum PersonalWritingType {
    Journal,
    Memoir,
    Diary,
    Blog,
    Letter,
    Essay,
    TravelJournal,
    CreativeJournal,
    Reflection,
}

#[derive(Clone, Debug)]
pub enum PersonalLength {
    Brief,
    Standard,
    Extended,
    Comprehensive,
}

#[derive(Clone, Debug)]
pub enum MarketingType {
    SocialMediaAd,
    SocialAd,      // Alternative name
    DisplayAd,
    VideoScript,
    PressRelease,
    MediaKit,
    ProductDescription,
    LandingPage,
    EmailCampaign,
    Brochure,
}

#[derive(Clone, Debug)]
pub enum MarketingLength {
    Brief,
    Standard,
    Extended,
    Comprehensive,
}

#[derive(Clone, Debug)]
pub enum BlogContentType {
    BlogPost,
    SeoArticle,
    Tutorial,
    Listicle,
    Review,
    NewsArticle,
    Opinion,
    Interview,
    CaseStudy,
}

#[derive(Clone, Debug)]
pub enum BlogLength {
    Short,
    Medium,
    Long,
    VeryLong,
    Epic,      // Missing variant
}

#[derive(Clone, Debug)]
pub enum StrategicDocType {
    StrategicPlan,
    BusinessPlan,
    ProjectPlan,
    Roadmap,
    VisionDoc,
    Governance,
    RiskAssessment,
    BudgetPlan,
}

#[derive(Clone, Debug)]
pub enum StrategicLength {
    Brief,
    Standard,
    Comprehensive,
    Extensive,
}

#[derive(Clone, Debug)]
pub enum MeetingDocType {
    MeetingNotes,
    ActionItems,
    Summary,
    Transcript,
    DecisionLog,
    Agenda,
}

#[derive(Clone, Debug)]
pub enum MeetingLength {
    Brief,
    Standard,
    Extended,
    Comprehensive,
    Detailed,    // Missing variant
}