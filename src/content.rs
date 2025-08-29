use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::dynamic_length::{DynamicSectionLength, generate_dynamic_section_lengths};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Book,
    Screenplay,
    Play,
    TvScript,
    AudioScript,
    GameScript,
    Document,
    TechnicalDoc,
    WhitePaper,
    ResearchReport,
    Poetry,
    InteractiveFiction,
    Journal,
    Memoir,
    MarketingAd,
    PressRelease,
    MediaKit,
    BlogPost,
    SeoArticle,
    StrategicDoc,
    PlanningDoc,
    MeetingNotes,
    MeetingSummary,
    Dictionary,
    EducationalLesson,
    ChildrensBook,
    Encyclopedia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub content_type: ContentType,
    pub genre: String,
    pub writing_style: String,
    pub premise: String,
    pub outline: String,
    pub sections: Vec<Section>,
    pub metadata: ContentMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed: bool,
    pub world_state: Option<WorldState>, // Multi-plot narrative support
    pub stylistic_profile: Option<StylisticProfile>, // Literary enhancement system
}

// Legacy alias for backward compatibility
pub type Book = Content;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub number: usize,
    pub title: String,
    pub content: String,
    pub word_count: usize,
    pub outline: String,
    pub section_type: SectionType,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
    pub plot_thread: Option<String>, // Which storyline this section belongs to (plot ID)
    pub narrative_context: Option<NarrativeContext>, // Timeline, location, POV context
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SectionType {
    Chapter,          // Book chapters
    Scene,            // Screenplay/play scenes
    Act,              // Play acts
    Episode,          // TV episodes
    Segment,          // Audio segments
    Interaction,      // Game interactions
    Section,          // Document sections
}

// Legacy alias for backward compatibility
pub type Chapter = Section;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub target_size: String,
    pub target_word_count: Option<usize>,
    pub current_word_count: usize,
    pub target_sections: usize,
    pub model_used: String,
    pub generation_parameters: GenerationParameters,
    pub estimated_completion: Option<f32>, // Percentage
    pub content_specific: ContentSpecificMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentSpecificMetadata {
    Book { target_chapters: usize },
    Screenplay { target_pages: usize, format: ScreenplayFormat },
    Play { acts: usize, format: PlayFormat },
    TvScript { episodes: u32, episode_length: u32 },
    AudioScript { duration_minutes: u32, format: AudioFormat },
    GameScript { characters: u32, branches: u32 },
    Document { pages: usize, format: DocumentFormat },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScreenplayFormat {
    Hollywood,
    BBC,
    Continental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayFormat {
    Standard,
    Musical,
    OneAct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioFormat {
    Drama,
    Podcast,
    RadioPlay,
    Documentary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentFormat {
    Business,
    Technical,
    Academic,
    Legal,
    Educational,
}

// Legacy alias for backward compatibility
pub type BookMetadata = ContentMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParameters {
    pub temperature: f32,
    pub max_tokens_per_chapter: u32,
    pub context_window: usize,
}

impl Content {
    pub fn new_book(
        title: String,
        author: String,
        genre: String,
        writing_style: String,
        premise: String,
        target_size: String,
        target_word_count: Option<usize>,
        target_chapters: usize,
        model: String,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            title,
            author,
            content_type: ContentType::Book,
            genre,
            writing_style,
            premise,
            outline: String::new(),
            sections: Vec::new(),
            metadata: ContentMetadata {
                target_size,
                target_word_count,
                current_word_count: 0,
                target_sections: target_chapters,
                model_used: model,
                generation_parameters: GenerationParameters {
                    temperature: 0.8,
                    max_tokens_per_chapter: 5000,  // Support up to ~3,750 words per chapter
                    context_window: 3,
                },
                estimated_completion: Some(0.0),
                content_specific: ContentSpecificMetadata::Book { target_chapters },
            },
            created_at: now,
            updated_at: now,
            completed: false,
            world_state: None, // Initialize without multi-plot system by default
            stylistic_profile: None, // Initialize without stylistic enhancements by default
        }
    }
    
    pub fn new_screenplay(
        title: String,
        author: String,
        genre: String,
        writing_style: String,
        premise: String,
        target_pages: usize,
        model: String,
    ) -> Self {
        let now = Utc::now();
        let target_sections = target_pages / 10; // Rough estimate: 10 pages per scene
        
        Self {
            id: Uuid::new_v4(),
            title,
            author,
            content_type: ContentType::Screenplay,
            genre,
            writing_style,
            premise,
            outline: String::new(),
            sections: Vec::new(),
            metadata: ContentMetadata {
                target_size: format!("{} pages", target_pages),
                target_word_count: Some(target_pages * 250), // ~250 words per page
                current_word_count: 0,
                target_sections,
                model_used: model,
                generation_parameters: GenerationParameters {
                    temperature: 0.8,
                    max_tokens_per_chapter: 3000,  // Support longer screenplay scenes
                    context_window: 3,
                },
                estimated_completion: Some(0.0),
                content_specific: ContentSpecificMetadata::Screenplay { 
                    target_pages, 
                    format: ScreenplayFormat::Hollywood 
                },
            },
            created_at: now,
            updated_at: now,
            completed: false,
            world_state: None,
            stylistic_profile: Some(StylisticProfile::default()),
        }
    }
    
    pub fn new_document(
        title: String,
        author: String,
        writing_style: String,
        premise: String,
        target_pages: usize,
        doc_format: DocumentFormat,
        model: String,
    ) -> Self {
        let now = Utc::now();
        let target_sections = (target_pages / 2).max(1); // Rough estimate: 2 pages per section
        
        Self {
            id: Uuid::new_v4(),
            title,
            author,
            content_type: ContentType::Document,
            genre: "Professional".to_string(),
            writing_style,
            premise,
            outline: String::new(),
            sections: Vec::new(),
            metadata: ContentMetadata {
                target_size: format!("{} pages", target_pages),
                target_word_count: Some(target_pages * 500), // ~500 words per page
                current_word_count: 0,
                target_sections,
                model_used: model,
                generation_parameters: GenerationParameters {
                    temperature: 0.7,
                    max_tokens_per_chapter: 2500,  // Support longer document sections
                    context_window: 2,
                },
                estimated_completion: Some(0.0),
                content_specific: ContentSpecificMetadata::Document { 
                    pages: target_pages, 
                    format: doc_format 
                },
            },
            created_at: now,
            updated_at: now,
            completed: false,
            world_state: None,
            stylistic_profile: Some(StylisticProfile::default()),
        }
    }
    
    // Legacy method for backward compatibility
    pub fn new(
        title: String,
        author: String,
        genre: String,
        writing_style: String,
        premise: String,
        target_size: String,
        target_word_count: Option<usize>,
        target_chapters: usize,
        model: String,
    ) -> Self {
        Self::new_book(title, author, genre, writing_style, premise, target_size, target_word_count, target_chapters, model)
    }
    
    pub fn add_section(&mut self, section: Section) {
        self.metadata.current_word_count += section.word_count;
        self.sections.push(section);
        self.updated_at = Utc::now();
        self.update_completion_estimate();
    }
    
    // Legacy method for backward compatibility
    pub fn add_chapter(&mut self, chapter: Chapter) {
        self.add_section(chapter);
    }
    
    // Getter for backward compatibility
    pub fn chapters(&self) -> &Vec<Section> {
        &self.sections
    }
    
    pub fn update_completion_estimate(&mut self) {
        if let Some(target) = self.metadata.target_word_count {
            let progress = (self.metadata.current_word_count as f32) / (target as f32);
            self.metadata.estimated_completion = Some((progress * 100.0).min(100.0));
        } else {
            // For unlimited mode, base on section count
            let section_progress = (self.sections.len() as f32) / (self.metadata.target_sections as f32);
            self.metadata.estimated_completion = Some((section_progress * 100.0).min(100.0));
        }
    }
    
    pub fn should_continue(&self) -> bool {
        if self.completed {
            return false;
        }
        
        // Check if we've reached target word count
        if let Some(target) = self.metadata.target_word_count {
            if self.metadata.current_word_count >= target {
                return false;
            }
        }
        
        // For unlimited mode, we can continue indefinitely
        // But we might want to check for natural conclusion signals
        true
    }
    
    pub fn get_context_for_next_section(&self) -> String {
        let context_sections = self.metadata.generation_parameters.context_window;
        let start_idx = if self.sections.len() > context_sections {
            self.sections.len() - context_sections
        } else {
            0
        };
        
        let content_type_name = match self.content_type {
            ContentType::Book => "Book",
            ContentType::Screenplay => "Screenplay",
            ContentType::Play => "Play",
            ContentType::TvScript => "TV Script",
            ContentType::AudioScript => "Audio Script", 
            ContentType::GameScript => "Game Script",
            ContentType::Document => "Document",
            ContentType::TechnicalDoc => "Technical Documentation",
            ContentType::WhitePaper => "White Paper",
            ContentType::ResearchReport => "Research Report",
            ContentType::Poetry => "Poetry",
            ContentType::InteractiveFiction => "Interactive Fiction",
            ContentType::Journal => "Journal",
            ContentType::Memoir => "Memoir",
            ContentType::MarketingAd => "Marketing Content",
            ContentType::PressRelease => "Press Release",
            ContentType::MediaKit => "Media Kit",
            ContentType::BlogPost => "Blog Post",
            ContentType::SeoArticle => "SEO Article",
            ContentType::StrategicDoc => "Strategic Document",
            ContentType::PlanningDoc => "Planning Document",
            ContentType::MeetingNotes => "Meeting Notes",
            ContentType::MeetingSummary => "Meeting Summary",
            ContentType::Dictionary => "Dictionary",
            ContentType::EducationalLesson => "Educational Lesson",
            ContentType::ChildrensBook => "Children's Book",
            ContentType::Encyclopedia => "Encyclopedia",
        };
        
        let mut context = format!(
            "{}: {}\nGenre: {}\nStyle: {}\nPremise: {}\n\n",
            content_type_name, self.title, self.genre, self.writing_style, self.premise
        );
        
        if !self.outline.is_empty() {
            context.push_str(&format!("Outline:\n{}\n\n", self.outline));
        }
        
        if !self.sections.is_empty() {
            let section_name = match self.content_type {
                ContentType::Book => "chapters",
                ContentType::Screenplay | ContentType::Play => "scenes",
                ContentType::TvScript => "episodes",
                ContentType::AudioScript => "segments",
                ContentType::GameScript => "interactions",
                ContentType::Document => "sections",
                ContentType::TechnicalDoc => "sections",
                ContentType::WhitePaper => "sections",
                ContentType::ResearchReport => "sections",
                ContentType::Poetry => "poems",
                ContentType::InteractiveFiction => "chapters",
                ContentType::Journal => "entries",
                ContentType::Memoir => "chapters",
                ContentType::MarketingAd => "sections",
                ContentType::PressRelease => "sections",
                ContentType::MediaKit => "sections",
                ContentType::BlogPost => "sections",
                ContentType::SeoArticle => "sections",
                ContentType::StrategicDoc => "sections",
                ContentType::PlanningDoc => "sections",
                ContentType::MeetingNotes => "sections",
                ContentType::MeetingSummary => "sections",
                ContentType::Dictionary => "entries",
                ContentType::EducationalLesson => "lessons",
                ContentType::ChildrensBook => "chapters",
                ContentType::Encyclopedia => "entries",
            };
            
            context.push_str(&format!("Previous {}:\n", section_name));
            for section in &self.sections[start_idx..] {
                let section_label = match section.section_type {
                    SectionType::Chapter => "Chapter",
                    SectionType::Scene => "Scene",
                    SectionType::Act => "Act",
                    SectionType::Episode => "Episode",
                    SectionType::Segment => "Segment",
                    SectionType::Interaction => "Interaction",
                    SectionType::Section => "Section",
                };
                
                context.push_str(&format!(
                    "{} {}: {}\n{}\n\n",
                    section_label, section.number, section.title, 
                    truncate_text(&section.content, 500)
                ));
            }
        }
        
        context
    }
    
    // Legacy method for backward compatibility
    pub fn get_context_for_next_chapter(&self) -> String {
        self.get_context_for_next_section()
    }
    
    pub fn get_clean_context(&self) -> String {
        if self.sections.is_empty() {
            return self.outline.clone();
        }
        
        // Provide a summary of completed sections
        let mut context = String::new();
        context.push_str(&format!("Title: {}\n", self.title));
        context.push_str(&format!("Premise: {}\n\n", self.premise));
        
        // Add section summaries
        for section in &self.sections {
            let section_type = match self.content_type {
                ContentType::Book => "Chapter",
                ContentType::Screenplay => "Scene",
                ContentType::Play => "Act",
                ContentType::TvScript => "Episode",
                ContentType::AudioScript => "Segment",
                ContentType::GameScript => "Interaction",
                ContentType::Document => "Section",
                ContentType::TechnicalDoc => "Section",
                ContentType::WhitePaper => "Section",
                ContentType::ResearchReport => "Section",
                ContentType::Poetry => "Poem",
                ContentType::InteractiveFiction => "Chapter",
                ContentType::Journal => "Entry",
                ContentType::Memoir => "Chapter",
                ContentType::MarketingAd => "Section",
                ContentType::PressRelease => "Section",
                ContentType::MediaKit => "Section",
                ContentType::BlogPost => "Section",
                ContentType::SeoArticle => "Section",
                ContentType::StrategicDoc => "Section",
                ContentType::PlanningDoc => "Section",
                ContentType::MeetingNotes => "Section",
                ContentType::MeetingSummary => "Section",
                ContentType::Dictionary => "Entry",
                ContentType::EducationalLesson => "Lesson",
                ContentType::ChildrensBook => "Chapter",
                ContentType::Encyclopedia => "Entry",
            };
            
            context.push_str(&format!("{} {}: {}\n", section_type, section.number, section.title));
            
            // Add first 100 words of content as summary
            let words: Vec<&str> = section.content.split_whitespace().collect();
            let summary = if words.len() > 100 {
                format!("{}...", words[..100].join(" "))
            } else {
                section.content.clone()
            };
            context.push_str(&format!("Summary: {}\n\n", summary));
        }
        
        context
    }
    
    pub fn to_text(&self) -> String {
        let content_type_name = match self.content_type {
            ContentType::Book => "Book",
            ContentType::Screenplay => "Screenplay",
            ContentType::Play => "Play",
            ContentType::TvScript => "TV Script",
            ContentType::AudioScript => "Audio Script",
            ContentType::GameScript => "Game Script", 
            ContentType::Document => "Document",
            ContentType::TechnicalDoc => "Technical Documentation",
            ContentType::WhitePaper => "White Paper",
            ContentType::ResearchReport => "Research Report",
            ContentType::Poetry => "Poetry",
            ContentType::InteractiveFiction => "Interactive Fiction",
            ContentType::Journal => "Journal",
            ContentType::Memoir => "Memoir",
            ContentType::MarketingAd => "Marketing Content",
            ContentType::PressRelease => "Press Release",
            ContentType::MediaKit => "Media Kit",
            ContentType::BlogPost => "Blog Post",
            ContentType::SeoArticle => "SEO Article",
            ContentType::StrategicDoc => "Strategic Document",
            ContentType::PlanningDoc => "Planning Document",
            ContentType::MeetingNotes => "Meeting Notes",
            ContentType::MeetingSummary => "Meeting Summary",
            ContentType::Dictionary => "Dictionary",
            ContentType::EducationalLesson => "Educational Lesson",
            ContentType::ChildrensBook => "Children's Book",
            ContentType::Encyclopedia => "Encyclopedia",
        };
        
        let mut content_text = format!(
            "{}\n\
            by {}\n\
            \n\
            Type: {}\n\
            Genre: {}\n\
            Style: {}\n\
            \n\
            {}\n\
            \n",
            self.title, self.author, content_type_name, self.genre, self.writing_style, self.premise
        );
        
        for section in &self.sections {
            let section_label = match section.section_type {
                SectionType::Chapter => "Chapter",
                SectionType::Scene => "Scene", 
                SectionType::Act => "Act",
                SectionType::Episode => "Episode",
                SectionType::Segment => "Segment",
                SectionType::Interaction => "Interaction",
                SectionType::Section => "Section",
            };
            
            // Apply content-specific formatting
            let formatted_content = match self.content_type {
                ContentType::Screenplay => self.format_screenplay_scene(&section.content),
                ContentType::Play => self.format_play_scene(&section.content),
                ContentType::AudioScript => self.format_audio_script(&section.content),
                ContentType::GameScript => self.format_game_script(&section.content),
                ContentType::Dictionary => self.format_dictionary_entry(&section.content),
                ContentType::EducationalLesson => self.format_educational_content(&section.content),
                ContentType::ChildrensBook => self.format_childrens_content(&section.content),
                ContentType::Encyclopedia => self.format_encyclopedia_entry(&section.content),
                _ => section.content.clone(),
            };
            
            content_text.push_str(&format!(
                "\n\n--- {} {} ---\n\
                {}\n\
                \n\
                {}\n",
                section_label, section.number, section.title, formatted_content
            ));
        }
        
        content_text
    }
    
    // Content-specific formatting methods
    pub fn format_screenplay_scene(&self, content: &str) -> String {
        // Basic screenplay formatting
        content.lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    String::new()
                } else if line.starts_with("INT.") || line.starts_with("EXT.") {
                    format!("{}\n", line.to_uppercase())
                } else if line.chars().all(|c| c.is_uppercase() || c.is_whitespace()) && !line.contains(':') {
                    format!("                    {}\n", line) // Character name
                } else if line.starts_with('(') && line.ends_with(')') {
                    format!("                {}\n", line) // Action/direction
                } else {
                    format!("          {}\n", line) // Dialogue
                }
            })
            .collect()
    }
    
    pub fn format_play_scene(&self, content: &str) -> String {
        // Basic stage play formatting
        content.lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    String::new()
                } else if line.starts_with('[') && line.ends_with(']') {
                    format!("{}\n", line) // Stage directions
                } else if line.contains(':') {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        format!("{}: {}\n", parts[0].to_uppercase(), parts[1])
                    } else {
                        format!("{}\n", line)
                    }
                } else {
                    format!("{}\n", line)
                }
            })
            .collect()
    }
    
    pub fn format_audio_script(&self, content: &str) -> String {
        // Audio script formatting with sound cues
        content.lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    String::new()
                } else if line.starts_with("SFX:") || line.starts_with("MUSIC:") {
                    format!("[{}]\n", line)
                } else if line.starts_with("NARRATOR:") {
                    format!("**{}**\n", line)
                } else if line.contains(':') {
                    format!("{}\n", line)
                } else {
                    format!("{}\n", line)
                }
            })
            .collect()
    }
    
    pub fn format_game_script(&self, content: &str) -> String {
        // Game script with dialogue options
        content.lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    String::new()
                } else if line.starts_with("CHOICE:") {
                    format!("  > {}\n", line.strip_prefix("CHOICE:").unwrap_or(line).trim())
                } else if line.starts_with("CONDITION:") {
                    format!("    [{}: {}]\n", "IF", line.strip_prefix("CONDITION:").unwrap_or(line).trim())
                } else if line.starts_with("ACTION:") {
                    format!("    *{}*\n", line.strip_prefix("ACTION:").unwrap_or(line).trim())
                } else {
                    format!("{}\n", line)
                }
            })
            .collect()
    }
    
    pub fn format_dictionary_entry(&self, content: &str) -> String {
        // Dictionary entry formatting with definitions
        content.lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    String::new()
                } else if line.starts_with("WORD:") {
                    format!("**{}**\n", line.strip_prefix("WORD:").unwrap_or(line).trim())
                } else if line.starts_with("DEFINITION:") {
                    format!("  {}\n", line.strip_prefix("DEFINITION:").unwrap_or(line).trim())
                } else if line.starts_with("ETYMOLOGY:") {
                    format!("  *Etymology:* {}\n", line.strip_prefix("ETYMOLOGY:").unwrap_or(line).trim())
                } else {
                    format!("{}\n", line)
                }
            })
            .collect()
    }
    
    pub fn format_encyclopedia_entry(&self, content: &str) -> String {
        // Encyclopedia entry formatting with structured information
        content.lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    String::new()
                } else if line.starts_with("TOPIC:") {
                    format!("# {}\n\n", line.strip_prefix("TOPIC:").unwrap_or(line).trim())
                } else if line.starts_with("DEFINITION:") {
                    format!("**Definition:** {}\n\n", line.strip_prefix("DEFINITION:").unwrap_or(line).trim())
                } else if line.starts_with("HISTORY:") {
                    format!("## History\n{}\n\n", line.strip_prefix("HISTORY:").unwrap_or(line).trim())
                } else if line.starts_with("SIGNIFICANCE:") {
                    format!("## Significance\n{}\n\n", line.strip_prefix("SIGNIFICANCE:").unwrap_or(line).trim())
                } else if line.starts_with("SEE_ALSO:") {
                    format!("**See Also:** {}\n\n", line.strip_prefix("SEE_ALSO:").unwrap_or(line).trim())
                } else if line.starts_with("CATEGORIES:") {
                    format!("**Categories:** {}\n\n", line.strip_prefix("CATEGORIES:").unwrap_or(line).trim())
                } else {
                    format!("{}\n", line)
                }
            })
            .collect()
    }
    
    pub fn format_educational_content(&self, content: &str) -> String {
        // Educational lesson formatting with objectives
        content.lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    String::new()
                } else if line.starts_with("OBJECTIVE:") {
                    format!("📚 **Learning Objective:** {}\n", line.strip_prefix("OBJECTIVE:").unwrap_or(line).trim())
                } else if line.starts_with("ACTIVITY:") {
                    format!("🎯 **Activity:** {}\n", line.strip_prefix("ACTIVITY:").unwrap_or(line).trim())
                } else if line.starts_with("ASSESSMENT:") {
                    format!("📝 **Assessment:** {}\n", line.strip_prefix("ASSESSMENT:").unwrap_or(line).trim())
                } else {
                    format!("{}\n", line)
                }
            })
            .collect()
    }
    
    pub fn format_childrens_content(&self, content: &str) -> String {
        // Children's book formatting with simple, engaging style
        content.lines()
            .map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    String::new()
                } else if line.starts_with("ILLUSTRATION:") {
                    format!("🎨 *[{}]*\n", line.strip_prefix("ILLUSTRATION:").unwrap_or(line).trim())
                } else if line.starts_with("DIALOGUE:") {
                    format!("💬 \"{}\"\n", line.strip_prefix("DIALOGUE:").unwrap_or(line).trim())
                } else {
                    format!("{}\n", line)
                }
            })
            .collect()
    }
    
    pub fn to_markdown(&self) -> String {
        let content_type_name = match self.content_type {
            ContentType::Book => "Book",
            ContentType::Screenplay => "Screenplay",
            ContentType::Play => "Play",
            ContentType::TvScript => "TV Script",
            ContentType::AudioScript => "Audio Script",
            ContentType::GameScript => "Game Script",
            ContentType::Document => "Document",
            ContentType::TechnicalDoc => "Technical Documentation",
            ContentType::WhitePaper => "White Paper",
            ContentType::ResearchReport => "Research Report",
            ContentType::Poetry => "Poetry",
            ContentType::InteractiveFiction => "Interactive Fiction",
            ContentType::Journal => "Journal",
            ContentType::Memoir => "Memoir",
            ContentType::MarketingAd => "Marketing Content",
            ContentType::PressRelease => "Press Release",
            ContentType::MediaKit => "Media Kit",
            ContentType::BlogPost => "Blog Post",
            ContentType::SeoArticle => "SEO Article",
            ContentType::StrategicDoc => "Strategic Document",
            ContentType::PlanningDoc => "Planning Document",
            ContentType::MeetingNotes => "Meeting Notes",
            ContentType::MeetingSummary => "Meeting Summary",
            ContentType::Dictionary => "Dictionary",
            ContentType::EducationalLesson => "Educational Lesson",
            ContentType::ChildrensBook => "Children's Book",
            ContentType::Encyclopedia => "Encyclopedia",
        };
        
        let mut markdown = format!(
            "# {}\n\
            \n\
            **Author:** {}\n\
            **Type:** {}\n\
            **Genre:** {}\n\
            **Writing Style:** {}\n\
            **Created:** {}\n\
            **Word Count:** {}\n\
            \n\
            ## Premise\n\
            \n\
            {}\n\
            \n",
            self.title,
            self.author,
            content_type_name,
            self.genre,
            self.writing_style,
            self.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            self.metadata.current_word_count,
            self.premise
        );
        
        for section in &self.sections {
            let section_label = match section.section_type {
                SectionType::Chapter => "Chapter",
                SectionType::Scene => "Scene",
                SectionType::Act => "Act", 
                SectionType::Episode => "Episode",
                SectionType::Segment => "Segment",
                SectionType::Interaction => "Interaction",
                SectionType::Section => "Section",
            };
            
            markdown.push_str(&format!(
                "## {} {}: {}\n\
                \n\
                {}\n\
                \n",
                section_label, section.number, section.title, section.content
            ));
        }
        
        markdown
    }
}

impl Section {
    pub fn new(number: usize, title: String, outline: String, section_type: SectionType) -> Self {
        Self {
            number,
            title,
            content: String::new(),
            word_count: 0,
            outline,
            section_type,
            created_at: Utc::now(),
            completed: false,
            plot_thread: None, // No plot assignment by default
            narrative_context: None, // No narrative context by default
        }
    }
    
    // Legacy constructor for backward compatibility
    pub fn new_chapter(number: usize, title: String, outline: String) -> Self {
        Self::new(number, title, outline, SectionType::Chapter)
    }
    
    pub fn set_content(&mut self, content: String) {
        self.word_count = count_words(&content);
        self.content = content;
        self.completed = true;
    }
}

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredOutline {
    pub title: String,
    pub premise: String,
    pub genre: String,
    pub target_audience: String,
    pub sections: Vec<OutlineSection>,
    pub themes: Vec<String>,
    pub characters: Option<Vec<Character>>,
    pub settings: Option<Vec<Setting>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineSection {
    pub number: usize,
    pub title: String,
    pub description: String,
    pub key_events: Vec<String>,
    pub target_words: usize,
    pub section_type: SectionType,
    pub character_focus: Option<Vec<String>>,
    pub setting: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub role: String,
    pub description: String,
    pub traits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub name: String,
    pub description: String,
    pub time_period: Option<String>,
    pub mood: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredPrompt {
    pub template_type: PromptType,
    pub instructions: Vec<String>,
    pub context: PromptContext,
    pub output_format: OutputFormat,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptType {
    OutlineGeneration,
    SectionGeneration,
    CharacterDevelopment,
    DialogueGeneration,
    DescriptiveWriting,
    TechnicalWriting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptContext {
    pub content_type: ContentType,
    pub genre: String,
    pub style: String,
    pub target_audience: String,
    pub previous_content: Option<String>,
    pub current_section: Option<usize>,
    pub total_sections: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    PlainText,
    Json,
    Markdown,
    Structured,
}

pub fn truncate_text(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(max_chars).collect();
        format!("{}...", truncated)
    }
}

impl StructuredOutline {
    pub fn new(title: String, premise: String, genre: String, target_audience: String, num_sections: usize, content_type: ContentType) -> Self {
        let section_type = match content_type {
            ContentType::Book | ContentType::Memoir | ContentType::InteractiveFiction | ContentType::ChildrensBook => SectionType::Chapter,
            ContentType::Screenplay | ContentType::Play => SectionType::Scene,
            ContentType::TvScript => SectionType::Episode,
            ContentType::AudioScript => SectionType::Segment,
            ContentType::GameScript => SectionType::Interaction,
            ContentType::Dictionary => SectionType::Section,
            ContentType::EducationalLesson => SectionType::Section,
            _ => SectionType::Section,
        };

        // Generate dynamic section lengths based on content type
        let dynamic_lengths = generate_dynamic_section_lengths(content_type, num_sections, None);

        let sections = (1..=num_sections).map(|i| {
            let dynamic_length = &dynamic_lengths[i - 1];
            OutlineSection {
                number: i,
                title: format!("Section {}", i),
                description: "To be generated".to_string(),
                key_events: vec![],
                target_words: dynamic_length.target_words,
                section_type,
                character_focus: None,
                setting: None,
            }
        }).collect();

        Self {
            title,
            premise,
            genre,
            target_audience,
            sections,
            themes: vec![],
            characters: None,
            settings: None,
        }
    }

    pub fn new_with_dynamic_seed(title: String, premise: String, genre: String, target_audience: String, num_sections: usize, content_type: ContentType, seed: u64) -> Self {
        let section_type = match content_type {
            ContentType::Book | ContentType::Memoir | ContentType::InteractiveFiction | ContentType::ChildrensBook => SectionType::Chapter,
            ContentType::Screenplay | ContentType::Play => SectionType::Scene,
            ContentType::TvScript => SectionType::Episode,
            ContentType::AudioScript => SectionType::Segment,
            ContentType::GameScript => SectionType::Interaction,
            ContentType::Dictionary => SectionType::Section,
            ContentType::EducationalLesson => SectionType::Section,
            _ => SectionType::Section,
        };

        // Generate dynamic section lengths with specific seed for reproducibility
        let dynamic_lengths = generate_dynamic_section_lengths(content_type, num_sections, Some(seed));

        let sections = (1..=num_sections).map(|i| {
            let dynamic_length = &dynamic_lengths[i - 1];
            OutlineSection {
                number: i,
                title: format!("Section {}", i),
                description: "To be generated".to_string(),
                key_events: vec![],
                target_words: dynamic_length.target_words,
                section_type,
                character_focus: None,
                setting: None,
            }
        }).collect();

        Self {
            title,
            premise,
            genre,
            target_audience,
            sections,
            themes: vec![],
            characters: None,
            settings: None,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_text_outline(&self) -> String {
        let mut outline = format!("Title: {}\nPremise: {}\nGenre: {}\nTarget Audience: {}\n\n", 
            self.title, self.premise, self.genre, self.target_audience);

        if !self.themes.is_empty() {
            outline.push_str(&format!("Themes: {}\n\n", self.themes.join(", ")));
        }

        if let Some(characters) = &self.characters {
            outline.push_str("Characters:\n");
            for character in characters {
                outline.push_str(&format!("- {}: {} ({})\n", character.name, character.role, character.description));
            }
            outline.push_str("\n");
        }

        if let Some(settings) = &self.settings {
            outline.push_str("Settings:\n");
            for setting in settings {
                outline.push_str(&format!("- {}: {}\n", setting.name, setting.description));
            }
            outline.push_str("\n");
        }

        outline.push_str("Sections:\n");
        for section in &self.sections {
            outline.push_str(&format!("{} {}: {}\n", 
                section.section_type.to_string(), section.number, section.title));
            outline.push_str(&format!("  Description: {}\n", section.description));
            if !section.key_events.is_empty() {
                outline.push_str(&format!("  Key Events: {}\n", section.key_events.join(", ")));
            }
            outline.push_str(&format!("  Target Words: {}\n\n", section.target_words));
        }

        outline
    }
}

impl StructuredPrompt {
    pub fn new_outline_generation(context: PromptContext, num_sections: usize) -> Self {
        let instructions = match context.content_type {
            ContentType::Book => vec![
                "Create a detailed book outline with compelling character arcs".to_string(),
                format!("Generate exactly {} chapters with clear progression", num_sections),
                "Include character development and plot twists".to_string(),
                "Ensure each chapter advances the story meaningfully".to_string(),
            ],
            ContentType::TechnicalDoc => vec![
                "Create a logical technical documentation structure".to_string(),
                format!("Generate exactly {} sections with clear learning progression", num_sections),
                "Include prerequisites, examples, and troubleshooting".to_string(),
                "Ensure comprehensive coverage of the subject".to_string(),
            ],
            ContentType::BlogPost => vec![
                "Create an engaging blog post structure".to_string(),
                format!("Generate exactly {} sections with clear value proposition", num_sections),
                "Include actionable insights and examples".to_string(),
                "Ensure reader engagement throughout".to_string(),
            ],
            _ => vec![
                format!("Create a detailed outline for {} content", context.content_type.to_string()),
                format!("Generate exactly {} sections with clear progression", num_sections),
                "Ensure logical flow and comprehensive coverage".to_string(),
            ],
        };

        let constraints = vec![
            "Output must be valid JSON format".to_string(),
            "Each section must have a clear purpose".to_string(),
            "Maintain consistency in tone and style".to_string(),
            "Include specific details and actionable content".to_string(),
        ];

        Self {
            template_type: PromptType::OutlineGeneration,
            instructions,
            context,
            output_format: OutputFormat::Json,
            constraints,
        }
    }

    pub fn new_section_generation(context: PromptContext, section_outline: &str, target_words: usize) -> Self {
        let instructions = vec![
            format!("Generate a detailed {} section", context.content_type.to_string()),
            format!("Target length: {} words", target_words),
            "Follow the provided outline exactly".to_string(),
            "Maintain consistent style and voice".to_string(),
            "Include specific details and examples".to_string(),
        ];

        let constraints = vec![
            format!("Must be approximately {} words", target_words),
            "Must match the requested section exactly".to_string(),
            "No chapter numbers or section markers in output".to_string(),
            "Maintain narrative flow and engagement".to_string(),
        ];

        Self {
            template_type: PromptType::SectionGeneration,
            instructions,
            context,
            output_format: OutputFormat::PlainText,
            constraints,
        }
    }

    pub fn to_formatted_prompt(&self, specific_request: &str) -> String {
        let mut prompt = String::new();

        // Context section
        prompt.push_str("CONTEXT:\n");
        prompt.push_str(&format!("Content Type: {}\n", self.context.content_type.to_string()));
        prompt.push_str(&format!("Genre: {}\n", self.context.genre));
        prompt.push_str(&format!("Style: {}\n", self.context.style));
        prompt.push_str(&format!("Target Audience: {}\n", self.context.target_audience));
        
        if let Some(section) = self.context.current_section {
            if let Some(total) = self.context.total_sections {
                prompt.push_str(&format!("Section: {} of {}\n", section, total));
            }
        }
        prompt.push_str("\n");

        // Instructions section
        prompt.push_str("INSTRUCTIONS:\n");
        for (i, instruction) in self.instructions.iter().enumerate() {
            prompt.push_str(&format!("{}. {}\n", i + 1, instruction));
        }
        prompt.push_str("\n");

        // Previous content context
        if let Some(prev_content) = &self.context.previous_content {
            prompt.push_str("PREVIOUS CONTENT:\n");
            prompt.push_str(&format!("{}\n\n", truncate_text(prev_content, 2000)));
        }

        // Specific request
        prompt.push_str("REQUEST:\n");
        prompt.push_str(&format!("{}\n\n", specific_request));

        // Output format
        prompt.push_str("OUTPUT FORMAT:\n");
        match self.output_format {
            OutputFormat::Json => prompt.push_str("Respond with valid JSON only, no additional text or explanations.\n"),
            OutputFormat::PlainText => prompt.push_str("Respond with plain text content only, no formatting markers.\n"),
            OutputFormat::Markdown => prompt.push_str("Respond with properly formatted Markdown.\n"),
            OutputFormat::Structured => prompt.push_str("Respond with structured text following the specified format.\n"),
        }
        prompt.push_str("\n");

        // Constraints
        prompt.push_str("CONSTRAINTS:\n");
        for (i, constraint) in self.constraints.iter().enumerate() {
            prompt.push_str(&format!("{}. {}\n", i + 1, constraint));
        }
        prompt.push_str("\n");

        prompt.push_str("RESPONSE:\n");
        prompt
    }
}

impl std::fmt::Display for SectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SectionType::Chapter => "Chapter",
            SectionType::Scene => "Scene",
            SectionType::Act => "Act",
            SectionType::Episode => "Episode",
            SectionType::Segment => "Segment",
            SectionType::Interaction => "Interaction",
            SectionType::Section => "Section",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ContentType::Book => "Book",
            ContentType::Screenplay => "Screenplay",
            ContentType::Play => "Play",
            ContentType::TvScript => "TV Script",
            ContentType::AudioScript => "Audio Script",
            ContentType::GameScript => "Game Script",
            ContentType::Document => "Document",
            ContentType::TechnicalDoc => "Technical Documentation",
            ContentType::WhitePaper => "White Paper",
            ContentType::ResearchReport => "Research Report",
            ContentType::Poetry => "Poetry",
            ContentType::InteractiveFiction => "Interactive Fiction",
            ContentType::Journal => "Journal",
            ContentType::Memoir => "Memoir",
            ContentType::MarketingAd => "Marketing Content",
            ContentType::PressRelease => "Press Release",
            ContentType::MediaKit => "Media Kit",
            ContentType::BlogPost => "Blog Post",
            ContentType::SeoArticle => "SEO Article",
            ContentType::StrategicDoc => "Strategic Document",
            ContentType::PlanningDoc => "Planning Document",
            ContentType::MeetingNotes => "Meeting Notes",
            ContentType::MeetingSummary => "Meeting Summary",
            ContentType::Dictionary => "Dictionary",
            ContentType::EducationalLesson => "Educational Lesson",
            ContentType::ChildrensBook => "Children's Book",
            ContentType::Encyclopedia => "Encyclopedia",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct ContentProgress {
    pub current_section: usize,
    pub total_sections: usize,
    pub current_words: usize,
    pub target_words: Option<usize>,
    pub estimated_completion: f32,
}

impl ContentProgress {
    pub fn from_content(content: &Content) -> Self {
        Self {
            current_section: content.sections.len(),
            total_sections: content.metadata.target_sections,
            current_words: content.metadata.current_word_count,
            target_words: content.metadata.target_word_count,
            estimated_completion: content.metadata.estimated_completion.unwrap_or(0.0),
        }
    }
}

// Legacy alias for backward compatibility
pub type BookProgress = ContentProgress;

#[derive(Debug, Clone, Copy)]
pub enum AgeGroup {
    Children,       // 6-12 years
    YoungAdult,     // 13-17 years  
    Adult,          // 18-64 years
    Mature,         // 65+ years
}

#[derive(Debug, Clone)]
pub struct WritingAdjustments {
    pub target_age_group: AgeGroup,
    pub complexity_level: f32,      // 0.0-1.0, higher = more complex
    pub pacing_style: String,       // "fast", "moderate", "slow", "deliberate"
    pub vocabulary_level: String,   // "simple", "standard", "advanced", "literary"
    pub sentence_style: String,     // "short", "mixed", "complex", "flowing"
    pub dialogue_density: f32,      // 0.0-1.0, higher = more dialogue
    pub description_depth: f32,     // 0.0-1.0, higher = more descriptive
}

impl WritingAdjustments {
    pub fn for_genre(genre: &str) -> Self {
        match genre.to_lowercase().as_str() {
            "thriller" | "crime" => Self {
                target_age_group: AgeGroup::Adult,
                complexity_level: 0.7,
                pacing_style: "fast".to_string(),
                vocabulary_level: "standard".to_string(),
                sentence_style: "short".to_string(),
                dialogue_density: 0.8,
                description_depth: 0.4,
            },
            "romance" => Self {
                target_age_group: AgeGroup::Adult,
                complexity_level: 0.6,
                pacing_style: "moderate".to_string(),
                vocabulary_level: "standard".to_string(),
                sentence_style: "flowing".to_string(),
                dialogue_density: 0.7,
                description_depth: 0.8,
            },
            "fantasy" | "scifi" => Self {
                target_age_group: AgeGroup::YoungAdult,
                complexity_level: 0.8,
                pacing_style: "moderate".to_string(),
                vocabulary_level: "advanced".to_string(),
                sentence_style: "complex".to_string(),
                dialogue_density: 0.6,
                description_depth: 0.9,
            },
            "horror" => Self {
                target_age_group: AgeGroup::Adult,
                complexity_level: 0.7,
                pacing_style: "deliberate".to_string(),
                vocabulary_level: "advanced".to_string(),
                sentence_style: "complex".to_string(),
                dialogue_density: 0.5,
                description_depth: 0.9,
            },
            "mystery" => Self {
                target_age_group: AgeGroup::Adult,
                complexity_level: 0.8,
                pacing_style: "moderate".to_string(),
                vocabulary_level: "standard".to_string(),
                sentence_style: "mixed".to_string(),
                dialogue_density: 0.7,
                description_depth: 0.6,
            },
            "comedy" | "drama" => Self {
                target_age_group: AgeGroup::Adult,
                complexity_level: 0.6,
                pacing_style: "moderate".to_string(),
                vocabulary_level: "standard".to_string(),
                sentence_style: "mixed".to_string(),
                dialogue_density: 0.9,
                description_depth: 0.5,
            },
            "historical" | "biography" | "memoir" => Self {
                target_age_group: AgeGroup::Adult,
                complexity_level: 0.8,
                pacing_style: "slow".to_string(),
                vocabulary_level: "literary".to_string(),
                sentence_style: "flowing".to_string(),
                dialogue_density: 0.4,
                description_depth: 0.9,
            },
            "selfhelp" | "nonfiction" | "technical" => Self {
                target_age_group: AgeGroup::Adult,
                complexity_level: 0.7,
                pacing_style: "deliberate".to_string(),
                vocabulary_level: "standard".to_string(),
                sentence_style: "short".to_string(),
                dialogue_density: 0.2,
                description_depth: 0.6,
            },
            "adventure" => Self {
                target_age_group: AgeGroup::YoungAdult,
                complexity_level: 0.6,
                pacing_style: "fast".to_string(),
                vocabulary_level: "standard".to_string(),
                sentence_style: "short".to_string(),
                dialogue_density: 0.7,
                description_depth: 0.6,
            },
            "poetry" => Self {
                target_age_group: AgeGroup::Adult,
                complexity_level: 0.9,
                pacing_style: "deliberate".to_string(),
                vocabulary_level: "literary".to_string(),
                sentence_style: "flowing".to_string(),
                dialogue_density: 0.1,
                description_depth: 0.9,
            },
            _ => Self::default(), // Default for "fiction" and others
        }
    }
    
    pub fn get_style_instructions(&self) -> String {
        format!(
            "Writing guidelines for optimal readability:\
            \n- Target audience: {:?} readers\
            \n- Pacing: {} paced narrative\
            \n- Vocabulary: {} language level\
            \n- Sentence style: {} sentences\
            \n- Dialogue: {} dialogue density\
            \n- Description: {} descriptive detail\
            \n- Complexity: {:.0}% narrative complexity",
            self.target_age_group,
            self.pacing_style,
            self.vocabulary_level,
            self.sentence_style,
            if self.dialogue_density > 0.7 { "high" } 
            else if self.dialogue_density > 0.4 { "moderate" } 
            else { "low" },
            if self.description_depth > 0.7 { "rich" } 
            else if self.description_depth > 0.4 { "moderate" } 
            else { "minimal" },
            self.complexity_level * 100.0
        )
    }
}

impl Default for WritingAdjustments {
    fn default() -> Self {
        Self {
            target_age_group: AgeGroup::Adult,
            complexity_level: 0.7,
            pacing_style: "moderate".to_string(),
            vocabulary_level: "standard".to_string(),
            sentence_style: "mixed".to_string(),
            dialogue_density: 0.6,
            description_depth: 0.7,
        }
    }
}

// Multi-Plot Narrative System Structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub characters: Vec<MultiPlotCharacter>,
    pub locations: Vec<Location>,
    pub plot_threads: Vec<PlotThread>,
    pub timeline: Timeline,
    pub world_rules: Vec<WorldRule>,
    pub active_plots: Vec<String>, // Plot IDs currently being developed
    pub narrative_style: NarrativeStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiPlotCharacter {
    pub id: String,
    pub name: String,
    pub description: String,
    pub role: CharacterRole,
    pub affiliations: Vec<String>, // Plot threads they're involved in
    pub current_status: CharacterStatus,
    pub timeline_appearances: Vec<TimelineEntry>,
    pub character_arc: Option<String>,
    pub relationships: Vec<CharacterRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: String,
    pub location_type: LocationType,
    pub time_period: Option<String>,
    pub connected_locations: Vec<String>,
    pub significance: Vec<String>, // Plot threads that use this location
    pub current_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotThread {
    pub id: String,
    pub name: String,
    pub description: String,
    pub plot_type: PlotType,
    pub status: PlotStatus,
    pub main_characters: Vec<String>, // Character IDs
    pub key_locations: Vec<String>, // Location IDs
    pub timeline_span: TimeSpan,
    pub theme: String,
    pub current_stage: PlotStage,
    pub intersections: Vec<PlotIntersection>, // How this plot connects with others
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub events: Vec<TimelineEvent>,
    pub current_time_markers: std::collections::HashMap<String, String>, // Plot ID -> current time
    pub temporal_structure: TemporalStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldRule {
    pub id: String,
    pub rule_type: RuleType,
    pub description: String,
    pub scope: RuleScope, // Which parts of the world this affects
    pub consistency_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeContext {
    pub plot_thread_id: String,
    pub point_of_view: PointOfView,
    pub time_marker: String,
    pub primary_location: String,
    pub active_characters: Vec<String>,
    pub narrative_tension: TensionLevel,
    pub context_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub plot_thread_id: String,
    pub time_marker: String,
    pub event_description: String,
    pub character_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRelationship {
    pub other_character_id: String,
    pub relationship_type: RelationshipType,
    pub description: String,
    pub plot_relevance: Vec<String>, // Which plots this relationship affects
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: String,
    pub time_marker: String,
    pub event_type: EventType,
    pub description: String,
    pub affected_plots: Vec<String>,
    pub affected_characters: Vec<String>,
    pub location: String,
    pub significance: EventSignificance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotIntersection {
    pub other_plot_id: String,
    pub intersection_type: IntersectionType,
    pub description: String,
    pub timing: String, // When in the narrative this happens
    pub impact_level: ImpactLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSpan {
    pub start_marker: String,
    pub end_marker: Option<String>, // None if ongoing
    pub duration_description: String,
}

// Enums for the multi-plot system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NarrativeStyle {
    Linear,        // Traditional single-plot progression
    Parallel,      // Multiple simultaneous storylines
    Interwoven,    // Plots that regularly intersect
    Episodic,      // Separate but connected episodes
    Cyclical,      // Storylines that repeat themes/structures
    Experimental,  // Non-traditional narrative structures
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterRole {
    Protagonist,
    Antagonist,
    Supporting,
    Background,
    PlotDevice,
    Narrator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterStatus {
    Active,     // Currently involved in the narrative
    Inactive,   // Not currently in the story
    Mentioned,  // Referenced but not present
    Unknown,    // Status unclear
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    City,
    Building,
    Natural,
    Fantastical,
    Conceptual, // Abstract or metaphorical locations
    Traveling,  // Moving locations (ships, caravans, etc.)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotType {
    MainStory,      // Primary narrative arc
    Subplot,        // Secondary storyline
    Backstory,      // Historical context/flashbacks
    Parallel,       // Simultaneous alternate storyline
    Framing,        // Story-within-a-story setup
    Thematic,       // Reinforces themes rather than advancing plot
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotStatus {
    Planning,       // Being outlined
    Active,         // Currently being written
    Paused,         // Temporarily set aside
    Resolved,       // Completed
    Abandoned,      // No longer being developed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotStage {
    Introduction,   // Setting up the storyline
    Development,    // Building tension/complexity
    Climax,         // Peak dramatic moment
    Resolution,     // Wrapping up the thread
    Aftermath,      // Consequences/epilogue
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalStructure {
    Chronological,  // Events in time order
    Flashbacks,     // Past events interspersed
    Parallel,       // Multiple time periods simultaneously
    NonLinear,      // Complex time structure
    Cyclical,       // Repeating time patterns
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    Physics,        // How the world works physically
    Magic,          // Supernatural rules
    Social,         // Cultural/political structures
    Economic,       // How commerce/resources work
    Linguistic,     // Language rules
    Narrative,      // Story-specific constraints
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleScope {
    Global,         // Affects entire world
    Regional,       // Specific area
    PlotSpecific,   // Only relevant to certain storylines
    Character,      // Applies to specific characters
    Temporal,       // Time-period specific
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PointOfView {
    FirstPerson(String),    // Character ID for "I" narrator
    ThirdPersonLimited(String), // Character ID for limited POV
    ThirdPersonOmniscient,  // All-knowing narrator
    SecondPerson,           // "You" narrator (rare)
    Multiple,               // Switching between characters
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TensionLevel {
    Low,            // Calm, exposition
    Building,       // Rising tension
    High,           // Intense drama/action
    Climactic,      // Peak tension
    Release,        // Resolution/aftermath
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Family,
    Romantic,
    Friendship,
    Professional,
    Adversarial,
    Mentor,
    Unknown,
    Complex,        // Multiple or changing relationships
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Plot,           // Advances storyline
    Character,      // Character development
    World,          // World-building
    Conflict,       // Introduces/resolves conflict
    Revelation,     // Information revealed
    Transition,     // Scene/time/location change
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSignificance {
    Minor,          // Small detail
    Moderate,       // Noticeable but not crucial
    Major,          // Important plot point
    Critical,       // Pivotal moment
    Climactic,      // Peak dramatic moment
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntersectionType {
    CharacterMeeting,   // Characters from different plots meet
    SharedEvent,        // Same event affects multiple plots
    CauseEffect,        // One plot causes events in another
    Thematic,           // Plots share themes/parallels
    LocationBased,      // Plots occur in same location
    Temporal,           // Time-based connection
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Minimal,        // Barely affects other plot
    Low,            // Small influence
    Moderate,       // Noticeable impact
    High,           // Significant effect
    Transformative, // Fundamentally changes other plot
}

// Advanced Literary Stylistic Enhancement System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylisticProfile {
    pub narrative_voice: NarrativeVoice,
    pub tone_modulation: ToneModulation,
    pub symbolic_elements: Vec<SymbolicElement>,
    pub pacing_profile: PacingProfile,
    pub figurative_language: FigurativeLanguageSystem,
    pub dialogue_system: DialogueSystem,
    pub sensory_profile: SensoryProfile,
    pub genre_overlays: Vec<GenreOverlay>,
    pub meta_narrative: Option<MetaNarrativeElements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeVoice {
    pub primary_pov: PointOfViewStyle,
    pub voice_consistency: VoiceConsistency,
    pub reliability: NarratorReliability,
    pub intimacy_level: IntimacyLevel,
    pub temporal_perspective: TemporalPerspective,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneModulation {
    pub base_tone: EmotionalTone,
    pub dynamic_shifts: Vec<ToneShift>,
    pub tone_triggers: Vec<ToneTrigger>,
    pub intensity_curve: IntensityCurve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicElement {
    pub id: String,
    pub symbol_type: SymbolType,
    pub core_meaning: String,
    pub evolution_stages: Vec<SymbolEvolution>,
    pub context_associations: Vec<ContextAssociation>,
    pub recurrence_pattern: RecurrencePattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingProfile {
    pub default_rhythm: WritingRhythm,
    pub scene_pacing: std::collections::HashMap<String, ScenePacing>, // Scene type -> pacing
    pub tension_curve: Vec<PacingPoint>,
    pub breath_patterns: Vec<BreathPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigurativeLanguageSystem {
    pub character_metaphor_styles: std::collections::HashMap<String, MetaphorStyle>, // Character -> style
    pub thematic_imagery: Vec<ImageryTheme>,
    pub figurative_density: FigurativeDensity,
    pub contextual_triggers: Vec<FigurativeTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSystem {
    pub character_voices: std::collections::HashMap<String, CharacterVoice>, // Character -> voice
    pub subtext_engine: SubtextEngine,
    pub power_dynamics: PowerDynamicsTracker,
    pub dialogue_tags: DialogueTagStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryProfile {
    pub sensory_priorities: Vec<SensoryChannel>,
    pub descriptive_density: std::collections::HashMap<String, DescriptiveDensity>, // Scene type -> density
    pub sensory_associations: Vec<SensoryAssociation>,
    pub synesthesia_patterns: Vec<SynesthesiaPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreOverlay {
    pub genre_type: GenreType,
    pub intensity: f32, // 0.0-1.0
    pub stylistic_markers: Vec<StylisticMarker>,
    pub lexical_preferences: LexicalPreferences,
    pub structural_influences: StructuralInfluences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaNarrativeElements {
    pub self_awareness_level: SelfAwarenessLevel,
    pub meta_commentary: Vec<MetaCommentary>,
    pub structural_references: Vec<StructuralReference>,
    pub reader_address: ReaderAddressStyle,
}

// Supporting structures for the stylistic system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneShift {
    pub trigger: String,
    pub from_tone: EmotionalTone,
    pub to_tone: EmotionalTone,
    pub transition_style: TransitionStyle,
    pub duration: ToneShiftDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolEvolution {
    pub stage: String,
    pub meaning_shift: String,
    pub context_change: String,
    pub manifestation: SymbolManifestation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenePacing {
    pub sentence_length: SentenceLength,
    pub paragraph_structure: ParagraphStructure,
    pub rhythm_pattern: RhythmPattern,
    pub tension_markers: Vec<TensionMarker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterVoice {
    pub idiolect: Idiolect,
    pub speech_patterns: Vec<SpeechPattern>,
    pub vocabulary_level: VocabularyLevel,
    pub emotional_markers: Vec<EmotionalMarker>,
    pub power_indicators: PowerIndicators,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageryTheme {
    pub theme_name: String,
    pub core_images: Vec<String>,
    pub metaphorical_domains: Vec<MetaphoricalDomain>,
    pub emotional_resonance: EmotionalResonance,
}

// Enums for the stylistic system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PointOfViewStyle {
    FirstPersonIntimate,    // "I felt..."
    FirstPersonDetached,    // "I observed..."
    ThirdPersonLimited,     // Close to one character
    ThirdPersonOmniscient,  // All-knowing
    ThirdPersonObjective,   // Camera-like
    SecondPersonDirect,     // "You walk..."
    StreamOfConsciousness, // Joyce/Woolf style
    Epistolary,            // Letters, documents
    MultipleNarrators,     // Switching POVs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionalTone {
    Melancholic,   // Deep sadness, reflection
    Euphoric,      // Joy, excitement
    Ironic,        // Dry, sardonic
    Clinical,      // Detached, analytical
    Lyrical,       // Poetic, flowing
    Urgent,        // Immediate, pressing
    Contemplative, // Thoughtful, philosophical
    Bitter,        // Resentful, harsh
    Whimsical,     // Playful, light
    Ominous,       // Foreboding, dark
    Nostalgic,     // Wistful, remembering
    Passionate,    // Intense, fervent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolType {
    ObjectSymbol,     // Physical objects with meaning
    ColorSymbol,      // Color-based symbolism
    NatureSymbol,     // Natural elements
    ArchetypalSymbol, // Universal symbols
    PersonalSymbol,   // Character-specific
    CulturalSymbol,   // Society-specific
    MetaSymbol,       // Self-referential
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WritingRhythm {
    Staccato,      // Short, choppy sentences
    Flowing,       // Long, connected prose
    Syncopated,    // Irregular, jazz-like
    Hypnotic,      // Repetitive, trance-like
    Conversational, // Natural speech patterns
    Formal,        // Structured, academic
    Breathless,    // Run-on, urgent
    Measured,      // Careful, deliberate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensoryChannel {
    Visual,      // Sight-based descriptions
    Auditory,    // Sound and hearing
    Tactile,     // Touch and texture
    Olfactory,   // Smell
    Gustatory,   // Taste
    Kinesthetic, // Movement and position
    Emotional,   // Emotional sensations
    Synesthetic, // Combined senses
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenreType {
    NoirCrime,        // Dark, urban, cynical
    GothicHorror,     // Atmospheric, sublime
    SciFiHard,        // Technical, precise
    SciFiSoft,        // Social, speculative
    MagicalRealism,   // Subtle fantastic elements
    HistoricalLiterary, // Period-appropriate, literary
    ContemporaryLiterary, // Modern literary fiction
    RomanticDrama,    // Emotional, relationship-focused
    ThrillerSuspense, // Fast-paced, tense
    FantasyEpic,      // Grand, mythic
    PostModern,       // Experimental, fragmented
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelfAwarenessLevel {
    None,        // Traditional narrative
    Subtle,      // Occasional nods to structure
    Moderate,    // Regular meta-commentary
    Explicit,    // Direct reader address
    Experimental, // Postmodern techniques
}

// Additional supporting enums
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoiceConsistency {
    Rigid,       // Never changes
    Stable,      // Minor variations
    Fluid,       // Adapts to situation
    Chameleon,   // Major shifts
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NarratorReliability {
    Reliable,        // Trustworthy narrator
    SlightlyUnreliable, // Minor inconsistencies
    ModeratelyUnreliable, // Significant bias/gaps
    HighlyUnreliable, // Major deception/delusion
    UnknownReliability, // Ambiguous
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntimacyLevel {
    Distant,     // Formal, removed
    Professional, // Respectful distance
    Friendly,    // Warm but boundaried
    Intimate,    // Personal, close
    Confessional, // Very personal
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FigurativeDensity {
    Sparse,      // Minimal figurative language
    Moderate,    // Balanced use
    Rich,        // Frequent metaphors/similes
    Lyrical,     // Poetry-like density
    Experimental, // Unusual combinations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentenceLength {
    VeryShort,   // 1-5 words average
    Short,       // 6-10 words average
    Medium,      // 11-20 words average
    Long,        // 21-30 words average
    VeryLong,    // 30+ words average
    Mixed,       // Varied lengths
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VocabularyLevel {
    Simple,      // Common words
    Standard,    // Educated level
    Sophisticated, // Advanced vocabulary
    Archaic,     // Period/formal language
    Technical,   // Specialized terms
    Poetic,      // Lyrical language
    Vernacular,  // Colloquial/slang
}

// Implementation structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneTrigger {
    pub trigger_type: TriggerType,
    pub condition: String,
    pub target_tone: EmotionalTone,
    pub strength: f32, // 0.0-1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAssociation {
    pub context: String,
    pub association_strength: f32,
    pub emotional_valence: EmotionalValence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphorStyle {
    pub primary_domain: MetaphoricalDomain,
    pub secondary_domains: Vec<MetaphoricalDomain>,
    pub complexity_level: ComplexityLevel,
    pub originality: OriginalityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechPattern {
    pub pattern_type: SpeechPatternType,
    pub frequency: f32, // 0.0-1.0
    pub examples: Vec<String>,
    pub context_triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylisticMarker {
    pub marker_type: MarkerType,
    pub implementation: String,
    pub frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerType {
    EmotionalState,
    PlotEvent,
    CharacterInteraction,
    Setting,
    Time,
    Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaphoricalDomain {
    Nature,      // Plants, animals, weather
    Architecture, // Buildings, structures
    Music,       // Sound, rhythm, harmony
    War,         // Battle, conflict
    Journey,     // Travel, paths
    Ocean,       // Water, tides, depths
    Light,       // Illumination, shadow
    Fire,        // Heat, burning, destruction
    Art,         // Painting, sculpture
    Technology,  // Modern metaphors
    Body,        // Physical, anatomical
    Mind,        // Psychology, consciousness
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionalValence {
    VeryNegative,
    Negative,
    Neutral,
    Positive,
    VeryPositive,
    Ambivalent, // Mixed emotions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeechPatternType {
    Interruption,    // Often cuts off mid-sentence
    Repetition,      // Repeats words/phrases
    Hesitation,      // Uses filler words
    Formality,       // Formal speech patterns
    Colloquialism,   // Uses slang/informal
    Precision,       // Very exact word choice
    Rambling,        // Long, wandering sentences
    Clipped,         // Short, to the point
}

// Placeholder types for complex structures
pub type IntensityCurve = Vec<(f32, EmotionalTone)>; // Time -> Tone intensity
pub type SubtextEngine = Vec<SubtextRule>; // TODO: implement SubtextRule
// SubtextRule struct defined below
pub type PowerDynamicsTracker = std::collections::HashMap<String, f32>; // Character power levels
// StylisticMarker struct defined below, removing duplicate type alias
pub type RecurrencePattern = Vec<i32>; // Section numbers where symbol appears
pub type BreathPattern = Vec<PacingPoint>;
pub type PacingPoint = (usize, f32); // Position, intensity
pub type SymbolManifestation = String;
pub type TemporalPerspective = String;
pub type TransitionStyle = String;
pub type ToneShiftDuration = String;
pub type ParagraphStructure = String;
pub type RhythmPattern = String;
pub type TensionMarker = String;
pub type Idiolect = Vec<String>;
pub type EmotionalMarker = String;
pub type PowerIndicators = Vec<String>;
pub type EmotionalResonance = f32;
pub type DialogueTagStyle = String;
pub type DescriptiveDensity = f32;
pub type SensoryAssociation = String;
pub type SynesthesiaPattern = String;
pub type LexicalPreferences = Vec<String>;
pub type StructuralInfluences = Vec<String>;
pub type MetaCommentary = String;
pub type StructuralReference = String;
pub type ReaderAddressStyle = String;
pub type ComplexityLevel = f32;
pub type OriginalityLevel = f32;
pub type MarkerType = String;
pub type PowerLevel = f32;
pub type FigurativeTrigger = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtextRule {
    pub surface_meaning: String,
    pub hidden_meaning: String,
    pub revelation_method: RevelationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RevelationMethod {
    Implication,   // Suggested through context
    Contradiction, // Actions contradict words
    Repetition,    // Emphasized through patterns
    Omission,      // What's not said
    Timing,        // When something is said
}

// Default implementations for stylistic profile system
impl Default for StylisticProfile {
    fn default() -> Self {
        Self {
            narrative_voice: NarrativeVoice::default(),
            tone_modulation: ToneModulation::default(),
            symbolic_elements: Vec::new(),
            pacing_profile: PacingProfile::default(),
            figurative_language: FigurativeLanguageSystem::default(),
            dialogue_system: DialogueSystem::default(),
            sensory_profile: SensoryProfile::default(),
            genre_overlays: Vec::new(),
            meta_narrative: None,
        }
    }
}

impl Default for NarrativeVoice {
    fn default() -> Self {
        Self {
            primary_pov: PointOfViewStyle::ThirdPersonLimited,
            voice_consistency: VoiceConsistency::Stable,
            reliability: NarratorReliability::Reliable,
            intimacy_level: IntimacyLevel::Friendly,
            temporal_perspective: "present".to_string(),
        }
    }
}

impl Default for ToneModulation {
    fn default() -> Self {
        Self {
            base_tone: EmotionalTone::Contemplative,
            dynamic_shifts: Vec::new(),
            tone_triggers: Vec::new(),
            intensity_curve: Vec::new(), // Vec<(f32, EmotionalTone)>
        }
    }
}

impl Default for PacingProfile {
    fn default() -> Self {
        Self {
            default_rhythm: WritingRhythm::Conversational,
            scene_pacing: std::collections::HashMap::new(),
            tension_curve: Vec::new(),
            breath_patterns: Vec::new(),
        }
    }
}

impl Default for FigurativeLanguageSystem {
    fn default() -> Self {
        Self {
            character_metaphor_styles: std::collections::HashMap::new(),
            thematic_imagery: Vec::new(),
            figurative_density: FigurativeDensity::Moderate,
            contextual_triggers: Vec::new(),
        }
    }
}

impl Default for DialogueSystem {
    fn default() -> Self {
        Self {
            character_voices: std::collections::HashMap::new(),
            subtext_engine: Vec::new(), // Vec<SubtextRule>
            power_dynamics: std::collections::HashMap::new(), // HashMap<String, f32>  
            dialogue_tags: "minimal".to_string(),
        }
    }
}

impl Default for SensoryProfile {
    fn default() -> Self {
        Self {
            sensory_priorities: vec![SensoryChannel::Visual],
            descriptive_density: std::collections::HashMap::new(),
            sensory_associations: Vec::new(),
            synesthesia_patterns: Vec::new(),
        }
    }
}