// File continuation system for Pundit Writer
use anyhow::{anyhow, Result};
use dialoguer::{Input, Select, Confirm};
use std::fs;
use std::path::PathBuf;
use crate::content::{Content, ContentType};

#[derive(Debug, Clone)]
pub struct ContinuationFile {
    pub path: PathBuf,
    pub content_type: Option<ContentType>,
    pub word_count: usize,
    pub exists: bool,
}

impl ContinuationFile {
    pub fn new(path: PathBuf) -> Self {
        let exists = path.exists();
        let (content_type, word_count) = if exists {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    let word_count = content.split_whitespace().count();
                    let content_type = detect_content_type_from_content(&content);
                    (Some(content_type), word_count)
                },
                Err(_) => (None, 0),
            }
        } else {
            (None, 0)
        };

        ContinuationFile {
            path,
            content_type,
            word_count,
            exists,
        }
    }

    pub fn display_info(&self) -> String {
        if self.exists {
            let type_str = match &self.content_type {
                Some(ct) => format!("{:?}", ct),
                None => "Unknown".to_string(),
            };
            format!(
                "{} ({} words, {})",
                self.path.display(),
                self.word_count,
                type_str
            )
        } else {
            format!("{} (file not found)", self.path.display())
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContinuationProject {
    pub files: Vec<ContinuationFile>,
    pub primary_file: Option<PathBuf>,
    pub project_type: Option<ContentType>,
}

impl ContinuationProject {
    pub fn new() -> Self {
        ContinuationProject {
            files: Vec::new(),
            primary_file: None,
            project_type: None,
        }
    }

    pub fn add_file(&mut self, path: PathBuf) -> Result<()> {
        // Check if file already exists in the list
        if self.files.iter().any(|f| f.path == path) {
            return Err(anyhow!("File already added to the project"));
        }

        let continuation_file = ContinuationFile::new(path.clone());
        
        // If this is the first file, make it the primary file
        if self.files.is_empty() {
            self.primary_file = Some(path);
            self.project_type = continuation_file.content_type.clone();
        }

        self.files.push(continuation_file);
        Ok(())
    }

    pub fn remove_file(&mut self, index: usize) -> Result<()> {
        if index >= self.files.len() {
            return Err(anyhow!("Invalid file index"));
        }

        let removed_file = self.files.remove(index);

        // If we removed the primary file, update it
        if Some(&removed_file.path) == self.primary_file.as_ref() {
            self.primary_file = self.files.first().map(|f| f.path.clone());
            self.project_type = self.files.first().and_then(|f| f.content_type.clone());
        }

        Ok(())
    }

    pub fn get_combined_content(&self) -> Result<String> {
        let mut combined_content = String::new();
        
        for file in &self.files {
            if file.exists {
                match fs::read_to_string(&file.path) {
                    Ok(content) => {
                        combined_content.push_str(&format!("\n\n--- Content from {} ---\n\n", file.path.display()));
                        combined_content.push_str(&content);
                    },
                    Err(e) => {
                        eprintln!("Warning: Could not read file {}: {}", file.path.display(), e);
                    }
                }
            }
        }

        if combined_content.is_empty() {
            return Err(anyhow!("No readable files found in the project"));
        }

        Ok(combined_content)
    }

    pub fn total_word_count(&self) -> usize {
        self.files.iter().map(|f| f.word_count).sum()
    }

    pub fn validate_project(&self) -> Result<()> {
        if self.files.is_empty() {
            return Err(anyhow!("No files added to the project"));
        }

        let existing_files: Vec<_> = self.files.iter().filter(|f| f.exists).collect();
        if existing_files.is_empty() {
            return Err(anyhow!("None of the added files exist or are readable"));
        }

        Ok(())
    }
}

// Interactive file selection and management
pub async fn interactive_continuation_setup() -> Result<ContinuationProject> {
    let mut project = ContinuationProject::new();

    println!("\n📂 Setting up Content Continuation");
    println!("Add the files you want to continue working on...\n");

    loop {
        // Show current project status
        if !project.files.is_empty() {
            println!("\nCurrent project files:");
            for (i, file) in project.files.iter().enumerate() {
                let status = if file.exists { "✅" } else { "❌" };
                println!("  {}. {} {}", i + 1, status, file.display_info());
            }
            println!("Total words: {}\n", project.total_word_count());
        }

        // Main menu
        let mut options = vec![
            "📁 Add file to project",
        ];

        if !project.files.is_empty() {
            options.push("📝 Review project files");
            options.push("🗑️ Remove a file");
            options.push("✅ Continue with this project");
        }

        options.push("← Back to main menu");

        let choice = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        match choice {
            0 => {
                // Add file
                if let Err(e) = add_file_to_project(&mut project).await {
                    println!("❌ Error adding file: {}", e);
                    continue;
                }
            },
            i if options[i] == "📝 Review project files" => {
                review_project_files(&project)?;
            },
            i if options[i] == "🗑️ Remove a file" => {
                if let Err(e) = remove_file_from_project(&mut project) {
                    println!("❌ Error removing file: {}", e);
                }
            },
            i if options[i] == "✅ Continue with this project" => {
                match project.validate_project() {
                    Ok(_) => {
                        println!("✅ Project validated successfully!");
                        return Ok(project);
                    },
                    Err(e) => {
                        println!("❌ Project validation failed: {}", e);
                        continue;
                    }
                }
            },
            _ => {
                // Back to main menu
                return Err(anyhow!("User cancelled continuation setup"));
            }
        }
    }
}

async fn add_file_to_project(project: &mut ContinuationProject) -> Result<()> {
    loop {
        let file_path: String = Input::new()
            .with_prompt("Enter file path (or 'back' to return)")
            .interact_text()?;

        if file_path.trim().to_lowercase() == "back" {
            return Ok(());
        }

        let path = PathBuf::from(file_path.trim());

        match project.add_file(path.clone()) {
            Ok(_) => {
                let file = project.files.last().unwrap();
                if file.exists {
                    println!("✅ Added: {} ({} words)", path.display(), file.word_count);
                } else {
                    println!("⚠️ Added: {} (file not found - will be created)", path.display());
                }

                // Ask if they want to add more files
                let add_more = Confirm::new()
                    .with_prompt("Add another file?")
                    .default(false)
                    .interact()?;

                if !add_more {
                    break;
                }
            },
            Err(e) => {
                println!("❌ Error: {}", e);
                continue;
            }
        }
    }

    Ok(())
}

fn remove_file_from_project(project: &mut ContinuationProject) -> Result<()> {
    if project.files.is_empty() {
        return Err(anyhow!("No files to remove"));
    }

    let file_options: Vec<String> = project.files
        .iter()
        .enumerate()
        .map(|(i, f)| format!("{}. {}", i + 1, f.display_info()))
        .collect();

    let mut options = file_options;
    options.push("← Cancel".to_string());

    let choice = Select::new()
        .with_prompt("Which file would you like to remove?")
        .items(&options)
        .default(0)
        .interact()?;

    if choice == options.len() - 1 {
        return Ok(()); // Cancel
    }

    project.remove_file(choice)?;
    println!("✅ File removed from project");

    Ok(())
}

fn review_project_files(project: &ContinuationProject) -> Result<()> {
    if project.files.is_empty() {
        println!("No files in the project.");
        return Ok(());
    }

    println!("\n📋 Project Review");
    println!("================");

    if let Some(ref primary) = project.primary_file {
        println!("Primary file: {}", primary.display());
    }

    if let Some(ref project_type) = project.project_type {
        println!("Detected type: {:?}", project_type);
    }

    println!("Total files: {}", project.files.len());
    println!("Total words: {}", project.total_word_count());

    println!("\nFiles:");
    for (i, file) in project.files.iter().enumerate() {
        let status = if file.exists { "✅ Exists" } else { "❌ Not found" };
        println!("  {}. {} - {}", i + 1, file.display_info(), status);
    }

    // Ask if they want to preview content
    if project.files.iter().any(|f| f.exists) {
        let preview = Confirm::new()
            .with_prompt("Would you like to preview the combined content?")
            .default(false)
            .interact()?;

        if preview {
            match project.get_combined_content() {
                Ok(content) => {
                    let preview_length = 500; // Show first 500 characters
                    if content.len() > preview_length {
                        println!("\nContent preview (first {} characters):", preview_length);
                        println!("{}", &content[..preview_length]);
                        println!("... [truncated] ...");
                    } else {
                        println!("\nFull content:");
                        println!("{}", content);
                    }
                },
                Err(e) => {
                    println!("❌ Error reading content: {}", e);
                }
            }
        }
    }

    println!("\nPress Enter to continue...");
    let _: String = Input::new().with_prompt("").allow_empty(true).interact_text()?;

    Ok(())
}

// Content type detection from file content
fn detect_content_type_from_content(content: &str) -> ContentType {
    let content_lower = content.to_lowercase();

    // Look for common patterns to detect content type
    if content_lower.contains("chapter") && (content_lower.contains("novel") || content_lower.contains("book")) {
        ContentType::Book
    } else if content_lower.contains("scene") && content_lower.contains("fade in") {
        ContentType::Screenplay
    } else if content_lower.contains("act") && content_lower.contains("scene") && content_lower.contains("stage") {
        ContentType::Play
    } else if content_lower.contains("stanza") || content_lower.contains("verse") || content_lower.contains("poem") {
        ContentType::Poetry
    } else if content_lower.contains("# ") || content_lower.contains("## ") {
        // Markdown-style headers suggest technical documentation
        ContentType::TechnicalDoc
    } else if content_lower.contains("meeting") && content_lower.contains("agenda") {
        ContentType::MeetingNotes
    } else if content_lower.contains("marketing") || content_lower.contains("campaign") {
        ContentType::MarketingAd
    } else if content_lower.contains("blog") || content_lower.contains("article") {
        ContentType::BlogPost
    } else if content_lower.contains("strategic") || content_lower.contains("business plan") {
        ContentType::StrategicDoc
    } else if content_lower.contains("research") || content_lower.contains("white paper") {
        ContentType::ResearchReport
    } else {
        ContentType::Document // Default fallback
    }
}

// Convert ContinuationProject to Content for generation
pub fn continuation_project_to_content(
    project: &ContinuationProject,
    title: String,
    author: String,
) -> Result<Content> {
    let combined_content = project.get_combined_content()?;
    let content_type = project.project_type.clone().unwrap_or(ContentType::Document);
    
    // Create a new Content instance with the existing content
    let mut content = Content::new_document(
        title,
        author,
        "Continuation".to_string(),
        "Continuing existing work".to_string(),
        1, // Will be updated
        crate::content::DocumentFormat::Educational,
        "default".to_string(),
    );

    content.content_type = content_type;
    content.metadata.current_word_count = project.total_word_count();
    
    // Add existing content as context
    content.outline = format!("Continuation of existing work.\n\nExisting content summary:\n{}", 
        &combined_content[..combined_content.len().min(1000)]); // First 1000 chars as outline

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_continuation_file_creation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        
        // Test non-existent file
        let file = ContinuationFile::new(file_path.clone());
        assert!(!file.exists);
        assert_eq!(file.word_count, 0);

        // Create file and test again
        fs::write(&file_path, "Hello world test content").unwrap();
        let file = ContinuationFile::new(file_path);
        assert!(file.exists);
        assert_eq!(file.word_count, 4);
    }

    #[test]
    fn test_project_management() {
        let mut project = ContinuationProject::new();
        let path1 = PathBuf::from("test1.txt");
        let path2 = PathBuf::from("test2.txt");

        // Test adding files
        assert!(project.add_file(path1.clone()).is_ok());
        assert!(project.add_file(path2.clone()).is_ok());
        assert_eq!(project.files.len(), 2);

        // Test duplicate file
        assert!(project.add_file(path1).is_err());

        // Test removing file
        assert!(project.remove_file(0).is_ok());
        assert_eq!(project.files.len(), 1);
    }

    #[test]
    fn test_content_type_detection() {
        assert_eq!(detect_content_type_from_content("Chapter 1: The Beginning of a Novel"), ContentType::Book);
        assert_eq!(detect_content_type_from_content("FADE IN: EXT. FOREST - DAY"), ContentType::Screenplay);
        assert_eq!(detect_content_type_from_content("ACT I SCENE 1 - The stage is set"), ContentType::Play);
        assert_eq!(detect_content_type_from_content("Roses are red\nViolets are blue"), ContentType::Poetry);
        assert_eq!(detect_content_type_from_content("# Installation Guide\n## Prerequisites"), ContentType::TechnicalDoc);
    }
}