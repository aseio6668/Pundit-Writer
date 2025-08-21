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