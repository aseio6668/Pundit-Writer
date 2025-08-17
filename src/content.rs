use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ContentType {
    Book,
    Screenplay,
    Play,
    TvScript,
    AudioScript,
    GameScript,
    Document,
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
                    max_tokens_per_chapter: 2000,
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
                    max_tokens_per_chapter: 1500,
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
                    max_tokens_per_chapter: 1200,
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
    
    pub fn to_text(&self) -> String {
        let content_type_name = match self.content_type {
            ContentType::Book => "Book",
            ContentType::Screenplay => "Screenplay",
            ContentType::Play => "Play",
            ContentType::TvScript => "TV Script",
            ContentType::AudioScript => "Audio Script",
            ContentType::GameScript => "Game Script", 
            ContentType::Document => "Document",
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
    
    pub fn to_markdown(&self) -> String {
        let content_type_name = match self.content_type {
            ContentType::Book => "Book",
            ContentType::Screenplay => "Screenplay",
            ContentType::Play => "Play",
            ContentType::TvScript => "TV Script",
            ContentType::AudioScript => "Audio Script",
            ContentType::GameScript => "Game Script",
            ContentType::Document => "Document",
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

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        text.to_string()
    } else {
        format!("{}...", &text[..max_chars])
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