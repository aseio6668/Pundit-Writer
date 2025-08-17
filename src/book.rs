use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub writing_style: String,
    pub premise: String,
    pub outline: String,
    pub chapters: Vec<Chapter>,
    pub metadata: BookMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub number: usize,
    pub title: String,
    pub content: String,
    pub word_count: usize,
    pub outline: String,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMetadata {
    pub target_size: String,
    pub target_word_count: Option<usize>,
    pub current_word_count: usize,
    pub target_chapters: usize,
    pub model_used: String,
    pub generation_parameters: GenerationParameters,
    pub estimated_completion: Option<f32>, // Percentage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParameters {
    pub temperature: f32,
    pub max_tokens_per_chapter: u32,
    pub context_window: usize,
}

impl Book {
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
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            title,
            author,
            genre,
            writing_style,
            premise,
            outline: String::new(),
            chapters: Vec::new(),
            metadata: BookMetadata {
                target_size,
                target_word_count,
                current_word_count: 0,
                target_chapters,
                model_used: model,
                generation_parameters: GenerationParameters {
                    temperature: 0.8,
                    max_tokens_per_chapter: 2000,
                    context_window: 3,
                },
                estimated_completion: Some(0.0),
            },
            created_at: now,
            updated_at: now,
            completed: false,
        }
    }
    
    pub fn add_chapter(&mut self, chapter: Chapter) {
        self.metadata.current_word_count += chapter.word_count;
        self.chapters.push(chapter);
        self.updated_at = Utc::now();
        self.update_completion_estimate();
    }
    
    pub fn update_completion_estimate(&mut self) {
        if let Some(target) = self.metadata.target_word_count {
            let progress = (self.metadata.current_word_count as f32) / (target as f32);
            self.metadata.estimated_completion = Some((progress * 100.0).min(100.0));
        } else {
            // For unlimited mode, base on chapter count
            let chapter_progress = (self.chapters.len() as f32) / (self.metadata.target_chapters as f32);
            self.metadata.estimated_completion = Some((chapter_progress * 100.0).min(100.0));
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
    
    pub fn get_context_for_next_chapter(&self) -> String {
        let context_chapters = self.metadata.generation_parameters.context_window;
        let start_idx = if self.chapters.len() > context_chapters {
            self.chapters.len() - context_chapters
        } else {
            0
        };
        
        let mut context = format!(
            "Book: {}\nGenre: {}\nStyle: {}\nPremise: {}\n\n",
            self.title, self.genre, self.writing_style, self.premise
        );
        
        if !self.outline.is_empty() {
            context.push_str(&format!("Outline:\n{}\n\n", self.outline));
        }
        
        if !self.chapters.is_empty() {
            context.push_str("Previous chapters:\n");
            for chapter in &self.chapters[start_idx..] {
                context.push_str(&format!(
                    "Chapter {}: {}\n{}\n\n",
                    chapter.number, chapter.title, 
                    truncate_text(&chapter.content, 500)
                ));
            }
        }
        
        context
    }
    
    pub fn to_text(&self) -> String {
        let mut book_text = format!(
            "{}\n\
            by {}\n\
            \n\
            Genre: {}\n\
            Style: {}\n\
            \n\
            {}\n\
            \n",
            self.title, self.author, self.genre, self.writing_style, self.premise
        );
        
        for chapter in &self.chapters {
            book_text.push_str(&format!(
                "\n\n--- Chapter {} ---\n\
                {}\n\
                \n\
                {}\n",
                chapter.number, chapter.title, chapter.content
            ));
        }
        
        book_text
    }
    
    pub fn to_markdown(&self) -> String {
        let mut markdown = format!(
            "# {}\n\
            \n\
            **Author:** {}\n\
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
            self.genre,
            self.writing_style,
            self.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            self.metadata.current_word_count,
            self.premise
        );
        
        for chapter in &self.chapters {
            markdown.push_str(&format!(
                "## Chapter {}: {}\n\
                \n\
                {}\n\
                \n",
                chapter.number, chapter.title, chapter.content
            ));
        }
        
        markdown
    }
}

impl Chapter {
    pub fn new(number: usize, title: String, outline: String) -> Self {
        Self {
            number,
            title,
            content: String::new(),
            word_count: 0,
            outline,
            created_at: Utc::now(),
            completed: false,
        }
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
pub struct BookProgress {
    pub current_chapter: usize,
    pub total_chapters: usize,
    pub current_words: usize,
    pub target_words: Option<usize>,
    pub estimated_completion: f32,
}

impl BookProgress {
    pub fn from_book(book: &Book) -> Self {
        Self {
            current_chapter: book.chapters.len(),
            total_chapters: book.metadata.target_chapters,
            current_words: book.metadata.current_word_count,
            target_words: book.metadata.target_word_count,
            estimated_completion: book.metadata.estimated_completion.unwrap_or(0.0),
        }
    }
}