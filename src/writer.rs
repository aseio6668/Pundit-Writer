use crate::cli_types::{Genre, WritingStyle, BookSize, ScreenplayLength, PlayLength, TvShowType, AudioType, GameGenre, DocumentType, DocumentLength, TechnicalDocType, ResearchDocType, ResearchLength, PoetryStyle, PersonalWritingType, PersonalLength, MarketingType, MarketingLength, BlogContentType, BlogLength, StrategicDocType, StrategicLength, MeetingDocType, MeetingLength};
use crate::models::{HuggingFaceClient, get_model_recommendation};
use crate::ollama::{OllamaClient, get_ollama_recommendation, get_download_instructions};
use crate::content::{Content, Section, SectionType, ContentType, DocumentFormat, Book, truncate_text, count_words,
    WorldState, PlotThread, PlotType, PlotStatus, PlotStage, MultiPlotCharacter, Location, Timeline, TimelineEvent, 
    WorldRule, NarrativeStyle, NarrativeContext, TimeSpan, CharacterRole, CharacterStatus, LocationType, 
    RuleType, RuleScope, PointOfView, TensionLevel, EventType, EventSignificance, TemporalStructure};
use crate::config::{Config, save_book_state};
use crate::poetry_enhancements::{EnhancedPoetryPrompt, create_emotion_from_theme, post_process_poetry};
use crate::continuation::{interactive_continuation_setup, continuation_project_to_content, ContinuationProject};
use crate::dynamic_length::{DynamicSectionLength, generate_dynamic_section_lengths};
use crate::resilient_writer::{create_resilient_writing_session, ResilientWriter, NonStopWritingMode};
use crate::temporal_engine::{TemporalEngine, ChapterTemporalContext};
use crate::advanced_creativity_engine::{AdvancedCreativityEngine, CreativeChapterPlan};
use crate::intelligent_progression_tracker::{IntelligentProgressionTracker, ChapterGenerationContext, GenerationMetrics, InterruptionType};
use anyhow::{Result, anyhow};
use dialoguer::{Input, Select, Confirm};
use indicatif::{ProgressBar, ProgressStyle};
use console::Term;
use std::path::PathBuf;
use chrono::Utc;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

// Custom result type to handle back navigation without exiting the app
#[derive(Debug)]
pub struct BackToMenu;

impl std::fmt::Display for BackToMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User selected back to menu")
    }
}

impl std::error::Error for BackToMenu {}

// Convenience type alias
pub type MenuResult<T> = std::result::Result<T, anyhow::Error>;

enum AIClient {
    HuggingFace(HuggingFaceClient),
    Ollama(OllamaClient),
}

async fn check_model_availability(client: &AIClient, model: &str) -> Result<()> {
    match client {
        AIClient::HuggingFace(hf_client) => {
            match hf_client.check_model_availability().await {
                Ok(true) => println!("✅ Model is available"),
                Ok(false) => {
                    println!("⚠️  Warning: Model may not be available or requires authentication");
                    println!("   Continuing anyway - some models work even when availability check fails");
                },
                Err(e) => {
                    println!("⚠️  Warning: Could not check model availability: {}", e);
                    println!("   Continuing anyway - will attempt to use the model");
                }
            }
        },
        AIClient::Ollama(ollama_client) => {
            match ollama_client.check_server().await {
                Ok(true) => {
                    println!("✅ Ollama server is running");
                    match ollama_client.list_models().await {
                        Ok(models) => {
                            if models.iter().any(|m| m.contains(model)) {
                                println!("✅ Model '{}' is available", model);
                            } else {
                                println!("⚠️  Model '{}' not found locally", model);
                                println!("   Available models: {:?}", models);
                                println!("   Run: ollama pull {}", model);
                            }
                        },
                        Err(_) => println!("⚠️  Could not list Ollama models, continuing anyway"),
                    }
                },
                Ok(false) => {
                    println!("❌ Ollama server not running");
                    println!("   Please start Ollama or install it from https://ollama.ai");
                    return Err(anyhow::anyhow!("Ollama server not available"));
                },
                Err(e) => {
                    println!("❌ Could not connect to Ollama: {}", e);
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}

pub async fn write_book(
    genre: Genre,
    style: WritingStyle,
    size: BookSize,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("🖋️  Pundit - AI Book Writer").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client based on local flag
    
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    // Check model availability
    println!("🔍 Checking model availability...");
    match &client {
        AIClient::HuggingFace(hf_client) => {
            match hf_client.check_model_availability().await {
                Ok(true) => println!("✅ Model is available"),
                Ok(false) => {
                    println!("⚠️  Warning: Model may not be available or requires authentication");
                    println!("   Continuing anyway - some models work even when availability check fails");
                },
                Err(e) => {
                    println!("⚠️  Warning: Could not check model availability: {}", e);
                    println!("   Continuing anyway - will attempt to use the model");
                }
            }
        },
        AIClient::Ollama(ollama_client) => {
            match ollama_client.check_server().await {
                Ok(true) => {
                    println!("✅ Ollama server is running");
                    match ollama_client.list_models().await {
                        Ok(models) => {
                            if models.iter().any(|m| m.contains(&model)) {
                                println!("✅ Model '{}' is available", model);
                            } else {
                                println!("⚠️  Model '{}' not found locally", model);
                                println!("   Available models: {:?}", models);
                                println!("   Run: ollama pull {}", model);
                            }
                        },
                        Err(_) => println!("⚠️  Could not list Ollama models, continuing anyway"),
                    }
                },
                Ok(false) => {
                    println!("❌ Ollama server not running at {}", ollama_url);
                    println!("   Please start Ollama or install it from https://ollama.ai");
                    return Err(anyhow::anyhow!("Ollama server not available"));
                },
                Err(e) => {
                    println!("❌ Could not connect to Ollama: {}", e);
                    return Err(e);
                }
            }
        }
    }
    
    // Get book details
    let title: String = Input::new()
        .with_prompt("📚 Book title")
        .interact_text()?;
    
    let premise: String = Input::new()
        .with_prompt("💡 Book premise (brief description)")
        .interact_text()?;
    
    let author = config.default_author.clone();
    
    // Apply creative optimizations for fiction before creating book
    let estimated_pages = size.word_target().unwrap_or(50000) / 250; // ~250 words per page
    
    // Create book instance
    let mut book = Book::new(
        title,
        author,
        genre.to_string(),
        style.to_string(),
        premise.clone(),
        format!("{:?}", size),
        size.word_target(),
        size.chapter_target(),
        model.clone(),
    );
    
    optimize_fiction_creativity(&genre, &premise, estimated_pages, &mut book)?;
    
    println!("\n🎯 Book Configuration:");
    println!("   Genre: {}", book.genre);
    println!("   Style: {}", book.writing_style);
    println!("   Target: {:?}", size);
    if let Some(target) = book.metadata.target_word_count {
        println!("   Words: {} target", target);
    }
    println!("   Sections: {} planned", book.metadata.target_sections);
    
    // Show any creative enhancements applied
    if book.world_state.is_some() {
        println!("   🎭 Enhanced: Multi-plot narrative enabled");
    }
    if let Some(ref profile) = book.stylistic_profile {
        if !profile.genre_overlays.is_empty() {
            println!("   🎨 Enhanced: Genre-specific stylistic optimizations applied");
        }
    }
    println!();
    
    // Generate book outline
    println!("📋 Generating book outline...");
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            match hf_client.generate_outline(
                &book.genre,
                &book.writing_style,
                &book.premise,
                book.metadata.target_sections,
            ).await {
                Ok(outline) => outline,
                Err(e) => {
                    eprintln!("❌ Failed to generate outline: {}", e);
                    eprintln!("💡 Suggestions:");
                    eprintln!("   1. Try a different model (use --model gpt2 or --model distilgpt2)");
                    eprintln!("   2. Add a Hugging Face API key (--api-key YOUR_KEY)");
                    eprintln!("   3. Check your internet connection");
                    eprintln!("   4. Try local models with --local flag");
                    return Err(e);
                }
            }
        },
        AIClient::Ollama(ollama_client) => {
            match ollama_client.generate_outline(
                &model,
                &book.genre,
                &book.writing_style,
                &book.premise,
                book.metadata.target_sections,
            ).await {
                Ok(outline) => outline,
                Err(e) => {
                    eprintln!("❌ Failed to generate outline: {}", e);
                    eprintln!("💡 Suggestions:");
                    eprintln!("   1. Make sure the model is downloaded: ollama pull {}", model);
                    eprintln!("   2. Try a different model: {}", get_ollama_recommendation(&size));
                    eprintln!("   3. Check if Ollama is running: ollama list");
                    eprintln!("\n{}", get_download_instructions(&model));
                    return Err(e);
                }
            }
        }
    };
    book.outline = outline;
    
    println!("✅ Outline generated!");
    println!("\n{}", console::style("Generated Outline:").bold());
    println!("{}", book.outline);
    println!();
    
    if !Confirm::new()
        .with_prompt("Continue with this outline?")
        .default(true)
        .interact()? {
        println!("📝 You can manually edit the outline if needed.");
        book.outline = Input::new()
            .with_prompt("Enter revised outline")
            .with_initial_text(&book.outline)
            .interact_text()?;
    }
    
    // Start writing process
    println!("\n🚀 Starting book writing process...");
    let progress_bar = create_progress_bar(&book);
    
    // Save initial state
    if config.auto_save {
        save_book_state(&book)?;
    }
    
    // Write chapters progressively
    let mut chapter_count = 1;
    while book.should_continue() && chapter_count <= book.metadata.target_sections {
        if let Err(e) = write_next_chapter(&client, &model, &mut book, chapter_count, &progress_bar).await {
            eprintln!("❌ Error writing chapter {}: {}", chapter_count, e);
            
            if Confirm::new()
                .with_prompt("Retry this chapter?")
                .default(true)
                .interact()? {
                continue;
            } else {
                break;
            }
        }
        
        // Auto-save after each chapter
        if config.auto_save && chapter_count % (config.save_frequency as usize) == 0 {
            save_book_state(&book)?;
        }
        
        chapter_count += 1;
        
        // For unlimited mode, check if we should continue
        if size == BookSize::Unlimited && chapter_count > book.metadata.target_sections {
            if !Confirm::new()
                .with_prompt("Continue writing more chapters?")
                .default(false)
                .interact()? {
                break;
            }
            book.metadata.target_sections += 10; // Extend target
        }
    }
    
    progress_bar.finish_with_message("✅ Book completed!");
    book.completed = true;
    
    // Final save
    save_book_state(&book)?;
    
    // Output the book
    output_book(&book, output, &config).await?;
    
    println!("\n🎉 Book writing completed!");
    println!("📊 Final Statistics:");
    println!("   Total sections: {}", book.chapters().len());
    println!("   Total words: {}", book.metadata.current_word_count);
    println!("   Average words per chapter: {}", 
        if book.chapters().is_empty() { 0 } else { book.metadata.current_word_count / book.chapters().len() }
    );
    
    Ok(())
}

pub async fn write_screenplay(
    genre: Genre,
    style: WritingStyle,
    length: ScreenplayLength,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let target_pages = match length {
        ScreenplayLength::Short => 25,
        ScreenplayLength::Feature => 110,
        ScreenplayLength::Epic => 150,
    };
    
    write_content(
        ContentType::Screenplay,
        genre.to_string(),
        style,
        format!("{} pages", target_pages),
        output,
        model,
        api_key,
        use_local,
        ollama_url,
        target_pages,
        SectionType::Scene,
        "Screenplay",
    ).await
}

pub async fn write_play(
    genre: Genre,
    style: WritingStyle,
    length: PlayLength,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let target_acts = match length {
        PlayLength::OneAct => 1,
        PlayLength::Full => 3,
        PlayLength::Musical => 2,
        PlayLength::Epic => 4,
    };
    
    write_content(
        ContentType::Play,
        genre.to_string(),
        style,
        format!("{} acts", target_acts),
        output,
        model,
        api_key,
        use_local,
        ollama_url,
        target_acts * 20, // Rough estimate: 20 pages per act
        SectionType::Act,
        "Stage Play",
    ).await
}

pub async fn write_tv_script(
    show_type: TvShowType,
    genre: Genre,
    style: WritingStyle,
    episodes: u32,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let show_type_str = match show_type {
        TvShowType::Sitcom => "Sitcom",
        TvShowType::Drama => "Drama Series",
        TvShowType::MiniSeries => "Mini-Series",
        TvShowType::Anthology => "Anthology Series",
        TvShowType::Comedy => "Comedy",
        TvShowType::Miniseries => "Miniseries",
        TvShowType::Special => "Special",
        TvShowType::Reality => "Reality TV",
        TvShowType::Documentary => "Documentary Series",
        TvShowType::News => "News Program",
        TvShowType::Talk => "Talk Show",
        TvShowType::Variety => "Variety Show",
    };
    
    write_content(
        ContentType::TvScript,
        format!("{} {}", show_type_str, genre),
        style,
        format!("{} episodes", episodes),
        output,
        model,
        api_key,
        use_local,
        ollama_url,
        episodes as usize * 25, // Rough estimate: 25 pages per episode
        SectionType::Episode,
        "TV Script",
    ).await
}

pub async fn write_audio_script(
    audio_type: AudioType,
    genre: Genre,
    style: WritingStyle,
    duration: u32,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let audio_type_str = match audio_type {
        AudioType::AudioDrama => "Audio Drama",
        AudioType::Podcast => "Podcast",
        AudioType::RadioPlay => "Radio Play",
        AudioType::Documentary => "Documentary",
        AudioType::RadioDrama => "Radio Drama",
        AudioType::Audiobook => "Audiobook",
        AudioType::Commercial => "Commercial",
    };
    
    let segments = (duration / 10).max(1); // Rough estimate: 10 minutes per segment
    
    write_content(
        ContentType::AudioScript,
        format!("{} {}", audio_type_str, genre),
        style,
        format!("{} minutes", duration),
        output,
        model,
        api_key,
        use_local,
        ollama_url,
        segments as usize * 5, // Rough estimate: 5 pages per segment
        SectionType::Segment,
        "Audio Script",
    ).await
}

pub async fn write_game_script(
    genre: GameGenre,
    style: WritingStyle,
    characters: u32,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let genre_str = match genre {
        GameGenre::RPG => "RPG",
        GameGenre::Adventure => "Adventure",
        GameGenre::VisualNovel => "Visual Novel",
        GameGenre::Action => "Action",
        GameGenre::Strategy => "Strategy",
        GameGenre::Horror => "Horror",
        GameGenre::SciFi => "Sci-Fi",
        GameGenre::Fantasy => "Fantasy",
        GameGenre::Mystery => "Mystery",
        GameGenre::Historical => "Historical",
        GameGenre::Contemporary => "Contemporary",
    };
    
    write_content(
        ContentType::GameScript,
        format!("{} Game", genre_str),
        style,
        format!("{} character interactions", characters),
        output,
        model,
        api_key,
        use_local,
        ollama_url,
        characters as usize * 3, // Rough estimate: 3 pages per character interaction
        SectionType::Interaction,
        "Game Script",
    ).await
}

pub async fn write_document(
    doc_type: DocumentType,
    style: WritingStyle,
    length: DocumentLength,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let (doc_type_str, target_pages) = match (&doc_type, &length) {
        (DocumentType::BusinessPlan, DocumentLength::Brief) => ("Business Plan", 5),
        (DocumentType::BusinessPlan, DocumentLength::Standard) => ("Business Plan", 15),
        (DocumentType::BusinessPlan, DocumentLength::Comprehensive) => ("Business Plan", 30),
        (DocumentType::BusinessPlan, DocumentLength::Extensive) => ("Business Plan", 60),
        (DocumentType::TechnicalManual, DocumentLength::Brief) => ("Technical Manual", 8),
        (DocumentType::TechnicalManual, DocumentLength::Standard) => ("Technical Manual", 20),
        (DocumentType::TechnicalManual, DocumentLength::Comprehensive) => ("Technical Manual", 40),
        (DocumentType::TechnicalManual, DocumentLength::Extensive) => ("Technical Manual", 80),
        (DocumentType::UserGuide, DocumentLength::Brief) => ("User Guide", 6),
        (DocumentType::UserGuide, DocumentLength::Standard) => ("User Guide", 15),
        (DocumentType::UserGuide, DocumentLength::Comprehensive) => ("User Guide", 25),
        (DocumentType::UserGuide, DocumentLength::Extensive) => ("User Guide", 50),
        (DocumentType::Report, DocumentLength::Brief) => ("Report", 3),
        (DocumentType::Report, DocumentLength::Standard) => ("Report", 10),
        (DocumentType::Report, DocumentLength::Comprehensive) => ("Report", 20),
        (DocumentType::Report, DocumentLength::Extensive) => ("Report", 40),
        (DocumentType::Proposal, DocumentLength::Brief) => ("Proposal", 4),
        (DocumentType::Proposal, DocumentLength::Standard) => ("Proposal", 12),
        (DocumentType::Proposal, DocumentLength::Comprehensive) => ("Proposal", 25),
        (DocumentType::Proposal, DocumentLength::Extensive) => ("Proposal", 50),
        (DocumentType::MarketingCopy, DocumentLength::Brief) => ("Marketing Copy", 2),
        (DocumentType::MarketingCopy, DocumentLength::Standard) => ("Marketing Copy", 6),
        (DocumentType::MarketingCopy, DocumentLength::Comprehensive) => ("Marketing Copy", 12),
        (DocumentType::MarketingCopy, DocumentLength::Extensive) => ("Marketing Copy", 25),
        (DocumentType::LegalTemplate, DocumentLength::Brief) => ("Legal Template", 3),
        (DocumentType::LegalTemplate, DocumentLength::Standard) => ("Legal Template", 8),
        (DocumentType::LegalTemplate, DocumentLength::Comprehensive) => ("Legal Template", 15),
        (DocumentType::LegalTemplate, DocumentLength::Extensive) => ("Legal Template", 30),
        (DocumentType::LessonPlan, DocumentLength::Brief) => ("Lesson Plan", 2),
        (DocumentType::LessonPlan, DocumentLength::Standard) => ("Lesson Plan", 5),
        (DocumentType::LessonPlan, DocumentLength::Comprehensive) => ("Lesson Plan", 10),
        (DocumentType::LessonPlan, DocumentLength::Extensive) => ("Lesson Plan", 20),
        (DocumentType::CourseOutline, DocumentLength::Brief) => ("Course Outline", 4),
        (DocumentType::CourseOutline, DocumentLength::Standard) => ("Course Outline", 12),
        (DocumentType::CourseOutline, DocumentLength::Comprehensive) => ("Course Outline", 25),
        (DocumentType::CourseOutline, DocumentLength::Extensive) => ("Course Outline", 50),
        (DocumentType::Business, DocumentLength::Brief) => ("Business Document", 3),
        (DocumentType::Business, DocumentLength::Standard) => ("Business Document", 10),
        (DocumentType::Business, DocumentLength::Comprehensive) => ("Business Document", 20),
        (DocumentType::Business, DocumentLength::Extensive) => ("Business Document", 40),
        (DocumentType::Academic, DocumentLength::Brief) => ("Academic Document", 5),
        (DocumentType::Academic, DocumentLength::Standard) => ("Academic Document", 15),
        (DocumentType::Academic, DocumentLength::Comprehensive) => ("Academic Document", 30),
        (DocumentType::Academic, DocumentLength::Extensive) => ("Academic Document", 60),
        (DocumentType::Technical, DocumentLength::Brief) => ("Technical Document", 6),
        (DocumentType::Technical, DocumentLength::Standard) => ("Technical Document", 18),
        (DocumentType::Technical, DocumentLength::Comprehensive) => ("Technical Document", 35),
        (DocumentType::Technical, DocumentLength::Extensive) => ("Technical Document", 70),
        (DocumentType::Legal, DocumentLength::Brief) => ("Legal Document", 3),
        (DocumentType::Legal, DocumentLength::Standard) => ("Legal Document", 10),
        (DocumentType::Legal, DocumentLength::Comprehensive) => ("Legal Document", 20),
        (DocumentType::Legal, DocumentLength::Extensive) => ("Legal Document", 40),
        (DocumentType::Creative, DocumentLength::Brief) => ("Creative Document", 4),
        (DocumentType::Creative, DocumentLength::Standard) => ("Creative Document", 12),
        (DocumentType::Creative, DocumentLength::Comprehensive) => ("Creative Document", 24),
        (DocumentType::Creative, DocumentLength::Extensive) => ("Creative Document", 48),
    };
    
    write_content(
        ContentType::Document,
        "Professional".to_string(),
        style,
        format!("{} pages", target_pages),
        output,
        model,
        api_key,
        use_local,
        ollama_url,
        target_pages,
        SectionType::Section,
        doc_type_str,
    ).await
}

async fn write_content(
    content_type: ContentType,
    genre: String,
    style: WritingStyle,
    target_size: String,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
    target_pages: usize,
    section_type: SectionType,
    content_name: &str,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style(format!("🖋️  Pundit - AI {} Writer", content_name)).bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client based on local flag
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    // Check model availability (same as existing code)
    println!("🔍 Checking model availability...");
    match &client {
        AIClient::HuggingFace(hf_client) => {
            match hf_client.check_model_availability().await {
                Ok(true) => println!("✅ Model is available"),
                Ok(false) => {
                    println!("⚠️  Warning: Model may not be available or requires authentication");
                    println!("   Continuing anyway - some models work even when availability check fails");
                },
                Err(e) => {
                    println!("⚠️  Warning: Could not check model availability: {}", e);
                    println!("   Continuing anyway - will attempt to use the model");
                }
            }
        },
        AIClient::Ollama(ollama_client) => {
            match ollama_client.check_server().await {
                Ok(true) => {
                    println!("✅ Ollama server is running");
                    match ollama_client.list_models().await {
                        Ok(models) => {
                            if models.iter().any(|m| m.contains(&model)) {
                                println!("✅ Model '{}' is available", model);
                            } else {
                                println!("⚠️  Model '{}' not found locally", model);
                                println!("   Available models: {:?}", models);
                                println!("   Run: ollama pull {}", model);
                            }
                        },
                        Err(_) => println!("⚠️  Could not list Ollama models, continuing anyway"),
                    }
                },
                Ok(false) => {
                    println!("❌ Ollama server not running at {}", ollama_url);
                    println!("   Please start Ollama or install it from https://ollama.ai");
                    return Err(anyhow::anyhow!("Ollama server not available"));
                },
                Err(e) => {
                    println!("❌ Could not connect to Ollama: {}", e);
                    return Err(e);
                }
            }
        }
    }
    
    // Get content details
    let title: String = Input::new()
        .with_prompt(&format!("📚 {} title", content_name))
        .interact_text()?;
    
    let premise: String = Input::new()
        .with_prompt(&format!("💡 {} premise (brief description)", content_name))
        .interact_text()?;
    
    let author = config.default_author.clone();
    
    // Create content instance based on type
    let mut content = match content_type {
        ContentType::Book => {
            // This should use the existing book creation logic
            Content::new_book(
                title,
                author,
                genre,
                style.to_string(),
                premise,
                target_size.clone(),
                Some(target_pages * 250), // Rough word estimate
                target_pages / 10, // Rough chapter estimate
                model.clone(),
            )
        },
        ContentType::Screenplay => {
            Content::new_screenplay(
                title,
                author,
                genre,
                style.to_string(),
                premise,
                target_pages,
                model.clone(),
            )
        },
        ContentType::Document => {
            Content::new_document(
                title,
                author,
                style.to_string(),
                premise,
                target_pages,
                DocumentFormat::Business, // Default format
                model.clone(),
            )
        },
        _ => {
            // Generic content creation for other types
            let mut content = Content::new_book(
                title,
                author,
                genre,
                style.to_string(),
                premise,
                target_size.clone(),
                Some(target_pages * 250),
                target_pages / 5, // Rough section estimate
                model.clone(),
            );
            content.content_type = content_type;
            content
        }
    };
    
    println!("\n🎯 {} Configuration:", content_name);
    println!("   Genre: {}", content.genre);
    println!("   Style: {}", content.writing_style);
    println!("   Target: {}", &target_size);
    if let Some(target) = content.metadata.target_word_count {
        println!("   Words: {} target", target);
    }
    println!("   Sections: {} planned", content.metadata.target_sections);
    println!();
    
    // Generate content outline
    println!("📋 Generating {} outline...", content_name.to_lowercase());
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            match hf_client.generate_content_outline(
                &content.content_type,
                &content.genre,
                &content.writing_style,
                &content.premise,
                content.metadata.target_sections,
            ).await {
                Ok(outline) => outline,
                Err(e) => {
                    eprintln!("❌ Failed to generate outline: {}", e);
                    eprintln!("💡 Suggestions:");
                    eprintln!("   1. Try a different model (use --model gpt2 or --model distilgpt2)");
                    eprintln!("   2. Add a Hugging Face API key (--api-key YOUR_KEY)");
                    eprintln!("   3. Check your internet connection");
                    eprintln!("   4. Try local models with --local flag");
                    return Err(e);
                }
            }
        },
        AIClient::Ollama(ollama_client) => {
            match ollama_client.generate_content_outline(
                &model,
                &content.content_type,
                &content.genre,
                &content.writing_style,
                &content.premise,
                content.metadata.target_sections,
            ).await {
                Ok(outline) => outline,
                Err(e) => {
                    eprintln!("❌ Failed to generate outline: {}", e);
                    eprintln!("💡 Suggestions:");
                    eprintln!("   1. Make sure the model is downloaded: ollama pull {}", model);
                    eprintln!("   2. Try a different model");
                    eprintln!("   3. Check if Ollama is running: ollama list");
                    return Err(e);
                }
            }
        }
    };
    content.outline = outline;
    
    println!("✅ Outline generated!");
    println!("\n{}", console::style("Generated Outline:").bold());
    println!("{}", content.outline);
    println!();
    
    if !Confirm::new()
        .with_prompt("Continue with this outline?")
        .default(true)
        .interact()? {
        println!("📝 You can manually edit the outline if needed.");
        content.outline = Input::new()
            .with_prompt("Enter revised outline")
            .with_initial_text(&content.outline)
            .interact_text()?;
    }
    
    // Start writing process
    println!("\n🚀 Starting {} writing process...", content_name.to_lowercase());
    let progress_bar = create_content_progress_bar(&content);
    
    // Save initial state
    if config.auto_save {
        save_content_state(&content)?;
    }
    
    // Write sections progressively
    let mut section_count = 1;
    while content.should_continue() && section_count <= content.metadata.target_sections {
        if let Err(e) = write_next_section(&client, &model, &mut content, section_count, section_type, &progress_bar).await {
            eprintln!("❌ Error writing section {}: {}", section_count, e);
            
            if Confirm::new()
                .with_prompt("Retry this section?")
                .default(true)
                .interact()? {
                continue;
            } else {
                break;
            }
        }
        
        // Auto-save after each section
        if config.auto_save && section_count % (config.save_frequency as usize) == 0 {
            save_content_state(&content)?;
        }
        
        section_count += 1;
    }
    
    progress_bar.finish_with_message("✅ Content completed!");
    content.completed = true;
    
    // Final save
    save_content_state(&content)?;
    
    // Output the content
    output_content(&content, output, &config).await?;
    
    println!("\n🎉 {} writing completed!", content_name);
    println!("📊 Final Statistics:");
    println!("   Total sections: {}", content.sections.len());
    println!("   Total words: {}", content.metadata.current_word_count);
    println!("   Average words per section: {}", 
        if content.sections.is_empty() { 0 } else { content.metadata.current_word_count / content.sections.len() }
    );
    
    Ok(())
}

async fn write_next_section(
    client: &AIClient,
    model: &str,
    content: &mut Content,
    section_number: usize,
    section_type: SectionType,
    progress_bar: &ProgressBar,
) -> Result<()> {
    // Check for duplicate section numbers (important for unlimited mode)
    if content.sections.iter().any(|s| s.number == section_number) {
        return Err(anyhow!("Section {} already exists - preventing duplicate creation", section_number));
    }
    
    // Check if this section is beyond the original outline (for unlimited mode)
    let is_beyond_outline = section_number > count_sections_in_outline(&content.outline, &section_type);
    
    let context = get_clean_context_for_section(content, section_number, &section_type, is_beyond_outline);
    
    // Extract section outline from the content's overall outline
    let section_outline = extract_section_outline(&content.outline, section_number, &section_type)
        .unwrap_or_else(|| {
            // Generate a better default outline for missing sections
            match section_type {
                SectionType::Chapter => format!("{} {}: Develop the narrative further, introducing new developments or deepening existing themes", section_type_name(&section_type), section_number),
                SectionType::Scene => format!("{} {}: Continue the scene with new dialogue and action", section_type_name(&section_type), section_number),
                SectionType::Act => format!("{} {}: Progress the story with significant developments", section_type_name(&section_type), section_number),
                _ => format!("{} {}: Advance the content with new material", section_type_name(&section_type), section_number)
            }
        });
    
    progress_bar.set_message(format!("Writing {} {}...", section_type_name(&section_type), section_number));
    
    // Calculate dynamic target words for this specific section
    let dynamic_length = DynamicSectionLength::calculate_for_section(
        &crate::dynamic_length::DynamicLengthConfig::for_content_type(content.content_type),
        section_number,
        content.metadata.target_sections,
        None, // Use random seed for variety
    );
    
    let target_words = dynamic_length.target_words;
    
    // Create enhanced prompt that prevents chapter number confusion and repetition
    
    // Get dynamic length guidance for this section
    let length_guidance = dynamic_length.get_generation_prompt_addition();
    
    // Analyze previous openings to prevent repetitive patterns
    let opening_variation_guidance = generate_opening_variation_guidance(content, section_number, &section_type);
    
    let enhanced_outline = if is_beyond_outline {
        // For unlimited mode chapters beyond the original outline
        format!(
            "Write {} {} (section number {}). This is beyond the original outline, so create NEW developments. {}. 
            
            IMPORTANT FOR UNLIMITED MODE:
            - This chapter is BEYOND the original outline - create fresh new content
            - Do NOT repeat themes or events from previous chapters
            - Introduce NEW plot developments, characters, or settings
            - Do NOT include chapter/section numbers in your output
            - Do NOT reuse previous chapter endings or conclusions
            - Advance the story in a meaningful NEW direction
            - If approaching story conclusion, make it DEFINITIVE and FINAL
            
            {}
            
            {}",
            section_type_name(&section_type),
            section_number,
            section_number,
            section_outline,
            length_guidance,
            opening_variation_guidance
        )
    } else {
        // For regular outlined chapters
        format!(
            "Write {} {} (this should be exactly section number {}). {}. 
            
            IMPORTANT: 
            - Do NOT include chapter/section numbers in your output
            - Do NOT repeat previous chapter titles or content
            - Write only the content for THIS specific section
            - Focus on new, original content that continues the story
            
            {}
            
            {}",
            section_type_name(&section_type),
            section_number,
            section_number,
            section_outline,
            length_guidance,
            opening_variation_guidance
        )
    };
    
    // Generate section content with enhanced instructions and robust error handling
    let mut content_text = {
        // Create a clean, isolated prompt to prevent contamination
        let clean_prompt = create_isolated_generation_prompt(&content.content_type, &content.genre, &context, &enhanced_outline, target_words);
        
        // Validate prompt isn't corrupted before sending to AI
        if is_prompt_corrupted(&clean_prompt) {
            return Err(anyhow!("Generated prompt contains corrupted data - aborting to prevent AI contamination"));
        }
        
        let raw_result = if target_words > 3000 { // Use segmented generation for large sections
            generate_segmented_content(client, model, content, section_number, &enhanced_outline, target_words, &section_type).await?
        } else {
            // Use safer direct text generation to avoid complex processing chain
            match client {
                AIClient::HuggingFace(hf_client) => {
                    hf_client.generate_text(&clean_prompt, (target_words as f32 * 1.3) as u32, 0.8).await?
                },
                AIClient::Ollama(ollama_client) => {
                    ollama_client.generate_text(model, &clean_prompt, (target_words as f32 * 1.3) as i32, 0.8).await?
                }
            }
        };
        
        // Validate the AI output before processing
        if is_ai_output_corrupted(&raw_result) {
            return Err(anyhow!("AI generated corrupted or contaminated output - content rejected: {}", 
                truncate_for_error(&raw_result, 100)));
        }
        
        raw_result
    };
    
    // Clean the generated content to remove any chapter number references and AI meta-commentary
    content_text = clean_generated_content(&content_text, section_number, &section_type);
    content_text = filter_ai_meta_commentary(&content_text);
    
    // Final validation after cleaning
    if content_text.trim().is_empty() {
        return Err(anyhow!("Content generation resulted in empty text after cleaning"));
    }
    
    if is_ai_output_corrupted(&content_text) {
        return Err(anyhow!("Content still corrupted after cleaning - regeneration required"));
    }
    
    // Validate content isn't a duplicate
    if is_duplicate_content(&content_text, &content.sections) {
        return Err(anyhow!("Generated content appears to be duplicate - retrying would be recommended"));
    }
    
    // Extract title from the cleaned generated content or use a default
    let title = extract_section_title(&content_text, &section_type)
        .unwrap_or_else(|| {
            // Generate a more specific default title based on outline
            if let Some(outline_title) = extract_title_from_outline(&section_outline) {
                outline_title
            } else {
                format!("{} {}", section_type_name(&section_type), section_number)
            }
        });
    
    let mut section = Section::new(section_number, title, section_outline, section_type);
    section.set_content(content_text);
    
    content.add_section(section);
    
    progress_bar.set_position(content.sections.len() as u64);
    progress_bar.set_message(format!("{} {} completed ({} words)", 
        section_type_name(&section_type), section_number, content.metadata.current_word_count));
    
    // Brief pause to avoid rate limiting
    sleep(Duration::from_millis(1000)).await;
    
    Ok(())
}

fn section_type_name(section_type: &SectionType) -> &'static str {
    match section_type {
        SectionType::Chapter => "Chapter",
        SectionType::Scene => "Scene",
        SectionType::Act => "Act",
        SectionType::Episode => "Episode",
        SectionType::Segment => "Segment",
        SectionType::Interaction => "Interaction",
        SectionType::Section => "Section",
    }
}

async fn write_next_chapter(
    client: &AIClient,
    model: &str,
    book: &mut Book,
    chapter_number: usize,
    progress_bar: &ProgressBar,
) -> Result<()> {
    // Legacy function - redirect to new section-based function
    write_next_section(client, model, book, chapter_number, SectionType::Chapter, progress_bar).await
}

fn extract_section_outline(full_outline: &str, section_number: usize, section_type: &SectionType) -> Option<String> {
    let section_name = section_type_name(section_type);
    
    // First, try exact match
    for line in full_outline.lines() {
        if line.trim().starts_with(&format!("{} {}:", section_name, section_number)) {
            return Some(line.trim().to_string());
        }
    }
    
    // For large content where outline might not have all sections, try to infer from context
    // Look for patterns like "Chapter X" followed by some content
    for line in full_outline.lines() {
        let line = line.trim();
        if line.contains(&format!("{} {}", section_name, section_number)) && line.contains(':') {
            return Some(line.to_string());
        }
    }
    
    // If still not found and this is beyond the original outline, generate a contextual outline
    if section_number > count_sections_in_outline(full_outline, section_type) {
        // This section is beyond the original outline - it's for unlimited mode
        return None; // Will trigger our improved fallback
    }
    
    None
}

fn count_sections_in_outline(outline: &str, section_type: &SectionType) -> usize {
    let section_name = section_type_name(section_type);
    outline.lines()
        .filter(|line| line.trim().starts_with(&format!("{} ", section_name)) && line.contains(':'))
        .count()
}

fn extract_section_title(content: &str, section_type: &SectionType) -> Option<String> {
    // Try to find a title in the first few lines based on content type
    // This function should only extract titles from the BEGINNING of content, not from within
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return None;
    }
    
    // Only look at the very first line for the title to avoid picking up chapter references from content body
    let first_line = lines[0].trim();
    
    match section_type {
        SectionType::Scene => {
            if first_line.starts_with("INT.") || first_line.starts_with("EXT.") {
                return Some(first_line.to_string());
            }
        },
        SectionType::Act => {
            if first_line.starts_with("ACT") || first_line.starts_with("Act") {
                return Some(first_line.to_string());
            }
        },
        SectionType::Episode => {
            if first_line.starts_with("Episode") || first_line.starts_with("EPISODE") {
                return Some(first_line.to_string());
            }
        },
        SectionType::Chapter => {
            // For chapters, be very specific about what constitutes a valid title
            // Only accept if it's EXACTLY at the start and follows proper chapter format
            if first_line.starts_with("Chapter ") || first_line.starts_with("CHAPTER ") {
                // Extract only the title part, not any chapter numbering
                if let Some(colon_pos) = first_line.find(':') {
                    let title_part = first_line[colon_pos + 1..].trim();
                    if !title_part.is_empty() && title_part.len() < 100 {
                        return Some(title_part.to_string());
                    }
                }
            }
            // Also accept markdown-style headers only if they're properly formatted
            if first_line.starts_with("# ") && !first_line.contains("Chapter ") {
                let title = first_line.strip_prefix("# ").unwrap_or(first_line).trim();
                if !title.is_empty() && title.len() < 100 {
                    return Some(title.to_string());
                }
            }
        },
        _ => {
            // For other section types, be more permissive but still careful
            if first_line.starts_with(&format!("{}", section_type_name(section_type))) || first_line.starts_with("#") {
                let cleaned = first_line.replace("#", "").trim().to_string();
                if !cleaned.is_empty() && cleaned.len() < 100 {
                    return Some(cleaned);
                }
            }
        }
    }
    
    // If no proper title found, return None (will use default numbering)
    None
}

// Helper functions for content cleaning and validation
fn get_clean_context_for_section(content: &Content, section_number: usize, section_type: &SectionType, is_beyond_outline: bool) -> String {
    let context_sections = if is_beyond_outline {
        // For unlimited mode, use smaller context to prevent repetition
        content.metadata.generation_parameters.context_window.min(2)
    } else {
        content.metadata.generation_parameters.context_window
    };
    
    let start_idx = if content.sections.len() > context_sections {
        content.sections.len() - context_sections
    } else {
        0
    };
    
    let content_type_name = match content.content_type {
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
        content_type_name, content.title, content.genre, content.writing_style, content.premise
    );
    
    // For unlimited mode beyond outline, provide different guidance
    if is_beyond_outline {
        context.push_str("UNLIMITED MODE: You are writing beyond the original outline. ");
        context.push_str("Create fresh developments and avoid repeating previous themes.\n\n");
        
        // Don't include the original outline for unlimited sections to prevent constraint
        if content.sections.len() > 20 {
            context.push_str("Story Progress: This is a long-form work. Focus on meaningful progression and avoid repetitive elements.\n\n");
        }
    } else if !content.outline.is_empty() {
        context.push_str(&format!("Outline:\n{}\n\n", content.outline));
    }
    
    if !content.sections.is_empty() {
        let section_name = section_type_name(section_type);
        
        if is_beyond_outline {
            context.push_str(&format!("Recent sections for context (you are writing NEW {} {}):\n", section_name.to_lowercase(), section_number));
        } else {
            context.push_str(&format!("Previous sections for context (you are writing {} {}):\n", section_name.to_lowercase(), section_number));
        }
        
        for section in &content.sections[start_idx..] {
            // Only include essential context, avoid chapter numbering confusion
            let clean_title = clean_title_for_context(&section.title);
            let context_length = if is_beyond_outline { 200 } else { 300 }; // Even shorter for unlimited mode
            context.push_str(&format!(
                "Previous section: {}\n{}\n\n",
                clean_title,
                truncate_text(&section.content, context_length)
            ));
        }
        
        // Add explicit instruction for unlimited mode
        if is_beyond_outline {
            context.push_str("IMPORTANT: Create NEW content that builds on but does NOT repeat the above sections.\n\n");
        }
    }
    
    context
}

fn clean_title_for_context(title: &str) -> String {
    // Remove chapter numbers and repetitive elements from titles when using as context
    let cleaned = title
        .replace("Chapter ", "")
        .replace("CHAPTER ", "")
        .replace("chapter ", "");
    
    // If the title starts with a number followed by colon, remove that part
    if let Some(colon_pos) = cleaned.find(':') {
        if cleaned[..colon_pos].chars().all(|c| c.is_numeric() || c.is_whitespace()) {
            return cleaned[colon_pos + 1..].trim().to_string();
        }
    }
    
    cleaned.trim().to_string()
}

fn clean_generated_content(content: &str, expected_section_number: usize, section_type: &SectionType) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut cleaned_lines: Vec<&str> = Vec::new();
    let section_name = section_type_name(section_type);
    
    for (i, line) in lines.iter().enumerate() {
        let line = line.trim();
        
        // Skip lines that incorrectly reference chapter numbers
        if line.starts_with(&format!("{} ", section_name)) && i > 0 {
            // If this is not the first line and contains chapter numbering, skip it
            if line.contains(&format!("{} {}", section_name, expected_section_number)) {
                continue;
            }
            // Skip any other chapter number references that don't match expected
            if line.matches("Chapter ").count() > 0 || line.matches("CHAPTER ").count() > 0 {
                continue;
            }
        }
        
        // Skip duplicate markdown headers with chapter numbers
        if line.starts_with("##") && (line.contains("Chapter ") || line.contains("CHAPTER ")) {
            // Only keep if it's exactly the expected chapter
            if !line.contains(&format!("{} {}", section_name, expected_section_number)) {
                continue;
            }
        }
        
        // Skip lines that are just repetitive chapter titles
        if line.len() < 100 && line.contains(&format!("{} ", section_name)) && line.contains(":") {
            // Check if this line appears to be a duplicated chapter title
            let potential_chapter_num = extract_chapter_number_from_line(line);
            if potential_chapter_num.is_some() && potential_chapter_num != Some(expected_section_number) {
                continue;
            }
        }
        
        cleaned_lines.push(line);
    }
    
    cleaned_lines.join("\n").trim().to_string()
}

fn extract_chapter_number_from_line(line: &str) -> Option<usize> {
    // Try to extract chapter number from a line like "Chapter 28: Title"
    if let Some(start) = line.find("Chapter ") {
        let after_chapter = &line[start + 8..];
        if let Some(colon_pos) = after_chapter.find(':') {
            let number_part = after_chapter[..colon_pos].trim();
            return number_part.parse::<usize>().ok();
        }
    }
    None
}

fn is_duplicate_content(new_content: &str, existing_sections: &[Section]) -> bool {
    let new_content_trimmed = new_content.trim().to_lowercase();
    if new_content_trimmed.len() < 100 {
        return false; // Too short to be meaningful
    }
    
    // Check similarity with existing sections
    for section in existing_sections {
        let existing_content = section.content.trim().to_lowercase();
        
        // Simple similarity check - if more than 70% of words match, consider duplicate
        let similarity = calculate_content_similarity(&new_content_trimmed, &existing_content);
        if similarity > 0.7 {
            return true;
        }
    }
    
    false
}

fn calculate_content_similarity(content1: &str, content2: &str) -> f64 {
    let words1: std::collections::HashSet<&str> = content1.split_whitespace().collect();
    let words2: std::collections::HashSet<&str> = content2.split_whitespace().collect();
    
    if words1.is_empty() || words2.is_empty() {
        return 0.0;
    }
    
    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();
    
    intersection as f64 / union as f64
}

fn extract_title_from_outline(outline: &str) -> Option<String> {
    // Extract a clean title from an outline entry like "Chapter 5: The Great Discovery"
    if let Some(colon_pos) = outline.find(':') {
        let title_part = outline[colon_pos + 1..].trim();
        
        // Don't use instructional text as titles - these are fallback instructions, not real titles
        if title_part.contains("Develop the narrative") || 
           title_part.contains("Continue the") ||
           title_part.contains("Progress the") ||
           title_part.contains("Advance the") ||
           title_part.len() > 80 {  // Real titles are usually shorter
            return None;
        }
        
        if !title_part.is_empty() && title_part.len() < 80 {
            return Some(title_part.to_string());
        }
    }
    None
}

fn filter_ai_meta_commentary(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut filtered_lines: Vec<&str> = Vec::new();
    
    for line in lines {
        let line_lower = line.to_lowercase();
        
        // Skip lines that contain AI meta-commentary
        if line_lower.contains("as an ai") ||
           line_lower.contains("i'm an ai") ||
           line_lower.contains("as a language model") ||
           line_lower.contains("i need to focus") ||
           line_lower.contains("i should write") ||
           line_lower.contains("i will write") ||
           line_lower.contains("i'll write") ||
           line_lower.contains("i'm going to write") ||
           line_lower.contains("let me write") ||
           line_lower.contains("i cannot") ||
           line_lower.contains("i can't") ||
           line_lower.contains("as requested") ||
           line_lower.contains("here's the") && line_lower.contains("chapter") ||
           line_lower.contains("here is the") && line_lower.contains("chapter") ||
           line_lower.contains("i'll continue") ||
           line_lower.contains("continuing with") ||
           line_lower.contains("based on the outline") ||
           line_lower.contains("following the outline") ||
           line_lower.starts_with("note:") ||
           line_lower.starts_with("please note") ||
           line_lower.contains("as ai") ||
           line_lower.contains("ai assistant") ||
           line_lower.contains("my role") {
            continue; // Skip this line
        }
        
        // Also skip lines that are just meta instructions
        if line.trim().starts_with("[") && line.trim().ends_with("]") && 
           (line_lower.contains("write") || line_lower.contains("continue") || line_lower.contains("chapter")) {
            continue;
        }
        
        filtered_lines.push(line);
    }
    
    filtered_lines.join("\n").trim().to_string()
}

// Legacy functions for backward compatibility
fn extract_chapter_outline(full_outline: &str, chapter_number: usize) -> Option<String> {
    extract_section_outline(full_outline, chapter_number, &SectionType::Chapter)
}

fn extract_chapter_title(content: &str) -> Option<String> {
    extract_section_title(content, &SectionType::Chapter)
}

fn create_content_progress_bar(content: &Content) -> ProgressBar {
    let total_sections = content.metadata.target_sections as u64;
    let pb = ProgressBar::new(total_sections);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} sections ({msg})")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("#>-"),
    );
    pb.set_position(content.sections.len() as u64);
    pb
}

fn save_content_state(content: &Content) -> Result<()> {
    // For now, just delegate to the existing book save function
    // TODO: Implement proper content state saving
    save_book_state(content)
}

async fn output_content(content: &Content, output_path: Option<PathBuf>, config: &Config) -> Result<()> {
    let output_dir = output_path.unwrap_or_else(|| config.output_directory.clone());
    
    // Ensure output directory exists
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }
    
    let safe_title = sanitize_filename(&content.title);
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    
    let content_type_name = match content.content_type {
        ContentType::Book => "book",
        ContentType::Screenplay => "screenplay",
        ContentType::Play => "play",
        ContentType::TvScript => "tvscript",
        ContentType::AudioScript => "audioscript",
        ContentType::GameScript => "gamescript",
        ContentType::Document => "document",
        ContentType::TechnicalDoc => "technicaldoc",
        ContentType::WhitePaper => "whitepaper",
        ContentType::ResearchReport => "researchreport",
        ContentType::Poetry => "poetry",
        ContentType::InteractiveFiction => "interactivefiction",
        ContentType::Journal => "journal",
        ContentType::Memoir => "memoir",
        ContentType::MarketingAd => "marketingad",
        ContentType::PressRelease => "pressrelease",
        ContentType::MediaKit => "mediakit",
        ContentType::BlogPost => "blogpost",
        ContentType::SeoArticle => "seoarticle",
        ContentType::StrategicDoc => "strategicdoc",
        ContentType::PlanningDoc => "planningdoc",
        ContentType::MeetingNotes => "meetingnotes",
        ContentType::MeetingSummary => "meetingsummary",
        ContentType::Dictionary => "dictionary",
        ContentType::EducationalLesson => "educationallesson",
        ContentType::ChildrensBook => "childrensbook",
    };
    
    // Save as plain text with UTF-8 encoding
    let txt_path = output_dir.join(format!("{}_{}_{}.txt", content_type_name, safe_title, timestamp));
    std::fs::write(&txt_path, content.to_text().as_bytes())?;
    
    // Save as markdown with UTF-8 encoding
    let md_path = output_dir.join(format!("{}_{}_{}.md", content_type_name, safe_title, timestamp));
    std::fs::write(&md_path, content.to_markdown().as_bytes())?;
    
    // Save format-specific version for certain content types
    match content.content_type {
        ContentType::Screenplay => {
            let fdx_path = output_dir.join(format!("screenplay_{}_{}.fdx", safe_title, timestamp));
            std::fs::write(&fdx_path, format_as_final_draft(content).as_bytes())?;
            println!("   Final Draft: {}", fdx_path.display());
        },
        ContentType::Play => {
            let play_path = output_dir.join(format!("play_{}_{}.txt", safe_title, timestamp));
            std::fs::write(&play_path, format_as_stage_play(content).as_bytes())?;
            println!("   Stage Format: {}", play_path.display());
        },
        _ => {}
    }
    
    println!("\n📁 Content saved to:");
    println!("   Text: {}", txt_path.display());
    println!("   Markdown: {}", md_path.display());
    
    Ok(())
}

fn format_as_final_draft(content: &Content) -> String {
    // Basic Final Draft XML format
    let mut fdx = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\" ?>\n");
    fdx.push_str("<FinalDraft DocumentType=\"Script\" Template=\"No\" Version=\"1\">\n");
    fdx.push_str(&format!("  <TitlePage>\n    <Content>\n      <Paragraph>\n        <Text>{}</Text>\n      </Paragraph>\n    </Content>\n  </TitlePage>\n", content.title));
    fdx.push_str("  <Content>\n");
    
    for section in &content.sections {
        fdx.push_str("    <Paragraph Type=\"Scene Heading\">\n");
        fdx.push_str(&format!("      <Text>{}</Text>\n", section.title));
        fdx.push_str("    </Paragraph>\n");
        fdx.push_str("    <Paragraph Type=\"Action\">\n");
        fdx.push_str(&format!("      <Text>{}</Text>\n", section.content));
        fdx.push_str("    </Paragraph>\n");
    }
    
    fdx.push_str("  </Content>\n");
    fdx.push_str("</FinalDraft>");
    fdx
}

fn format_as_stage_play(content: &Content) -> String {
    let mut play = format!("{}\n", content.title.to_uppercase());
    play.push_str(&format!("by {}\n\n", content.author));
    
    for section in &content.sections {
        play.push_str(&format!("{}\n\n", section.title.to_uppercase()));
        play.push_str(&content.format_play_scene(&section.content));
        play.push_str("\n\n");
    }
    
    play
}

pub async fn interactive_mode() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    'main: loop {
        println!("{}", console::style("🎭 Pundit - Interactive Content Creator").bold().magenta());
        println!("═══════════════════════════════════════");
        println!();
        println!("Welcome to Pundit's interactive content creation mode!");
        println!("I'll help you choose the perfect content type and guide you through the creation process.");
        println!();
        
        // Content type selection (each will offer new/continue options)
        'content: loop {
            // Content type selection
            let content_types = vec![
                // === CREATIVE WRITING ===
                "📚 Book - Traditional long-form narrative",
                "👶 Children's Book - Age-appropriate stories and learning",
                "🎨 Poetry - Sonnets, haiku, free verse, and more",
                
                // === SCREENWRITING & SCRIPTS ===
                "🎬 Screenplay - Movie script with professional formatting",
                "🎭 Stage Play - Theater script with stage directions",
                "📺 TV Script - Television episode or series",
                "🎧 Audio Script - Podcast, radio play, or audio drama",
                "🎮 Game Script - Interactive dialogue with branching choices",
                
                // === EDUCATIONAL & REFERENCE ===
                "🎓 Educational Lesson - Language learning, tutorials, instruction",
                "📖 Educational Textbooks - Academic books for students (K-12 to University)",
                "📖 Dictionary/Lexicon - Word definitions, etymologies, terminology",
                
                // === BUSINESS & PROFESSIONAL ===
                "📋 Strategic Planning - Business plans, roadmaps, vision docs",
                "📄 Business Document - Professional or technical document",
                "📅 Meeting Documentation - Notes, summaries, action items",
                "📝 Marketing Content - Ads, press releases, media kits",
                
                // === RESEARCH & TECHNICAL ===
                "📊 Research & White Papers - Reports, case studies, analysis",
                "🔬 Technical Documentation - Manuals, APIs, guides",
                "📰 Blog & SEO Articles - Posts, tutorials, reviews",
                
                // === SPECIAL MODES ===
                "✨ Freeform Writing - Describe what you want with files/context",
                "❌ Exit",
            ];
            
            let content_idx = Select::new()
                .with_prompt("What type of content would you like to create?")
                .items(&content_types)
                .default(0)
                .interact()?;
            
            if content_idx == content_types.len() - 1 {
                // Exit
                println!("👋 Thanks for using Pundit!");
                return Ok(());
            }
            
            let result = match content_idx {
                // === CREATIVE WRITING ===
                0 => {
                    // Books - offer new/continue options
                    interactive_book_mode().await
                },
                1 => {
                    // Children's book creation
                    interactive_childrens_book_creation().await
                },
                2 => {
                    // Poetry creation
                    interactive_poetry_creation().await
                },
                
                // === SCREENWRITING & SCRIPTS ===
                3 => {
                    // Screenplay creation
                    interactive_screenplay_creation().await
                },
                4 => {
                    // Stage play creation
                    interactive_play_creation().await
                },
                5 => {
                    // TV script creation
                    interactive_tv_creation().await
                },
                6 => {
                    // Audio script creation
                    interactive_audio_creation().await
                },
                7 => {
                    // Game script creation
                    interactive_game_creation().await
                },
                
                // === EDUCATIONAL & REFERENCE ===
                8 => {
                    // Educational lesson creation
                    interactive_educational_lesson_creation().await
                },
                9 => {
                    // Educational textbooks
                    interactive_educational_textbook_creation().await
                },
                10 => {
                    // Dictionary creation
                    interactive_dictionary_creation().await
                },
                
                // === BUSINESS & PROFESSIONAL ===
                11 => {
                    // Strategic planning
                    interactive_strategic_doc_creation().await
                },
                12 => {
                    // Document creation
                    interactive_document_creation().await
                },
                13 => {
                    // Meeting documentation
                    interactive_meeting_doc_creation().await
                },
                14 => {
                    // Marketing content
                    interactive_marketing_creation().await
                },
                
                // === RESEARCH & TECHNICAL ===
                15 => {
                    // Research & white papers
                    interactive_research_doc_creation().await
                },
                16 => {
                    // Technical documentation
                    interactive_technical_doc_creation().await
                },
                17 => {
                    // Blog & SEO articles
                    interactive_blog_creation().await
                },
                
                // === SPECIAL MODES ===
                18 => {
                    // Freeform writing mode
                    interactive_freeform_writing().await
                },
                _ => {
                    println!("Invalid selection");
                    Ok(())
                }
            };
            
            // Handle the result
            match result {
                Ok(_) => return Ok(()), // User completed task successfully, exit app
                Err(e) => {
                    // Check if this is a "back to menu" request
                    if e.downcast_ref::<BackToMenu>().is_some() {
                        // User wants to go back to main menu, continue the content loop
                        continue 'content;
                    } else {
                        // Real error occurred
                        println!("Error: {}", e);
                        continue 'content;
                    }
                }
            }
        }
    }
}

async fn interactive_screenplay_creation() -> Result<()> {
    loop {
        println!("\n🎬 Creating a Screenplay");
        println!("Let me ask you a few questions to create the perfect screenplay...\n");
        
        // Genre selection for screenplay
        let genres = vec![
            "Action", "Comedy", "Drama", "Horror", "Romance", "Sci-Fi", "Thriller",
            "← Back to main menu",
        ];
        let genre_idx = Select::new()
            .with_prompt("What genre is your screenplay?")
            .items(&genres)
            .default(2)
            .interact()?;
        
        if genre_idx == genres.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let genre = match genre_idx {
            0 => Genre::Adventure,
            1 => Genre::Comedy,
            2 => Genre::Drama,
            3 => Genre::Horror,
            4 => Genre::Romance,
            5 => Genre::SciFi,
            6 => Genre::Thriller,
            _ => Genre::Drama,
        };
    
        // Length selection
        let lengths = vec![
            "Short Film (5-30 pages)",
            "Feature Film (90-120 pages)",
            "Epic Film (120+ pages)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("What length screenplay would you like?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue; // Back to genre selection
        }
        
        let length = match length_idx {
            0 => ScreenplayLength::Short,
            1 => ScreenplayLength::Feature,
            2 => ScreenplayLength::Epic,
            _ => ScreenplayLength::Feature,
        };
    
        // Style selection
        let style = WritingStyle::Dramatic; // Default for screenplays
        
        // Model selection with proper recommendations
        let length_desc = match length {
            ScreenplayLength::Short => "Short",
            ScreenplayLength::Feature => "Medium", 
            ScreenplayLength::Epic => "Long",
        };
        
        let (use_local, model) = match interactive_model_selection("screenplay", length_desc) {
            Ok((use_local, model)) => (use_local, model),
            Err(_) => continue, // Back was selected
        };
        
        // Create the screenplay
        return write_screenplay(genre, style, length, None, model, None, use_local, "http://localhost:11434".to_string()).await;
    }
}

async fn interactive_play_creation() -> Result<()> {
    loop {
        println!("\n🎭 Creating a Stage Play");
        println!("Let me ask you a few questions to create the perfect stage play...\n");
        
        // Genre selection for stage play
        let genres = vec![
            "Drama", "Comedy", "Musical", "Tragedy", "Romance", 
            "Historical", "Contemporary", "Experimental", "Children's",
            "← Back to main menu",
        ];
        let genre_idx = Select::new()
            .with_prompt("What genre is your stage play?")
            .items(&genres)
            .default(0)
            .interact()?;
        
        if genre_idx == genres.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let genre = match genre_idx {
            0 => Genre::Drama,
            1 => Genre::Comedy,
        2 => Genre::Drama, // Musical dramas
        3 => Genre::Drama, // Tragedy
        4 => Genre::Romance,
        5 => Genre::Historical,
        6 => Genre::Drama, // Contemporary
        7 => Genre::Drama, // Experimental
        8 => Genre::Children,
        _ => Genre::Drama,
    };
    
        // Play length selection
        let lengths = vec![
            "One-Act Play (10-30 minutes)",
            "Full-Length Play (90-120 minutes)",
            "Musical (2-3 hours with songs)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("What length play would you like?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue; // Back to genre selection
        }
        
        let length = match length_idx {
            0 => PlayLength::OneAct,
            1 => PlayLength::Full,
            2 => PlayLength::Musical,
            _ => PlayLength::Full,
        };
    
        // Style selection
        let styles = vec![
            "Dramatic", "Conversational", "Poetic", "Humorous", 
            "Formal", "Creative", "Minimalist",
            "← Back",
        ];
        
        let style_idx = Select::new()
            .with_prompt("What writing style do you prefer?")
            .items(&styles)
            .default(0)
            .interact()?;
        
        if style_idx == styles.len() - 1 {
            continue; // Back to length selection
        }
        
        let style = match style_idx {
            0 => WritingStyle::Dramatic,
            1 => WritingStyle::Conversational,
            2 => WritingStyle::Poetic,
            3 => WritingStyle::Humorous,
        4 => WritingStyle::Formal,
        5 => WritingStyle::Creative,
        6 => WritingStyle::Minimalist,
        _ => WritingStyle::Dramatic,
        };
        
        // Model selection with proper recommendations  
        let length_desc = match length {
            PlayLength::OneAct => "Short",
            PlayLength::Full => "Medium",
            PlayLength::Epic => "Long",
            PlayLength::Musical => "Medium",
        };
        
        let (use_local, model) = match interactive_model_selection("stage play", length_desc) {
            Ok((use_local, model)) => (use_local, model),
            Err(_) => continue, // Back was selected
        };
        
        // Create the stage play
        return write_play(genre, style, length, None, model, None, use_local, "http://localhost:11434".to_string()).await;
    }
}

async fn interactive_tv_creation() -> Result<()> {
    println!("\n📺 Creating a TV Script");
    println!("Let me ask you a few questions to create the perfect TV script...\n");
    
    // TV show type selection
    let show_types = vec![
        "Sitcom (22-30 minutes, comedy format)",
        "Drama Series (42-60 minutes, dramatic format)",
        "Mini-Series (Limited series, 4-8 episodes)",
        "Anthology (Standalone episodes, varied themes)",
    ];
    
    let show_type_idx = Select::new()
        .with_prompt("What type of TV show is this?")
        .items(&show_types)
        .default(1)
        .interact()?;
    
    let show_type = match show_type_idx {
        0 => TvShowType::Sitcom,
        1 => TvShowType::Drama,
        2 => TvShowType::MiniSeries,
        3 => TvShowType::Anthology,
        _ => TvShowType::Drama,
    };
    
    // Genre selection
    let genres = vec![
        "Drama", "Comedy", "Crime", "Sci-Fi", "Fantasy", 
        "Horror", "Thriller", "Romance", "Mystery", "Adventure"
    ];
    let genre_idx = Select::new()
        .with_prompt("What genre is your TV show?")
        .items(&genres)
        .default(0)
        .interact()?;
    
    let genre = match genre_idx {
        0 => Genre::Drama,
        1 => Genre::Comedy,
        2 => Genre::Crime,
        3 => Genre::SciFi,
        4 => Genre::Fantasy,
        5 => Genre::Horror,
        6 => Genre::Thriller,
        7 => Genre::Romance,
        8 => Genre::Mystery,
        9 => Genre::Adventure,
        _ => Genre::Drama,
    };
    
    // Number of episodes
    let episode_options = vec![
        "1 episode (Pilot)",
        "3 episodes (Short arc)",
        "6 episodes (Half season)",
        "10 episodes (Full season)",
        "Custom number",
    ];
    
    let episode_idx = Select::new()
        .with_prompt("How many episodes would you like to create?")
        .items(&episode_options)
        .default(0)
        .interact()?;
    
    let episodes = match episode_idx {
        0 => 1,
        1 => 3,
        2 => 6,
        3 => 10,
        4 => {
            let custom: String = Input::new()
                .with_prompt("Enter number of episodes")
                .with_initial_text("1")
                .interact_text()?;
            custom.parse::<u32>().unwrap_or(1).max(1).min(50)
        },
        _ => 1,
    };
    
    // Style selection
    let styles = vec![
        "Dramatic", "Conversational", "Humorous", "Formal", 
        "Creative", "Journalistic", "Minimalist"
    ];
    
    let style_idx = Select::new()
        .with_prompt("What writing style do you prefer?")
        .items(&styles)
        .default(0)
        .interact()?;
    
    let style = match style_idx {
        0 => WritingStyle::Dramatic,
        1 => WritingStyle::Conversational,
        2 => WritingStyle::Humorous,
        3 => WritingStyle::Formal,
        4 => WritingStyle::Creative,
        5 => WritingStyle::Journalistic,
        6 => WritingStyle::Minimalist,
        _ => WritingStyle::Dramatic,
    };
    
    // Model selection with proper recommendations
    let episodes_desc = match episodes {
        1..=3 => "Short",
        4..=8 => "Medium", 
        _ => "Large",
    };
    
    let (use_local, model) = interactive_model_selection("TV script", episodes_desc)?;
    
    println!("\n🎬 Creating {} {} episodes...", episodes, show_type_name(&show_type));
    
    // Create the TV script
    write_tv_script(show_type, genre, style, episodes, None, model, None, use_local, "http://localhost:11434".to_string()).await
}

fn show_type_name(show_type: &TvShowType) -> &'static str {
    match show_type {
        TvShowType::Sitcom => "sitcom",
        TvShowType::Drama => "drama",
        TvShowType::MiniSeries => "mini-series",
        TvShowType::Anthology => "anthology",
        TvShowType::Comedy => "comedy",
        TvShowType::Miniseries => "miniseries",
        TvShowType::Special => "special",
        TvShowType::Reality => "reality",
        TvShowType::Documentary => "documentary",
        TvShowType::News => "news",
        TvShowType::Talk => "talk",
        TvShowType::Variety => "variety",
    }
}

async fn interactive_audio_creation() -> Result<()> {
    println!("\n🎧 Creating an Audio Script");
    println!("Let me ask you a few questions to create the perfect audio content...\n");
    
    // Audio type selection
    let audio_types = vec![
        "Audio Drama (Narrative story with multiple characters)",
        "Podcast (Interview or discussion format)",
        "Radio Play (Traditional radio drama with sound effects)",
        "Documentary (Factual presentation with narration)",
    ];
    
    let audio_type_idx = Select::new()
        .with_prompt("What type of audio content is this?")
        .items(&audio_types)
        .default(0)
        .interact()?;
    
    let audio_type = match audio_type_idx {
        0 => AudioType::AudioDrama,
        1 => AudioType::Podcast,
        2 => AudioType::RadioPlay,
        3 => AudioType::Documentary,
        _ => AudioType::AudioDrama,
    };
    
    // Genre selection (varies by audio type)
    let genres = match audio_type {
        AudioType::AudioDrama | AudioType::RadioPlay | AudioType::RadioDrama => vec![
            "Drama", "Comedy", "Horror", "Mystery", "Sci-Fi", 
            "Fantasy", "Thriller", "Romance", "Adventure"
        ],
        AudioType::Podcast => vec![
            "Educational", "Comedy", "News", "Technology", "Health", 
            "Business", "True Crime", "Interview", "Storytelling"
        ],
        AudioType::Documentary => vec![
            "History", "Science", "Nature", "Biography", "Travel", 
            "Technology", "Social Issues", "Culture", "Politics"
        ],
        AudioType::Audiobook => vec![
            "Fiction", "Non-Fiction", "Biography", "Self-Help", "Educational",
            "Romance", "Mystery", "Sci-Fi", "Fantasy", "History"
        ],
        AudioType::Commercial => vec![
            "Product", "Service", "Brand", "Promotional", "Educational",
            "Entertainment", "Public Service", "Corporate", "Retail"
        ],
    };
    
    let genre_idx = Select::new()
        .with_prompt("What genre/topic is your audio content?")
        .items(&genres)
        .default(0)
        .interact()?;
    
    let genre = match audio_type {
        AudioType::AudioDrama | AudioType::RadioPlay | AudioType::RadioDrama => match genre_idx {
            0 => Genre::Drama,
            1 => Genre::Comedy,
            2 => Genre::Horror,
            3 => Genre::Mystery,
            4 => Genre::SciFi,
            5 => Genre::Fantasy,
            6 => Genre::Thriller,
            7 => Genre::Romance,
            8 => Genre::Adventure,
            _ => Genre::Drama,
        },
        AudioType::Podcast => match genre_idx {
            0 => Genre::SelfHelp, // Educational
            1 => Genre::Comedy,
            2 => Genre::NonFiction, // News
            3 => Genre::Technical,
            4 => Genre::SelfHelp, // Health
            5 => Genre::NonFiction, // Business
            6 => Genre::Crime,
            7 => Genre::NonFiction, // Interview
            8 => Genre::Fiction, // Storytelling
            _ => Genre::NonFiction,
        },
        AudioType::Documentary => match genre_idx {
            0 => Genre::History,
            1 => Genre::Science,
            2 => Genre::Science, // Nature
            3 => Genre::Biography,
            4 => Genre::Travel,
            5 => Genre::Technical,
            6 => Genre::NonFiction, // Social Issues
            7 => Genre::NonFiction, // Culture
            8 => Genre::NonFiction, // Politics
            _ => Genre::History,
        },
        AudioType::Audiobook => match genre_idx {
            0 => Genre::Fiction,
            1 => Genre::NonFiction,
            2 => Genre::Biography,
            3 => Genre::SelfHelp,
            4 => Genre::SelfHelp, // Educational
            5 => Genre::Romance,
            6 => Genre::Mystery,
            7 => Genre::SciFi,
            8 => Genre::Fantasy,
            9 => Genre::History,
            _ => Genre::Fiction,
        },
        AudioType::Commercial => match genre_idx {
            0 => Genre::NonFiction, // Product
            1 => Genre::NonFiction, // Service
            2 => Genre::NonFiction, // Brand
            3 => Genre::NonFiction, // Promotional
            4 => Genre::SelfHelp, // Educational
            5 => Genre::Comedy, // Entertainment
            6 => Genre::NonFiction, // Public Service
            7 => Genre::NonFiction, // Corporate
            8 => Genre::NonFiction, // Retail
            _ => Genre::NonFiction,
        },
    };
    
    // Duration selection
    let duration_options = vec![
        "Short (10-15 minutes)",
        "Standard (20-30 minutes)",
        "Long (45-60 minutes)",
        "Extended (90+ minutes)",
        "Custom duration",
    ];
    
    let duration_idx = Select::new()
        .with_prompt("How long should the audio content be?")
        .items(&duration_options)
        .default(1)
        .interact()?;
    
    let duration = match duration_idx {
        0 => 15,
        1 => 30,
        2 => 60,
        3 => 120,
        4 => {
            let custom: String = Input::new()
                .with_prompt("Enter duration in minutes")
                .with_initial_text("30")
                .interact_text()?;
            custom.parse::<u32>().unwrap_or(30).max(5).min(300)
        },
        _ => 30,
    };
    
    // Style selection
    let styles = match audio_type {
        AudioType::AudioDrama | AudioType::RadioPlay | AudioType::RadioDrama => vec![
            "Dramatic", "Conversational", "Descriptive", "Humorous", "Poetic"
        ],
        AudioType::Podcast => vec![
            "Conversational", "Formal", "Casual", "Journalistic", "Humorous"
        ],
        AudioType::Documentary => vec![
            "Formal", "Descriptive", "Journalistic", "Academic", "Narrative"
        ],
        AudioType::Audiobook => vec![
            "Narrative", "Descriptive", "Conversational", "Dramatic", "Formal"
        ],
        AudioType::Commercial => vec![
            "Persuasive", "Conversational", "Humorous", "Formal", "Dramatic"
        ],
    };
    
    let style_idx = Select::new()
        .with_prompt("What writing style do you prefer?")
        .items(&styles)
        .default(0)
        .interact()?;
    
    let style = match audio_type {
        AudioType::AudioDrama | AudioType::RadioPlay | AudioType::RadioDrama => match style_idx {
            0 => WritingStyle::Dramatic,
            1 => WritingStyle::Conversational,
            2 => WritingStyle::Descriptive,
            3 => WritingStyle::Humorous,
            4 => WritingStyle::Poetic,
            _ => WritingStyle::Dramatic,
        },
        AudioType::Podcast => match style_idx {
            0 => WritingStyle::Conversational,
            1 => WritingStyle::Formal,
            2 => WritingStyle::Casual,
            3 => WritingStyle::Journalistic,
            4 => WritingStyle::Humorous,
            _ => WritingStyle::Conversational,
        },
        AudioType::Documentary => match style_idx {
            0 => WritingStyle::Formal,
            1 => WritingStyle::Descriptive,
            2 => WritingStyle::Journalistic,
            3 => WritingStyle::Academic,
            4 => WritingStyle::Narrative,
            _ => WritingStyle::Formal,
        },
        AudioType::Audiobook => match style_idx {
            0 => WritingStyle::Narrative,
            1 => WritingStyle::Descriptive,
            2 => WritingStyle::Conversational,
            3 => WritingStyle::Dramatic,
            4 => WritingStyle::Formal,
            _ => WritingStyle::Narrative,
        },
        AudioType::Commercial => match style_idx {
            0 => WritingStyle::Persuasive,
            1 => WritingStyle::Conversational,
            2 => WritingStyle::Humorous,
            3 => WritingStyle::Formal,
            4 => WritingStyle::Dramatic,
            _ => WritingStyle::Persuasive,
        },
    };
    
    // Model selection with proper recommendations
    let duration_desc = match duration {
        1..=15 => "Short",
        16..=60 => "Medium",
        _ => "Large",
    };
    
    let (use_local, model) = interactive_model_selection("audio script", duration_desc)?;
    
    println!("\n🎙️ Creating {} minute {} script...", duration, audio_type_name(&audio_type));
    
    // Create the audio script
    write_audio_script(audio_type, genre, style, duration, None, model, None, use_local, "http://localhost:11434".to_string()).await
}

fn audio_type_name(audio_type: &AudioType) -> &'static str {
    match audio_type {
        AudioType::AudioDrama => "audio drama",
        AudioType::Podcast => "podcast",
        AudioType::RadioPlay => "radio play",
        AudioType::Documentary => "documentary",
        AudioType::RadioDrama => "radio drama",
        AudioType::Audiobook => "audiobook",
        AudioType::Commercial => "commercial",
    }
}

async fn interactive_game_creation() -> Result<()> {
    println!("\n🎮 Creating a Game Script");
    println!("Let me ask you a few questions to create the perfect interactive game script...\n");
    
    // Game genre selection
    let game_genres = vec![
        "RPG (Role-Playing Game)",
        "Adventure (Story-driven exploration)",
        "Visual Novel (Interactive storytelling)",
        "Action (Fast-paced combat and movement)",
        "Strategy (Tactical decision-making)",
        "Horror (Suspenseful and frightening)",
        "Sci-Fi (Science fiction setting)",
        "Fantasy (Magical and mythical setting)",
    ];
    
    let genre_idx = Select::new()
        .with_prompt("What genre is your game?")
        .items(&game_genres)
        .default(0)
        .interact()?;
    
    let genre = match genre_idx {
        0 => GameGenre::RPG,
        1 => GameGenre::Adventure,
        2 => GameGenre::VisualNovel,
        3 => GameGenre::Action,
        4 => GameGenre::Strategy,
        5 => GameGenre::Horror,
        6 => GameGenre::SciFi,
        7 => GameGenre::Fantasy,
        _ => GameGenre::RPG,
    };
    
    // Number of character interactions
    let character_options = vec![
        "Few characters (3-5 main interactions)",
        "Standard cast (6-10 character interactions)",
        "Large cast (11-20 character interactions)",
        "Epic scope (21+ character interactions)",
        "Custom number",
    ];
    
    let character_idx = Select::new()
        .with_prompt("How many character interactions should the script include?")
        .items(&character_options)
        .default(1)
        .interact()?;
    
    let characters = match character_idx {
        0 => 5,
        1 => 8,
        2 => 15,
        3 => 25,
        4 => {
            let custom: String = Input::new()
                .with_prompt("Enter number of character interactions")
                .with_initial_text("8")
                .interact_text()?;
            custom.parse::<u32>().unwrap_or(8).max(1).min(100)
        },
        _ => 8,
    };
    
    // Style selection tailored for games
    let styles = vec![
        "Dramatic (Emotional and intense dialogue)",
        "Conversational (Natural, casual interactions)",
        "Humorous (Comedy and wit)",
        "Formal (Serious, professional tone)",
        "Creative (Unique and artistic expression)",
        "Descriptive (Rich world-building details)",
    ];
    
    let style_idx = Select::new()
        .with_prompt("What writing style do you prefer for the dialogue?")
        .items(&styles)
        .default(0)
        .interact()?;
    
    let style = match style_idx {
        0 => WritingStyle::Dramatic,
        1 => WritingStyle::Conversational,
        2 => WritingStyle::Humorous,
        3 => WritingStyle::Formal,
        4 => WritingStyle::Creative,
        5 => WritingStyle::Descriptive,
        _ => WritingStyle::Dramatic,
    };
    
    // Complexity of branching
    let should_include_complex_branching = Confirm::new()
        .with_prompt("Include complex branching dialogue with multiple choice outcomes?")
        .default(true)
        .interact()?;
    
    let should_include_conditions = Confirm::new()
        .with_prompt("Include conditional dialogue based on player choices/stats?")
        .default(true)
        .interact()?;
    
    // Model selection with proper recommendations
    let characters_desc = match characters {
        1..=3 => "Short",
        4..=8 => "Medium",
        _ => "Large", 
    };
    
    let (use_local, model) = interactive_model_selection("game script", characters_desc)?;
    
    println!("\n🎯 Creating {} {} script with {} character interactions...", 
             game_genre_name(&genre), "game", characters);
    
    if should_include_complex_branching {
        println!("✨ Including complex branching dialogue options");
    }
    if should_include_conditions {
        println!("🔀 Including conditional dialogue based on player state");
    }
    
    // Create the game script
    write_game_script(genre, style, characters, None, model, None, use_local, "http://localhost:11434".to_string()).await
}

fn game_genre_name(genre: &GameGenre) -> &'static str {
    match genre {
        GameGenre::RPG => "RPG",
        GameGenre::Adventure => "adventure",
        GameGenre::VisualNovel => "visual novel",
        GameGenre::Action => "action",
        GameGenre::Strategy => "strategy",
        GameGenre::Horror => "horror",
        GameGenre::SciFi => "sci-fi",
        GameGenre::Fantasy => "fantasy",
        GameGenre::Mystery => "mystery",
        GameGenre::Historical => "historical",
        GameGenre::Contemporary => "contemporary",
    }
}

async fn interactive_document_creation() -> Result<()> {
    println!("\n📄 Creating a Professional Document");
    println!("Let me ask you a few questions to create the perfect business or technical document...\n");
    
    // Document type selection
    let doc_types = vec![
        "Business Plan (Comprehensive business strategy)",
        "Technical Manual (How-to guides and instructions)",
        "User Guide (Product or service documentation)",
        "Report (Analysis and findings)",
        "Proposal (Project or business proposals)",
        "Marketing Copy (Promotional content)",
        "Legal Template (Contracts and legal documents)",
        "Lesson Plan (Educational content structure)",
        "Course Outline (Educational curriculum)",
    ];
    
    let doc_type_idx = Select::new()
        .with_prompt("What type of document would you like to create?")
        .items(&doc_types)
        .default(0)
        .interact()?;
    
    let doc_type = match doc_type_idx {
        0 => DocumentType::BusinessPlan,
        1 => DocumentType::TechnicalManual,
        2 => DocumentType::UserGuide,
        3 => DocumentType::Report,
        4 => DocumentType::Proposal,
        5 => DocumentType::MarketingCopy,
        6 => DocumentType::LegalTemplate,
        7 => DocumentType::LessonPlan,
        8 => DocumentType::CourseOutline,
        _ => DocumentType::BusinessPlan,
    };
    
    // Length selection
    let length_options = vec![
        "Brief (1-5 pages, executive summary style)",
        "Standard (5-20 pages, detailed overview)",
        "Comprehensive (20-50 pages, thorough analysis)",
        "Extensive (50+ pages, complete documentation)",
    ];
    
    let length_idx = Select::new()
        .with_prompt("How detailed should the document be?")
        .items(&length_options)
        .default(1)
        .interact()?;
    
    let length = match length_idx {
        0 => DocumentLength::Brief,
        1 => DocumentLength::Standard,
        2 => DocumentLength::Comprehensive,
        3 => DocumentLength::Extensive,
        _ => DocumentLength::Standard,
    };
    
    // Style selection based on document type
    let styles = match doc_type {
        DocumentType::BusinessPlan | DocumentType::Report | DocumentType::Proposal => vec![
            "Formal (Professional business tone)",
            "Persuasive (Compelling and convincing)",
            "Academic (Scholarly and analytical)",
            "Expository (Clear and explanatory)",
        ],
        DocumentType::TechnicalManual | DocumentType::UserGuide => vec![
            "Technical (Precise and instructional)",
            "Formal (Professional and clear)",
            "Descriptive (Detailed explanations)",
            "Expository (Step-by-step guidance)",
        ],
        DocumentType::MarketingCopy => vec![
            "Persuasive (Compelling and sales-focused)",
            "Creative (Engaging and original)",
            "Conversational (Friendly and approachable)",
            "Formal (Professional brand voice)",
        ],
        DocumentType::LegalTemplate => vec![
            "Formal (Legal and precise language)",
            "Technical (Specific legal terminology)",
            "Academic (Scholarly legal analysis)",
        ],
        DocumentType::LessonPlan | DocumentType::CourseOutline => vec![
            "Academic (Educational and structured)",
            "Formal (Professional educational tone)",
            "Descriptive (Clear learning objectives)",
            "Expository (Explanatory teaching style)",
        ],
        DocumentType::Business => vec![
            "Formal (Professional business tone)",
            "Persuasive (Business-focused)",
            "Technical (Business processes)",
            "Expository (Clear business communication)",
        ],
        DocumentType::Academic => vec![
            "Academic (Scholarly and analytical)",
            "Formal (Academic standards)",
            "Technical (Research methodology)",
            "Expository (Educational content)",
        ],
        DocumentType::Technical => vec![
            "Technical (Precise and instructional)",
            "Formal (Professional documentation)",
            "Descriptive (Technical specifications)",
            "Expository (Technical guidance)",
        ],
        DocumentType::Legal => vec![
            "Formal (Legal documentation)",
            "Technical (Legal terminology)",
            "Academic (Legal analysis)",
            "Expository (Legal explanation)",
        ],
        DocumentType::Creative => vec![
            "Creative (Innovative and original)",
            "Narrative (Storytelling approach)",
            "Descriptive (Creative expression)",
            "Conversational (Engaging and personal)",
        ],
    };
    
    let style_idx = Select::new()
        .with_prompt("What writing style do you prefer?")
        .items(&styles)
        .default(0)
        .interact()?;
    
    let style = match doc_type {
        DocumentType::BusinessPlan | DocumentType::Report | DocumentType::Proposal => match style_idx {
            0 => WritingStyle::Formal,
            1 => WritingStyle::Persuasive,
            2 => WritingStyle::Academic,
            3 => WritingStyle::Expository,
            _ => WritingStyle::Formal,
        },
        DocumentType::TechnicalManual | DocumentType::UserGuide => match style_idx {
            0 => WritingStyle::Technical,
            1 => WritingStyle::Formal,
            2 => WritingStyle::Descriptive,
            3 => WritingStyle::Expository,
            _ => WritingStyle::Technical,
        },
        DocumentType::MarketingCopy => match style_idx {
            0 => WritingStyle::Persuasive,
            1 => WritingStyle::Creative,
            2 => WritingStyle::Conversational,
            3 => WritingStyle::Formal,
            _ => WritingStyle::Persuasive,
        },
        DocumentType::LegalTemplate => match style_idx {
            0 => WritingStyle::Formal,
            1 => WritingStyle::Technical,
            2 => WritingStyle::Academic,
            _ => WritingStyle::Formal,
        },
        DocumentType::LessonPlan | DocumentType::CourseOutline => match style_idx {
            0 => WritingStyle::Academic,
            1 => WritingStyle::Formal,
            2 => WritingStyle::Descriptive,
            3 => WritingStyle::Expository,
            _ => WritingStyle::Academic,
        },
        DocumentType::Business => match style_idx {
            0 => WritingStyle::Formal,
            1 => WritingStyle::Persuasive,
            2 => WritingStyle::Technical,
            3 => WritingStyle::Expository,
            _ => WritingStyle::Formal,
        },
        DocumentType::Academic => match style_idx {
            0 => WritingStyle::Academic,
            1 => WritingStyle::Formal,
            2 => WritingStyle::Technical,
            3 => WritingStyle::Expository,
            _ => WritingStyle::Academic,
        },
        DocumentType::Technical => match style_idx {
            0 => WritingStyle::Technical,
            1 => WritingStyle::Formal,
            2 => WritingStyle::Descriptive,
            3 => WritingStyle::Expository,
            _ => WritingStyle::Technical,
        },
        DocumentType::Legal => match style_idx {
            0 => WritingStyle::Formal,
            1 => WritingStyle::Technical,
            2 => WritingStyle::Academic,
            3 => WritingStyle::Expository,
            _ => WritingStyle::Formal,
        },
        DocumentType::Creative => match style_idx {
            0 => WritingStyle::Creative,
            1 => WritingStyle::Narrative,
            2 => WritingStyle::Descriptive,
            3 => WritingStyle::Conversational,
            _ => WritingStyle::Creative,
        },
    };
    
    // Additional options for specific document types
    let include_examples = match doc_type {
        DocumentType::TechnicalManual | DocumentType::UserGuide | DocumentType::LessonPlan => {
            Confirm::new()
                .with_prompt("Include practical examples and step-by-step instructions?")
                .default(true)
                .interact()?
        },
        DocumentType::BusinessPlan | DocumentType::Report => {
            Confirm::new()
                .with_prompt("Include charts, graphs, and data analysis sections?")
                .default(true)
                .interact()?
        },
        DocumentType::MarketingCopy => {
            Confirm::new()
                .with_prompt("Include call-to-action sections and testimonials?")
                .default(true)
                .interact()?
        },
        _ => false,
    };
    
    // Model selection with proper recommendations
    let length_desc = match length {
        DocumentLength::Brief => "Short",
        DocumentLength::Standard => "Medium",
        DocumentLength::Comprehensive => "Large",
        DocumentLength::Extensive => "Large",
    };
    
    let (use_local, model) = interactive_model_selection("document", length_desc)?;
    
    println!("\n📋 Creating {} {} {}...", 
             document_length_name(&length), 
             document_type_name(&doc_type).to_lowercase(), 
             if include_examples { "with examples" } else { "" });
    
    // Create the document
    write_document(doc_type, style, length, None, model, None, use_local, "http://localhost:11434".to_string()).await
}

fn document_type_name(doc_type: &DocumentType) -> &'static str {
    match doc_type {
        DocumentType::BusinessPlan => "Business Plan",
        DocumentType::TechnicalManual => "Technical Manual",
        DocumentType::UserGuide => "User Guide",
        DocumentType::Report => "Report",
        DocumentType::Proposal => "Proposal",
        DocumentType::MarketingCopy => "Marketing Copy",
        DocumentType::LegalTemplate => "Legal Template",
        DocumentType::LessonPlan => "Lesson Plan",
        DocumentType::CourseOutline => "Course Outline",
        DocumentType::Business => "Business Document",
        DocumentType::Academic => "Academic Document",
        DocumentType::Technical => "Technical Document",
        DocumentType::Legal => "Legal Document",
        DocumentType::Creative => "Creative Document",
    }
}

fn document_length_name(length: &DocumentLength) -> &'static str {
    match length {
        DocumentLength::Brief => "brief",
        DocumentLength::Standard => "standard",
        DocumentLength::Comprehensive => "comprehensive",
        DocumentLength::Extensive => "extensive",
    }
}

fn create_progress_bar(book: &Book) -> ProgressBar {
    // Legacy function - delegate to new function
    create_content_progress_bar(book)
}

async fn output_book(book: &Book, output_path: Option<PathBuf>, config: &Config) -> Result<()> {
    let output_dir = output_path.unwrap_or_else(|| config.output_directory.clone());
    
    // Ensure output directory exists
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }
    
    let safe_title = sanitize_filename(&book.title);
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    
    // Save as plain text with UTF-8 encoding
    let txt_path = output_dir.join(format!("{}_{}.txt", safe_title, timestamp));
    std::fs::write(&txt_path, book.to_text().as_bytes())?;
    
    // Save as markdown with UTF-8 encoding  
    let md_path = output_dir.join(format!("{}_{}.md", safe_title, timestamp));
    std::fs::write(&md_path, book.to_markdown().as_bytes())?;
    
    println!("\n📁 Book saved to:");
    println!("   Text: {}", txt_path.display());
    println!("   Markdown: {}", md_path.display());
    
    Ok(())
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

pub async fn narrate_mode() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    'book_setup: loop {
        println!("{}", console::style("🎭 Pundit - Narration Mode").bold().magenta());
        println!("═══════════════════════════════════════");
        println!();
        println!("Welcome to Pundit's interactive narration mode!");
        println!("I'll ask you a few questions to understand what kind of book you'd like me to write.");
        println!();
        
        // Genre selection
        'genre_selection: loop {
            let genres = vec![
                "Fiction", "Non-Fiction", "Mystery", "Romance", "Science Fiction", 
                "Fantasy", "Horror", "Thriller", "Biography", "History", "Self-Help",
                "Technical", "Poetry", "Drama", "Comedy", "Adventure", "Crime",
                "Educational (Language Learning, Textbooks, Courses)", 
                "← Back to main menu"
            ];
            
            let genre_idx = Select::new()
                .with_prompt("What genre would you like your book to be?")
                .items(&genres)
                .default(0)
                .interact()?;
            
            if genre_idx == genres.len() - 1 {
                return Err(BackToMenu.into()); // Back to main menu
            }
            
            // Handle Educational books separately
            if genre_idx == 17 { // Educational option
                return create_educational_book().await;
            }
            
            let genre = match genre_idx {
                0 => Genre::Fiction,
                1 => Genre::NonFiction,
                2 => Genre::Mystery,
                3 => Genre::Romance,
                4 => Genre::SciFi,
                5 => Genre::Fantasy,
                6 => Genre::Horror,
                7 => Genre::Thriller,
                8 => Genre::Biography,
                9 => Genre::History,
                10 => Genre::SelfHelp,
                11 => Genre::Technical,
                12 => Genre::Poetry,
                13 => Genre::Drama,
                14 => Genre::Comedy,
                15 => Genre::Adventure,
                16 => Genre::Crime,
                _ => Genre::Fiction,
            };
            
            // Writing style selection
            'style_selection: loop {
                let styles = vec![
                    "Conversational", "Descriptive", "Narrative", "Creative", "Formal",
                    "Casual", "Poetic", "Humorous", "Dramatic", "First Person", "Third Person",
                    "← Back to genre selection"
                ];
                
                let style_idx = Select::new()
                    .with_prompt("What writing style do you prefer?")
                    .items(&styles)
                    .default(0)
                    .interact()?;
                
                if style_idx == styles.len() - 1 {
                    continue 'genre_selection; // Back to genre selection
                }
                
                let style = match style_idx {
                    0 => WritingStyle::Conversational,
                    1 => WritingStyle::Descriptive,
                    2 => WritingStyle::Narrative,
                    3 => WritingStyle::Creative,
                    4 => WritingStyle::Formal,
                    5 => WritingStyle::Casual,
                    6 => WritingStyle::Poetic,
                    7 => WritingStyle::Humorous,
                    8 => WritingStyle::Dramatic,
                    9 => WritingStyle::FirstPerson,
                    10 => WritingStyle::ThirdPerson,
                    _ => WritingStyle::Conversational,
                };
                
                // Book size selection
                'size_selection: loop {
                    let sizes = vec![
                        "Short Story (1,000-7,500 words)",
                        "Short Book (20,000-50,000 words)",
                        "Medium Book (50,000-80,000 words)",
                        "Large Book (80,000-120,000 words)",
                        "Very Large Book (120,000-200,000 words)",
                        "Unlimited (Let Pundit decide when to stop)",
                        "← Back to style selection"
                    ];
                    
                    let size_idx = Select::new()
                        .with_prompt("How long should your book be?")
                        .items(&sizes)
                        .default(2)
                        .interact()?;
                    
                    if size_idx == sizes.len() - 1 {
                        continue 'style_selection; // Back to style selection
                    }
                    
                    let size = match size_idx {
                        0 => BookSize::ShortStory,
                        1 => BookSize::Short,
                        2 => BookSize::Medium,
                        3 => BookSize::Large,
                        4 => BookSize::VeryLarge,
                        5 => BookSize::Unlimited,
                        _ => BookSize::Medium,
                    };
                    
                    // Model type selection
                    'model_selection: loop {
                        println!("\n🤖 Model Selection:");
                        let model_options = vec![
                            "🏠 Local models (Ollama) - No API key needed, runs offline",
                            "☁️  Cloud models (HuggingFace) - Requires API key or limited free tier",
                            "← Back to size selection"
                        ];
                        
                        let model_type_idx = Select::new()
                            .with_prompt("Which type of model would you like to use?")
                            .items(&model_options)
                            .default(0) // Default to local
                            .interact()?;
                        
                        if model_type_idx == model_options.len() - 1 {
                            continue 'size_selection; // Back to size selection
                        }
                        
                        let (use_local, model) = if model_type_idx == 0 {
                            // Local models
                            let recommended_local = get_ollama_recommendation(&size);
                            println!("\n🏠 Local Model Options:");
                            println!("   Recommended: {} for {} books", recommended_local, size_to_description(&size));
                            println!("   Note: Make sure you have Ollama installed and the model downloaded");
                            
                            let use_recommended_local = Confirm::new()
                                .with_prompt(&format!("Use recommended local model ({}) for this book size?", recommended_local))
                                .default(true)
                                .interact()?;
                            
                            let local_model = if use_recommended_local {
                                recommended_local.to_string()
                            } else {
                                println!("\n📋 Available Ollama models:");
                                println!("   Fast: llama3.2:1b, gemma2:2b, phi3:mini");
                                println!("   Balanced: llama3.2, mistral:7b, qwen2:7b");
                                println!("   High Quality: llama3.1:8b, gemma2:9b, codellama:7b");
                                
                                Input::new()
                                    .with_prompt("Enter Ollama model name")
                                    .with_initial_text(recommended_local)
                                    .interact_text()?
                            };
                            
                            (true, local_model)
                        } else {
                            // Cloud models
                            let recommended_cloud = get_model_recommendation(&size);
                            println!("\n☁️ Cloud Model Options:");
                            println!("   Recommended: {} for {} books", recommended_cloud, size_to_description(&size));
                            println!("   Note: Some models work without API key, others may require authentication");
                            
                            let use_recommended_cloud = Confirm::new()
                                .with_prompt(&format!("Use recommended cloud model ({}) for this book size?", recommended_cloud))
                                .default(true)
                                .interact()?;
                            
                            let cloud_model = if use_recommended_cloud {
                                recommended_cloud.to_string()
                            } else {
                                Input::new()
                                    .with_prompt("Enter HuggingFace model name")
                                    .with_initial_text(recommended_cloud)
                                    .interact_text()?
                            };
                            
                            (false, cloud_model)
                        };
                        
                        // API Key (only for cloud models)
                        let api_key = if !use_local {
                            let has_api_key = Confirm::new()
                                .with_prompt("Do you have a Hugging Face API key? (recommended for better performance)")
                                .default(false)
                                .interact()?;
                            
                            if has_api_key {
                                Some(Input::new()
                                    .with_prompt("Enter your Hugging Face API key")
                                    .interact_text()?)
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        
                        println!("\n✨ Perfect! Let me start writing your {} {} book in {} style.", 
                            genre, size_to_description(&size), style);
                        
                        if use_local {
                            println!("🏠 Using local model: {}", model);
                            println!("📡 Connecting to Ollama server...");
                        } else {
                            println!("☁️  Using cloud model: {}", model);
                        }
                        println!();
                        
                        // Start writing
                        return write_book(genre, style, size, None, model, api_key, use_local, "http://localhost:11434".to_string()).await;
                    }
                }
            }
        }
    }
}

fn size_to_description(size: &BookSize) -> &'static str {
    match size {
        BookSize::ShortStory => "short story",
        BookSize::Short => "short",
        BookSize::Medium => "medium-length",
        BookSize::Large => "large",
        BookSize::VeryLarge => "very large",
        BookSize::Epic => "epic",
        BookSize::Unlimited => "unlimited-length",
    }
}

pub async fn continue_content(
    source_paths: Vec<PathBuf>,
    content_type: Option<String>,
    premise: String,
    target_length: Option<String>,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("🔄 Pundit - Content Continuation").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Read and analyze source documents
    let mut source_content = String::new();
    let mut detected_genre = String::new();
    let mut detected_style = String::new();
    let mut detected_content_type = ContentType::Book;
    
    println!("📖 Reading source documents...");
    for path in &source_paths {
        if path.exists() {
            println!("   Reading: {}", path.display());
            let content = fs::read_to_string(path)
                .map_err(|e| anyhow!("Failed to read file {}: {}", path.display(), e))?;
            
            // Add separator between documents
            if !source_content.is_empty() {
                source_content.push_str("\n\n--- END OF PREVIOUS DOCUMENT ---\n\n");
            }
            source_content.push_str(&content);
        } else {
            eprintln!("⚠️  Warning: File not found: {}", path.display());
        }
    }
    
    if source_content.is_empty() {
        return Err(anyhow!("No valid source content found. Please provide existing documents to continue from."));
    }
    
    // Analyze content to detect characteristics
    detected_content_type = analyze_content_type(&source_content);
    detected_genre = analyze_genre(&source_content);
    detected_style = analyze_writing_style(&source_content);
    
    println!("🔍 Analysis complete:");
    println!("   Detected type: {:?}", detected_content_type);
    println!("   Detected genre: {}", detected_genre);
    println!("   Detected style: {}", detected_style);
    println!();
    
    // Allow user to override content type
    let final_content_type = if let Some(user_type) = content_type {
        parse_content_type(&user_type).unwrap_or(detected_content_type)
    } else {
        detected_content_type
    };
    
    // Determine target sections/length
    let target_sections = parse_target_length(&target_length.as_ref().unwrap_or(&"10".to_string()));
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    let author = config.default_author.clone();
    
    // Extract key information for continuation
    let continuation_context = extract_continuation_context(&source_content, final_content_type);
    
    // Create a new content instance for the continuation
    let title = format!("Continuation: {}", premise);
    let mut content = match final_content_type {
        ContentType::Book => Content::new_book(
            title,
            author,
            detected_genre.clone(),
            detected_style.clone(),
            premise.clone(),
            target_length.unwrap_or_else(|| "Medium".to_string()),
            Some(target_sections * 3000), // Approximate words per section
            target_sections,
            model.clone(),
        ),
        ContentType::Screenplay => Content::new_screenplay(
            title,
            author,
            detected_genre.clone(),
            detected_style.clone(),
            premise.clone(),
            target_sections,
            model.clone(),
        ),
        _ => Content::new_book( // Default fallback
            title,
            author,
            detected_genre.clone(),
            detected_style.clone(),
            premise.clone(),
            target_length.unwrap_or_else(|| "Medium".to_string()),
            Some(target_sections * 3000),
            target_sections,
            model.clone(),
        ),
    };
    
    println!("🎬 Generating continuation outline...");
    
    // Generate outline for continuation
    let continuation_prompt = format!(
        "Based on the following existing content, create an outline for a continuation:\n\n\
        EXISTING CONTENT SUMMARY:\n{}\n\n\
        CONTINUATION PREMISE: {}\n\n\
        Create an outline for {} sections that continue the story naturally. \
        Maintain consistency with established characters, world, and tone.",
        continuation_context, premise, target_sections
    );
    
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&continuation_prompt, 1000, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(&model, &continuation_prompt, 1000, 0.7).await?
        }
    };
    
    content.outline = outline;
    println!("✅ Outline generated!");
    
    // Create progress bar
    let progress_bar = create_content_progress_bar(&content);
    progress_bar.set_message("Starting continuation generation...");
    
    // Generate continuation content
    println!("\n📝 Writing continuation...");
    let section_type = match final_content_type {
        ContentType::Book | ContentType::Memoir | ContentType::InteractiveFiction => SectionType::Chapter,
        ContentType::Screenplay | ContentType::Play => SectionType::Scene,
        ContentType::TvScript => SectionType::Episode,
        ContentType::AudioScript => SectionType::Segment,
        ContentType::GameScript => SectionType::Interaction,
        ContentType::Document | ContentType::TechnicalDoc | ContentType::WhitePaper | 
        ContentType::ResearchReport | ContentType::Poetry | ContentType::Journal |
        ContentType::MarketingAd | ContentType::PressRelease | ContentType::MediaKit |
        ContentType::BlogPost | ContentType::SeoArticle | ContentType::StrategicDoc |
        ContentType::PlanningDoc | ContentType::MeetingNotes | ContentType::MeetingSummary |
        ContentType::Dictionary | ContentType::EducationalLesson => SectionType::Section,
        ContentType::ChildrensBook => SectionType::Chapter,
    };
    
    // Write sections with continuation context
    for section_num in 1..=target_sections {
        progress_bar.set_message(format!("Writing {} {}...", section_type_name(&section_type), section_num));
        
        // Combine original context with new content context
        let combined_context = format!(
            "ORIGINAL CONTEXT:\n{}\n\nCONTINUATION SO FAR:\n{}",
            continuation_context,
            content.get_clean_context()
        );
        
        if let Err(e) = write_next_section(&client, &model, &mut content, section_num, section_type, &progress_bar).await {
            eprintln!("❌ Error writing section {}: {}", section_num, e);
            if !Confirm::new()
                .with_prompt("Continue with next section?")
                .default(true)
                .interact()? {
                break;
            }
        }
        
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Continuation completed!");
    
    // Save the continuation
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("continuation_{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save continuation: {}", e))?;
    
    println!("\n🎉 Continuation saved to: {}", output_path.display());
    println!("📊 Generated {} sections with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

fn analyze_content_type(content: &str) -> ContentType {
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("int.") || content_lower.contains("ext.") || content_lower.contains("fade in:") {
        ContentType::Screenplay
    } else if content_lower.contains("[") && content_lower.contains("]") && content_lower.contains(":") {
        ContentType::Play
    } else if content_lower.contains("episode") || content_lower.contains("commercial break") {
        ContentType::TvScript
    } else if content_lower.contains("sfx:") || content_lower.contains("music:") {
        ContentType::AudioScript
    } else if content_lower.contains("choice:") || content_lower.contains("action:") {
        ContentType::GameScript
    } else if content_lower.contains("executive summary") || content_lower.contains("recommendation") {
        ContentType::Document
    } else {
        ContentType::Book // Default
    }
}

fn analyze_genre(content: &str) -> String {
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("magic") || content_lower.contains("wizard") || content_lower.contains("dragon") {
        "Fantasy".to_string()
    } else if content_lower.contains("space") || content_lower.contains("robot") || content_lower.contains("alien") {
        "SciFi".to_string()
    } else if content_lower.contains("love") || content_lower.contains("romance") || content_lower.contains("heart") {
        "Romance".to_string()
    } else if content_lower.contains("murder") || content_lower.contains("detective") || content_lower.contains("mystery") {
        "Mystery".to_string()
    } else if content_lower.contains("horror") || content_lower.contains("scary") || content_lower.contains("terror") {
        "Horror".to_string()
    } else {
        "Fiction".to_string()
    }
}

fn analyze_writing_style(content: &str) -> String {
    let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
    let avg_sentence_length: f32 = sentences.iter()
        .map(|s| s.split_whitespace().count())
        .sum::<usize>() as f32 / sentences.len() as f32;
    
    if avg_sentence_length > 20.0 {
        "Literary".to_string()
    } else if avg_sentence_length > 15.0 {
        "Descriptive".to_string()
    } else if avg_sentence_length > 10.0 {
        "Narrative".to_string()
    } else {
        "Concise".to_string()
    }
}

fn parse_content_type(type_str: &str) -> Option<ContentType> {
    match type_str.to_lowercase().as_str() {
        "book" => Some(ContentType::Book),
        "screenplay" => Some(ContentType::Screenplay),
        "play" => Some(ContentType::Play),
        "tv" | "tvscript" => Some(ContentType::TvScript),
        "audio" | "audioscript" => Some(ContentType::AudioScript),
        "game" | "gamescript" => Some(ContentType::GameScript),
        "document" => Some(ContentType::Document),
        _ => None,
    }
}

fn parse_target_length(length_str: &str) -> usize {
    length_str.parse::<usize>().unwrap_or(10)
}

fn extract_continuation_context(content: &str, content_type: ContentType) -> String {
    // Extract the most relevant parts for continuation
    let words: Vec<&str> = content.split_whitespace().collect();
    let max_context_words = 2000; // Limit context to prevent overwhelming the AI
    
    if words.len() <= max_context_words {
        return content.to_string();
    }
    
    // Take the last portion of the content for continuation context
    let start_index = words.len().saturating_sub(max_context_words);
    let context_excerpt = words[start_index..].join(" ");
    
    format!(
        "CONTENT SUMMARY: This is a {} that needs to be continued.\n\n\
        RECENT CONTEXT (last {} words):\n{}",
        match content_type {
            ContentType::Book => "book",
            ContentType::Screenplay => "screenplay",
            ContentType::Play => "stage play",
            ContentType::TvScript => "TV script",
            ContentType::AudioScript => "audio script",
            ContentType::GameScript => "game script",
            ContentType::Document => "document",
            ContentType::TechnicalDoc => "technical documentation",
            ContentType::WhitePaper => "white paper",
            ContentType::ResearchReport => "research report",
            ContentType::Poetry => "poetry collection",
            ContentType::InteractiveFiction => "interactive fiction",
            ContentType::Journal => "journal",
            ContentType::Memoir => "memoir",
            ContentType::MarketingAd => "marketing content",
            ContentType::PressRelease => "press release",
            ContentType::MediaKit => "media kit",
            ContentType::BlogPost => "blog post",
            ContentType::SeoArticle => "SEO article",
            ContentType::StrategicDoc => "strategic document",
            ContentType::PlanningDoc => "planning document",
            ContentType::MeetingNotes => "meeting notes",
            ContentType::MeetingSummary => "meeting summary",
            ContentType::Dictionary => "dictionary",
            ContentType::EducationalLesson => "educational lesson",
            ContentType::ChildrensBook => "children's book",
        },
        max_context_words,
        context_excerpt
    )
}

pub async fn write_technical_doc(
    doc_type: TechnicalDocType,
    audience: String,
    subject: String,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("📋 Pundit - Technical Documentation Writer").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    let author = config.default_author.clone();
    
    // Create content instance for technical documentation
    let content_type = ContentType::TechnicalDoc;
    let title = format!("{} - {}", doc_type_name(&doc_type), subject);
    let sections = estimate_technical_sections(&doc_type);
    
    let mut content = Content::new_document(
        title,
        author,
        "Technical".to_string(),
        format!("Technical documentation for {}", audience),
        sections,
        DocumentFormat::Technical,
        model.clone(),
    );
    
    println!("📝 Document: {}", content.title);
    println!("   Type: {:?}", doc_type);
    println!("   Audience: {}", audience);
    println!("   Sections: {} planned", sections);
    println!();
    
    // Generate technical documentation outline
    println!("📋 Generating technical documentation outline...");
    let outline_prompt = format!(
        "Create a detailed outline for {} technical documentation about {}.\n\
        Target audience: {}\n\
        Create exactly {} sections with clear, technical section titles.\n\
        Focus on practical, actionable information.\n\
        Format as:\n\
        Section 1: [Title] - [Brief description]\n\
        Section 2: [Title] - [Brief description]\n\
        ...\n\nOutline:",
        doc_type_name(&doc_type), subject, audience, sections
    );
    
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&outline_prompt, 1000, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(&model, &outline_prompt, 1000, 0.7).await?
        }
    };
    
    content.outline = outline;
    println!("✅ Outline generated!");
    
    // Create progress bar
    let progress_bar = create_content_progress_bar(&content);
    progress_bar.set_message("Starting technical documentation generation...");
    
    // Write sections
    println!("\n📝 Writing technical documentation...");
    for section_num in 1..=sections {
        if let Err(e) = write_next_section(&client, &model, &mut content, section_num, SectionType::Section, &progress_bar).await {
            eprintln!("❌ Error writing section {}: {}", section_num, e);
            if !Confirm::new()
                .with_prompt("Continue with next section?")
                .default(true)
                .interact()? {
                break;
            }
        }
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Technical documentation completed!");
    
    // Save the documentation
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save technical documentation: {}", e))?;
    
    println!("\n🎉 Technical documentation saved to: {}", output_path.display());
    println!("📊 Generated {} sections with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

pub async fn write_research_doc(
    doc_type: ResearchDocType,
    topic: String,
    length: ResearchLength,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("🔬 Pundit - Research Documentation Writer").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    let author = config.default_author.clone();
    
    // Create content instance for research documentation
    let content_type = get_research_content_type(&doc_type);
    let title = format!("{}: {}", research_doc_type_name(&doc_type), topic);
    let sections = estimate_research_sections(&length);
    
    let mut content = Content::new_document(
        title,
        author,
        "Research".to_string(),
        format!("Professional research document on {}", topic),
        sections,
        DocumentFormat::Academic,
        model.clone(),
    );
    
    // Update content type
    content.content_type = content_type;
    
    println!("📝 Document: {}", content.title);
    println!("   Type: {:?}", doc_type);
    println!("   Length: {:?}", length);
    println!("   Sections: {} planned", sections);
    println!();
    
    // Generate research documentation outline
    println!("📋 Generating research documentation outline...");
    let outline_prompt = format!(
        "Create a detailed outline for a {} about {}.\n\
        Target length: {:?}\n\
        Create exactly {} sections following academic/professional standards.\n\
        Include methodology, analysis, findings, and conclusions.\n\
        Format as:\n\
        Section 1: [Title] - [Brief description]\n\
        Section 2: [Title] - [Brief description]\n\
        ...\n\nOutline:",
        research_doc_type_name(&doc_type), topic, length, sections
    );
    
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&outline_prompt, 1000, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(&model, &outline_prompt, 1000, 0.7).await?
        }
    };
    
    content.outline = outline;
    println!("✅ Outline generated!");
    
    // Create progress bar
    let progress_bar = create_content_progress_bar(&content);
    progress_bar.set_message("Starting research document generation...");
    
    // Write sections
    println!("\n📝 Writing research document...");
    for section_num in 1..=sections {
        if let Err(e) = write_next_section(&client, &model, &mut content, section_num, SectionType::Section, &progress_bar).await {
            eprintln!("❌ Error writing section {}: {}", section_num, e);
            if !Confirm::new()
                .with_prompt("Continue with next section?")
                .default(true)
                .interact()? {
                break;
            }
        }
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Research document completed!");
    
    // Save the documentation
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save research document: {}", e))?;
    
    println!("\n🎉 Research document saved to: {}", output_path.display());
    println!("📊 Generated {} sections with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

fn doc_type_name(doc_type: &TechnicalDocType) -> &'static str {
    match doc_type {
        TechnicalDocType::Manual => "User Manual",
        TechnicalDocType::ApiDocs => "API Documentation",
        TechnicalDocType::InstallGuide => "Installation Guide",
        TechnicalDocType::Tutorial => "Tutorial",
        TechnicalDocType::Troubleshooting => "Troubleshooting Guide",
        TechnicalDocType::AdminGuide => "Administrator Guide",
    }
}

fn research_doc_type_name(doc_type: &ResearchDocType) -> &'static str {
    match doc_type {
        ResearchDocType::WhitePaper => "White Paper",
        ResearchDocType::ResearchReport => "Research Report",
        ResearchDocType::CaseStudy => "Case Study",
        ResearchDocType::Analysis => "Analysis",
        ResearchDocType::Survey => "Survey Report",
        ResearchDocType::FeasibilityStudy => "Feasibility Study",
    }
}

fn estimate_technical_sections(doc_type: &TechnicalDocType) -> usize {
    match doc_type {
        TechnicalDocType::Manual => 8,
        TechnicalDocType::ApiDocs => 6,
        TechnicalDocType::InstallGuide => 5,
        TechnicalDocType::Tutorial => 7,
        TechnicalDocType::Troubleshooting => 6,
        TechnicalDocType::AdminGuide => 10,
    }
}

fn estimate_research_sections(length: &ResearchLength) -> usize {
    match length {
        ResearchLength::Brief => 5,
        ResearchLength::Standard => 8,
        ResearchLength::Comprehensive => 12,
        ResearchLength::Extensive => 16,
    }
}

fn get_research_content_type(doc_type: &ResearchDocType) -> ContentType {
    match doc_type {
        ResearchDocType::WhitePaper => ContentType::WhitePaper,
        ResearchDocType::ResearchReport => ContentType::ResearchReport,
        ResearchDocType::CaseStudy => ContentType::ResearchReport,
        ResearchDocType::Analysis => ContentType::ResearchReport,
        ResearchDocType::Survey => ContentType::ResearchReport,
        ResearchDocType::FeasibilityStudy => ContentType::WhitePaper,
    }
}

pub async fn write_poetry(
    style: PoetryStyle,
    theme: String,
    count: Option<usize>,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("🎭 Pundit - Poetry Writer").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    let author = config.default_author.clone();
    let poem_count = count.unwrap_or(5);
    
    // Create content instance for poetry
    let title = format!("{} Poetry: {}", poetry_style_name(&style), theme);
    
    let mut content = Content::new_document(
        title,
        author,
        "Poetry".to_string(),
        format!("{} poems on the theme of {}", poetry_style_name(&style), theme),
        poem_count,
        DocumentFormat::Educational,
        model.clone(),
    );
    
    // Update content type
    content.content_type = ContentType::Poetry;
    
    println!("🎭 Poetry Collection: {}", content.title);
    println!("   Style: {:?}", style);
    println!("   Theme: {}", theme);
    println!("   Poems: {} planned", poem_count);
    println!();
    
    // Generate poetry collection outline
    println!("📋 Generating poetry collection outline...");
    let outline_prompt = format!(
        "Create an outline for a collection of {} {} poems on the theme of {}.\n\
        Create exactly {} poem titles with brief descriptions.\n\
        Each poem should explore different aspects of the theme.\n\
        Format as:\n\
        Poem 1: [Title] - [Brief description]\n\
        Poem 2: [Title] - [Brief description]\n\
        ...\n\nOutline:",
        poem_count, poetry_style_name(&style), theme, poem_count
    );
    
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&outline_prompt, 800, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(&model, &outline_prompt, 800, 0.8).await?
        }
    };
    
    content.outline = outline;
    println!("✅ Outline generated!");
    
    // Create progress bar
    let progress_bar = create_content_progress_bar(&content);
    progress_bar.set_message("Starting poetry generation...");
    
    // Write poems using enhanced poetry generation
    println!("\n📝 Writing poetry collection...");
    for poem_num in 1..=poem_count {
        if let Err(e) = write_enhanced_poem(&client, &model, &mut content, poem_num, &style, &theme, &progress_bar).await {
            eprintln!("❌ Error writing poem {}: {}", poem_num, e);
            if !Confirm::new()
                .with_prompt("Continue with next poem?")
                .default(true)
                .interact()? {
                break;
            }
        }
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Poetry collection completed!");
    
    // Save the poetry collection
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save poetry collection: {}", e))?;
    
    println!("\n🎉 Poetry collection saved to: {}", output_path.display());
    println!("📊 Generated {} poems with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

pub async fn write_interactive_fiction(
    genre: Genre,
    premise: String,
    chapters: Option<usize>,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("🎮 Pundit - Interactive Fiction Writer").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    let author = config.default_author.clone();
    let chapter_count = chapters.unwrap_or(8);
    
    // Create content instance for interactive fiction
    let title = format!("Interactive Fiction: {}", premise);
    
    let mut content = Content::new_book(
        title,
        author,
        format!("{:?}", genre),
        "Interactive".to_string(),
        premise.clone(),
        "Interactive Fiction".to_string(),
        Some(chapter_count * 2000), // Approximate words
        chapter_count,
        model.clone(),
    );
    
    // Update content type
    content.content_type = ContentType::InteractiveFiction;
    
    println!("🎮 Interactive Story: {}", content.title);
    println!("   Genre: {:?}", genre);
    println!("   Premise: {}", premise);
    println!("   Chapters: {} planned", chapter_count);
    println!();
    
    // Generate interactive fiction outline
    println!("📋 Generating interactive fiction outline...");
    let outline_prompt = format!(
        "Create an outline for an interactive {} fiction story.\n\
        Premise: {}\n\
        Create exactly {} chapters with choice-driven narrative.\n\
        Include branching storylines and player decision points.\n\
        Format as:\n\
        Chapter 1: [Title] - [Brief description with choice options]\n\
        Chapter 2: [Title] - [Brief description with choice options]\n\
        ...\n\nOutline:",
        format!("{:?}", genre).to_lowercase(), premise, chapter_count
    );
    
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&outline_prompt, 1200, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(&model, &outline_prompt, 1200, 0.8).await?
        }
    };
    
    content.outline = outline;
    println!("✅ Outline generated!");
    
    // Create progress bar
    let progress_bar = create_content_progress_bar(&content);
    progress_bar.set_message("Starting interactive fiction generation...");
    
    // Write chapters
    println!("\n📝 Writing interactive fiction...");
    for chapter_num in 1..=chapter_count {
        if let Err(e) = write_next_section(&client, &model, &mut content, chapter_num, SectionType::Chapter, &progress_bar).await {
            eprintln!("❌ Error writing chapter {}: {}", chapter_num, e);
            if !Confirm::new()
                .with_prompt("Continue with next chapter?")
                .default(true)
                .interact()? {
                break;
            }
        }
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Interactive fiction completed!");
    
    // Save the interactive fiction
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save interactive fiction: {}", e))?;
    
    println!("\n🎉 Interactive fiction saved to: {}", output_path.display());
    println!("📊 Generated {} chapters with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

pub async fn write_personal_writing(
    writing_type: PersonalWritingType,
    subject: String,
    length: PersonalLength,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("📖 Pundit - Personal Writing Assistant").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    let author = config.default_author.clone();
    let entry_count = estimate_personal_entries(&length);
    
    // Create content instance for personal writing
    let content_type = get_personal_content_type(&writing_type);
    let title = format!("{}: {}", personal_writing_type_name(&writing_type), subject);
    
    let mut content = Content::new_document(
        title,
        author,
        "Personal".to_string(),
        format!("Personal writing about {}", subject),
        entry_count,
        DocumentFormat::Educational,
        model.clone(),
    );
    
    // Update content type
    content.content_type = content_type;
    
    println!("📖 Personal Writing: {}", content.title);
    println!("   Type: {:?}", writing_type);
    println!("   Subject: {}", subject);
    println!("   Entries: {} planned", entry_count);
    println!();
    
    // Generate personal writing outline
    println!("📋 Generating personal writing outline...");
    let outline_prompt = format!(
        "Create an outline for a {} about {}.\n\
        Create exactly {} entries with personal, reflective content.\n\
        Include emotional depth and personal insights.\n\
        Format as:\n\
        Entry 1: [Title/Date] - [Brief description]\n\
        Entry 2: [Title/Date] - [Brief description]\n\
        ...\n\nOutline:",
        personal_writing_type_name(&writing_type), subject, entry_count
    );
    
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&outline_prompt, 1000, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(&model, &outline_prompt, 1000, 0.8).await?
        }
    };
    
    content.outline = outline;
    println!("✅ Outline generated!");
    
    // Create progress bar
    let progress_bar = create_content_progress_bar(&content);
    progress_bar.set_message("Starting personal writing generation...");
    
    // Write entries
    println!("\n📝 Writing personal entries...");
    for entry_num in 1..=entry_count {
        if let Err(e) = write_next_section(&client, &model, &mut content, entry_num, SectionType::Section, &progress_bar).await {
            eprintln!("❌ Error writing entry {}: {}", entry_num, e);
            if !Confirm::new()
                .with_prompt("Continue with next entry?")
                .default(true)
                .interact()? {
                break;
            }
        }
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Personal writing completed!");
    
    // Save the personal writing
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save personal writing: {}", e))?;
    
    println!("\n🎉 Personal writing saved to: {}", output_path.display());
    println!("📊 Generated {} entries with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

fn poetry_style_name(style: &PoetryStyle) -> &'static str {
    match style {
        PoetryStyle::Sonnet => "Sonnet",
        PoetryStyle::Haiku => "Haiku",
        PoetryStyle::FreeVerse => "Free Verse",
        PoetryStyle::Ballad => "Ballad",
        PoetryStyle::Limerick => "Limerick",
        PoetryStyle::Epic => "Epic",
        PoetryStyle::Lyric => "Lyric",
        PoetryStyle::Acrostic => "Acrostic",
    }
}

fn personal_writing_type_name(writing_type: &PersonalWritingType) -> &'static str {
    match writing_type {
        PersonalWritingType::Journal => "Journal",
        PersonalWritingType::Memoir => "Memoir",
        PersonalWritingType::Diary => "Diary",
        PersonalWritingType::TravelJournal => "Travel Journal",
        PersonalWritingType::CreativeJournal => "Creative Journal",
        PersonalWritingType::Reflection => "Reflection",
        PersonalWritingType::Blog => "Blog",
        PersonalWritingType::Letter => "Letter",
        PersonalWritingType::Essay => "Essay",
    }
}

fn estimate_personal_entries(length: &PersonalLength) -> usize {
    match length {
        PersonalLength::Brief => 8,
        PersonalLength::Standard => 15,
        PersonalLength::Extended => 25,
        PersonalLength::Comprehensive => 40,
    }
}

fn get_personal_content_type(writing_type: &PersonalWritingType) -> ContentType {
    match writing_type {
        PersonalWritingType::Journal | PersonalWritingType::Diary | 
        PersonalWritingType::TravelJournal | PersonalWritingType::CreativeJournal |
        PersonalWritingType::Reflection => ContentType::Journal,
        PersonalWritingType::Memoir => ContentType::Memoir,
        PersonalWritingType::Blog => ContentType::BlogPost,
        PersonalWritingType::Letter => ContentType::Document,
        PersonalWritingType::Essay => ContentType::Document,
    }
}

pub async fn write_marketing_content(
    marketing_type: MarketingType,
    product: String,
    audience: String,
    length: Option<MarketingLength>,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("📢 Pundit - Marketing Content Writer").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    let author = config.default_author.clone();
    let content_length = length.unwrap_or(MarketingLength::Standard);
    let sections = estimate_marketing_sections(&content_length);
    
    // Create content instance for marketing content
    let content_type = get_marketing_content_type(&marketing_type);
    let title = format!("{}: {}", marketing_type_name(&marketing_type), product);
    
    let mut content = Content::new_document(
        title,
        author,
        "Marketing".to_string(),
        format!("{} for {} targeting {}", marketing_type_name(&marketing_type), product, audience),
        sections,
        DocumentFormat::Business,
        model.clone(),
    );
    
    // Update content type
    content.content_type = content_type;
    
    println!("📢 Marketing Content: {}", content.title);
    println!("   Type: {:?}", marketing_type);
    println!("   Product: {}", product);
    println!("   Audience: {}", audience);
    println!("   Sections: {} planned", sections);
    println!();
    
    // Generate marketing content outline
    println!("📋 Generating marketing content outline...");
    let outline_prompt = format!(
        "Create a detailed outline for {} for the product/service: {}.\n\
        Target audience: {}\n\
        Create exactly {} sections with compelling marketing content.\n\
        Focus on benefits, features, and persuasive messaging.\n\
        Include calls-to-action and engagement elements.\n\
        Format as:\n\
        Section 1: [Title] - [Brief description]\n\
        Section 2: [Title] - [Brief description]\n\
        ...\n\nOutline:",
        marketing_type_name(&marketing_type), product, audience, sections
    );
    
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&outline_prompt, 1000, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(&model, &outline_prompt, 1000, 0.7).await?
        }
    };
    
    content.outline = outline;
    println!("✅ Outline generated!");
    
    // Create progress bar
    let progress_bar = create_content_progress_bar(&content);
    progress_bar.set_message("Starting marketing content generation...");
    
    // Write sections
    println!("\n📝 Writing marketing content...");
    for section_num in 1..=sections {
        if let Err(e) = write_next_section(&client, &model, &mut content, section_num, SectionType::Section, &progress_bar).await {
            eprintln!("❌ Error writing section {}: {}", section_num, e);
            if !Confirm::new()
                .with_prompt("Continue with next section?")
                .default(true)
                .interact()? {
                break;
            }
        }
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Marketing content completed!");
    
    // Save the marketing content
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save marketing content: {}", e))?;
    
    println!("\n🎉 Marketing content saved to: {}", output_path.display());
    println!("📊 Generated {} sections with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

fn marketing_type_name(marketing_type: &MarketingType) -> &'static str {
    match marketing_type {
        MarketingType::SocialAd => "Social Media Advertisement",
        MarketingType::SocialMediaAd => "Social Media Advertisement",
        MarketingType::DisplayAd => "Display Advertisement",
        MarketingType::VideoScript => "Video Advertisement Script",
        MarketingType::PressRelease => "Press Release",
        MarketingType::MediaKit => "Media Kit",
        MarketingType::ProductDescription => "Product Description",
        MarketingType::LandingPage => "Landing Page",
        MarketingType::EmailCampaign => "Email Campaign",
        MarketingType::Brochure => "Marketing Brochure",
    }
}

fn estimate_marketing_sections(length: &MarketingLength) -> usize {
    match length {
        MarketingLength::Brief => 2,
        MarketingLength::Standard => 4,
        MarketingLength::Extended => 6,
        MarketingLength::Comprehensive => 8,
    }
}

fn get_marketing_content_type(marketing_type: &MarketingType) -> ContentType {
    match marketing_type {
        MarketingType::SocialAd | MarketingType::SocialMediaAd | MarketingType::DisplayAd | 
        MarketingType::VideoScript | MarketingType::ProductDescription |
        MarketingType::LandingPage | MarketingType::EmailCampaign |
        MarketingType::Brochure => ContentType::MarketingAd,
        MarketingType::PressRelease => ContentType::PressRelease,
        MarketingType::MediaKit => ContentType::MediaKit,
    }
}

pub async fn write_blog_content(
    content_type: BlogContentType,
    topic: String,
    keywords: Option<String>,
    audience: Option<String>,
    length: Option<BlogLength>,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("📝 Pundit - Blog & SEO Content Writer").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    let author = config.default_author.clone();
    let content_length = length.unwrap_or(BlogLength::Medium);
    let sections = estimate_blog_sections(&content_length);
    let target_audience = audience.unwrap_or_else(|| "general readers".to_string());
    
    // Create content instance for blog content
    let blog_content_type = get_blog_content_type(&content_type);
    let title = format!("{}: {}", blog_content_type_name(&content_type), topic);
    
    let mut content = Content::new_document(
        title,
        author,
        "Blog".to_string(),
        format!("{} about {} for {}", blog_content_type_name(&content_type), topic, target_audience),
        sections,
        DocumentFormat::Business,
        model.clone(),
    );
    
    // Update content type
    content.content_type = blog_content_type;
    
    println!("📝 Blog Content: {}", content.title);
    println!("   Type: {:?}", content_type);
    println!("   Topic: {}", topic);
    println!("   Audience: {}", target_audience);
    if let Some(ref kw) = keywords {
        println!("   Keywords: {}", kw);
    }
    println!("   Sections: {} planned", sections);
    println!();
    
    // Generate blog content outline
    println!("📋 Generating blog content outline...");
    let mut outline_prompt = format!(
        "Create a detailed outline for {} about {}.\n\
        Target audience: {}\n\
        Create exactly {} sections with engaging, informative content.\n\
        Include compelling headlines and valuable information.\n",
        blog_content_type_name(&content_type), topic, target_audience, sections
    );
    
    // Add SEO considerations if keywords are provided
    if let Some(ref kw) = keywords {
        outline_prompt.push_str(&format!(
            "SEO Keywords to incorporate naturally: {}\n\
            Focus on search engine optimization and keyword integration.\n",
            kw
        ));
    }
    
    outline_prompt.push_str(
        "Format as:\n\
        Section 1: [Title] - [Brief description]\n\
        Section 2: [Title] - [Brief description]\n\
        ...\n\nOutline:"
    );
    
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&outline_prompt, 1200, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(&model, &outline_prompt, 1200, 0.7).await?
        }
    };
    
    content.outline = outline;
    println!("✅ Outline generated!");
    
    // Create progress bar
    let progress_bar = create_content_progress_bar(&content);
    progress_bar.set_message("Starting blog content generation...");
    
    // Write sections
    println!("\n📝 Writing blog content...");
    for section_num in 1..=sections {
        if let Err(e) = write_next_section(&client, &model, &mut content, section_num, SectionType::Section, &progress_bar).await {
            eprintln!("❌ Error writing section {}: {}", section_num, e);
            if !Confirm::new()
                .with_prompt("Continue with next section?")
                .default(true)
                .interact()? {
                break;
            }
        }
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Blog content completed!");
    
    // Save the blog content
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save blog content: {}", e))?;
    
    println!("\n🎉 Blog content saved to: {}", output_path.display());
    println!("📊 Generated {} sections with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

fn blog_content_type_name(content_type: &BlogContentType) -> &'static str {
    match content_type {
        BlogContentType::BlogPost => "Blog Post",
        BlogContentType::SeoArticle => "SEO Article",
        BlogContentType::Tutorial => "Tutorial",
        BlogContentType::Listicle => "Listicle",
        BlogContentType::Review => "Review",
        BlogContentType::NewsArticle => "News Article",
        BlogContentType::Opinion => "Opinion Piece",
        BlogContentType::Interview => "Interview",
        BlogContentType::CaseStudy => "Case Study",
    }
}

fn estimate_blog_sections(length: &BlogLength) -> usize {
    match length {
        BlogLength::Short => 3,    // 500-800 words
        BlogLength::Medium => 5,   // 800-1500 words
        BlogLength::Long => 8,     // 1500-2500 words
        BlogLength::VeryLong => 10,// 2000-3000 words
        BlogLength::Epic => 12,    // 2500+ words
    }
}

fn get_blog_content_type(content_type: &BlogContentType) -> ContentType {
    match content_type {
        BlogContentType::SeoArticle => ContentType::SeoArticle,
        BlogContentType::BlogPost | BlogContentType::Tutorial | 
        BlogContentType::Listicle | BlogContentType::Review |
        BlogContentType::NewsArticle | BlogContentType::Opinion |
        BlogContentType::Interview | BlogContentType::CaseStudy => ContentType::BlogPost,
    }
}

pub async fn write_strategic_doc(
    doc_type: StrategicDocType,
    organization: String,
    timeframe: String,
    length: Option<StrategicLength>,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("📋 Pundit - Strategic Document Writer").bold().cyan());
    println!("═══════════════════════════════════════════════");
    println!();
    
    let config = Config::load()?;
    
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    println!("🔍 Checking model availability...");
    check_model_availability(&client, &model).await?;
    
    let doc_length = length.unwrap_or(StrategicLength::Standard);
    let content_type = ContentType::StrategicDoc;
    
    let title = format!("{} for {}", strategic_doc_type_name(&doc_type), organization);
    
    let premise = format!(
        "Create a comprehensive {} for {} with a {} timeframe. Focus on strategic objectives, implementation plans, and measurable outcomes.",
        strategic_doc_type_name(&doc_type).to_lowercase(),
        organization,
        timeframe
    );
    
    println!("📄 Document Type: {}", strategic_doc_type_name(&doc_type));
    println!("🏢 Organization: {}", organization);
    println!("⏰ Timeframe: {}", timeframe);
    println!("📊 Length: {:?}", doc_length);
    println!();
    
    let num_sections = estimate_strategic_sections(&doc_length);
    
    let mut content = Content::new(
        title.clone(),
        "AI Assistant".to_string(),
        "Business".to_string(),
        "Professional".to_string(),
        premise.clone(),
        "Strategic document structure".to_string(),
        None,
        num_sections,
        model.clone()
    );
    
    // Set the content type
    content.content_type = content_type;
    
    println!("🎯 Generating strategic document outline...");
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_content_outline(&content_type, "Business", "Professional", &premise, num_sections).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_content_outline(&model, &content_type, "Business", "Professional", &premise, num_sections).await?
        }
    };
    
    content.outline = outline;
    
    let progress_bar = ProgressBar::new(num_sections as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("#>-")
    );
    
    println!("\n✍️  Generating {} sections...", num_sections);
    
    for i in 1..=num_sections {
        progress_bar.set_message(format!("Section {}", i));
        
        let section_outline = extract_section_outline(&content.outline, i, &SectionType::Section);
        let section_title = section_outline.as_ref()
            .and_then(|outline| extract_section_title(outline, &SectionType::Section));
        
        let target_words = estimate_strategic_words(&doc_length) / num_sections;
        
        let context = get_clean_context_for_section(&content, i, &SectionType::Section, false);
        let section_content = match &client {
            AIClient::HuggingFace(hf_client) => {
                hf_client.generate_content_section(&content_type, "Business", &context, &section_outline.as_deref().unwrap_or(""), target_words).await?
            },
            AIClient::Ollama(ollama_client) => {
                ollama_client.generate_content_section(&model, &content_type, "Business", &context, &section_outline.as_deref().unwrap_or(""), target_words).await?
            }
        };
        
        let cleaned_content = clean_generated_content(&section_content, i, &SectionType::Section);
        let filtered_content = filter_ai_meta_commentary(&cleaned_content);
        
        let section = Section {
            number: i,
            title: section_title.unwrap_or_else(|| format!("Section {}", i)),
            content: filtered_content,
            section_type: SectionType::Section,
            word_count: count_words(&section_content),
            outline: section_outline.unwrap_or_default(),
            created_at: Utc::now(),
            completed: true,
            plot_thread: None,
            narrative_context: None,
        };
        
        content.sections.push(section);
        
        sleep(Duration::from_millis(100)).await;
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Strategic document completed!");
    
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save strategic document: {}", e))?;
    
    println!("\n🎉 Strategic document saved to: {}", output_path.display());
    println!("📊 Generated {} sections with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

pub async fn write_meeting_doc(
    doc_type: MeetingDocType,
    purpose: String,
    attendees: Option<u32>,
    duration: Option<String>,
    length: Option<MeetingLength>,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("📝 Pundit - Meeting Document Writer").bold().cyan());
    println!("══════════════════════════════════════════════");
    println!();
    
    let config = Config::load()?;
    
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    println!("🔍 Checking model availability...");
    check_model_availability(&client, &model).await?;
    
    let doc_length = length.unwrap_or(MeetingLength::Standard);
    let content_type = get_meeting_content_type(&doc_type);
    
    let title = format!("{}: {}", meeting_doc_type_name(&doc_type), purpose);
    
    let mut premise = format!(
        "Create detailed {} for a meeting about {}.",
        meeting_doc_type_name(&doc_type).to_lowercase(),
        purpose
    );
    
    if let Some(num_attendees) = attendees {
        premise.push_str(&format!(" Meeting has {} attendees.", num_attendees));
    }
    
    if let Some(ref meeting_duration) = duration {
        premise.push_str(&format!(" Meeting duration: {}.", meeting_duration));
    }
    
    println!("📄 Document Type: {}", meeting_doc_type_name(&doc_type));
    println!("🎯 Purpose: {}", purpose);
    if let Some(num) = attendees {
        println!("👥 Attendees: {}", num);
    }
    if let Some(ref dur) = duration {
        println!("⏱️  Duration: {}", dur);
    }
    println!("📊 Length: {:?}", doc_length);
    println!();
    
    let num_sections = estimate_meeting_sections(&doc_length);
    
    let mut content = Content::new(
        title.clone(),
        "Meeting Secretary".to_string(),
        "Business".to_string(),
        "Professional".to_string(),
        premise.clone(),
        "Meeting document structure".to_string(),
        None,
        num_sections,
        model.clone()
    );
    
    // Set the content type
    content.content_type = content_type;
    
    println!("🎯 Generating meeting document outline...");
    let outline = match &client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_content_outline(&content_type, "Business", "Professional", &premise, num_sections).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_content_outline(&model, &content_type, "Business", "Professional", &premise, num_sections).await?
        }
    };
    
    content.outline = outline;
    
    let progress_bar = ProgressBar::new(num_sections as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("#>-")
    );
    
    println!("\n✍️  Generating {} sections...", num_sections);
    
    for i in 1..=num_sections {
        progress_bar.set_message(format!("Section {}", i));
        
        let section_outline = extract_section_outline(&content.outline, i, &SectionType::Section);
        let section_title = section_outline.as_ref()
            .and_then(|outline| extract_section_title(outline, &SectionType::Section));
        
        let target_words = estimate_meeting_words(&doc_length) / num_sections;
        
        let context = get_clean_context_for_section(&content, i, &SectionType::Section, false);
        let section_content = match &client {
            AIClient::HuggingFace(hf_client) => {
                hf_client.generate_content_section(&content_type, "Business", &context, &section_outline.as_deref().unwrap_or(""), target_words).await?
            },
            AIClient::Ollama(ollama_client) => {
                ollama_client.generate_content_section(&model, &content_type, "Business", &context, &section_outline.as_deref().unwrap_or(""), target_words).await?
            }
        };
        
        let cleaned_content = clean_generated_content(&section_content, i, &SectionType::Section);
        let filtered_content = filter_ai_meta_commentary(&cleaned_content);
        
        let section = Section {
            number: i,
            title: section_title.unwrap_or_else(|| format!("Section {}", i)),
            content: filtered_content,
            section_type: SectionType::Section,
            word_count: count_words(&section_content),
            outline: section_outline.unwrap_or_default(),
            created_at: Utc::now(),
            completed: true,
            plot_thread: None,
            narrative_context: None,
        };
        
        content.sections.push(section);
        
        sleep(Duration::from_millis(100)).await;
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("✅ Meeting document completed!");
    
    let output_path = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.md", 
            content.title.to_lowercase().replace(' ', "_")))
    });
    
    let final_content = content.to_markdown();
    fs::write(&output_path, final_content)
        .map_err(|e| anyhow!("Failed to save meeting document: {}", e))?;
    
    println!("\n🎉 Meeting document saved to: {}", output_path.display());
    println!("📊 Generated {} sections with {} words total", 
        content.sections.len(), content.metadata.current_word_count);
    
    Ok(())
}

fn strategic_doc_type_name(doc_type: &StrategicDocType) -> &'static str {
    match doc_type {
        StrategicDocType::StrategicPlan => "Strategic Plan",
        StrategicDocType::BusinessPlan => "Business Plan",
        StrategicDocType::ProjectPlan => "Project Plan",
        StrategicDocType::Roadmap => "Roadmap",
        StrategicDocType::VisionDoc => "Vision Document",
        StrategicDocType::Governance => "Governance Document",
        StrategicDocType::RiskAssessment => "Risk Assessment",
        StrategicDocType::BudgetPlan => "Budget Plan",
    }
}

fn meeting_doc_type_name(doc_type: &MeetingDocType) -> &'static str {
    match doc_type {
        MeetingDocType::MeetingNotes => "Meeting Notes",
        MeetingDocType::ActionItems => "Action Items",
        MeetingDocType::Summary => "Meeting Summary",
        MeetingDocType::Transcript => "Meeting Transcript",
        MeetingDocType::DecisionLog => "Decision Log",
        MeetingDocType::Agenda => "Meeting Agenda",
    }
}

fn estimate_strategic_sections(length: &StrategicLength) -> usize {
    match length {
        StrategicLength::Brief => 5,          // 3-8 pages
        StrategicLength::Standard => 8,       // 8-20 pages
        StrategicLength::Comprehensive => 12, // 20-50 pages
        StrategicLength::Extensive => 20,     // 50+ pages
    }
}

fn estimate_strategic_words(length: &StrategicLength) -> usize {
    match length {
        StrategicLength::Brief => 2000,       // 3-8 pages
        StrategicLength::Standard => 6000,    // 8-20 pages
        StrategicLength::Comprehensive => 15000, // 20-50 pages
        StrategicLength::Extensive => 30000,  // 50+ pages
    }
}

fn estimate_meeting_sections(length: &MeetingLength) -> usize {
    match length {
        MeetingLength::Brief => 4,        // 1-2 pages
        MeetingLength::Standard => 6,     // 2-5 pages
        MeetingLength::Extended => 10,    // 5-8 pages
        MeetingLength::Detailed => 8,     // 5-10 pages
        MeetingLength::Comprehensive => 12, // 10+ pages
    }
}

fn estimate_meeting_words(length: &MeetingLength) -> usize {
    match length {
        MeetingLength::Brief => 800,       // 1-2 pages
        MeetingLength::Standard => 2000,   // 2-5 pages
        MeetingLength::Extended => 3000,   // 3-5 pages
        MeetingLength::Detailed => 4000,   // 5-10 pages
        MeetingLength::Comprehensive => 8000, // 10+ pages
    }
}

fn get_meeting_content_type(doc_type: &MeetingDocType) -> ContentType {
    match doc_type {
        MeetingDocType::Summary => ContentType::MeetingSummary,
        MeetingDocType::MeetingNotes | MeetingDocType::ActionItems |
        MeetingDocType::Transcript | MeetingDocType::DecisionLog |
        MeetingDocType::Agenda => ContentType::MeetingNotes,
    }
}

async fn generate_section_summary(client: &AIClient, model: &str, section: &Section, content_type: &ContentType) -> Result<String> {
    let prompt = format!(
        "Write a brief 2-3 sentence summary of this {} section titled '{}'. Focus on key plot points, character developments, and important information:\n\n{}\n\nSummary:",
        match content_type {
            ContentType::Book => "chapter",
            ContentType::Screenplay => "scene",
            ContentType::Play => "act",
            _ => "section"
        },
        section.title,
        {
            let limit = 1000.min(section.content.chars().count());
            let content_preview: String = section.content.chars().take(limit).collect();
            content_preview
        }
    );
    
    match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&prompt, 150, 0.7).await
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &prompt, 150, 0.7).await
        }
    }
}

async fn generate_segmented_content(
    client: &AIClient,
    model: &str,
    content: &mut Content,
    section_number: usize,
    section_outline: &str,
    target_words: usize,
    section_type: &SectionType,
) -> Result<String> {
    const SEGMENT_SIZE: usize = 1500; // Words per segment
    const MAX_CONTEXT_CHARS: usize = 8000; // Maximum context length
    
    if target_words <= SEGMENT_SIZE * 2 {
        // For smaller content, use regular generation
        let context = get_enhanced_context_for_section(content, section_number, section_type, false);
        return match client {
            AIClient::HuggingFace(hf_client) => {
                hf_client.generate_content_section(&content.content_type, &content.genre, &context, section_outline, target_words).await
            },
            AIClient::Ollama(ollama_client) => {
                ollama_client.generate_content_section(model, &content.content_type, &content.genre, &context, section_outline, target_words).await
            }
        };
    }
    
    // For large content, use segmented generation
    let num_segments = (target_words + SEGMENT_SIZE - 1) / SEGMENT_SIZE; // Ceiling division
    let mut generated_segments = Vec::new();
    let mut accumulated_content = String::new();
    
    println!("🔄 Generating large section in {} parts...", num_segments);
    
    for segment_idx in 0..num_segments {
        let segment_words = if segment_idx == num_segments - 1 {
            target_words - (segment_idx * SEGMENT_SIZE) // Last segment gets remaining words
        } else {
            SEGMENT_SIZE
        };
        
        // Build context for this segment
        let mut context = get_enhanced_context_for_section(content, section_number, section_type, false);
        
        // Add accumulated content from previous segments
        if !accumulated_content.is_empty() {
            let truncated_content = if accumulated_content.len() > MAX_CONTEXT_CHARS {
                format!("...\n{}", &accumulated_content[accumulated_content.len() - MAX_CONTEXT_CHARS..])
            } else {
                accumulated_content.clone()
            };
            context.push_str(&format!("\n\nContent generated so far:\n{}\n\n", truncated_content));
        }
        
        // Add segment-specific instruction
        let segment_prompt = if segment_idx == 0 {
            format!("{}\n\nGenerate the opening part (~{} words) of this section. This is part 1 of {}.", section_outline, segment_words, num_segments)
        } else if segment_idx == num_segments - 1 {
            format!("{}\n\nGenerate the concluding part (~{} words) of this section. This is the final part {} of {}. Provide a satisfying conclusion.", section_outline, segment_words, segment_idx + 1, num_segments)
        } else {
            format!("{}\n\nGenerate the middle part (~{} words) of this section. This is part {} of {}. Continue naturally from the previous content.", section_outline, segment_words, segment_idx + 1, num_segments)
        };
        
        println!("  📝 Part {}/{} ({} words)...", segment_idx + 1, num_segments, segment_words);
        
        let segment_content = match client {
            AIClient::HuggingFace(hf_client) => {
                hf_client.generate_content_section(&content.content_type, &content.genre, &context, &segment_prompt, segment_words).await?
            },
            AIClient::Ollama(ollama_client) => {
                ollama_client.generate_content_section(model, &content.content_type, &content.genre, &context, &segment_prompt, segment_words).await?
            }
        };
        
        let cleaned_segment = clean_generated_content(&segment_content, section_number, section_type);
        generated_segments.push(cleaned_segment.clone());
        accumulated_content.push_str(&cleaned_segment);
        accumulated_content.push_str("\n\n");
        
        // Brief pause between segments
        sleep(Duration::from_millis(200)).await;
    }
    
    // Stitch segments together with smooth transitions
    let stitched_content = stitch_content_segments(generated_segments);
    println!("  ✅ Section generation completed");
    
    Ok(stitched_content)
}

fn get_enhanced_context_for_section(content: &Content, section_number: usize, section_type: &SectionType, is_beyond_outline: bool) -> String {
    let context_sections = if is_beyond_outline {
        content.metadata.generation_parameters.context_window.min(2)
    } else {
        content.metadata.generation_parameters.context_window.min(5) // Increased context window
    };
    
    let start_idx = if content.sections.len() > context_sections {
        content.sections.len() - context_sections
    } else {
        0
    };
    
    let content_type_name = match content.content_type {
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
        content_type_name, content.title, content.genre, content.writing_style, content.premise
    );
    
    // Add recent sections with summaries for better context
    if content.sections.len() > 3 {
        context.push_str("Recent sections summary:\n");
        for (idx, section) in content.sections[start_idx..].iter().enumerate() {
            let summary = generate_section_summary_sync(&section.content);
            context.push_str(&format!("{}. {}: {}\n", start_idx + idx + 1, section.title, summary));
        }
        context.push_str("\n");
    } else {
        // For early sections, include more recent content
        context.push_str("Previous content:\n");
        for (idx, section) in content.sections[start_idx..].iter().enumerate() {
            let truncated = if section.content.chars().count() > 500 {
                let truncated_content: String = section.content.chars().take(500).collect();
                format!("{}...", truncated_content)
            } else {
                section.content.clone()
            };
            context.push_str(&format!("{}. {}: {}\n\n", start_idx + idx + 1, section.title, truncated));
        }
    }
    
    if is_beyond_outline {
        context.push_str("Note: Continue the story naturally beyond the original outline. Maintain consistency with established characters, plot, and tone.\n\n");
    }
    
    context
}

fn generate_section_summary_sync(content: &str) -> String {
    // Simple extractive summary - takes first and last sentences
    let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
    if sentences.len() > 2 {
        format!("{}.{}", sentences[0].trim(), sentences[sentences.len()-2].trim())
    } else {
        content.chars().take(100).collect::<String>() + "..."
    }
}

fn stitch_content_segments(segments: Vec<String>) -> String {
    if segments.is_empty() {
        return String::new();
    }
    
    if segments.len() == 1 {
        return segments[0].clone();
    }
    
    let mut result = String::new();
    
    for (i, segment) in segments.iter().enumerate() {
        if i == 0 {
            result.push_str(segment);
        } else {
            // Add smooth transition between segments
            let transition = create_segment_transition(&segments[i-1], segment);
            if !transition.is_empty() {
                result.push_str(&transition);
            }
            result.push_str(segment);
        }
        
        if i < segments.len() - 1 {
            result.push_str("\n\n");
        }
    }
    
    result
}

fn create_segment_transition(prev_segment: &str, next_segment: &str) -> String {
    // Simple transition logic - could be enhanced with AI generation
    let prev_end = prev_segment.trim_end();
    let next_start = next_segment.trim_start();
    
    // Check if transition is needed
    if prev_end.ends_with(['.', '!', '?', '"']) && next_start.chars().next().map_or(false, |c| c.is_uppercase()) {
        return String::new(); // Good natural break
    }
    
    // Add minimal transition
    "\n\n".to_string()
}

// Interactive creation functions for new content types

async fn interactive_technical_doc_creation() -> Result<()> {
    println!("\n🔬 Creating Technical Documentation");
    println!("Let me help you create professional technical documentation...\n");
    
    // Document type selection
    let doc_types = vec![
        "Manual - User manual or instruction guide",
        "API Documentation - API reference and examples", 
        "Installation Guide - Setup and configuration instructions",
        "Tutorial - Step-by-step learning guide",
        "Troubleshooting Guide - Problem-solving reference",
        "Administrator Guide - Admin configuration and management",
    ];
    
    let doc_type_idx = Select::new()
        .with_prompt("What type of technical document would you like to create?")
        .items(&doc_types)
        .default(0)
        .interact()?;
    
    let doc_type = match doc_type_idx {
        0 => crate::cli_types::TechnicalDocType::Manual,
        1 => crate::cli_types::TechnicalDocType::ApiDocs,
        2 => crate::cli_types::TechnicalDocType::InstallGuide,
        3 => crate::cli_types::TechnicalDocType::Tutorial,
        4 => crate::cli_types::TechnicalDocType::Troubleshooting,
        5 => crate::cli_types::TechnicalDocType::AdminGuide,
        _ => crate::cli_types::TechnicalDocType::Manual,
    };
    
    // Get target audience
    let audience: String = Input::new()
        .with_prompt("Who is the target audience? (e.g., developers, end users, administrators)")
        .interact_text()?;
    
    // Get subject/topic
    let subject: String = Input::new()
        .with_prompt("What is the subject or product being documented?")
        .interact_text()?;
    
    // Get output file path
    let output_path: String = Input::new()
        .with_prompt("Output file path (optional)")
        .default("technical_documentation.md".to_string())
        .interact_text()?;
    
    let output = if output_path.trim().is_empty() {
        None
    } else {
        Some(PathBuf::from(output_path))
    };
    
    // Get model preferences
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama instead of HuggingFace API?")
        .default(true)
        .interact()?;
    
    let (model, api_key, ollama_url) = if use_local {
        let model: String = Input::new()
            .with_prompt("Ollama model name")
            .default("llama3.2".to_string())
            .interact_text()?;
        
        let url: String = Input::new()
            .with_prompt("Ollama server URL")
            .default("http://localhost:11434".to_string())
            .interact_text()?;
            
        (model, None, url)
    } else {
        let model: String = Input::new()
            .with_prompt("HuggingFace model name")
            .default("microsoft/DialoGPT-large".to_string())
            .interact_text()?;
            
        let key: String = Input::new()
            .with_prompt("HuggingFace API key")
            .interact_text()?;
            
        (model, Some(key), "http://localhost:11434".to_string())
    };
    
    println!("\n🚀 Generating technical documentation...");
    write_technical_doc(
        doc_type,
        audience,
        subject,
        output,
        model,
        api_key,
        use_local,
        ollama_url,
    ).await
}

async fn interactive_research_doc_creation() -> Result<()> {
    loop {
        println!("\n📊 Creating Research Document");
        println!("Let me help you create a professional research document...\n");
        
        // Document type selection
        let doc_types = vec![
            "White Paper - Authoritative report or guide",
            "Research Report - Detailed research findings",
            "Case Study - In-depth analysis of specific case",
            "Analysis - Analytical examination of topic",
            "Survey Report - Survey results and analysis", 
            "Feasibility Study - Viability assessment",
            "← Back to main menu",
        ];
        
        let doc_type_idx = Select::new()
            .with_prompt("What type of research document would you like to create?")
            .items(&doc_types)
            .default(0)
            .interact()?;
        
        if doc_type_idx == doc_types.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let doc_type = match doc_type_idx {
            0 => crate::cli_types::ResearchDocType::WhitePaper,
            1 => crate::cli_types::ResearchDocType::ResearchReport,
            2 => crate::cli_types::ResearchDocType::CaseStudy,
            3 => crate::cli_types::ResearchDocType::Analysis,
            4 => crate::cli_types::ResearchDocType::Survey,
            5 => crate::cli_types::ResearchDocType::FeasibilityStudy,
            _ => crate::cli_types::ResearchDocType::WhitePaper,
        };
        
        // Get research topic
        let topic_result: Result<String, _> = Input::new()
            .with_prompt("What is the research topic or focus area? (or 'back' to return)")
            .interact_text();
        
        let topic = match topic_result {
            Ok(t) if t.trim().to_lowercase() == "back" => continue,
            Ok(t) => t,
            Err(_) => continue,
        };
        
        // Length selection
        let lengths = vec![
            "Brief (5-15 pages)",
            "Standard (15-40 pages)", 
            "Comprehensive (40-80 pages)",
            "Extensive (80+ pages)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("What length document would you like?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue; // Back to doc type selection
        }
        
        let length = match length_idx {
            0 => crate::cli_types::ResearchLength::Brief,
            1 => crate::cli_types::ResearchLength::Standard,
            2 => crate::cli_types::ResearchLength::Comprehensive,
            3 => crate::cli_types::ResearchLength::Extensive,
            _ => crate::cli_types::ResearchLength::Standard,
        };
        
        // Get output file path
        let output_path: String = Input::new()
            .with_prompt("Output file path (optional, or 'back' to return)")
            .default("research_document.md".to_string())
            .interact_text()?;
        
        if output_path.trim().to_lowercase() == "back" {
            continue;
        }
        
        let output = if output_path.trim().is_empty() {
            None
        } else {
            Some(PathBuf::from(output_path))
        };
        
        // Get model preferences
        let model_options = vec![
            "Use local Ollama (recommended)",
            "Use HuggingFace API",
            "← Back",
        ];
        
        let model_idx = Select::new()
            .with_prompt("Choose your AI model source:")
            .items(&model_options)
            .default(0)
            .interact()?;
        
        if model_idx == model_options.len() - 1 {
            continue;
        }
        
        let use_local = model_idx == 0;
        
        let (model, api_key, ollama_url) = if use_local {
            let model: String = Input::new()
                .with_prompt("Ollama model name (or 'back' to return)")
                .default("llama3.2".to_string())
                .interact_text()?;
            
            if model.trim().to_lowercase() == "back" {
                continue;
            }
            
            let url: String = Input::new()
                .with_prompt("Ollama server URL")
                .default("http://localhost:11434".to_string())
                .interact_text()?;
                
            (model, None, url)
        } else {
            let model: String = Input::new()
                .with_prompt("HuggingFace model name")
                .default("microsoft/DialoGPT-large".to_string())
                .interact_text()?;
                
            let key: String = Input::new()
                .with_prompt("HuggingFace API key")
                .interact_text()?;
                
            (model, Some(key), "http://localhost:11434".to_string())
        };
        
        println!("\n🚀 Generating research document...");
        return write_research_doc(
            doc_type,
            topic,
            length,
            output,
            model,
            api_key,
            use_local,
            ollama_url,
        ).await;
    }
}

async fn interactive_poetry_creation() -> Result<()> {
    loop {
        println!("\n🎨 Creating Poetry with Enhanced Emotional Anchors");
        println!("Let me help you create beautiful, emotionally resonant poetry...\n");
    
    // Poetry style selection
    let styles = vec![
        "Sonnet - 14-line traditional sonnet",
        "Haiku - Traditional 3-line Japanese form",
        "Free Verse - Unstructured, flowing style",
        "Ballad - Narrative storytelling poem",
        "Limerick - Humorous 5-line form",
        "Epic - Long narrative poem",
        "Lyric - Personal, emotional expression",
        "Acrostic - First letters spell a word",
        "← Back to main menu",
    ];
    
    let style_idx = Select::new()
        .with_prompt("What style of poetry would you like to create?")
        .items(&styles)
        .default(2)
        .interact()?;
    
    if style_idx == styles.len() - 1 {
        return Err(BackToMenu.into()); // Back to main menu
    }
    
    let style = match style_idx {
        0 => crate::cli_types::PoetryStyle::Sonnet,
        1 => crate::cli_types::PoetryStyle::Haiku,
        2 => crate::cli_types::PoetryStyle::FreeVerse,
        3 => crate::cli_types::PoetryStyle::Ballad,
        4 => crate::cli_types::PoetryStyle::Limerick,
        5 => crate::cli_types::PoetryStyle::Epic,
        6 => crate::cli_types::PoetryStyle::Lyric,
        7 => crate::cli_types::PoetryStyle::Acrostic,
        _ => crate::cli_types::PoetryStyle::FreeVerse,
    };
    
    // Get theme with emotional guidance
    let theme: String = Input::new()
        .with_prompt("What is the theme or subject of your poetry? (or 'back' to return)\n(Examples: love and longing, nature's wonder, peaceful solitude, joyful celebration)")
        .interact_text()?;
    
    if theme.trim().to_lowercase() == "back" {
        continue;
    }
    
    // Suggest emotion based on theme
    let suggested_emotion = create_emotion_from_theme(&theme);
    println!("💡 Based on your theme, I suggest writing with the feeling of: {}", suggested_emotion);
    
    let use_suggestion = Confirm::new()
        .with_prompt("Use this emotional direction? (or ESC to go back)")
        .default(true)
        .interact()?;
    
    let emotion = if use_suggestion {
        suggested_emotion
    } else {
        let emotions = vec!["longing", "joy", "melancholy", "wonder", "peace", "← Back"];
        let emotion_idx = Select::new()
            .with_prompt("Choose the emotional feeling for your poetry:")
            .items(&emotions)
            .default(0)
            .interact()?;
        if emotion_idx == emotions.len() - 1 {
            continue;
        }
        emotions[emotion_idx].to_string()
    };
    
    // Get number of poems
    let count_str: String = Input::new()
        .with_prompt("How many poems would you like? (default: 3)")
        .default("3".to_string())
        .interact_text()?;
    
    let count = count_str.parse::<usize>().unwrap_or(3);
    
    // Get output file path
    let output_path: String = Input::new()
        .with_prompt("Output file path (optional)")
        .default("enhanced_poetry.md".to_string())
        .interact_text()?;
    
    let output = if output_path.trim().is_empty() {
        None
    } else {
        Some(PathBuf::from(output_path))
    };
    
    // Get model preferences
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama instead of HuggingFace API?")
        .default(true)
        .interact()?;
    
    let (model, api_key, ollama_url) = if use_local {
        let model: String = Input::new()
            .with_prompt("Ollama model name")
            .default("llama3.2".to_string())
            .interact_text()?;
        
        let url: String = Input::new()
            .with_prompt("Ollama server URL")
            .default("http://localhost:11434".to_string())
            .interact_text()?;
            
        (model, None, url)
    } else {
        let model: String = Input::new()
            .with_prompt("HuggingFace model name")
            .default("microsoft/DialoGPT-large".to_string())
            .interact_text()?;
            
        let key: String = Input::new()
            .with_prompt("HuggingFace API key")
            .interact_text()?;
            
        (model, Some(key), "http://localhost:11434".to_string())
    };
    
    println!("\n🚀 Generating {} poems with {} emotional feeling...", count, emotion);
    println!("💫 Using enhanced prompting with emotional anchors and rhyme guidance...");
    
    return write_poetry(
        style,
        theme,
        Some(count),
        output,
        model,
        api_key,
        use_local,
        ollama_url,
    ).await;
    }
}

async fn interactive_marketing_creation() -> Result<()> {
    loop {
        println!("\n📝 Creating Marketing Content");
        println!("Let me help you create compelling marketing content...\n");
        
        // Marketing type selection
        let marketing_types = vec![
            "Social Media Ad - Social media advertisement",
            "Display Ad - Banner or display advertisement",
            "Video Script - Video advertisement script",
            "Press Release - News announcement",
            "Media Kit - Complete media package",
            "Product Description - Detailed product copy",
            "Landing Page - Website landing page copy",
            "Email Campaign - Email marketing content",
            "Brochure - Marketing brochure content",
            "← Back to main menu",
        ];
        
        let type_idx = Select::new()
            .with_prompt("What type of marketing content would you like to create?")
            .items(&marketing_types)
            .default(0)
            .interact()?;
        
        if type_idx == marketing_types.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let marketing_type = match type_idx {
            0 => crate::cli_types::MarketingType::SocialMediaAd,
            1 => crate::cli_types::MarketingType::DisplayAd,
            2 => crate::cli_types::MarketingType::VideoScript,
            3 => crate::cli_types::MarketingType::PressRelease,
            4 => crate::cli_types::MarketingType::MediaKit,
            5 => crate::cli_types::MarketingType::ProductDescription,
            6 => crate::cli_types::MarketingType::LandingPage,
            7 => crate::cli_types::MarketingType::EmailCampaign,
            8 => crate::cli_types::MarketingType::Brochure,
            _ => crate::cli_types::MarketingType::SocialMediaAd,
        };
        
        // Get product/service
        let product: String = Input::new()
            .with_prompt("What product or service are you marketing? (or 'back' to return)")
            .interact_text()?;
        
        if product.trim().to_lowercase() == "back" {
            continue;
        }
        
        // Get target audience
        let audience: String = Input::new()
            .with_prompt("Who is your target audience? (e.g., young professionals, families, tech enthusiasts) (or 'back' to return)")
            .interact_text()?;
        
        if audience.trim().to_lowercase() == "back" {
            continue;
        }
        
        // Length selection
        let lengths = vec![
            "Brief (1-2 sections)",
            "Standard (3-5 sections)",
            "Extended (6-8 sections)",
            "Comprehensive (9+ sections)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("What length content would you like?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue;
        }
        
        let length = Some(match length_idx {
            0 => crate::cli_types::MarketingLength::Brief,
            1 => crate::cli_types::MarketingLength::Standard,
            2 => crate::cli_types::MarketingLength::Extended,
            3 => crate::cli_types::MarketingLength::Comprehensive,
            _ => crate::cli_types::MarketingLength::Standard,
        });
        
        // Get output file path
        let output_path: String = Input::new()
            .with_prompt("Output file path (optional)")
            .default("marketing_content.md".to_string())
            .interact_text()?;
        
        if output_path.trim().to_lowercase() == "back" {
            continue;
        }
        
        let output = if output_path.trim().is_empty() {
            None
        } else {
            Some(PathBuf::from(output_path))
        };
        
        // Get model preferences
        let model_options = vec![
            "Use local Ollama (recommended)",
            "Use HuggingFace API",
            "← Back",
        ];
        
        let model_idx = Select::new()
            .with_prompt("Choose your AI model source:")
            .items(&model_options)
            .default(0)
            .interact()?;
        
        if model_idx == model_options.len() - 1 {
            continue;
        }
        
        let use_local = model_idx == 0;
        
        let (model, api_key, ollama_url) = if use_local {
            let model: String = Input::new()
                .with_prompt("Ollama model name")
                .default("llama3.2".to_string())
                .interact_text()?;
            
            if model.trim().to_lowercase() == "back" {
                continue;
            }
            
            let url: String = Input::new()
                .with_prompt("Ollama server URL")
                .default("http://localhost:11434".to_string())
                .interact_text()?;
                
            (model, None, url)
        } else {
            let model: String = Input::new()
                .with_prompt("HuggingFace model name")
                .default("microsoft/DialoGPT-large".to_string())
                .interact_text()?;
                
            let key: String = Input::new()
                .with_prompt("HuggingFace API key")
                .interact_text()?;
                
            (model, Some(key), "http://localhost:11434".to_string())
        };
        
        println!("\n🚀 Generating marketing content...");
        return write_marketing_content(
            marketing_type,
            product,
            audience,
            length,
            output,
            model,
            api_key,
            use_local,
            ollama_url,
        ).await;
    }
}

async fn interactive_blog_creation() -> Result<()> {
    loop {
        println!("\n📰 Creating Blog Content");
        println!("Let me help you create engaging blog content...\n");
        
        // Content type selection
        let content_types = vec![
            "Blog Post - General blog article",
            "SEO Article - Search-optimized article",
            "Tutorial - Step-by-step guide",
            "Listicle - List-based article",
            "Review - Product or service review",
            "News Article - News or announcement",
            "Opinion Piece - Editorial or opinion",
            "Interview - Interview format article",
            "Case Study - Business case study",
            "← Back to main menu",
        ];
        
        let type_idx = Select::new()
            .with_prompt("What type of blog content would you like to create?")
            .items(&content_types)
            .default(0)
            .interact()?;
        
        if type_idx == content_types.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let content_type = match type_idx {
            0 => crate::cli_types::BlogContentType::BlogPost,
            1 => crate::cli_types::BlogContentType::SeoArticle,
            2 => crate::cli_types::BlogContentType::Tutorial,
            3 => crate::cli_types::BlogContentType::Listicle,
            4 => crate::cli_types::BlogContentType::Review,
            5 => crate::cli_types::BlogContentType::NewsArticle,
            6 => crate::cli_types::BlogContentType::Opinion,
            7 => crate::cli_types::BlogContentType::Interview,
            8 => crate::cli_types::BlogContentType::CaseStudy,
            _ => crate::cli_types::BlogContentType::BlogPost,
        };
        
        // Get topic
        let topic: String = Input::new()
            .with_prompt("What is the topic or subject of your blog content? (or 'back' to return)")
            .interact_text()?;
        
        if topic.trim().to_lowercase() == "back" {
            continue;
        }
        
        // Get keywords (for SEO)
        let keywords: String = Input::new()
            .with_prompt("Target keywords for SEO (optional, comma-separated) (or 'back' to return)")
            .default("".to_string())
            .interact_text()?;
        
        if keywords.trim().to_lowercase() == "back" {
            continue;
        }
        
        let keywords_opt = if keywords.trim().is_empty() {
            None
        } else {
            Some(keywords)
        };
        
        // Get target audience
        let audience: String = Input::new()
            .with_prompt("Who is your target audience? (optional) (or 'back' to return)")
            .default("".to_string())
            .interact_text()?;
        
        if audience.trim().to_lowercase() == "back" {
            continue;
        }
        
        let audience_opt = if audience.trim().is_empty() {
            None
        } else {
            Some(audience)
        };
        
        // Length selection
        let lengths = vec![
            "Short (500-800 words)",
            "Medium (800-1500 words)",
            "Long (1500-2500 words)",
            "Very Long (2500-4000 words)",
            "Epic (4000+ words)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("What length content would you like?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue;
        }
        
        let length = Some(match length_idx {
            0 => crate::cli_types::BlogLength::Short,
            1 => crate::cli_types::BlogLength::Medium,
            2 => crate::cli_types::BlogLength::Long,
            3 => crate::cli_types::BlogLength::VeryLong,
            4 => crate::cli_types::BlogLength::Epic,
            _ => crate::cli_types::BlogLength::Medium,
        });
        
        // Get output file path
        let output_path: String = Input::new()
            .with_prompt("Output file path (optional)")
            .default("blog_content.md".to_string())
            .interact_text()?;
        
        if output_path.trim().to_lowercase() == "back" {
            continue;
        }
        
        let output = if output_path.trim().is_empty() {
            None
        } else {
            Some(PathBuf::from(output_path))
        };
        
        // Get model preferences
        let model_options = vec![
            "Use local Ollama (recommended)",
            "Use HuggingFace API",
            "← Back",
        ];
        
        let model_idx = Select::new()
            .with_prompt("Choose your AI model source:")
            .items(&model_options)
            .default(0)
            .interact()?;
        
        if model_idx == model_options.len() - 1 {
            continue;
        }
        
        let use_local = model_idx == 0;
        
        let (model, api_key, ollama_url) = if use_local {
            let model: String = Input::new()
                .with_prompt("Ollama model name")
                .default("llama3.2".to_string())
                .interact_text()?;
            
            if model.trim().to_lowercase() == "back" {
                continue;
            }
            
            let url: String = Input::new()
                .with_prompt("Ollama server URL")
                .default("http://localhost:11434".to_string())
                .interact_text()?;
                
            (model, None, url)
        } else {
            let model: String = Input::new()
                .with_prompt("HuggingFace model name")
                .default("microsoft/DialoGPT-large".to_string())
                .interact_text()?;
                
            let key: String = Input::new()
                .with_prompt("HuggingFace API key")
                .interact_text()?;
                
            (model, Some(key), "http://localhost:11434".to_string())
        };
        
        println!("\n🚀 Generating blog content...");
        return write_blog_content(
            content_type,
            topic,
            keywords_opt,
            audience_opt,
            length,
            output,
            model,
            api_key,
            use_local,
            ollama_url,
        ).await;
    }
}

async fn interactive_strategic_doc_creation() -> Result<()> {
    loop {
        println!("\n📋 Creating Strategic Planning Document");
        println!("Let me help you create a comprehensive strategic document...\n");
        
        // Document type selection
        let doc_types = vec![
            "Strategic Plan - Long-term strategic planning document",
            "Business Plan - Comprehensive business planning document",
            "Project Plan - Project planning and management document",
            "Roadmap - Product or business development roadmap",
            "Vision Document - Vision and mission statement document",
            "Governance Document - Governance and policy framework",
            "Risk Assessment - Risk analysis and mitigation planning",
            "Budget Plan - Financial planning and budget document",
            "← Back to main menu",
        ];
        
        let type_idx = Select::new()
            .with_prompt("What type of strategic document would you like to create?")
            .items(&doc_types)
            .default(0)
            .interact()?;
        
        if type_idx == doc_types.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let doc_type = match type_idx {
            0 => crate::cli_types::StrategicDocType::StrategicPlan,
            1 => crate::cli_types::StrategicDocType::BusinessPlan,
            2 => crate::cli_types::StrategicDocType::ProjectPlan,
            3 => crate::cli_types::StrategicDocType::Roadmap,
            4 => crate::cli_types::StrategicDocType::VisionDoc,
            5 => crate::cli_types::StrategicDocType::Governance,
            6 => crate::cli_types::StrategicDocType::RiskAssessment,
            7 => crate::cli_types::StrategicDocType::BudgetPlan,
            _ => crate::cli_types::StrategicDocType::StrategicPlan,
        };
        
        // Get organization name
        let organization: String = Input::new()
            .with_prompt("What is the organization or project name? (or 'back' to return)")
            .interact_text()?;
        
        if organization.trim().to_lowercase() == "back" {
            continue;
        }
        
        // Get timeframe
        let timeframe: String = Input::new()
            .with_prompt("What is the time horizon? (e.g., '1 year', '3 years', '5 years') (or 'back' to return)")
            .default("3 years".to_string())
            .interact_text()?;
        
        if timeframe.trim().to_lowercase() == "back" {
            continue;
        }
        
        // Length selection
        let lengths = vec![
            "Brief (3-8 pages)",
            "Standard (8-20 pages)",
            "Comprehensive (20-50 pages)",
            "Extensive (50+ pages)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("What length document would you like?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue;
        }
        
        let length = Some(match length_idx {
            0 => crate::cli_types::StrategicLength::Brief,
            1 => crate::cli_types::StrategicLength::Standard,
            2 => crate::cli_types::StrategicLength::Comprehensive,
            3 => crate::cli_types::StrategicLength::Extensive,
            _ => crate::cli_types::StrategicLength::Standard,
        });
        
        // Get output file path
        let output_path: String = Input::new()
            .with_prompt("Output file path (optional)")
            .default("strategic_document.md".to_string())
            .interact_text()?;
        
        if output_path.trim().to_lowercase() == "back" {
            continue;
        }
        
        let output = if output_path.trim().is_empty() {
            None
        } else {
            Some(PathBuf::from(output_path))
        };
        
        // Get model preferences
        let model_options = vec![
            "Use local Ollama (recommended)",
            "Use HuggingFace API",
            "← Back",
        ];
        
        let model_idx = Select::new()
            .with_prompt("Choose your AI model source:")
            .items(&model_options)
            .default(0)
            .interact()?;
        
        if model_idx == model_options.len() - 1 {
            continue;
        }
        
        let use_local = model_idx == 0;
        
        let (model, api_key, ollama_url) = if use_local {
            let model: String = Input::new()
                .with_prompt("Ollama model name")
                .default("llama3.2".to_string())
                .interact_text()?;
            
            if model.trim().to_lowercase() == "back" {
                continue;
            }
            
            let url: String = Input::new()
                .with_prompt("Ollama server URL")
                .default("http://localhost:11434".to_string())
                .interact_text()?;
                
            (model, None, url)
        } else {
            let model: String = Input::new()
                .with_prompt("HuggingFace model name")
                .default("microsoft/DialoGPT-large".to_string())
                .interact_text()?;
                
            let key: String = Input::new()
                .with_prompt("HuggingFace API key")
                .interact_text()?;
                
            (model, Some(key), "http://localhost:11434".to_string())
        };
        
        println!("\n🚀 Generating strategic document...");
        return write_strategic_doc(
            doc_type,
            organization,
            timeframe,
            length,
            output,
            model,
            api_key,
            use_local,
            ollama_url,
        ).await;
    }
}

async fn interactive_meeting_doc_creation() -> Result<()> {
    loop {
        println!("\n📅 Creating Meeting Documentation");
        println!("Let me help you create professional meeting documentation...\n");
        
        // Document type selection
        let doc_types = vec![
            "Meeting Notes - Detailed meeting minutes and notes",
            "Action Items - Action items and follow-up tasks",
            "Meeting Summary - Executive summary of meeting",
            "Meeting Transcript - Transcript-style documentation",
            "Decision Log - Record of decisions made",
            "Meeting Agenda - Structured meeting agenda",
            "← Back to main menu",
        ];
        
        let type_idx = Select::new()
            .with_prompt("What type of meeting document would you like to create?")
            .items(&doc_types)
            .default(0)
            .interact()?;
        
        if type_idx == doc_types.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let doc_type = match type_idx {
            0 => crate::cli_types::MeetingDocType::MeetingNotes,
            1 => crate::cli_types::MeetingDocType::ActionItems,
            2 => crate::cli_types::MeetingDocType::Summary,
            3 => crate::cli_types::MeetingDocType::Transcript,
            4 => crate::cli_types::MeetingDocType::DecisionLog,
            5 => crate::cli_types::MeetingDocType::Agenda,
            _ => crate::cli_types::MeetingDocType::MeetingNotes,
        };
        
        // Get meeting purpose
        let purpose: String = Input::new()
            .with_prompt("What is the meeting purpose or main agenda? (or 'back' to return)")
            .interact_text()?;
        
        if purpose.trim().to_lowercase() == "back" {
            continue;
        }
        
        // Get number of attendees
        let attendees_str: String = Input::new()
            .with_prompt("How many attendees? (optional) (or 'back' to return)")
            .default("".to_string())
            .interact_text()?;
        
        if attendees_str.trim().to_lowercase() == "back" {
            continue;
        }
        
        let attendees = if attendees_str.trim().is_empty() {
            None
        } else {
            attendees_str.parse::<u32>().ok()
        };
        
        // Get duration
        let duration: String = Input::new()
            .with_prompt("Meeting duration (e.g., '1 hour', '2 hours', '30 minutes') - optional (or 'back' to return)")
            .default("".to_string())
            .interact_text()?;
        
        if duration.trim().to_lowercase() == "back" {
            continue;
        }
        
        let duration_opt = if duration.trim().is_empty() {
            None
        } else {
            Some(duration)
        };
        
        // Length selection
        let lengths = vec![
            "Brief (1-2 pages)",
            "Standard (2-5 pages)",
            "Extended (5-10 pages)",
            "Comprehensive (10+ pages)",
            "Detailed (15+ pages)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("What length document would you like?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue;
        }
        
        let length = Some(match length_idx {
            0 => crate::cli_types::MeetingLength::Brief,
            1 => crate::cli_types::MeetingLength::Standard,
            2 => crate::cli_types::MeetingLength::Extended,
            3 => crate::cli_types::MeetingLength::Comprehensive,
            4 => crate::cli_types::MeetingLength::Detailed,
            _ => crate::cli_types::MeetingLength::Standard,
        });
        
        // Get output file path
        let output_path: String = Input::new()
            .with_prompt("Output file path (optional)")
            .default("meeting_documentation.md".to_string())
            .interact_text()?;
        
        if output_path.trim().to_lowercase() == "back" {
            continue;
        }
        
        let output = if output_path.trim().is_empty() {
            None
        } else {
            Some(PathBuf::from(output_path))
        };
        
        // Get model preferences
        let model_options = vec![
            "Use local Ollama (recommended)",
            "Use HuggingFace API",
            "← Back",
        ];
        
        let model_idx = Select::new()
            .with_prompt("Choose your AI model source:")
            .items(&model_options)
            .default(0)
            .interact()?;
        
        if model_idx == model_options.len() - 1 {
            continue;
        }
        
        let use_local = model_idx == 0;
        
        let (model, api_key, ollama_url) = if use_local {
            let model: String = Input::new()
                .with_prompt("Ollama model name")
                .default("llama3.2".to_string())
                .interact_text()?;
            
            if model.trim().to_lowercase() == "back" {
                continue;
            }
            
            let url: String = Input::new()
                .with_prompt("Ollama server URL")
                .default("http://localhost:11434".to_string())
                .interact_text()?;
                
            (model, None, url)
        } else {
            let model: String = Input::new()
                .with_prompt("HuggingFace model name")
                .default("microsoft/DialoGPT-large".to_string())
                .interact_text()?;
                
            let key: String = Input::new()
                .with_prompt("HuggingFace API key")
                .interact_text()?;
                
            (model, Some(key), "http://localhost:11434".to_string())
        };
        
        println!("\n🚀 Generating meeting documentation...");
        return write_meeting_doc(
            doc_type,
            purpose,
            attendees,
            duration_opt,
            length,
            output,
            model,
            api_key,
            use_local,
            ollama_url,
        ).await;
    }
}

// Enhanced poetry generation function with improved rhyming and emotional anchors
async fn write_enhanced_poem(
    client: &AIClient,
    model: &str,
    content: &mut Content,
    poem_number: usize,
    style: &PoetryStyle,
    theme: &str,
    progress_bar: &ProgressBar,
) -> Result<()> {
    // Extract poem title from outline
    let poem_title = extract_poem_title_from_outline(&content.outline, poem_number)
        .unwrap_or_else(|| format!("Untitled Poem {}", poem_number));
    
    progress_bar.set_message(format!("Writing poem: {}", poem_title));
    
    // Create emotional anchor based on theme and specific poem
    let emotion = create_emotion_from_theme(theme);
    let poetry_style_str = poetry_style_name(style);
    
    // Create enhanced poetry prompt with emotional anchors and rhyme hints
    let enhanced_prompt = EnhancedPoetryPrompt::new(theme, &emotion, poetry_style_str);
    let prompt = enhanced_prompt.create_enhanced_prompt(&poem_title);
    
    // Generate the poem with enhanced prompting
    let generated_text = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&prompt, 800, 0.9).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &prompt, 800, 0.9).await?
        }
    };
    
    // Post-process the generated poem for better coherence and rhyme alignment
    let processed_poem = post_process_poetry(&generated_text, &emotion);
    
    // Create section for the poem
    let section = Section {
        number: poem_number,
        title: poem_title.clone(),
        content: processed_poem.clone(),
        word_count: count_words(&processed_poem),
        outline: format!("Poem: {}", poem_title),
        section_type: SectionType::Section,
        created_at: chrono::Utc::now(),
        completed: true,
        plot_thread: None,
        narrative_context: None,
    };
    
    content.sections.push(section);
    content.metadata.current_word_count += count_words(&processed_poem);
    
    println!("  ✅ Poem {} completed: {} words", poem_number, count_words(&processed_poem));
    
    Ok(())
}

fn extract_poem_title_from_outline(outline: &str, poem_number: usize) -> Option<String> {
    // Look for poem titles in the outline
    for line in outline.lines() {
        let line = line.trim();
        if line.starts_with(&format!("Poem {}:", poem_number)) {
            // Extract title after "Poem N: "
            if let Some(title_part) = line.split(':').nth(1) {
                if let Some(title) = title_part.split('-').next() {
                    return Some(title.trim().to_string());
                }
            }
        }
    }
    
    // Fallback patterns
    for line in outline.lines() {
        let line = line.trim();
        if line.starts_with(&format!("{}.", poem_number)) ||
           line.starts_with(&format!("{})", poem_number)) {
            if let Some(title_part) = line.split_once(' ') {
                if let Some(title) = title_part.1.split('-').next() {
                    return Some(title.trim().to_string());
                }
            }
        }
    }
    
    None
}

// Interactive mode for continuing existing work
async fn interactive_continuation_mode() -> Result<()> {
    println!("\n📂 Continue Existing Work Mode");
    println!("═══════════════════════════════════");
    println!();
    
    // Set up the continuation project with file selection
    let project = match interactive_continuation_setup().await {
        Ok(project) => project,
        Err(_) => {
            println!("📂 Returning to main menu...");
            return Ok(());
        }
    };
    
    println!("\n✅ Project setup complete!");
    println!("Total files: {}", project.files.len());
    println!("Total word count: {}", project.total_word_count());
    
    if let Some(ref project_type) = project.project_type {
        println!("Detected content type: {:?}", project_type);
    }
    
    // Get basic information for continuation
    let title: String = Input::new()
        .with_prompt("Title for this continuation (or existing title)")
        .default("Continued Work".to_string())
        .interact_text()?;
    
    let author: String = Input::new()
        .with_prompt("Author name")
        .default("Anonymous".to_string())
        .interact_text()?;
    
    // Convert the project to a Content object
    let content = continuation_project_to_content(&project, title.clone(), author.clone())?;
    
    // Get the combined existing content for context
    let existing_content = project.get_combined_content()?;
    println!("\n📖 Loaded {} characters of existing content for context", existing_content.len());
    
    // Get continuation parameters
    let continuation_prompt: String = Input::new()
        .with_prompt("What would you like to add/continue? (describe the new content you want)")
        .interact_text()?;
    
    let target_words: String = Input::new()
        .with_prompt("How many words would you like to add?")
        .default("1000".to_string())
        .interact_text()?;
    
    let target_words_num: usize = target_words.parse().unwrap_or(1000);
    
    // Model selection
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama instead of HuggingFace API?")
        .default(true)
        .interact()?;
    
    let (model, api_key, ollama_url) = if use_local {
        let model: String = Input::new()
            .with_prompt("Ollama model name")
            .default("llama3.2".to_string())
            .interact_text()?;
        
        let url: String = Input::new()
            .with_prompt("Ollama server URL")
            .default("http://localhost:11434".to_string())
            .interact_text()?;
            
        (model, None, url)
    } else {
        let model: String = Input::new()
            .with_prompt("HuggingFace model name")
            .default("microsoft/DialoGPT-large".to_string())
            .interact_text()?;
            
        let key: String = Input::new()
            .with_prompt("HuggingFace API key")
            .interact_text()?;
            
        (model, Some(key), "http://localhost:11434".to_string())
    };
    
    // Generate continuation
    println!("\n🚀 Generating continuation...");
    
    let continuation_result = generate_continuation(
        &existing_content,
        &continuation_prompt,
        target_words_num,
        &content.content_type,
        &model,
        api_key,
        use_local,
        &ollama_url,
    ).await?;
    
    // Save the continuation
    let output_filename = format!("{}_continuation.md", title.replace(" ", "_").to_lowercase());
    let output_path = std::path::PathBuf::from(&output_filename);
    
    println!("\n💾 Saving continuation to: {}", output_path.display());
    
    let full_content = format!(
        "# {} - Continuation\n\nAuthor: {}\nGenerated: {}\n\n## Original Content Context\n\n{}\n\n## New Content\n\n{}\n",
        title,
        author,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        {
            let context_limit = 500.min(existing_content.chars().count());
            let context_preview: String = existing_content.chars().take(context_limit).collect();
            context_preview
        },
        continuation_result
    );
    
    std::fs::write(&output_path, full_content)?;
    
    println!("✅ Continuation saved successfully!");
    println!("📝 Added {} words to your existing work", target_words_num);
    println!("📁 File: {}", output_path.display());
    
    Ok(())
}

// Generate continuation content
async fn generate_continuation(
    existing_content: &str,
    continuation_prompt: &str,
    target_words: usize,
    content_type: &ContentType,
    model: &str,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: &str,
) -> Result<String> {
    // Create AI client
    let client = if use_local {
        let ollama_client = OllamaClient::new(ollama_url.to_string())?;
        AIClient::Ollama(ollama_client)
    } else {
        let hf_client = HuggingFaceClient::new(model.to_string(), api_key.clone())?;
        AIClient::HuggingFace(hf_client)
    };
    
    // Create context-aware prompt for continuation
    let context_length = 2000; // Use last 2000 characters for context
    let context = if existing_content.len() > context_length {
        &existing_content[existing_content.len() - context_length..]
    } else {
        existing_content
    };
    
    let content_type_guidance = match content_type {
        ContentType::Book => "Continue the narrative naturally, maintaining character voice and story progression.",
        ContentType::Screenplay => "Continue with proper screenplay format, including scene headings and character dialogue.",
        ContentType::Play => "Continue with stage directions and character dialogue appropriate for theater.",
        ContentType::Poetry => "Continue with the established poetic style and meter.",
        ContentType::TechnicalDoc => "Continue with technical accuracy and clear documentation style.",
        ContentType::BlogPost => "Continue with engaging, readable content appropriate for blog readers.",
        ContentType::ResearchReport => "Continue with academic rigor and proper research documentation.",
        _ => "Continue naturally while maintaining the established style and tone.",
    };
    
    let full_prompt = format!(
        "You are continuing an existing piece of writing. Here is the context from the existing content:\n\n--- EXISTING CONTENT (last part) ---\n{}\n--- END EXISTING CONTENT ---\n\nContinuation Request: {}\n\nGuidance: {}\n\nPlease write approximately {} words that continue naturally from the existing content. Maintain consistency in style, tone, and any established characters or themes. Do not repeat what has already been written.\n\nContinuation:",
        context,
        continuation_prompt,
        content_type_guidance,
        target_words
    );
    
    // Generate the continuation
    let result = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&full_prompt, (target_words * 2) as u32, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &full_prompt, (target_words * 2) as i32, 0.8).await?
        }
    };
    
    // Clean up the result
    let cleaned_result = result
        .trim()
        .lines()
        .filter(|line| {
            !line.trim().is_empty() && 
            !line.to_lowercase().contains("continuation:") &&
            !line.to_lowercase().contains("here is") &&
            !line.to_lowercase().contains("i'll continue")
        })
        .collect::<Vec<&str>>()
        .join("\n");
    
    Ok(cleaned_result)
}

// Dictionary creation with etymological features
async fn interactive_dictionary_creation() -> Result<()> {
    loop {
        println!("\n📖 Creating a Dictionary/Lexicon");
        println!("Let me help you create a comprehensive dictionary with etymological features...\n");
        
        // Dictionary type selection
        let dict_types = vec![
            "Standard Dictionary - Traditional word definitions",
            "Etymological Dictionary - Word origins and evolution", 
            "Thematic Dictionary - Words grouped by themes",
            "Bilingual Dictionary - Two-language dictionary",
            "Technical Dictionary - Specialized terminology",
            "Slang Dictionary - Informal language and expressions",
            "Historical Dictionary - Historical word usage",
            "Creative Dictionary - Imaginative/fictional lexicon",
            "← Back to main menu",
        ];
        
        let dict_type_idx = Select::new()
            .with_prompt("What type of dictionary would you like to create?")
            .items(&dict_types)
            .default(0)
            .interact()?;
        
        if dict_type_idx == dict_types.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let dict_type = match dict_type_idx {
            0 => crate::cli_types::DictionaryType::Standard,
            1 => crate::cli_types::DictionaryType::Etymological,
            2 => crate::cli_types::DictionaryType::Thematic,
            3 => crate::cli_types::DictionaryType::Bilingual,
            4 => crate::cli_types::DictionaryType::Technical,
            5 => crate::cli_types::DictionaryType::Slang,
            6 => crate::cli_types::DictionaryType::Historical,
            7 => crate::cli_types::DictionaryType::Creative,
            _ => crate::cli_types::DictionaryType::Standard,
        };
        
        // Get topic/theme for the dictionary
        let topic_result: Result<String, _> = Input::new()
            .with_prompt("What topic, theme, or subject area should this dictionary focus on? (or 'back' to return)")
            .interact_text();
        
        let topic = match topic_result {
            Ok(t) if t.trim().to_lowercase() == "back" => continue,
            Ok(t) => t,
            Err(_) => continue,
        };
        
        // Length selection
        let lengths = vec![
            "Pocket Dictionary (500-2,000 entries)",
            "Standard Dictionary (2,000-10,000 entries)",
            "Comprehensive Dictionary (10,000-50,000 entries)",
            "Unabridged Dictionary (50,000+ entries)",
            "Unlimited - Let creativity flow",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("How extensive should your dictionary be?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue; // Back to dictionary type selection
        }
        
        let length = match length_idx {
            0 => crate::cli_types::DictionaryLength::Pocket,
            1 => crate::cli_types::DictionaryLength::Standard,
            2 => crate::cli_types::DictionaryLength::Comprehensive,
            3 => crate::cli_types::DictionaryLength::Unabridged,
            4 => crate::cli_types::DictionaryLength::Unlimited,
            _ => crate::cli_types::DictionaryLength::Standard,
        };
        
        // Model selection with proper recommendations
        let length_desc = match length {
            crate::cli_types::DictionaryLength::Pocket => "Short",
            crate::cli_types::DictionaryLength::Standard => "Medium",
            crate::cli_types::DictionaryLength::Comprehensive => "Large",
            crate::cli_types::DictionaryLength::Unabridged => "Large", 
            crate::cli_types::DictionaryLength::Unlimited => "Extended",
        };
        
        let (use_local, model) = match interactive_model_selection("dictionary", length_desc) {
            Ok((use_local, model)) => (use_local, model),
            Err(_) => continue, // Back was selected
        };
        
        // Create the dictionary
        return write_dictionary(dict_type, topic, length, None, model, None, use_local, "http://localhost:11434".to_string()).await;
    }
}

// Educational lesson creation with language learning support
async fn interactive_educational_lesson_creation() -> Result<()> {
    loop {
        println!("\n🎓 Creating Educational Lesson");
        println!("Let me help you create engaging educational content with learning support...\n");
        
        // Educational type selection
        let edu_types = vec![
            "Language Learning - Teaching a new language",
            "Translation Lesson - Between two languages",
            "Grammar Lesson - Grammar rules and examples",
            "Vocabulary Building - Word learning and usage",
            "Literature Analysis - Literary texts and interpretation",
            "History Lesson - Historical events and context",
            "Science Concepts - Scientific principles and examples",
            "Mathematics - Math concepts and problem solving",
            "Creative Arts - Art, music, and creative expression",
            "General Education - Broad educational topics",
            "← Back to main menu",
        ];
        
        let edu_type_idx = Select::new()
            .with_prompt("What type of educational content would you like to create?")
            .items(&edu_types)
            .default(0)
            .interact()?;
        
        if edu_type_idx == edu_types.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let edu_type = match edu_type_idx {
            0 => crate::cli_types::EducationalType::LanguageLearning,
            1 => crate::cli_types::EducationalType::Translation,
            2 => crate::cli_types::EducationalType::Grammar,
            3 => crate::cli_types::EducationalType::Vocabulary,
            4 => crate::cli_types::EducationalType::Literature,
            5 => crate::cli_types::EducationalType::History,
            6 => crate::cli_types::EducationalType::Science,
            7 => crate::cli_types::EducationalType::Mathematics,
            8 => crate::cli_types::EducationalType::Arts,
            9 => crate::cli_types::EducationalType::General,
            _ => crate::cli_types::EducationalType::General,
        };
        
        // Get subject/topic
        let topic_result: Result<String, _> = Input::new()
            .with_prompt("What specific topic or subject should this lesson cover? (or 'back' to return)")
            .interact_text();
        
        let topic = match topic_result {
            Ok(t) if t.trim().to_lowercase() == "back" => continue,
            Ok(t) => t,
            Err(_) => continue,
        };
        
        // Audience selection
        let audiences = vec![
            "Preschool (Ages 3-5)",
            "Elementary (Ages 6-11)",
            "Middle School (Ages 12-14)",
            "High School (Ages 15-18)",
            "College (Ages 18+)",
            "Adult Learners",
            "Professional Development",
            "Senior Learners",
            "All Audiences",
            "← Back",
        ];
        
        let audience_idx = Select::new()
            .with_prompt("Who is your target audience?")
            .items(&audiences)
            .default(4)
            .interact()?;
        
        if audience_idx == audiences.len() - 1 {
            continue; // Back to educational type selection
        }
        
        let audience = match audience_idx {
            0 => crate::cli_types::EducationalAudience::Preschool,
            1 => crate::cli_types::EducationalAudience::Elementary,
            2 => crate::cli_types::EducationalAudience::MiddleSchool,
            3 => crate::cli_types::EducationalAudience::HighSchool,
            4 => crate::cli_types::EducationalAudience::College,
            5 => crate::cli_types::EducationalAudience::Adult,
            6 => crate::cli_types::EducationalAudience::Professional,
            7 => crate::cli_types::EducationalAudience::Senior,
            8 => crate::cli_types::EducationalAudience::All,
            _ => crate::cli_types::EducationalAudience::All,
        };
        
        // Length selection
        let lengths = vec![
            "Quick Lesson (5-15 minutes)",
            "Standard Lesson (15-45 minutes)",
            "Extended Lesson (45-90 minutes)",
            "Course Module (Multiple sessions)",
            "Full Course (Complete curriculum)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("How long should this educational content be?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue; // Back to audience selection
        }
        
        let length = match length_idx {
            0 => crate::cli_types::EducationalLength::QuickLesson,
            1 => crate::cli_types::EducationalLength::StandardLesson,
            2 => crate::cli_types::EducationalLength::ExtendedLesson,
            3 => crate::cli_types::EducationalLength::CourseModule,
            4 => crate::cli_types::EducationalLength::FullCourse,
            _ => crate::cli_types::EducationalLength::StandardLesson,
        };
        
        // Model selection with proper recommendations
        let length_desc = match length {
            crate::cli_types::EducationalLength::QuickLesson => "Short",
            crate::cli_types::EducationalLength::StandardLesson => "Medium",
            crate::cli_types::EducationalLength::ExtendedLesson => "Extended",
            crate::cli_types::EducationalLength::CourseModule => "Large",
            crate::cli_types::EducationalLength::FullCourse => "Large",
        };
        
        let (use_local, model) = match interactive_model_selection("educational lesson", length_desc) {
            Ok((use_local, model)) => (use_local, model),
            Err(_) => continue, // Back was selected
        };
        
        // Create the educational lesson
        return write_educational_lesson(edu_type, topic, audience, length, None, model, None, use_local, "http://localhost:11434".to_string()).await;
    }
}

// Educational textbook creation for academic institutions
async fn interactive_educational_textbook_creation() -> Result<()> {
    loop {
        println!("\n📖 Creating Educational Textbook");
        println!("Let me help you create comprehensive academic textbooks for educational institutions...\n");
        
        // Education level selection
        let education_levels = vec![
            "Elementary School (Grades K-5) - Ages 5-11",
            "Middle School (Grades 6-8) - Ages 11-14",
            "High School (Grades 9-12) - Ages 14-18",
            "Community College - Ages 18+",
            "University Undergraduate - Ages 18-22",
            "Graduate/Masters Level - Ages 22+",
            "Professional/Doctoral Level - Advanced",
            "Adult Education - Continuing education",
            "← Back",
        ];
        
        let level_idx = Select::new()
            .with_prompt("What educational level is this textbook for?")
            .items(&education_levels)
            .default(2)
            .interact()?;
        
        if level_idx == education_levels.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        // Subject selection
        let subjects = vec![
            "Mathematics - Algebra, geometry, calculus, statistics",
            "Science - Physics, chemistry, biology, earth science", 
            "English/Language Arts - Literature, writing, grammar",
            "Social Studies - History, geography, civics, economics",
            "Foreign Languages - Spanish, French, German, etc.",
            "Computer Science - Programming, algorithms, data structures",
            "Engineering - Mechanical, electrical, civil, software",
            "Business - Management, marketing, finance, accounting",
            "Health Sciences - Medicine, nursing, public health",
            "Arts - Visual arts, music, theater, art history",
            "Philosophy - Logic, ethics, metaphysics, epistemology",
            "Psychology - Cognitive, behavioral, developmental",
            "Other/Custom Subject",
            "← Back",
        ];
        
        let subject_idx = Select::new()
            .with_prompt("What subject area is this textbook for?")
            .items(&subjects)
            .default(0)
            .interact()?;
        
        if subject_idx == subjects.len() - 1 {
            continue; // Back to level selection
        }
        
        let subject = if subject_idx == subjects.len() - 2 {
            // Custom subject
            Input::new()
                .with_prompt("Enter the custom subject area")
                .interact_text()?
        } else {
            subjects[subject_idx].split(" - ").next().unwrap_or(&subjects[subject_idx]).to_string()
        };
        
        // Specific topic/course
        let topic: String = Input::new()
            .with_prompt("What specific topic or course is this textbook for? (e.g., 'Algebra II', 'American History', 'Introduction to Psychology')")
            .interact_text()?;
        
        // Textbook scope/length
        let scopes = vec![
            "Single Chapter - 1 chapter (10-30 pages)",
            "Short Course - 3-5 chapters (50-150 pages)",
            "Semester Course - 8-12 chapters (200-400 pages)",
            "Full Year Course - 15-20 chapters (400-600 pages)",
            "Comprehensive Reference - 20+ chapters (600+ pages)",
            "← Back",
        ];
        
        let scope_idx = Select::new()
            .with_prompt("What scope should this textbook have?")
            .items(&scopes)
            .default(2)
            .interact()?;
        
        if scope_idx == scopes.len() - 1 {
            continue; // Back to subject selection
        }
        
        // Teaching approach
        let approaches = vec![
            "Traditional - Lectures, examples, exercises",
            "Interactive - Activities, discussions, case studies",
            "Problem-Based - Real-world problems and solutions",
            "Inquiry-Based - Questions leading to discovery",
            "Hands-On - Labs, experiments, practical work",
            "Mixed Approach - Combination of methods",
            "← Back",
        ];
        
        let approach_idx = Select::new()
            .with_prompt("What teaching approach should the textbook use?")
            .items(&approaches)
            .default(0)
            .interact()?;
        
        if approach_idx == approaches.len() - 1 {
            continue; // Back to scope selection
        }
        
        // Learning objectives
        let objectives: String = Input::new()
            .with_prompt("What are the main learning objectives? (e.g., 'Students will understand algebraic equations and be able to solve linear problems')")
            .interact_text()?;
        
        // Prerequisites
        let prerequisites: String = Input::new()
            .with_prompt("What prerequisites should students have? (e.g., 'Basic arithmetic and introduction to variables')")
            .default("None".to_string())
            .interact_text()?;
        
        // Model selection
        let model_options = vec![
            "Use local Ollama (recommended)",
            "Use HuggingFace API",
            "← Back",
        ];
        
        let model_idx = Select::new()
            .with_prompt("Choose your AI model source:")
            .items(&model_options)
            .default(0)
            .interact()?;
        
        if model_idx == model_options.len() - 1 {
            continue;
        }
        
        let use_local = model_idx == 0;
        let model = if use_local {
            "llama2:latest".to_string()
        } else {
            "microsoft/DialoGPT-medium".to_string()
        };
        
        // Create comprehensive prompt for educational textbook
        let education_level = education_levels[level_idx].split(" - ").next().unwrap_or(&education_levels[level_idx]);
        let scope_description = scopes[scope_idx].split(" - ").next().unwrap_or(&scopes[scope_idx]);
        let approach = approaches[approach_idx].split(" - ").next().unwrap_or(&approaches[approach_idx]);
        
        let enhanced_prompt = format!(
            "Create a comprehensive educational textbook for {} students studying {}: {}.
            
Education Level: {}
Scope: {}
Teaching Approach: {}
Learning Objectives: {}
Prerequisites: {}

The textbook should include:
- Clear chapter organization with learning objectives
- Age-appropriate explanations and examples
- Practice problems and exercises
- Key terms and definitions
- Summary sections for review
- Assessment questions
- Real-world applications
- Visual aids descriptions where appropriate

Structure the content to build knowledge progressively and engage {} students effectively.",
            education_level, subject, topic, education_level, scope_description, approach, 
            objectives, prerequisites, education_level.to_lowercase()
        );
        
        println!("\n🎯 Creating {} textbook: '{}'", education_level, topic);
        println!("📚 Subject: {}", subject);
        println!("📖 Scope: {}", scope_description);
        println!("🎓 Approach: {}", approach);
        println!("🎯 Objectives: {}", objectives);
        if prerequisites != "None" {
            println!("📋 Prerequisites: {}", prerequisites);
        }
        println!("🤖 Using: {}", if use_local { "Local Ollama" } else { "HuggingFace API" });
        
        // Create client and generate content
        let client = create_ai_client(use_local, model.clone(), None, None)?;
        
        // Generate the textbook content
        let book_content = generate_educational_textbook_content(&client, &enhanced_prompt, &topic).await?;
        
        // Save the textbook
        let filename = format!("textbook_{}_{}.txt", 
            topic.replace(" ", "_").replace("/", "_").to_lowercase(),
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        
        std::fs::write(&filename, book_content.as_bytes())?;
        
        println!("\n✅ Educational textbook created successfully!");
        println!("📄 Saved as: {}", filename);
        
        return Ok(());
    }
}

// Children's book creation with age-appropriate content
async fn interactive_childrens_book_creation() -> Result<()> {
    loop {
        println!("\n👶 Creating Children's Book");
        println!("Let me help you create delightful, age-appropriate children's content...\n");
        
        // Children's book type selection
        let book_types = vec![
            "Picture Book - Ages 2-8, with illustration descriptions",
            "Early Reader - Ages 4-8, simple text and words",
            "Chapter Book - Ages 6-10, short chapters",
            "Middle Grade - Ages 8-12, longer adventure stories",
            "Young Adult - Ages 12+, mature themes and complex plots",
            "Educational Book - Learning-focused content",
            "Bedtime Stories - Calming and gentle stories",
            "Adventure Book - Action and exploration",
            "Fantasy Book - Magical and imaginative worlds",
            "Realistic Fiction - Real-world situations and growth",
            "← Back to main menu",
        ];
        
        let book_type_idx = Select::new()
            .with_prompt("What type of children's book would you like to create?")
            .items(&book_types)
            .default(0)
            .interact()?;
        
        if book_type_idx == book_types.len() - 1 {
            return Err(BackToMenu.into()); // Back to main menu
        }
        
        let book_type = match book_type_idx {
            0 => crate::cli_types::ChildrensBookType::PictureBook,
            1 => crate::cli_types::ChildrensBookType::EarlyReader,
            2 => crate::cli_types::ChildrensBookType::ChapterBook,
            3 => crate::cli_types::ChildrensBookType::MiddleGrade,
            4 => crate::cli_types::ChildrensBookType::YoungAdult,
            5 => crate::cli_types::ChildrensBookType::Educational,
            6 => crate::cli_types::ChildrensBookType::Bedtime,
            7 => crate::cli_types::ChildrensBookType::Adventure,
            8 => crate::cli_types::ChildrensBookType::Fantasy,
            9 => crate::cli_types::ChildrensBookType::Realistic,
            _ => crate::cli_types::ChildrensBookType::PictureBook,
        };
        
        // Get story concept
        let concept_result: Result<String, _> = Input::new()
            .with_prompt("What's your story concept or theme? (e.g., 'friendship', 'brave mouse', 'learning colors') - can be just one word! (or 'back' to return)")
            .interact_text();
        
        let concept = match concept_result {
            Ok(t) if t.trim().to_lowercase() == "back" => continue,
            Ok(t) => t,
            Err(_) => continue,
        };
        
        // Age group selection
        let age_groups = vec![
            "Toddler (Ages 1-3)",
            "Preschool (Ages 3-5)",
            "Kindergarten (Ages 5-6)",
            "Early Elementary (Ages 6-8)",
            "Elementary (Ages 8-11)",
            "Middle Grade (Ages 11-14)",
            "Young Adult (Ages 14+)",
            "← Back",
        ];
        
        let age_idx = Select::new()
            .with_prompt("What age group is this book for?")
            .items(&age_groups)
            .default(2)
            .interact()?;
        
        if age_idx == age_groups.len() - 1 {
            continue; // Back to book type selection
        }
        
        let age_group = match age_idx {
            0 => crate::cli_types::ChildrensAgeGroup::Toddler,
            1 => crate::cli_types::ChildrensAgeGroup::Preschool,
            2 => crate::cli_types::ChildrensAgeGroup::Kindergarten,
            3 => crate::cli_types::ChildrensAgeGroup::EarlyElementary,
            4 => crate::cli_types::ChildrensAgeGroup::Elementary,
            5 => crate::cli_types::ChildrensAgeGroup::MiddleGrade,
            6 => crate::cli_types::ChildrensAgeGroup::YoungAdult,
            _ => crate::cli_types::ChildrensAgeGroup::Kindergarten,
        };
        
        // Length selection
        let lengths = vec![
            "Board Book (10-100 words, for toddlers)",
            "Picture Book (100-1,000 words)",
            "Early Reader (1,000-2,500 words)",
            "Chapter Book (2,500-10,000 words)",
            "Middle Grade (10,000-40,000 words)",
            "Young Adult (40,000-80,000 words)",
            "← Back",
        ];
        
        let length_idx = Select::new()
            .with_prompt("How long should this children's book be?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue; // Back to age group selection
        }
        
        let length = match length_idx {
            0 => crate::cli_types::ChildrensBookLength::Board,
            1 => crate::cli_types::ChildrensBookLength::Picture,
            2 => crate::cli_types::ChildrensBookLength::Early,
            3 => crate::cli_types::ChildrensBookLength::Chapter,
            4 => crate::cli_types::ChildrensBookLength::Middle,
            5 => crate::cli_types::ChildrensBookLength::Young,
            _ => crate::cli_types::ChildrensBookLength::Picture,
        };
        
        // Model selection with proper recommendations
        let length_desc = match length {
            crate::cli_types::ChildrensBookLength::Board => "Short",
            crate::cli_types::ChildrensBookLength::Picture => "Short",
            crate::cli_types::ChildrensBookLength::Early => "Medium",
            crate::cli_types::ChildrensBookLength::Chapter => "Medium",
            crate::cli_types::ChildrensBookLength::Middle => "Large",
            crate::cli_types::ChildrensBookLength::Young => "Large",
        };
        
        let (use_local, model) = match interactive_model_selection("children's book", length_desc) {
            Ok((use_local, model)) => (use_local, model),
            Err(_) => continue, // Back was selected
        };
        
        // Create the children's book
        return write_childrens_book(book_type, concept, age_group, length, None, model, None, use_local, "http://localhost:11434".to_string()).await;
    }
}

// Placeholder writing functions - these will need to be implemented
async fn write_dictionary(
    dict_type: crate::cli_types::DictionaryType,
    topic: String,
    length: crate::cli_types::DictionaryLength,
    output: Option<String>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("📖 Pundit - Dictionary Creator").bold().cyan());
    println!("═══════════════════════════════════════");
    println!();
    
    let dict_type_name = match dict_type {
        crate::cli_types::DictionaryType::Standard => "Standard Dictionary",
        crate::cli_types::DictionaryType::Etymological => "Etymological Dictionary",
        crate::cli_types::DictionaryType::Thematic => "Thematic Dictionary",
        crate::cli_types::DictionaryType::Bilingual => "Bilingual Dictionary",
        crate::cli_types::DictionaryType::Technical => "Technical Dictionary",
        crate::cli_types::DictionaryType::Slang => "Slang Dictionary",
        crate::cli_types::DictionaryType::Historical => "Historical Dictionary",
        crate::cli_types::DictionaryType::Creative => "Creative Dictionary",
    };
    
    let target_entries = match length {
        crate::cli_types::DictionaryLength::Pocket => 1000,
        crate::cli_types::DictionaryLength::Standard => 5000,
        crate::cli_types::DictionaryLength::Comprehensive => 25000,
        crate::cli_types::DictionaryLength::Unabridged => 75000,
        crate::cli_types::DictionaryLength::Unlimited => 10000, // Default reasonable size
    };
    
    println!("📝 Creating {} focused on: {}", dict_type_name, topic);
    println!("🎯 Target: {} entries", target_entries);
    println!();
    
    // Create the content
    let mut content = Content::new_document(
        format!("{} - {}", dict_type_name, topic),
        "Pundit AI".to_string(),
        "Lexicographical".to_string(),
        format!("A comprehensive {} covering {} terminology and definitions.", dict_type_name.to_lowercase(), topic),
        (target_entries / 100).max(10), // Estimate pages based on entries
        crate::content::DocumentFormat::Educational,
        model.clone(),
    );
    
    content.content_type = ContentType::Dictionary;
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        let ollama_client = OllamaClient::new(ollama_url)?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    // Generate entries in sections (100 entries per section)
    let entries_per_section = 100;
    let total_sections = (target_entries / entries_per_section).max(1);
    
    println!("📊 Generating {} dictionary entries in {} sections...", target_entries, total_sections);
    
    let progress_bar = ProgressBar::new(total_sections as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} sections ({percent}%)")
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("#>-"));
    
    for section_num in 1..=total_sections {
        let start_letter = ((section_num - 1) * 26 / total_sections) as u8 + b'A';
        let end_letter = (section_num * 26 / total_sections).min(26) as u8 + b'A' - 1;
        let letter_range = format!("{}-{}", start_letter as char, end_letter as char);
        
        progress_bar.set_message(format!("Creating entries {}", letter_range));
        
        if let Err(e) = write_dictionary_section(&client, &model, &mut content, section_num, &topic, &dict_type, entries_per_section, &letter_range, &progress_bar).await {
            println!("\n❌ Error writing section {}: {}", section_num, e);
            break;
        }
        
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("Dictionary creation complete!");
    
    // Save the content
    let filename = output.unwrap_or_else(|| {
        format!("{}_dictionary_{}.txt", 
            topic.replace(" ", "_").to_lowercase(),
            chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    });
    
    let content_text = content.to_text();
    fs::write(&filename, content_text)?;
    
    println!("\n✅ Dictionary saved to: {}", filename);
    println!("📊 Total entries: {}", content.sections.len() * entries_per_section);
    println!("📄 Total words: {}", content.metadata.current_word_count);
    
    Ok(())
}

async fn write_educational_lesson(
    edu_type: crate::cli_types::EducationalType,
    topic: String,
    audience: crate::cli_types::EducationalAudience,
    length: crate::cli_types::EducationalLength,
    output: Option<String>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("🎓 Pundit - Educational Content Creator").bold().green());
    println!("═══════════════════════════════════════");
    println!();
    
    let edu_type_name = match edu_type {
        crate::cli_types::EducationalType::LanguageLearning => "Language Learning",
        crate::cli_types::EducationalType::Translation => "Translation Lesson",
        crate::cli_types::EducationalType::Grammar => "Grammar Lesson",
        crate::cli_types::EducationalType::Vocabulary => "Vocabulary Building",
        crate::cli_types::EducationalType::Literature => "Literature Analysis",
        crate::cli_types::EducationalType::History => "History Lesson",
        crate::cli_types::EducationalType::Science => "Science Concepts",
        crate::cli_types::EducationalType::Mathematics => "Mathematics",
        crate::cli_types::EducationalType::Arts => "Creative Arts",
        crate::cli_types::EducationalType::General => "General Education",
    };
    
    let audience_name = match audience {
        crate::cli_types::EducationalAudience::Preschool => "Preschoolers (Ages 3-5)",
        crate::cli_types::EducationalAudience::Elementary => "Elementary Students (Ages 6-11)",
        crate::cli_types::EducationalAudience::MiddleSchool => "Middle School Students (Ages 12-14)",
        crate::cli_types::EducationalAudience::HighSchool => "High School Students (Ages 15-18)",
        crate::cli_types::EducationalAudience::College => "College Students (18+)",
        crate::cli_types::EducationalAudience::Adult => "Adult Learners",
        crate::cli_types::EducationalAudience::Professional => "Professional Development",
        crate::cli_types::EducationalAudience::Senior => "Senior Learners (65+)",
        crate::cli_types::EducationalAudience::All => "All Audiences",
    };
    
    let target_sections = match length {
        crate::cli_types::EducationalLength::QuickLesson => 3,      // 5-15 min lesson
        crate::cli_types::EducationalLength::StandardLesson => 5,  // 15-45 min lesson
        crate::cli_types::EducationalLength::ExtendedLesson => 8,  // 45-90 min lesson
        crate::cli_types::EducationalLength::CourseModule => 12,   // Multi-session course
        crate::cli_types::EducationalLength::FullCourse => 20, // Full curriculum
    };
    
    println!("📚 Creating {} lesson on: {}", edu_type_name, topic);
    println!("👥 Target audience: {}", audience_name);
    println!("📖 Lesson sections: {}", target_sections);
    println!();
    
    // Create the content
    let mut content = Content::new_document(
        format!("{} - {}", edu_type_name, topic),
        "Pundit AI".to_string(),
        "Educational".to_string(),
        format!("A comprehensive {} lesson covering {} designed for {}.", edu_type_name.to_lowercase(), topic, audience_name.to_lowercase()),
        target_sections.max(3),
        crate::content::DocumentFormat::Educational,
        model.clone(),
    );
    
    content.content_type = ContentType::EducationalLesson;
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        let ollama_client = OllamaClient::new(ollama_url)?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    println!("🏗️ Generating educational content...");
    
    let progress_bar = ProgressBar::new(target_sections as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} sections ({percent}%)")
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("#>-"));
    
    for section_num in 1..=target_sections {
        progress_bar.set_message(format!("Creating lesson section {}", section_num));
        
        if let Err(e) = write_educational_section(&client, &model, &mut content, section_num, &topic, &edu_type, &audience, &progress_bar).await {
            println!("\n❌ Error writing section {}: {}", section_num, e);
            break;
        }
        
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("Educational content complete!");
    
    // Save the content
    let filename = output.unwrap_or_else(|| {
        format!("{}_lesson_{}.txt", 
            topic.replace(" ", "_").to_lowercase(),
            chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    });
    
    let content_text = content.to_text();
    fs::write(&filename, content_text)?;
    
    println!("\n✅ Educational lesson saved to: {}", filename);
    println!("📚 Total sections: {}", content.sections.len());
    println!("📄 Total words: {}", content.metadata.current_word_count);
    
    Ok(())
}

async fn write_childrens_book(
    book_type: crate::cli_types::ChildrensBookType,
    concept: String,
    age_group: crate::cli_types::ChildrensAgeGroup,
    length: crate::cli_types::ChildrensBookLength,
    output: Option<String>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("👶 Pundit - Children's Book Creator").bold().yellow());
    println!("═══════════════════════════════════════");
    println!();
    
    let book_type_name = match book_type {
        crate::cli_types::ChildrensBookType::PictureBook => "Picture Book",
        crate::cli_types::ChildrensBookType::EarlyReader => "Early Reader",
        crate::cli_types::ChildrensBookType::ChapterBook => "Chapter Book",
        crate::cli_types::ChildrensBookType::MiddleGrade => "Middle Grade",
        crate::cli_types::ChildrensBookType::YoungAdult => "Young Adult",
        crate::cli_types::ChildrensBookType::Educational => "Educational Book",
        crate::cli_types::ChildrensBookType::Bedtime => "Bedtime Story",
        crate::cli_types::ChildrensBookType::Adventure => "Adventure Book",
        crate::cli_types::ChildrensBookType::Fantasy => "Fantasy Book",
        crate::cli_types::ChildrensBookType::Realistic => "Realistic Fiction",
    };
    
    let age_group_name = match age_group {
        crate::cli_types::ChildrensAgeGroup::Toddler => "Toddlers (1-3 years)",
        crate::cli_types::ChildrensAgeGroup::Preschool => "Preschoolers (3-5 years)",
        crate::cli_types::ChildrensAgeGroup::Kindergarten => "Kindergarteners (5-6 years)",
        crate::cli_types::ChildrensAgeGroup::EarlyElementary => "Early Elementary (6-8 years)",
        crate::cli_types::ChildrensAgeGroup::Elementary => "Elementary (8-12 years)",
        crate::cli_types::ChildrensAgeGroup::MiddleGrade => "Middle Grade (10-14 years)",
        crate::cli_types::ChildrensAgeGroup::YoungAdult => "Young Adult (12+ years)",
    };
    
    let target_chapters = match length {
        crate::cli_types::ChildrensBookLength::Board => 1,    // Very short for toddlers
        crate::cli_types::ChildrensBookLength::Picture => 3,   // Picture book
        crate::cli_types::ChildrensBookLength::Early => 8,    // Early reader
        crate::cli_types::ChildrensBookLength::Chapter => 15,  // Chapter book
        crate::cli_types::ChildrensBookLength::Middle => 25,  // Middle grade
        crate::cli_types::ChildrensBookLength::Young => 35,   // Young adult
    };
    
    println!("📖 Creating {} about: {}", book_type_name, concept);
    println!("🎂 Target age: {}", age_group_name);
    println!("📚 Chapters: {}", target_chapters);
    println!();
    
    // Create the content
    let mut content = Content::new_book(
        format!("{} - {}", book_type_name, concept),
        "Pundit AI".to_string(),
        "Children's Fiction".to_string(),
        "Age-appropriate".to_string(),
        format!("A delightful {} story about {} designed for {}.", book_type_name.to_lowercase(), concept, age_group_name.to_lowercase()),
        format!("{} chapters", target_chapters),
        Some(target_chapters * 500), // Estimate word count
        target_chapters,
        model.clone(),
    );
    
    content.content_type = ContentType::ChildrensBook;
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        let ollama_client = OllamaClient::new(ollama_url)?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    println!("✨ Creating magical children's content...");
    
    let progress_bar = ProgressBar::new(target_chapters as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} chapters ({percent}%)")
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("#>-"));
    
    for chapter_num in 1..=target_chapters {
        progress_bar.set_message(format!("Writing chapter {}", chapter_num));
        
        if let Err(e) = write_childrens_chapter(&client, &model, &mut content, chapter_num, &concept, &book_type, &age_group, &progress_bar).await {
            println!("\n❌ Error writing chapter {}: {}", chapter_num, e);
            break;
        }
        
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("Children's book complete!");
    
    // Save the content
    let filename = output.unwrap_or_else(|| {
        format!("{}_book_{}.txt", 
            concept.replace(" ", "_").to_lowercase(),
            chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    });
    
    let content_text = content.to_text();
    fs::write(&filename, content_text)?;
    
    println!("\n✅ Children's book saved to: {}", filename);
    println!("📚 Total chapters: {}", content.sections.len());
    println!("📄 Total words: {}", content.metadata.current_word_count);
    println!("🎨 Remember: This book includes illustration suggestions marked with 🎨");
    
    Ok(())
}

// Helper functions for new content types
async fn write_dictionary_section(
    client: &AIClient,
    model: &str,
    content: &mut Content,
    section_num: usize,
    topic: &str,
    dict_type: &crate::cli_types::DictionaryType,
    entries_per_section: usize,
    letter_range: &str,
    _progress_bar: &ProgressBar,
) -> Result<()> {
    let dict_style = match dict_type {
        crate::cli_types::DictionaryType::Etymological => "Include word origins and historical development",
        crate::cli_types::DictionaryType::Technical => "Focus on precise technical definitions",
        crate::cli_types::DictionaryType::Slang => "Include informal usage and cultural context",
        crate::cli_types::DictionaryType::Historical => "Include historical usage examples",
        crate::cli_types::DictionaryType::Bilingual => "Include translations and cross-language context",
        _ => "Standard dictionary format with clear definitions",
    };
    
    let section_title = format!("Entries {}", letter_range);
    let mut section = Section::new(section_num, section_title, String::new(), SectionType::Section);
    
    let prompt = format!(
        "Create {} dictionary entries for words related to '{}' starting with letters {}. 
        Style: {}
        
        Format each entry as:
        WORD: [word]
        DEFINITION: [clear, concise definition]
        ETYMOLOGY: [word origin and development - if applicable]
        
        Make entries informative and educational. Focus on {} terminology.",
        entries_per_section, topic, letter_range, dict_style, topic
    );
    
    let section_content = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&prompt, 2000, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &prompt, 2000, 0.8).await?
        }
    };
    
    section.set_content(section_content);
    content.add_section(section);
    
    Ok(())
}

async fn write_educational_section(
    client: &AIClient,
    model: &str,
    content: &mut Content,
    section_num: usize,
    topic: &str,
    edu_type: &crate::cli_types::EducationalType,
    audience: &crate::cli_types::EducationalAudience,
    _progress_bar: &ProgressBar,
) -> Result<()> {
    let learning_style = match edu_type {
        crate::cli_types::EducationalType::LanguageLearning => "Interactive language exercises with practical examples",
        crate::cli_types::EducationalType::Mathematics => "Step-by-step problem solving with clear explanations",
        crate::cli_types::EducationalType::Science => "Hands-on experiments and real-world applications",
        crate::cli_types::EducationalType::History => "Engaging narratives with timeline context",
        crate::cli_types::EducationalType::Arts => "Creative exercises and cultural appreciation",
        _ => "Clear explanations with practical examples and activities",
    };
    
    let audience_level = match audience {
        crate::cli_types::EducationalAudience::Preschool => "simple language appropriate for preschoolers ages 3-5",
        crate::cli_types::EducationalAudience::Elementary => "clear explanations for elementary students ages 6-11",
        crate::cli_types::EducationalAudience::MiddleSchool => "engaging content for middle school students ages 12-14",
        crate::cli_types::EducationalAudience::HighSchool => "comprehensive content for high school students ages 15-18",
        crate::cli_types::EducationalAudience::College => "detailed content for college-level learners",
        crate::cli_types::EducationalAudience::Adult => "practical content for adult learners",
        crate::cli_types::EducationalAudience::Professional => "professional development content",
        crate::cli_types::EducationalAudience::Senior => "accessible content for senior learners",
        crate::cli_types::EducationalAudience::All => "content appropriate for all audiences",
    };
    
    let section_title = format!("Lesson {}: {}", section_num, get_section_topic(topic, section_num));
    let mut section = Section::new(section_num, section_title, String::new(), SectionType::Section);
    
    let prompt = format!(
        "Create educational content for lesson section {} about '{}'. 
        
        Learning approach: {}
        Target audience: {}
        
        Structure the lesson with:
        OBJECTIVE: [clear learning goal for this section]
        CONTENT: [main educational content with examples]
        ACTIVITY: [hands-on exercise or practice]
        ASSESSMENT: [quick check for understanding]
        
        Make it engaging, educational, and appropriate for the target audience.",
        section_num, topic, learning_style, audience_level
    );
    
    let section_content = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&prompt, 2000, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &prompt, 2000, 0.8).await?
        }
    };
    
    section.set_content(section_content);
    content.add_section(section);
    
    Ok(())
}

async fn write_childrens_chapter(
    client: &AIClient,
    model: &str,
    content: &mut Content,
    chapter_num: usize,
    concept: &str,
    book_type: &crate::cli_types::ChildrensBookType,
    age_group: &crate::cli_types::ChildrensAgeGroup,
    _progress_bar: &ProgressBar,
) -> Result<()> {
    let writing_style = match age_group {
        crate::cli_types::ChildrensAgeGroup::Toddler => "very simple words, repetitive phrases, basic concepts",
        crate::cli_types::ChildrensAgeGroup::Preschool => "simple sentences, fun sounds, basic vocabulary",
        crate::cli_types::ChildrensAgeGroup::Kindergarten => "easy-to-read words, basic sentence structure",
        crate::cli_types::ChildrensAgeGroup::EarlyElementary => "easy-to-read words, short sentences, clear narrative",
        crate::cli_types::ChildrensAgeGroup::Elementary => "age-appropriate vocabulary, engaging dialogue, descriptive but simple",
        crate::cli_types::ChildrensAgeGroup::MiddleGrade => "more complex vocabulary, character development, longer passages",
        crate::cli_types::ChildrensAgeGroup::YoungAdult => "sophisticated themes, complex characters, mature vocabulary",
    };
    
    let book_style = match book_type {
        crate::cli_types::ChildrensBookType::PictureBook => "Include illustration descriptions with ILLUSTRATION: markers",
        crate::cli_types::ChildrensBookType::Bedtime => "Gentle, calming tone perfect for bedtime reading",
        crate::cli_types::ChildrensBookType::Adventure => "Exciting action and exploration while staying age-appropriate",
        crate::cli_types::ChildrensBookType::Educational => "Weave learning elements naturally into the story",
        crate::cli_types::ChildrensBookType::Fantasy => "Magical elements that spark imagination",
        _ => "Engaging storytelling appropriate for children",
    };
    
    let chapter_title = get_chapter_title(concept, chapter_num, content.sections.len());
    let mut chapter = Section::new(chapter_num, chapter_title, String::new(), SectionType::Chapter);
    
    let context = content.get_context_for_next_section();
    
    let prompt = format!(
        "Write chapter {} of a children's book about '{}'. 
        
        Writing style: {}
        Book approach: {}
        
        {}
        
        Make the chapter:
        - Age-appropriate and engaging
        - Include dialogue with DIALOGUE: markers when characters speak
        - Include illustration suggestions with ILLUSTRATION: markers for picture descriptions
        - Have a clear beginning, middle, and end for this chapter
        - Advance the overall story while being complete on its own
        - Use positive, encouraging themes
        
        Write approximately 300-800 words depending on the target age group.",
        chapter_num, concept, writing_style, book_style, context
    );
    
    let chapter_content = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&prompt, 3000, 0.9).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &prompt, 3000, 0.9).await?
        }
    };
    
    chapter.set_content(chapter_content);
    content.add_section(chapter);
    
    Ok(())
}

fn get_section_topic(main_topic: &str, section_num: usize) -> String {
    let topics = vec![
        format!("Introduction to {}", main_topic),
        format!("Fundamentals of {}", main_topic),
        format!("Exploring {}", main_topic),
        format!("Practical Applications of {}", main_topic),
        format!("Advanced Concepts in {}", main_topic),
        format!("Real-world Examples of {}", main_topic),
        format!("Common Challenges with {}", main_topic),
        format!("Best Practices for {}", main_topic),
        format!("Future Directions in {}", main_topic),
        format!("Mastering {}", main_topic),
    ];
    
    if section_num <= topics.len() {
        topics[section_num - 1].clone()
    } else {
        format!("Advanced Topics in {} (Part {})", main_topic, section_num - topics.len())
    }
}

fn get_chapter_title(concept: &str, chapter_num: usize, total_chapters: usize) -> String {
    match chapter_num {
        1 => format!("The Beginning of {}", concept),
        n if n == total_chapters => format!("The Resolution of {}", concept),
        n if n == total_chapters - 1 => format!("The Climax of {}", concept),
        n if n <= 3 => format!("Meeting {}", concept),
        n if n <= total_chapters / 2 => format!("Adventures with {}", concept),
        _ => format!("Challenges and {}", concept),
    }
}

// Helper function for consistent model selection across all interactive functions
fn interactive_model_selection(content_type: &str, estimated_length: &str) -> Result<(bool, String)> {
    println!("\n🤖 Model Selection:");
    let model_options = vec![
        "🏠 Local models (Ollama) - No API key needed, runs offline",
        "☁️ Cloud models (HuggingFace) - Requires API key or limited free tier",
        "← Back",
    ];
    
    let model_type_idx = Select::new()
        .with_prompt("Which type of model would you like to use?")
        .items(&model_options)
        .default(0) // Default to local
        .interact()?;
    
    if model_type_idx == model_options.len() - 1 {
        return Err(anyhow::anyhow!("Back selected")); // Signal to go back
    }
    
    let (use_local, model) = if model_type_idx == 0 {
        // Local models - use same logic as narrate_mode
        let recommended_local = match estimated_length {
            length if length.contains("Short") || length.contains("Quick") => "llama3.2:1b",
            length if length.contains("Medium") || length.contains("Standard") => "llama3.2",
            length if length.contains("Long") || length.contains("Extended") || length.contains("Large") => "llama3.1:8b",
            _ => "llama3.2", // Default
        };
        
        println!("\n🏠 Local Model Options:");
        println!("   Recommended: {} for {} content", recommended_local, estimated_length);
        println!("   Note: Make sure you have Ollama installed and the model downloaded");
        println!("   To install: ollama pull {}", recommended_local);
        
        let use_recommended_local = Confirm::new()
            .with_prompt(&format!("Use recommended local model ({}) for this {} {}?", recommended_local, estimated_length, content_type))
            .default(true)
            .interact()?;
        
        let local_model = if use_recommended_local {
            recommended_local.to_string()
        } else {
            println!("\n📋 Available Ollama models:");
            println!("   Fast: llama3.2:1b, gemma2:2b, phi3:mini");
            println!("   Balanced: llama3.2, mistral:7b, qwen2:7b");
            println!("   High Quality: llama3.1:8b, gemma2:9b, codellama:7b");
            
            Input::new()
                .with_prompt("Enter Ollama model name")
                .with_initial_text(recommended_local)
                .interact_text()?
        };
        
        (true, local_model)
    } else {
        // Cloud models
        let recommended_cloud = match estimated_length {
            length if length.contains("Short") || length.contains("Quick") => "distilgpt2",
            length if length.contains("Long") || length.contains("Extended") || length.contains("Large") => "gpt2-medium",
            _ => "gpt2", // Default
        };
        
        println!("\n☁️ Cloud Model Options:");
        println!("   Recommended: {} for {} content", recommended_cloud, estimated_length);
        println!("   Note: Some models work without API key, others may require authentication");
        
        let use_recommended_cloud = Confirm::new()
            .with_prompt(&format!("Use recommended cloud model ({}) for this {} {}?", recommended_cloud, estimated_length, content_type))
            .default(true)
            .interact()?;
        
        let cloud_model = if use_recommended_cloud {
            recommended_cloud.to_string()
        } else {
            Input::new()
                .with_prompt("Enter HuggingFace model name")
                .with_initial_text(recommended_cloud)
                .interact_text()?
        };
        
        (false, cloud_model)
    };
    
    Ok((use_local, model))
}

// Educational Book Creation - comprehensive system for educational content
async fn create_educational_book() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    'educational_book_setup: loop {
        println!("{}", console::style("📚 Pundit - Educational Book Creator").bold().green());
        println!("═══════════════════════════════════════");
        println!();
        println!("Create comprehensive educational books including:");
        println!("• Language Learning Books with lessons and exercises");
        println!("• Subject Textbooks (History, Science, Math, etc.)");
        println!("• Course Materials with structured curriculum");
        println!("• Training Manuals and Professional Development");
        println!();
        
        // Educational book type selection
        'type_selection: loop {
            let educational_types = vec![
                "📖 Language Learning Book - Learn a new language with structured lessons",
                "🔬 Science Textbook - Physics, Chemistry, Biology, or other sciences", 
                "📜 History Textbook - Historical events, periods, and analysis",
                "🧮 Mathematics Textbook - Math concepts from basic to advanced",
                "💼 Professional Training Manual - Skills and procedures for work",
                "🎓 Course Curriculum - Complete course with modules and assessments",
                "🗣️ Language Translation Guide - Between two specific languages",
                "📋 Study Guide - Exam prep and review materials",
                "🔤 Grammar & Writing Book - Language mechanics and composition",
                "🌍 Cultural Studies Book - Explore cultures and societies",
                "← Back to main menu"
            ];
            
            let type_idx = Select::new()
                .with_prompt("What type of educational book would you like to create?")
                .items(&educational_types)
                .default(0)
                .interact()?;
            
            if type_idx == educational_types.len() - 1 {
                return Err(BackToMenu.into()); // Back to main menu
            }
            
            match type_idx {
                0 => return create_language_learning_book().await,
                1 => return create_science_textbook().await,
                2 => return create_history_textbook().await,
                3 => return create_mathematics_textbook().await,
                4 => return create_training_manual().await,
                5 => return create_course_curriculum().await,
                6 => return create_translation_guide().await,
                7 => return create_study_guide().await,
                8 => return create_grammar_book().await,
                9 => return create_cultural_studies_book().await,
                _ => continue 'type_selection,
            }
        }
    }
}

// Language Learning Book Creation
async fn create_language_learning_book() -> Result<()> {
    'language_book: loop {
        println!("\n📖 Creating Language Learning Book");
        println!("═══════════════════════════════════");
        
        // Target language selection
        let languages = vec![
            "Spanish", "French", "German", "Italian", "Portuguese", "Russian",
            "Chinese (Mandarin)", "Japanese", "Korean", "Arabic", "Hindi", 
            "Dutch", "Swedish", "Norwegian", "Polish", "Czech", "Turkish",
            "Greek", "Hebrew", "Thai", "Vietnamese", "Indonesian", "Swahili",
            "Other (specify)", "← Back"
        ];
        
        let lang_idx = Select::new()
            .with_prompt("What language do you want to teach?")
            .items(&languages)
            .default(0)
            .interact()?;
        
        if lang_idx == languages.len() - 1 {
            return Ok(()); // Back
        }
        
        let target_language = if lang_idx == languages.len() - 2 {
            Input::new()
                .with_prompt("Enter the language name")
                .interact_text()?
        } else {
            languages[lang_idx].to_string()
        };
        
        // Difficulty level
        let levels = vec![
            "Beginner (A1) - Absolute basics",
            "Elementary (A2) - Basic communication",
            "Intermediate (B1) - Everyday conversations", 
            "Upper Intermediate (B2) - Complex topics",
            "Advanced (C1) - Fluency development",
            "Proficient (C2) - Near-native mastery",
            "← Back"
        ];
        
        let level_idx = Select::new()
            .with_prompt("What difficulty level should the book target?")
            .items(&levels)
            .default(0)
            .interact()?;
        
        if level_idx == levels.len() - 1 {
            continue 'language_book;
        }
        
        let difficulty_level = match level_idx {
            0 => "Beginner (A1)",
            1 => "Elementary (A2)", 
            2 => "Intermediate (B1)",
            3 => "Upper Intermediate (B2)",
            4 => "Advanced (C1)",
            5 => "Proficient (C2)",
            _ => "Beginner (A1)",
        };
        
        // Book focus
        let focuses = vec![
            "🗣️ Conversation & Speaking - Practical communication skills",
            "📚 Grammar & Structure - Language rules and patterns",
            "📖 Reading & Comprehension - Text understanding skills",
            "✍️ Writing & Composition - Written expression skills",
            "👂 Listening & Pronunciation - Audio comprehension and accent",
            "🌍 Culture & Context - Cultural understanding alongside language",
            "💼 Business Language - Professional and workplace communication",
            "🎓 Academic Language - Formal and educational contexts",
            "← Back"
        ];
        
        let focus_idx = Select::new()
            .with_prompt("What should be the primary focus of the book?")
            .items(&focuses)
            .default(0)
            .interact()?;
        
        if focus_idx == focuses.len() - 1 {
            continue 'language_book;
        }
        
        let focus_area = match focus_idx {
            0 => "Conversation",
            1 => "Grammar",
            2 => "Reading",
            3 => "Writing",
            4 => "Listening",
            5 => "Culture",
            6 => "Business",
            7 => "Academic",
            _ => "Conversation",
        };
        
        // Book length
        let lengths = vec![
            "Short Course (20-30 lessons, ~40,000 words)",
            "Standard Course (40-50 lessons, ~80,000 words)",
            "Comprehensive Course (60-80 lessons, ~120,000 words)",
            "Complete Program (100+ lessons, ~200,000 words)",
            "← Back"
        ];
        
        let length_idx = Select::new()
            .with_prompt("How comprehensive should the language course be?")
            .items(&lengths)
            .default(1)
            .interact()?;
        
        if length_idx == lengths.len() - 1 {
            continue 'language_book;
        }
        
        let (lesson_count, book_size) = match length_idx {
            0 => (25, BookSize::Medium),
            1 => (45, BookSize::Large), 
            2 => (70, BookSize::VeryLarge),
            3 => (120, BookSize::Unlimited),
            _ => (45, BookSize::Large),
        };
        
        // Model selection
        let length_desc = match length_idx {
            0 => "Medium",
            1 => "Large",
            2 => "Large", 
            3 => "Extended",
            _ => "Large",
        };
        
        let (use_local, model) = match interactive_model_selection("language learning book", length_desc) {
            Ok((use_local, model)) => (use_local, model),
            Err(_) => continue 'language_book,
        };
        
        println!("\n✨ Creating {} {} Language Learning Book", target_language, difficulty_level);
        println!("📚 Focus: {} skills", focus_area);
        println!("📖 Lessons: {}", lesson_count);
        println!("🤖 Model: {}", model);
        println!();
        
        // Create the educational book with language learning focus
        return write_language_learning_book(target_language, difficulty_level.to_string(), focus_area.to_string(), lesson_count, book_size, None, model, None, use_local, "http://localhost:11434".to_string()).await;
    }
}

// Science Textbook Creation  
async fn create_science_textbook() -> Result<()> {
    'science_book: loop {
        println!("\n🔬 Creating Science Textbook");
        println!("═══════════════════════════════");
        
        // Science subject selection
        let subjects = vec![
            "Physics - Mechanics, Thermodynamics, Electromagnetism",
            "Chemistry - Organic, Inorganic, Physical Chemistry",
            "Biology - Cell Biology, Genetics, Evolution, Ecology", 
            "Earth Science - Geology, Meteorology, Oceanography",
            "Astronomy - Solar System, Stars, Galaxies, Cosmology",
            "Environmental Science - Ecosystems, Climate, Sustainability",
            "Computer Science - Algorithms, Programming, Data Structures",
            "Engineering - Mechanical, Electrical, Civil principles",
            "Medical Science - Anatomy, Physiology, Health",
            "Other Science (specify)", "← Back"
        ];
        
        let subject_idx = Select::new()
            .with_prompt("Which science subject should the textbook cover?")
            .items(&subjects)
            .default(0)
            .interact()?;
        
        if subject_idx == subjects.len() - 1 {
            return Ok(());
        }
        
        let science_subject = if subject_idx == subjects.len() - 2 {
            Input::new()
                .with_prompt("Enter the science subject")
                .interact_text()?
        } else {
            subjects[subject_idx].split(" - ").next().unwrap_or(&subjects[subject_idx]).to_string()
        };
        
        // Education level
        let levels = vec![
            "Elementary (Ages 8-12) - Basic concepts and experiments",
            "Middle School (Ages 12-15) - Foundational principles", 
            "High School (Ages 15-18) - Advanced concepts and applications",
            "College Introductory - University-level fundamentals",
            "Advanced Undergraduate - Specialized topics and research",
            "Graduate Level - Research-focused and theoretical",
            "← Back"
        ];
        
        let level_idx = Select::new()
            .with_prompt("What education level should this textbook target?")
            .items(&levels)
            .default(2)
            .interact()?;
        
        if level_idx == levels.len() - 1 {
            continue 'science_book;
        }
        
        let education_level = match level_idx {
            0 => "Elementary",
            1 => "Middle School",
            2 => "High School", 
            3 => "College Introductory",
            4 => "Advanced Undergraduate",
            5 => "Graduate Level",
            _ => "High School",
        };
        
        // Textbook approach
        let approaches = vec![
            "📊 Theoretical Focus - Concepts, laws, and principles",
            "🧪 Experimental Focus - Labs, experiments, and practical work",
            "🔗 Applied Focus - Real-world applications and case studies",
            "📈 Problem-Solving Focus - Mathematics and analytical thinking",
            "🎯 Exam Preparation - Test prep with practice questions",
            "🌟 Conceptual Understanding - Visual and intuitive explanations",
            "← Back"
        ];
        
        let approach_idx = Select::new()
            .with_prompt("What should be the primary teaching approach?")
            .items(&approaches)
            .default(2)
            .interact()?;
        
        if approach_idx == approaches.len() - 1 {
            continue 'science_book;
        }
        
        let teaching_approach = match approach_idx {
            0 => "Theoretical",
            1 => "Experimental",
            2 => "Applied", 
            3 => "Problem-Solving",
            4 => "Exam Prep",
            5 => "Conceptual",
            _ => "Applied",
        };
        
        // Book size
        let sizes = vec![
            "Concise (15-20 chapters, ~60,000 words)",
            "Standard (25-30 chapters, ~100,000 words)", 
            "Comprehensive (35-45 chapters, ~150,000 words)",
            "Complete Reference (50+ chapters, ~250,000 words)",
            "← Back"
        ];
        
        let size_idx = Select::new()
            .with_prompt("How comprehensive should the textbook be?")
            .items(&sizes)
            .default(1)
            .interact()?;
        
        if size_idx == sizes.len() - 1 {
            continue 'science_book;
        }
        
        let (chapter_count, book_size) = match size_idx {
            0 => (18, BookSize::Medium),
            1 => (28, BookSize::Large),
            2 => (40, BookSize::VeryLarge),
            3 => (60, BookSize::Unlimited),
            _ => (28, BookSize::Large),
        };
        
        // Model selection
        let length_desc = match size_idx {
            0 => "Medium",
            1 => "Large",
            2 => "Large",
            3 => "Extended", 
            _ => "Large",
        };
        
        let (use_local, model) = match interactive_model_selection("science textbook", length_desc) {
            Ok((use_local, model)) => (use_local, model),
            Err(_) => continue 'science_book,
        };
        
        println!("\n🔬 Creating {} Science Textbook", science_subject);
        println!("🎓 Level: {}", education_level);
        println!("📚 Approach: {} focus", teaching_approach);
        println!("📖 Chapters: {}", chapter_count);
        println!("🤖 Model: {}", model);
        println!();
        
        // Create the science textbook
        return write_science_textbook(science_subject, education_level.to_string(), teaching_approach.to_string(), chapter_count, book_size, None, model, None, use_local, "http://localhost:11434".to_string()).await;
    }
}

// Placeholder functions for other educational book types
async fn create_history_textbook() -> Result<()> {
    println!("🚧 History textbook creation coming soon!");
    Ok(())
}

async fn create_mathematics_textbook() -> Result<()> {
    println!("🚧 Mathematics textbook creation coming soon!");
    Ok(())
}

async fn create_training_manual() -> Result<()> {
    println!("🚧 Training manual creation coming soon!");
    Ok(())
}

async fn create_course_curriculum() -> Result<()> {
    println!("🚧 Course curriculum creation coming soon!");
    Ok(())
}

async fn create_translation_guide() -> Result<()> {
    println!("🚧 Translation guide creation coming soon!");
    Ok(())
}

async fn create_study_guide() -> Result<()> {
    println!("🚧 Study guide creation coming soon!");
    Ok(())
}

async fn create_grammar_book() -> Result<()> {
    println!("🚧 Grammar book creation coming soon!");
    Ok(())
}

async fn create_cultural_studies_book() -> Result<()> {
    println!("🚧 Cultural studies book creation coming soon!");
    Ok(())
}

// Writing functions for educational books
async fn write_language_learning_book(
    target_language: String,
    difficulty_level: String, 
    focus_area: String,
    lesson_count: usize,
    _book_size: BookSize,
    output: Option<String>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("📖 Creating Language Learning Book").bold().green());
    println!("═══════════════════════════════════════");
    println!();
    
    println!("📚 Language: {}", target_language);
    println!("🎯 Level: {}", difficulty_level);
    println!("💬 Focus: {}", focus_area);
    println!("📖 Lessons: {}", lesson_count);
    println!();
    
    // Create the content structure
    let mut content = Content::new_book(
        format!("{} Language Learning Book - {}", target_language, difficulty_level),
        "Pundit AI".to_string(),
        "Educational".to_string(),
        focus_area.clone(),
        format!("A comprehensive {} language learning book focusing on {} skills for {} level learners.", target_language, focus_area.to_lowercase(), difficulty_level),
        format!("{} lessons", lesson_count),
        Some(lesson_count * 1500), // Estimate word count per lesson
        lesson_count,
        model.clone(),
    );
    
    content.content_type = ContentType::Book; // Educational book
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        let ollama_client = OllamaClient::new(ollama_url)?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    println!("🏗️ Generating language learning lessons...");
    
    let progress_bar = ProgressBar::new(lesson_count as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} lessons ({percent}%)")
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("#>-"));
    
    for lesson_num in 1..=lesson_count {
        progress_bar.set_message(format!("Creating lesson {}", lesson_num));
        
        if let Err(e) = write_language_lesson(&client, &model, &mut content, lesson_num, &target_language, &difficulty_level, &focus_area, lesson_count, &progress_bar).await {
            println!("\n❌ Error writing lesson {}: {}", lesson_num, e);
            break;
        }
        
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("Language learning book complete!");
    
    // Save the content
    let filename = output.unwrap_or_else(|| {
        format!("{}_{}_learning_book_{}.txt", 
            target_language.replace(" ", "_").to_lowercase(),
            focus_area.replace(" ", "_").to_lowercase(),
            chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    });
    
    let content_text = content.to_text();
    fs::write(&filename, content_text)?;
    
    println!("\n✅ Language learning book saved to: {}", filename);
    println!("📚 Total lessons: {}", content.sections.len());
    println!("📄 Total words: {}", content.metadata.current_word_count);
    
    Ok(())
}

async fn write_science_textbook(
    science_subject: String,
    education_level: String,
    teaching_approach: String,
    chapter_count: usize,
    _book_size: BookSize,
    output: Option<String>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", console::style("🔬 Creating Science Textbook").bold().blue());
    println!("═══════════════════════════════════════");
    println!();
    
    println!("🔬 Subject: {}", science_subject);
    println!("🎓 Level: {}", education_level);
    println!("📚 Approach: {}", teaching_approach);
    println!("📖 Chapters: {}", chapter_count);
    println!();
    
    // Create the content structure
    let mut content = Content::new_book(
        format!("{} Textbook - {}", science_subject, education_level),
        "Pundit AI".to_string(),
        "Educational".to_string(),
        teaching_approach.clone(),
        format!("A comprehensive {} textbook for {} students with a {} approach.", science_subject, education_level.to_lowercase(), teaching_approach.to_lowercase()),
        format!("{} chapters", chapter_count),
        Some(chapter_count * 3000), // Estimate word count per chapter
        chapter_count,
        model.clone(),
    );
    
    content.content_type = ContentType::Book; // Educational book
    
    // Load configuration
    let config = Config::load()?;
    
    // Create appropriate client
    let client = if use_local {
        let ollama_client = OllamaClient::new(ollama_url)?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    println!("🏗️ Generating science textbook chapters...");
    
    let progress_bar = ProgressBar::new(chapter_count as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} chapters ({percent}%)")
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("#>-"));
    
    for chapter_num in 1..=chapter_count {
        progress_bar.set_message(format!("Creating chapter {}", chapter_num));
        
        if let Err(e) = write_science_chapter(&client, &model, &mut content, chapter_num, &science_subject, &education_level, &teaching_approach, chapter_count, &progress_bar).await {
            println!("\n❌ Error writing chapter {}: {}", chapter_num, e);
            break;
        }
        
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message("Science textbook complete!");
    
    // Save the content
    let filename = output.unwrap_or_else(|| {
        format!("{}_textbook_{}_{}.txt", 
            science_subject.replace(" ", "_").to_lowercase(),
            education_level.replace(" ", "_").to_lowercase(),
            chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    });
    
    let content_text = content.to_text();
    fs::write(&filename, content_text)?;
    
    println!("\n✅ Science textbook saved to: {}", filename);
    println!("📚 Total chapters: {}", content.sections.len());
    println!("📄 Total words: {}", content.metadata.current_word_count);
    
    Ok(())
}

// Helper functions for educational content generation
async fn write_language_lesson(
    client: &AIClient,
    model: &str,
    content: &mut Content,
    lesson_num: usize,
    target_language: &str,
    difficulty_level: &str,
    focus_area: &str,
    total_lessons: usize,
    _progress_bar: &ProgressBar,
) -> Result<()> {
    let lesson_title = format!("Lesson {}: {}", lesson_num, get_language_lesson_title(target_language, focus_area, lesson_num, total_lessons));
    let mut lesson = Section::new(lesson_num, lesson_title, String::new(), SectionType::Chapter);
    
    let context = content.get_context_for_next_section();
    
    let prompt = format!(
        "Create lesson {} of a {} {} language learning book focusing on {} skills.
        
        Level: {}
        Lesson Context: {}
        
        Structure the lesson with:
        
        LESSON OBJECTIVE: What students will learn in this lesson
        VOCABULARY: 8-12 key words/phrases with translations and pronunciation guides
        GRAMMAR POINT: Main grammatical concept (if applicable)
        DIALOGUE: A practical conversation example using the new vocabulary
        EXERCISES: 3-4 practice activities
        CULTURAL NOTE: Brief cultural insight related to the language/topic
        HOMEWORK: Practice assignments for reinforcement
        
        Make the lesson:
        - Progressive (building on previous lessons)
        - Interactive with clear examples
        - Practical for real-world use
        - Appropriate for {} level learners
        - Focused on {} skills
        
        Write approximately 1200-1800 words.",
        lesson_num, target_language, difficulty_level, focus_area.to_lowercase(), 
        difficulty_level, context, difficulty_level, focus_area.to_lowercase()
    );
    
    let lesson_content = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&prompt, 4000, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &prompt, 4000, 0.8).await?
        }
    };
    
    lesson.set_content(lesson_content);
    content.add_section(lesson);
    
    Ok(())
}

async fn write_science_chapter(
    client: &AIClient,
    model: &str,
    content: &mut Content,
    chapter_num: usize,
    science_subject: &str,
    education_level: &str,
    teaching_approach: &str,
    total_chapters: usize,
    _progress_bar: &ProgressBar,
) -> Result<()> {
    let chapter_title = format!("Chapter {}: {}", chapter_num, get_science_chapter_title(science_subject, chapter_num, total_chapters));
    let mut chapter = Section::new(chapter_num, chapter_title, String::new(), SectionType::Chapter);
    
    let context = content.get_context_for_next_section();
    
    let approach_guidance = match teaching_approach {
        "Theoretical" => "Focus on concepts, laws, principles, and theoretical frameworks",
        "Experimental" => "Include lab procedures, experiments, and hands-on activities",
        "Applied" => "Emphasize real-world applications, case studies, and practical examples",
        "Problem-Solving" => "Include mathematical calculations, problem-solving strategies, and analytical thinking",
        "Exam Prep" => "Include practice questions, test strategies, and exam-focused content",
        "Conceptual" => "Use visual aids, analogies, and intuitive explanations to build understanding",
        _ => "Provide clear explanations with examples and applications",
    };
    
    let prompt = format!(
        "Create chapter {} of a {} textbook for {} students using a {} approach.
        
        Chapter Context: {}
        Teaching Approach: {}
        
        Structure the chapter with:
        
        CHAPTER OVERVIEW: Brief introduction to the topics covered
        LEARNING OBJECTIVES: What students will understand after this chapter
        KEY CONCEPTS: Main ideas and principles (3-5 concepts)
        DETAILED EXPLANATIONS: Comprehensive coverage of each concept
        EXAMPLES: Concrete examples and applications
        PRACTICE PROBLEMS: 5-8 problems or questions (if applicable)
        CHAPTER SUMMARY: Key takeaways and review points
        FURTHER READING: Additional resources for deeper study
        
        Make the chapter:
        - Scientifically accurate and current
        - Appropriate for {} level students
        - Progressive (building on previous chapters)
        - {} focused
        - Engaging with clear explanations
        
        Write approximately 2500-3500 words.",
        chapter_num, science_subject, education_level, teaching_approach.to_lowercase(),
        context, approach_guidance, education_level, teaching_approach.to_lowercase()
    );
    
    let chapter_content = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&prompt, 5000, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &prompt, 5000, 0.7).await?
        }
    };
    
    chapter.set_content(chapter_content);
    content.add_section(chapter);
    
    Ok(())
}

fn get_language_lesson_title(target_language: &str, focus_area: &str, lesson_num: usize, total_lessons: usize) -> String {
    match focus_area {
        "Conversation" => match lesson_num {
            1 => format!("Basic Greetings and Introductions in {}", target_language),
            2 => "Numbers, Time, and Dates".to_string(),
            3 => "Family and Personal Information".to_string(),
            4 => "Food and Dining Out".to_string(),
            5 => "Shopping and Directions".to_string(),
            n if n <= total_lessons / 3 => format!("Essential {} Conversations", target_language),
            n if n <= 2 * total_lessons / 3 => "Intermediate Conversational Topics".to_string(),
            _ => "Advanced Discussion and Fluency".to_string(),
        },
        "Grammar" => match lesson_num {
            1 => format!("{} Alphabet and Basic Sentence Structure", target_language),
            2 => "Nouns, Articles, and Gender".to_string(),
            3 => "Present Tense Verbs".to_string(),
            4 => "Question Formation and Negation".to_string(),
            5 => "Adjectives and Descriptions".to_string(),
            n if n <= total_lessons / 3 => "Essential Grammar Rules".to_string(),
            n if n <= 2 * total_lessons / 3 => "Intermediate Grammar Concepts".to_string(),
            _ => "Advanced Grammar and Style".to_string(),
        },
        "Reading" => match lesson_num {
            1 => format!("{} Writing System and Basic Words", target_language),
            2 => "Reading Simple Sentences".to_string(),
            3 => "Understanding Context and Meaning".to_string(),
            n if n <= total_lessons / 2 => "Reading Comprehension Skills".to_string(),
            _ => "Advanced Text Analysis".to_string(),
        },
        "Writing" => match lesson_num {
            1 => format!("Writing in {} Script", target_language),
            2 => "Basic Sentence Construction".to_string(),
            3 => "Paragraph Development".to_string(),
            n if n <= total_lessons / 2 => "Writing Skills and Practice".to_string(),
            _ => "Advanced Writing Techniques".to_string(),
        },
        "Business" => match lesson_num {
            1 => format!("Professional {} Vocabulary", target_language),
            2 => "Business Communications".to_string(),
            3 => "Meetings and Presentations".to_string(),
            _ => format!("Professional {} Skills", target_language),
        },
        _ => format!("{} Language Fundamentals", target_language),
    }
}

fn get_science_chapter_title(science_subject: &str, chapter_num: usize, total_chapters: usize) -> String {
    match science_subject {
        "Physics" => match chapter_num {
            1 => "Introduction to Physics and Measurement".to_string(),
            2 => "Motion in One Dimension".to_string(),
            3 => "Vectors and Two-Dimensional Motion".to_string(),
            4 => "Forces and Newton's Laws".to_string(),
            5 => "Work, Energy, and Power".to_string(),
            6 => "Momentum and Collisions".to_string(),
            7 => "Rotational Motion".to_string(),
            8 => "Gravitation".to_string(),
            9 => "Waves and Sound".to_string(),
            10 => "Thermodynamics".to_string(),
            11 => "Electric Fields and Potential".to_string(),
            12 => "Current and Circuits".to_string(),
            n if n <= total_chapters / 2 => "Classical Physics Principles".to_string(),
            _ => "Modern Physics Concepts".to_string(),
        },
        "Chemistry" => match chapter_num {
            1 => "Atoms, Molecules, and Ions".to_string(),
            2 => "Chemical Bonding".to_string(),
            3 => "Stoichiometry".to_string(),
            4 => "Chemical Reactions".to_string(),
            5 => "Gases and Gas Laws".to_string(),
            6 => "Thermochemistry".to_string(),
            7 => "Atomic Structure and Periodicity".to_string(),
            8 => "Solutions and Solubility".to_string(),
            9 => "Acids and Bases".to_string(),
            10 => "Chemical Equilibrium".to_string(),
            n if n <= total_chapters / 2 => "General Chemistry Principles".to_string(),
            _ => "Advanced Chemical Concepts".to_string(),
        },
        "Biology" => match chapter_num {
            1 => "The Chemistry of Life".to_string(),
            2 => "Cell Structure and Function".to_string(),
            3 => "Cellular Metabolism".to_string(),
            4 => "Cell Division and Reproduction".to_string(),
            5 => "Genetics and Heredity".to_string(),
            6 => "DNA and Protein Synthesis".to_string(),
            7 => "Evolution and Natural Selection".to_string(),
            8 => "Classification of Living Things".to_string(),
            9 => "Plant Biology".to_string(),
            10 => "Animal Biology".to_string(),
            11 => "Ecology and Ecosystems".to_string(),
            n if n <= total_chapters / 2 => "Cellular and Molecular Biology".to_string(),
            _ => "Ecology and Environmental Biology".to_string(),
        },
        _ => format!("{} Fundamentals", science_subject),
    }
}

// Helper function for book creation with new/continue options
async fn interactive_book_mode() -> Result<()> {
    loop {
        println!("\n📚 Book Creation Mode");
        println!("═══════════════════");
        println!();
        
        let mode_options = vec![
            "🆕 Create a new book from scratch",
            "📂 Continue/extend an existing book or work",
            "🔄 Continue as different format (e.g. book from screenplay)",
            "← Back to content types",
        ];
        
        let mode_choice = Select::new()
            .with_prompt("What would you like to do?")
            .items(&mode_options)
            .default(0)
            .interact()?;
        
        match mode_choice {
            0 => {
                // Create new book - use existing narrate mode
                return narrate_mode().await;
            },
            1 => {
                // Continue existing book - use continuation with book context
                return interactive_book_continuation().await;
            },
            2 => {
                // Cross-format continuation 
                return interactive_cross_format_continuation(ContentType::Book).await;
            },
            _ => {
                // Back to content types
                return Ok(());
            }
        }
    }
}

// Book-specific continuation that maintains book format
async fn interactive_book_continuation() -> Result<()> {
    println!("\n📂 Continue Existing Book");
    println!("═════════════════════════");
    println!();
    
    // Use existing continuation setup but with book context
    let project = match interactive_continuation_setup().await {
        Ok(project) => project,
        Err(_) => {
            println!("📂 Returning to book menu...");
            return Ok(());
        }
    };
    
    println!("\n✅ Project loaded!");
    println!("Total files: {}", project.files.len());
    println!("Total word count: {}", project.total_word_count());
    
    // Ensure we're working with book format
    let mut content = continuation_project_to_content(&project, 
        "Continued Book".to_string(), 
        "Author".to_string())?;
    content.content_type = ContentType::Book;
    
    // Get book-specific continuation details
    let title: String = Input::new()
        .with_prompt("Book title (for this continuation)")
        .default("Continued Book".to_string())
        .interact_text()?;
    
    let author: String = Input::new()
        .with_prompt("Author name")
        .default("Anonymous".to_string())
        .interact_text()?;
    
    // Book continuation options
    let continuation_options = vec![
        "📖 Add new chapters to the existing story",
        "🔄 Create a sequel/next book in series",
        "📝 Extend specific scenes or chapters",
        "🎭 Add character development or backstory",
    ];
    
    let continuation_idx = Select::new()
        .with_prompt("What type of book continuation would you like?")
        .items(&continuation_options)
        .default(0)
        .interact()?;
    
    let continuation_prompt = match continuation_idx {
        0 => {
            let chapters: String = Input::new()
                .with_prompt("How many new chapters would you like to add?")
                .default("3".to_string())
                .interact_text()?;
            format!("Continue this book by adding {} new chapters that build on the existing story. Maintain consistent character development and plot progression.", chapters)
        },
        1 => {
            Input::new()
                .with_prompt("Describe the sequel/next book (setting, time period, new conflicts)")
                .interact_text()?
        },
        2 => {
            Input::new()
                .with_prompt("Which scenes or chapters would you like to extend, and what should be added?")
                .interact_text()?
        },
        3 => {
            Input::new()
                .with_prompt("Which characters need development? What backstory or growth should be added?")
                .interact_text()?
        },
        _ => "Continue the book naturally".to_string(),
    };
    
    let target_words: String = Input::new()
        .with_prompt("Target word count for the continuation")
        .default("2000".to_string())
        .interact_text()?;
    
    // Use existing continuation logic but with book context
    let existing_content = project.get_combined_content()?;
    let target_words_num: usize = target_words.parse().unwrap_or(2000);
    
    println!("\n📖 Generating book continuation...");
    
    // Create a proper book content object
    content.title = title.clone();
    content.author = author.clone();
    content.genre = "Continued Book".to_string();
    
    let continuation_result = write_book_continuation(
        &content,
        &continuation_prompt,
        &existing_content,
        target_words_num
    ).await?;
    
    // Save the result
    let output_filename = format!("{}_continuation.md", title.replace(" ", "_").to_lowercase());
    let output_path = std::path::PathBuf::from(&output_filename);
    
    let full_content = format!(
        "# {} - Continuation\n\nAuthor: {}\nGenerated: {}\n\n## Original Content Context\n\n{}\n\n## New Content\n\n{}\n",
        title,
        content.author,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        {
            let context_limit = 500.min(existing_content.chars().count());
            let context_preview: String = existing_content.chars().take(context_limit).collect();
            context_preview
        },
        continuation_result
    );
    
    std::fs::write(&output_path, full_content)?;
    
    println!("\n✅ Book continuation completed!");
    println!("💾 Saved to: {}", output_path.display());
    println!("📊 Generated approximately {} words", continuation_result.split_whitespace().count());
    
    Ok(())
}

// Cross-format continuation (e.g., book from screenplay, etc.)
async fn interactive_cross_format_continuation(target_format: ContentType) -> Result<()> {
    println!("\n🔄 Cross-Format Continuation");
    println!("═══════════════════════════");
    println!("Convert and continue existing work in a different format");
    println!();
    
    // Load source material
    let project = match interactive_continuation_setup().await {
        Ok(project) => project,
        Err(_) => {
            println!("📂 Returning to menu...");
            return Ok(());
        }
    };
    
    println!("\n✅ Source material loaded!");
    println!("Total files: {}", project.files.len());
    println!("Total word count: {}", project.total_word_count());
    
    if let Some(ref source_type) = project.project_type {
        println!("Source format: {:?}", source_type);
        println!("Target format: {:?}", target_format);
    }
    
    let title: String = Input::new()
        .with_prompt("Title for the new format")
        .interact_text()?;
    
    let author: String = Input::new()
        .with_prompt("Author name")
        .default("Anonymous".to_string())
        .interact_text()?;
    
    let conversion_prompt: String = Input::new()
        .with_prompt(&format!(
            "Describe how you want to adapt this to a {:?}. (e.g., 'Create a novel from this screenplay', 'Turn this story into a movie script')",
            target_format
        ))
        .interact_text()?;
    
    let target_words: String = Input::new()
        .with_prompt("Target word count for the adaptation")
        .default("3000".to_string())
        .interact_text()?;
    
    let existing_content = project.get_combined_content()?;
    let target_words_num: usize = target_words.parse().unwrap_or(3000);
    
    println!("\n🔄 Generating cross-format adaptation...");
    
    // Create format-specific prompt
    let adaptation_prompt = format!(
        "Adapt the following content to a {:?} format. {}. Source content:\n\n{}",
        target_format,
        conversion_prompt,
        {
            let source_limit = 2000.min(existing_content.chars().count());
            let source_preview: String = existing_content.chars().take(source_limit).collect();
            source_preview
        }
    );
    
    // Generate the adapted content using Ollama client
    let ollama_url = "http://localhost:11434".to_string();
    let ollama_client = OllamaClient::new(ollama_url)?;
    let model = "llama3.2";
    
    let adaptation_result = ollama_client.generate_text(model, &adaptation_prompt, target_words_num as i32, 0.7).await?;
    
    // Save the result
    let output_filename = format!("{}_adapted_{:?}.md", 
        title.replace(" ", "_").to_lowercase(),
        target_format
    );
    let output_path = std::path::PathBuf::from(&output_filename);
    
    let full_content = format!(
        "# {} - Adapted to {:?}\n\nAuthor: {}\nGenerated: {}\n\nSource Format: {:?}\nTarget Format: {:?}\n\n## Adaptation Notes\n\n{}\n\n## Original Content (Preview)\n\n{}\n\n## Adapted Content\n\n{}\n",
        title,
        target_format,
        author,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        project.project_type.unwrap_or(ContentType::Book),
        target_format,
        conversion_prompt,
        {
            let context_limit = 500.min(existing_content.chars().count());
            let context_preview: String = existing_content.chars().take(context_limit).collect();
            context_preview
        },
        adaptation_result
    );
    
    std::fs::write(&output_path, full_content)?;
    
    println!("\n✅ Cross-format adaptation completed!");
    println!("💾 Saved to: {}", output_path.display());
    println!("📊 Generated approximately {} words", adaptation_result.split_whitespace().count());
    
    Ok(())
}

// Helper function for book continuation generation
async fn write_book_continuation(
    _content: &Content,
    continuation_prompt: &str,
    existing_content: &str,
    target_words: usize
) -> Result<String> {
    let ollama_url = "http://localhost:11434".to_string();
    let ollama_client = OllamaClient::new(ollama_url)?;
    let model = "llama3.2";
    
    // Create a comprehensive prompt for book continuation
    let full_prompt = format!(
        "You are continuing an existing book. Here is the context:\n\n## Existing Content Preview:\n{}\n\n## Continuation Instructions:\n{}\n\nPlease write a continuation that:\n1. Maintains consistent tone and style with the existing work\n2. Develops characters and plot naturally\n3. Uses proper chapter/section formatting for books\n4. Targets approximately {} words\n\nContinuation:",
        {
            let preview_limit = 1500.min(existing_content.chars().count());
            let preview: String = existing_content.chars().take(preview_limit).collect();
            preview
        },
        continuation_prompt,
        target_words
    );
    
    ollama_client.generate_text(model, &full_prompt, target_words as i32, 0.7).await
}

// Multi-plot narrative system implementation
// Optimize fiction creativity based on book parameters
fn optimize_fiction_creativity(
    genre: &Genre, 
    premise: &str, 
    target_pages: usize,
    content: &mut Content
) -> Result<()> {
    // Analyze if book would benefit from multi-plot structure
    let should_use_multiplot = detect_multiplot_potential(genre, premise, target_pages);
    
    if should_use_multiplot {
        println!("🎭 Enhanced: Detected complex narrative potential - enabling multi-plot storytelling");
        enable_multiplot_features(content)?;
    }
    
    // Apply genre-specific creative optimizations
    apply_genre_optimizations(genre, content)?;
    
    // Apply premise-based creative enhancements
    apply_premise_optimizations(premise, content)?;
    
    Ok(())
}

fn detect_multiplot_potential(genre: &Genre, premise: &str, target_pages: usize) -> bool {
    // Multi-plot works best for longer books (300+ pages)
    let is_long_book = target_pages >= 300;
    
    // Check genre suitability
    let genre_supports_multiplot = matches!(genre, 
        Genre::Fantasy | Genre::SciFi | Genre::Fiction | 
        Genre::Historical | Genre::Thriller | Genre::Drama
    );
    
    // Analyze premise for complexity indicators
    let premise_lower = premise.to_lowercase();
    let has_complexity_keywords = premise_lower.contains("family") ||
        premise_lower.contains("generation") ||
        premise_lower.contains("world") ||
        premise_lower.contains("kingdom") ||
        premise_lower.contains("war") ||
        premise_lower.contains("saga") ||
        premise_lower.contains("epic") ||
        premise_lower.contains("multiple") ||
        premise_lower.contains("several") ||
        premise_lower.contains("empire") ||
        premise_lower.contains("dynasty") ||
        premise_lower.contains("legacy");
        
    is_long_book && genre_supports_multiplot && has_complexity_keywords
}

fn enable_multiplot_features(content: &mut Content) -> Result<()> {
    // Create a world state for managing multiple plots
    let world_state = crate::content::WorldState {
        characters: Vec::new(),
        locations: Vec::new(),
        plot_threads: Vec::new(),
        timeline: crate::content::Timeline {
            events: Vec::new(),
            current_time_markers: std::collections::HashMap::new(),
            temporal_structure: crate::content::TemporalStructure::Chronological,
        },
        world_rules: Vec::new(),
        active_plots: Vec::new(),
        narrative_style: crate::content::NarrativeStyle::Parallel,
    };
    
    content.world_state = Some(world_state);
    
    // Update stylistic profile for multi-plot narratives
    if let Some(ref mut profile) = content.stylistic_profile {
        profile.narrative_voice.primary_pov = crate::content::PointOfViewStyle::ThirdPersonOmniscient;
        profile.pacing_profile.default_rhythm = crate::content::WritingRhythm::Syncopated; // Varied pacing for plot switching
    }
    
    Ok(())
}

fn apply_genre_optimizations(genre: &Genre, content: &mut Content) -> Result<()> {
    if let Some(ref mut profile) = content.stylistic_profile {
        match genre {
            Genre::Fantasy => {
                apply_fantasy_optimizations(profile)?;
            },
            Genre::SciFi => {
                apply_scifi_optimizations(profile)?;
            },
            Genre::Fiction => {
                apply_general_fiction_optimizations(profile)?;
            },
            Genre::Mystery | Genre::Thriller => {
                profile.tone_modulation.base_tone = crate::content::EmotionalTone::Ominous;
                profile.pacing_profile.default_rhythm = crate::content::WritingRhythm::Breathless;
                // Add tension-building techniques
                profile.dialogue_system.dialogue_tags = "tension-building".to_string();
            },
            Genre::Romance => {
                profile.tone_modulation.base_tone = crate::content::EmotionalTone::Lyrical;
                profile.dialogue_system.dialogue_tags = "emotionally expressive".to_string();
                // Focus on character relationship dynamics
                profile.sensory_profile.sensory_priorities = vec![
                    crate::content::SensoryChannel::Visual
                ];
            },
            Genre::Horror => {
                profile.tone_modulation.base_tone = crate::content::EmotionalTone::Ominous;
                profile.sensory_profile.sensory_priorities = vec![
                    crate::content::SensoryChannel::Visual,
                    crate::content::SensoryChannel::Auditory,
                    crate::content::SensoryChannel::Tactile
                ];
                profile.pacing_profile.default_rhythm = crate::content::WritingRhythm::Hypnotic;
            },
            _ => {} // Default optimizations
        }
    }
    Ok(())
}

fn apply_premise_optimizations(premise: &str, content: &mut Content) -> Result<()> {
    if let Some(ref mut profile) = content.stylistic_profile {
        let premise_lower = premise.to_lowercase();
        
        // Advanced Fiction Writing Techniques Based on Research
        
        // Detect emotional tone and character dynamics
        if premise_lower.contains("dark") || premise_lower.contains("tragedy") {
            profile.tone_modulation.base_tone = crate::content::EmotionalTone::Melancholic;
            apply_character_pressure_techniques(profile)?;
        } else if premise_lower.contains("comedy") || premise_lower.contains("humor") {
            profile.tone_modulation.base_tone = crate::content::EmotionalTone::Whimsical;
            profile.dialogue_system.dialogue_tags = "witty_character_revealing".to_string();
        } else if premise_lower.contains("epic") || premise_lower.contains("grand") {
            profile.tone_modulation.base_tone = crate::content::EmotionalTone::Lyrical;
            profile.figurative_language.figurative_density = crate::content::FigurativeDensity::Lyrical;
        }
        
        // Detect character-driven vs action-driven narratives
        if premise_lower.contains("journey") || premise_lower.contains("quest") || 
           premise_lower.contains("adventure") || premise_lower.contains("travel") {
            apply_journey_narrative_techniques(profile)?;
        }
        
        // Detect relationship dynamics
        if premise_lower.contains("family") || premise_lower.contains("relationship") ||
           premise_lower.contains("love") || premise_lower.contains("friendship") {
            apply_relationship_focus_techniques(profile)?;
        }
        
        // Detect conflict-driven narratives
        if premise_lower.contains("war") || premise_lower.contains("battle") ||
           premise_lower.contains("conflict") || premise_lower.contains("fight") {
            profile.pacing_profile.default_rhythm = crate::content::WritingRhythm::Breathless;
            profile.sensory_profile.sensory_priorities.insert(0, crate::content::SensoryChannel::Auditory);
        }
        
        // Detect philosophical or thematic complexity
        if premise_lower.contains("philosophical") || premise_lower.contains("meaning") ||
           premise_lower.contains("society") || premise_lower.contains("humanity") {
            profile.meta_narrative = Some(crate::content::MetaNarrativeElements {
                self_awareness_level: crate::content::SelfAwarenessLevel::Subtle,
                meta_commentary: vec!["thematic_exploration".to_string(), "societal_commentary".to_string()],
                structural_references: vec!["archetypal_patterns".to_string(), "symbolic_representation".to_string()],
                reader_address: "contemplative_invitation".to_string(),
            });
        }
        
        // Apply "awful things happen to characters" principle for character revelation
        if premise_lower.contains("survival") || premise_lower.contains("struggle") ||
           premise_lower.contains("challenge") || premise_lower.contains("test") {
            apply_character_pressure_techniques(profile)?;
        }
        
        // Apply academic fiction fundamentals for all premises
        apply_illusion_maintenance_techniques(profile)?;
        
        // Detect complexity level and apply appropriate reader-centric techniques
        let complexity_keywords = ["complex", "intricate", "layered", "multifaceted", "nuanced", "sophisticated"];
        let complexity_indicators = complexity_keywords.iter()
            .filter(|&keyword| premise_lower.contains(keyword))
            .count();
        
        if complexity_indicators > 0 {
            // For complex premises, emphasize clarity principles
            profile.tone_modulation.base_tone = crate::content::EmotionalTone::Contemplative;
            apply_reader_centric_optimizations(profile)?;
        }
        
        // Detect character-focused vs plot-focused premises
        let character_keywords = ["character", "person", "individual", "protagonist", "hero", "heroine", "personality", "identity"];
        let character_focus_indicators = character_keywords.iter()
            .filter(|&keyword| premise_lower.contains(keyword))
            .count();
        
        if character_focus_indicators >= 2 {
            apply_character_authenticity_techniques(profile)?;
        }
        
        // Always apply narrative momentum for fiction
        apply_narrative_momentum_techniques(profile)?;
    }
    Ok(())
}

// Apply detail economy principles (academic fundamental)
fn apply_detail_economy_techniques(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // "Economize reader's attention" - every detail must serve the story
    profile.symbolic_elements.push(crate::content::SymbolicElement {
        id: "detail_economy_symbol".to_string(),
        symbol_type: crate::content::SymbolType::MetaSymbol,
        core_meaning: "economical_use_of_detail_serving_story_purpose".to_string(),
        evolution_stages: Vec::new(),
        context_associations: Vec::new(),
        recurrence_pattern: vec![1, 5, 9, 13], // Regular attention economy checkpoints
    });
    
    // Adjust figurative language for clarity without sacrificing depth
    profile.figurative_language.figurative_density = crate::content::FigurativeDensity::Moderate;
    
    // Focus on essential sensory details
    profile.sensory_profile.sensory_priorities = vec![
        crate::content::SensoryChannel::Visual, // Most essential for scene-setting
        crate::content::SensoryChannel::Tactile  // Most immediate for immersion
    ];
    
    Ok(())
}

// Apply character development through pressure/conflict (fiction writing principle)
fn apply_character_pressure_techniques(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Add symbolic elements for character under pressure
    profile.symbolic_elements.push(crate::content::SymbolicElement {
        id: "character_pressure_symbol".to_string(),
        symbol_type: crate::content::SymbolType::ArchetypalSymbol,
        core_meaning: "character_revelation_through_adversity".to_string(),
        evolution_stages: Vec::new(),
        context_associations: Vec::new(),
        recurrence_pattern: vec![1, 3, 5], // Appear at key dramatic moments
    });
    
    // Emphasize character-revealing dialogue
    profile.dialogue_system.dialogue_tags = "pressure_revealing".to_string();
    
    Ok(())
}

// Apply quest/journey narrative structure
fn apply_journey_narrative_techniques(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Add quest structure symbolic elements
    profile.symbolic_elements.push(crate::content::SymbolicElement {
        id: "heroic_journey_symbol".to_string(),
        symbol_type: crate::content::SymbolType::ArchetypalSymbol,
        core_meaning: "character_transformation_through_quest".to_string(),
        evolution_stages: Vec::new(),
        context_associations: Vec::new(),
        recurrence_pattern: vec![1, 6, 10, 15], // Classic quest milestone points
    });
    
    // Enhanced sensory for world exploration
    profile.sensory_profile.sensory_priorities = vec![
        crate::content::SensoryChannel::Visual,
        crate::content::SensoryChannel::Tactile,
        crate::content::SensoryChannel::Olfactory
    ];
    
    Ok(())
}

// Apply relationship-focused narrative techniques  
fn apply_relationship_focus_techniques(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Emphasize emotional sensory channels
    profile.sensory_profile.sensory_priorities.insert(0, crate::content::SensoryChannel::Visual);
    
    // Character-revealing dialogue focus
    profile.dialogue_system.dialogue_tags = "relationship_dynamics_revealing".to_string();
    
    // Add relationship symbolic elements
    profile.symbolic_elements.push(crate::content::SymbolicElement {
        id: "relationship_dynamics_symbol".to_string(),
        symbol_type: crate::content::SymbolType::PersonalSymbol,
        core_meaning: "character_connection_and_growth".to_string(),
        evolution_stages: Vec::new(),
        context_associations: Vec::new(),
        recurrence_pattern: vec![2, 5, 8, 12], // Relationship milestone moments
    });
    
    Ok(())
}

// Advanced Fiction Writing Fundamentals (based on academic research)
fn apply_illusion_maintenance_techniques(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Core principle: Create and maintain reader immersion
    profile.genre_overlays.push(crate::content::GenreOverlay {
        genre_type: crate::content::GenreType::ContemporaryLiterary,
        intensity: 0.95,
        stylistic_markers: vec![
            crate::content::StylisticMarker {
                marker_type: "immersion_maintenance".to_string(),
                implementation: "Maintain consistent fictional reality without breaks that remind readers of artifice".to_string(),
                frequency: 1.0,
            },
            crate::content::StylisticMarker {
                marker_type: "clarity_over_complexity".to_string(),
                implementation: "Prioritize clarity and reader comprehension over stylistic complexity".to_string(),
                frequency: 0.9,
            },
            crate::content::StylisticMarker {
                marker_type: "reader_attention_economy".to_string(),
                implementation: "Economize reader's attention - every detail must serve the story".to_string(),
                frequency: 0.8,
            }
        ],
        lexical_preferences: vec!["clarity_focused".to_string(), "immersion_supporting".to_string(), "economical_detail".to_string()],
        structural_influences: vec!["consistent_momentum".to_string(), "reader_transportation".to_string(), "illusion_maintenance".to_string()],
    });
    
    // Adjust tone for transportive experience
    profile.tone_modulation.base_tone = crate::content::EmotionalTone::Contemplative;
    
    Ok(())
}

fn apply_reader_centric_optimizations(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Focus on reader's psychological response and engagement
    profile.dialogue_system.dialogue_tags = "reader_engagement_optimized".to_string();
    
    // Add symbolic elements for reader connection
    profile.symbolic_elements.push(crate::content::SymbolicElement {
        id: "reader_connection_symbol".to_string(),
        symbol_type: crate::content::SymbolType::PersonalSymbol,
        core_meaning: "elements_that_facilitate_reader_identification_and_emotional_engagement".to_string(),
        evolution_stages: Vec::new(),
        context_associations: Vec::new(),
        recurrence_pattern: vec![1, 4, 7, 10], // Strategic placement for sustained engagement
    });
    
    // Optimize sensory profile for immersion
    profile.sensory_profile.sensory_priorities = vec![
        crate::content::SensoryChannel::Visual,
        crate::content::SensoryChannel::Tactile,
        crate::content::SensoryChannel::Auditory
    ];
    
    Ok(())
}

fn apply_character_authenticity_techniques(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Create multidimensional, authentic characters readers can connect with
    profile.symbolic_elements.push(crate::content::SymbolicElement {
        id: "character_authenticity_symbol".to_string(),
        symbol_type: crate::content::SymbolType::ArchetypalSymbol,
        core_meaning: "multidimensional_character_development_avoiding_stereotypes".to_string(),
        evolution_stages: Vec::new(),
        context_associations: Vec::new(),
        recurrence_pattern: vec![2, 6, 10, 14], // Character development milestones
    });
    
    // Enhance dialogue for authentic character voice
    profile.dialogue_system.dialogue_tags = "authentic_character_voice".to_string();
    
    // Meta-narrative awareness for character depth
    if profile.meta_narrative.is_none() {
        profile.meta_narrative = Some(crate::content::MetaNarrativeElements {
            self_awareness_level: crate::content::SelfAwarenessLevel::Subtle,
            meta_commentary: vec!["character_depth_techniques".to_string(), "authenticity_markers".to_string()],
            structural_references: vec!["character_arc_consistency".to_string(), "emotional_believability".to_string()],
            reader_address: "empathetic_connection".to_string(),
        });
    }
    
    Ok(())
}

fn apply_narrative_momentum_techniques(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Maintain consistent narrative momentum without breaks in story flow
    profile.pacing_profile.default_rhythm = crate::content::WritingRhythm::Flowing;
    
    // Add structural elements for momentum maintenance
    profile.symbolic_elements.push(crate::content::SymbolicElement {
        id: "narrative_momentum_symbol".to_string(),
        symbol_type: crate::content::SymbolType::MetaSymbol,
        core_meaning: "consistent_story_progression_without_momentum_breaks".to_string(),
        evolution_stages: Vec::new(),
        context_associations: Vec::new(),
        recurrence_pattern: vec![3, 6, 9, 12], // Regular momentum checkpoints
    });
    
    Ok(())
}

// Advanced Fantasy Literature Optimizations
fn apply_fantasy_optimizations(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Apply fundamental fiction writing techniques first
    apply_illusion_maintenance_techniques(profile)?;
    apply_reader_centric_optimizations(profile)?;
    apply_character_authenticity_techniques(profile)?;
    apply_narrative_momentum_techniques(profile)?;
    
    // World-building focus with rich figurative language
    profile.figurative_language.figurative_density = crate::content::FigurativeDensity::Lyrical;
    
    // Multi-sensory world immersion
    profile.sensory_profile.sensory_priorities = vec![
        crate::content::SensoryChannel::Visual,
        crate::content::SensoryChannel::Tactile,
        crate::content::SensoryChannel::Olfactory,
        crate::content::SensoryChannel::Gustatory
    ];
    
    // Archetypal character development and mythological depth
    profile.genre_overlays.push(crate::content::GenreOverlay {
        genre_type: crate::content::GenreType::FantasyEpic,
        intensity: 0.9,
        stylistic_markers: vec![
            crate::content::StylisticMarker {
                marker_type: "archetypal_characters".to_string(),
                implementation: "Use archetypal character patterns and mythological depth".to_string(),
                frequency: 0.8,
            },
            crate::content::StylisticMarker {
                marker_type: "world_building_focus".to_string(),
                implementation: "Emphasize detailed world-building and immersive descriptions".to_string(),
                frequency: 0.9,
            }
        ],
        lexical_preferences: vec!["evocative_descriptive".to_string(), "avoid_archaic".to_string(), "immersive_language".to_string()],
        structural_influences: vec!["quest_structure".to_string(), "character_revelation_through_choice".to_string(), "magical_system_consistency".to_string()],
    });
    
    // Contemplative tone for deeper themes
    profile.tone_modulation.base_tone = crate::content::EmotionalTone::Contemplative;
    
    // Enable meta-narrative for symbolic representation
    profile.meta_narrative = Some(crate::content::MetaNarrativeElements {
        self_awareness_level: crate::content::SelfAwarenessLevel::Subtle,
        meta_commentary: vec!["societal_metaphors".to_string(), "archetypal_symbolism".to_string()],
        structural_references: vec!["mythological_patterns".to_string(), "folkloric_influence".to_string()],
        reader_address: "immersive_invitation".to_string(),
    });
    
    Ok(())
}

// Science Fiction Optimizations
fn apply_scifi_optimizations(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    profile.figurative_language.figurative_density = crate::content::FigurativeDensity::Rich;
    profile.sensory_profile.sensory_priorities = vec![
        crate::content::SensoryChannel::Visual,
        crate::content::SensoryChannel::Auditory
    ];
    
    profile.genre_overlays.push(crate::content::GenreOverlay {
        genre_type: crate::content::GenreType::SciFiHard,
        intensity: 0.8,
        stylistic_markers: vec![
            crate::content::StylisticMarker {
                marker_type: "technical_precision".to_string(),
                implementation: "Use precise technical language and scientific plausibility".to_string(),
                frequency: 0.7,
            },
            crate::content::StylisticMarker {
                marker_type: "speculative_elements".to_string(),
                implementation: "Integrate speculative concepts and future implications".to_string(),
                frequency: 0.8,
            }
        ],
        lexical_preferences: vec!["scientific_plausibility".to_string(), "future_concepts".to_string()],
        structural_influences: vec!["cause_effect_logic".to_string(), "technological_impact".to_string()],
    });
    
    profile.tone_modulation.base_tone = crate::content::EmotionalTone::Clinical;
    Ok(())
}

// General Fiction Optimizations Based on Research
fn apply_general_fiction_optimizations(profile: &mut crate::content::StylisticProfile) -> Result<()> {
    // Apply fundamental fiction writing techniques
    apply_illusion_maintenance_techniques(profile)?;
    apply_reader_centric_optimizations(profile)?;
    apply_character_authenticity_techniques(profile)?;
    apply_narrative_momentum_techniques(profile)?;
    
    // Focus on character-driven narrative
    profile.dialogue_system.dialogue_tags = "character_revealing_and_authentic".to_string();
    
    // Balanced sensory approach optimized for immersion
    profile.sensory_profile.sensory_priorities = vec![
        crate::content::SensoryChannel::Visual,
        crate::content::SensoryChannel::Tactile,
        crate::content::SensoryChannel::Auditory
    ];
    
    // Contemporary literary techniques enhanced with academic principles
    profile.genre_overlays.push(crate::content::GenreOverlay {
        genre_type: crate::content::GenreType::ContemporaryLiterary,
        intensity: 0.9, // Increased intensity for better optimization
        stylistic_markers: vec![
            crate::content::StylisticMarker {
                marker_type: "character_focus".to_string(),
                implementation: "Every sentence reveals character or advances action without breaking immersion".to_string(),
                frequency: 0.95,
            },
            crate::content::StylisticMarker {
                marker_type: "choice_revelation".to_string(),
                implementation: "Characters reveal themselves through authentic choices under pressure".to_string(),
                frequency: 0.9,
            },
            crate::content::StylisticMarker {
                marker_type: "technique_as_means".to_string(),
                implementation: "Technical skill serves storytelling without constraining creativity".to_string(),
                frequency: 0.8,
            }
        ],
        lexical_preferences: vec![
            "clear_understanding".to_string(), 
            "reader_engagement".to_string(),
            "economical_detail".to_string(),
            "immersion_supporting".to_string()
        ],
        structural_influences: vec![
            "scene_sequel_structure".to_string(), 
            "stimulus_response".to_string(), 
            "dramatic_tension".to_string(),
            "consistent_momentum".to_string(),
            "illusion_maintenance".to_string()
        ],
    });
    
    // Flowing rhythm for narrative momentum (academic principle)
    profile.pacing_profile.default_rhythm = crate::content::WritingRhythm::Flowing;
    
    Ok(())
}

async fn interactive_multi_plot_book_mode() -> Result<()> {
    println!("\n🎭 Multi-Plot Book Creation");
    println!("═══════════════════════════");
    println!("Create complex narratives with multiple storylines, characters, and interconnected plots");
    println!();

    let mode_options = vec![
        "🆕 Create new multi-plot saga",
        "📂 Continue existing multi-plot book",
        "🔧 Manage existing world/plots",
        "← Back to book creation",
    ];

    let mode_choice = Select::new()
        .with_prompt("What would you like to do?")
        .items(&mode_options)
        .default(0)
        .interact()?;

    match mode_choice {
        0 => create_new_multiplot_saga().await,
        1 => continue_multiplot_book().await,
        2 => manage_multiplot_world().await,
        _ => Ok(()),
    }
}

async fn create_new_multiplot_saga() -> Result<()> {
    println!("\n🎭 Creating New Multi-Plot Saga");
    println!("═══════════════════════════════");
    println!();

    // Basic book information
    let title: String = Input::new()
        .with_prompt("Title of your saga/series")
        .interact_text()?;

    let author: String = Input::new()
        .with_prompt("Author name")
        .default("Anonymous".to_string())
        .interact_text()?;

    let genre: String = Input::new()
        .with_prompt("Primary genre (Fantasy, Sci-Fi, Historical Fiction, etc.)")
        .default("Fantasy".to_string())
        .interact_text()?;

    let writing_style: String = Input::new()
        .with_prompt("Writing style (Epic, Literary, Adventure, etc.)")
        .default("Epic".to_string())
        .interact_text()?;

    let premise: String = Input::new()
        .with_prompt("Overall premise/world concept for your saga")
        .interact_text()?;

    // Choose narrative structure
    let narrative_styles = vec![
        "Parallel - Multiple simultaneous storylines (Game of Thrones style)",
        "Interwoven - Plots that regularly intersect (Lord of the Rings style)",
        "Episodic - Separate but connected episodes (Chronicles/anthology style)",
        "Cyclical - Storylines that repeat themes across generations",
        "Experimental - Non-traditional narrative structures",
    ];

    let narrative_idx = Select::new()
        .with_prompt("Choose your narrative structure")
        .items(&narrative_styles)
        .default(0)
        .interact()?;

    let narrative_style = match narrative_idx {
        0 => NarrativeStyle::Parallel,
        1 => NarrativeStyle::Interwoven,
        2 => NarrativeStyle::Episodic,
        3 => NarrativeStyle::Cyclical,
        _ => NarrativeStyle::Experimental,
    };

    // Set up multiple plot threads
    let num_plots: String = Input::new()
        .with_prompt("How many main plot threads? (2-6 recommended)")
        .default("3".to_string())
        .interact_text()?;

    let num_plots_parsed: usize = num_plots.parse().unwrap_or(3).max(2).min(8);

    let mut plot_threads = Vec::new();
    let mut all_characters = Vec::new();
    let mut all_locations = Vec::new();

    println!("\nNow let's set up your {} plot threads:", num_plots_parsed);

    for i in 1..=num_plots_parsed {
        let plot = create_plot_thread(i, &genre).await?;
        
        // Collect characters and locations from this plot
        for char_name in &plot.main_characters {
            if !all_characters.iter().any(|c: &MultiPlotCharacter| c.name == *char_name) {
                let character = create_character_for_plot(char_name, &plot.id)?;
                all_characters.push(character);
            }
        }

        for loc_name in &plot.key_locations {
            if !all_locations.iter().any(|l: &Location| l.name == *loc_name) {
                let location = create_location_for_plot(loc_name, &plot.id)?;
                all_locations.push(location);
            }
        }

        plot_threads.push(plot);
    }

    // Create world state
    let world_state = WorldState {
        characters: all_characters,
        locations: all_locations,
        plot_threads: plot_threads.clone(),
        timeline: create_initial_timeline(&plot_threads)?,
        world_rules: create_basic_world_rules(&genre)?,
        active_plots: plot_threads.iter().map(|p| p.id.clone()).collect(),
        narrative_style,
    };

    // Create the book content
    let target_chapters: String = Input::new()
        .with_prompt("Target number of chapters (will be distributed across plots)")
        .default("20".to_string())
        .interact_text()?;

    let target_chapters_num: usize = target_chapters.parse().unwrap_or(20);

    let mut content = Content::new_book(
        title.clone(),
        author,
        genre,
        writing_style,
        premise,
        "Large".to_string(),
        Some(target_chapters_num * 3000), // ~3000 words per chapter
        target_chapters_num,
        "llama3.2".to_string(),
    );

    content.world_state = Some(world_state);
    content.outline = generate_multiplot_outline(&content).await?;

    // Model selection and confirmation
    let (use_ollama, model) = interactive_model_selection("Multi-plot saga", "Large book with complex narrative")?;
    
    if !use_ollama {
        println!("❌ Multi-plot generation currently requires Ollama. Please set up Ollama and try again.");
        return Ok(());
    }

    if !Confirm::new()
        .with_prompt(&format!("Ready to start generating your multi-plot saga with {}?", model))
        .interact()? {
        println!("📚 Multi-plot creation cancelled.");
        return Ok(());
    }

    println!("\n🎭 Starting multi-plot saga generation...");
    println!("This will create chapters alternating between different plot threads");

    // Generate chapters using multi-plot logic
    generate_multiplot_chapters(&mut content, &model).await?;

    // Save the content
    let output_filename = format!("{}_multiplot.json", title.replace(" ", "_").to_lowercase());
    let output_path = std::path::PathBuf::from(&output_filename);
    
    let content_json = serde_json::to_string_pretty(&content)?;
    std::fs::write(&output_path, content_json)?;

    println!("\n✅ Multi-plot saga generation completed!");
    println!("💾 Saved to: {}", output_path.display());
    println!("📊 Total chapters: {}", content.sections.len());
    println!("📈 Total word count: {}", content.sections.iter().map(|s| s.word_count).sum::<usize>());

    Ok(())
}

async fn create_plot_thread(plot_number: usize, genre: &str) -> Result<PlotThread> {
    println!("\n--- Plot Thread {} ---", plot_number);

    let name: String = Input::new()
        .with_prompt("Plot thread name (e.g., 'The War in the North', 'Sarah's Journey')")
        .interact_text()?;

    let description: String = Input::new()
        .with_prompt("Brief description of this plot thread")
        .interact_text()?;

    let plot_types = vec![
        "MainStory - Primary narrative arc",
        "Subplot - Secondary storyline", 
        "Backstory - Historical context/flashbacks",
        "Parallel - Simultaneous alternate storyline",
        "Framing - Story-within-a-story setup",
    ];

    let plot_type_idx = Select::new()
        .with_prompt("What type of plot thread is this?")
        .items(&plot_types)
        .default(if plot_number == 1 { 0 } else { 1 })
        .interact()?;

    let plot_type = match plot_type_idx {
        0 => PlotType::MainStory,
        1 => PlotType::Subplot,
        2 => PlotType::Backstory,
        3 => PlotType::Parallel,
        _ => PlotType::Framing,
    };

    let theme: String = Input::new()
        .with_prompt("Central theme of this plot (love, power, redemption, etc.)")
        .default("adventure".to_string())
        .interact_text()?;

    // Get main characters for this plot
    let char_input: String = Input::new()
        .with_prompt("Main characters for this plot (comma-separated names)")
        .interact_text()?;
    
    let main_characters: Vec<String> = char_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Get key locations
    let loc_input: String = Input::new()
        .with_prompt("Key locations for this plot (comma-separated)")
        .interact_text()?;
    
    let key_locations: Vec<String> = loc_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let time_start: String = Input::new()
        .with_prompt("When does this plot begin? (e.g., 'Year 1', 'Ancient times', 'Modern day')")
        .default("Beginning".to_string())
        .interact_text()?;

    Ok(PlotThread {
        id: format!("plot_{}", plot_number),
        name,
        description,
        plot_type,
        status: PlotStatus::Planning,
        main_characters,
        key_locations,
        timeline_span: TimeSpan {
            start_marker: time_start,
            end_marker: None,
            duration_description: "Ongoing".to_string(),
        },
        theme,
        current_stage: PlotStage::Introduction,
        intersections: Vec::new(), // Will be filled later when analyzing plot connections
    })
}

fn create_character_for_plot(name: &str, plot_id: &str) -> Result<MultiPlotCharacter> {
    Ok(MultiPlotCharacter {
        id: format!("char_{}", name.to_lowercase().replace(" ", "_")),
        name: name.to_string(),
        description: format!("Character in {}", plot_id),
        role: CharacterRole::Protagonist, // Default, can be updated later
        affiliations: vec![plot_id.to_string()],
        current_status: CharacterStatus::Active,
        timeline_appearances: Vec::new(),
        character_arc: None,
        relationships: Vec::new(),
    })
}

fn create_location_for_plot(name: &str, plot_id: &str) -> Result<Location> {
    Ok(Location {
        id: format!("loc_{}", name.to_lowercase().replace(" ", "_")),
        name: name.to_string(),
        description: format!("Location featured in {}", plot_id),
        location_type: LocationType::City, // Default
        time_period: None,
        connected_locations: Vec::new(),
        significance: vec![plot_id.to_string()],
        current_state: "Active".to_string(),
    })
}

fn create_initial_timeline(plot_threads: &[PlotThread]) -> Result<Timeline> {
    let mut events = Vec::new();
    
    for plot in plot_threads {
        let event = TimelineEvent {
            id: format!("event_start_{}", plot.id),
            time_marker: plot.timeline_span.start_marker.clone(),
            event_type: EventType::Plot,
            description: format!("Beginning of {}", plot.name),
            affected_plots: vec![plot.id.clone()],
            affected_characters: plot.main_characters.clone(),
            location: plot.key_locations.first().cloned().unwrap_or_default(),
            significance: EventSignificance::Major,
        };
        events.push(event);
    }

    let mut current_markers = std::collections::HashMap::new();
    for plot in plot_threads {
        current_markers.insert(plot.id.clone(), plot.timeline_span.start_marker.clone());
    }

    Ok(Timeline {
        events,
        current_time_markers: current_markers,
        temporal_structure: TemporalStructure::Parallel,
    })
}

fn create_basic_world_rules(genre: &str) -> Result<Vec<WorldRule>> {
    let mut rules = Vec::new();

    match genre.to_lowercase().as_str() {
        "fantasy" => {
            rules.push(WorldRule {
                id: "magic_system".to_string(),
                rule_type: RuleType::Magic,
                description: "Magic system rules and limitations".to_string(),
                scope: RuleScope::Global,
                consistency_notes: vec!["Define magic costs, limitations, and consequences".to_string()],
            });
        },
        "sci-fi" | "science fiction" => {
            rules.push(WorldRule {
                id: "tech_level".to_string(),
                rule_type: RuleType::Physics,
                description: "Technology level and scientific constraints".to_string(),
                scope: RuleScope::Global,
                consistency_notes: vec!["Maintain consistent tech capabilities across storylines".to_string()],
            });
        },
        _ => {
            rules.push(WorldRule {
                id: "social_structure".to_string(),
                rule_type: RuleType::Social,
                description: "Social and political structures".to_string(),
                scope: RuleScope::Global,
                consistency_notes: vec!["Keep social dynamics consistent across plots".to_string()],
            });
        }
    }

    Ok(rules)
}

async fn generate_multiplot_outline(content: &Content) -> Result<String> {
    if let Some(ref world_state) = content.world_state {
        let mut outline = format!("# {} - Multi-Plot Outline\n\n", content.title);
        outline.push_str("## World Overview\n");
        outline.push_str(&format!("Premise: {}\n", content.premise));
        outline.push_str(&format!("Narrative Style: {:?}\n\n", world_state.narrative_style));
        
        outline.push_str("## Plot Threads\n");
        for plot in &world_state.plot_threads {
            outline.push_str(&format!("### {} ({:?})\n", plot.name, plot.plot_type));
            outline.push_str(&format!("- Theme: {}\n", plot.theme));
            outline.push_str(&format!("- Characters: {}\n", plot.main_characters.join(", ")));
            outline.push_str(&format!("- Locations: {}\n", plot.key_locations.join(", ")));
            outline.push_str(&format!("- Description: {}\n\n", plot.description));
        }
        
        Ok(outline)
    } else {
        Ok("Standard book outline".to_string())
    }
}

async fn generate_multiplot_chapters(content: &mut Content, model: &str) -> Result<()> {
    if let Some(ref world_state) = content.world_state {
        let total_chapters = content.metadata.target_sections;
        let active_plots = &world_state.active_plots;
        
        println!("📚 Generating {} chapters across {} plot threads", total_chapters, active_plots.len());
        
        let progress_bar = ProgressBar::new(total_chapters as u64);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} chapters ({eta})")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("#>-"));
        
        for chapter_num in 1..=total_chapters {
            // Determine which plot thread this chapter should focus on
            let plot_index = (chapter_num - 1) % active_plots.len();
            let current_plot_id = &active_plots[plot_index];
            
            // Find the plot thread
            let current_plot = world_state.plot_threads
                .iter()
                .find(|p| p.id == *current_plot_id)
                .expect("Plot ID should exist in multi-plot system");
            
            progress_bar.set_message(format!("Writing Chapter {}: {} storyline", chapter_num, current_plot.name));
            
            // Generate chapter with plot context
            let chapter_content = generate_multiplot_chapter(
                content, 
                chapter_num, 
                current_plot, 
                world_state, 
                model
            ).await?;
            
            // Create narrative context
            let narrative_context = NarrativeContext {
                plot_thread_id: current_plot.id.clone(),
                point_of_view: PointOfView::ThirdPersonLimited(
                    current_plot.main_characters.first().cloned().unwrap_or_default()
                ),
                time_marker: world_state.timeline.current_time_markers
                    .get(&current_plot.id)
                    .cloned()
                    .unwrap_or_default(),
                primary_location: current_plot.key_locations.first().cloned().unwrap_or_default(),
                active_characters: current_plot.main_characters.clone(),
                narrative_tension: TensionLevel::Building,
                context_notes: vec![format!("Chapter focuses on {}", current_plot.name)],
            };
            
            // Create section with plot thread assignment
            let section = Section {
                number: chapter_num,
                title: format!("Chapter {}: {}", chapter_num, 
                    generate_chapter_title(current_plot, chapter_num)?),
                content: chapter_content,
                word_count: 0, // Will be calculated
                outline: format!("Chapter focusing on {} plot thread", current_plot.name),
                section_type: SectionType::Chapter,
                created_at: Utc::now(),
                completed: true,
                plot_thread: Some(current_plot.id.clone()),
                narrative_context: Some(narrative_context),
            };
            
            let word_count = count_words(&section.content);
            let mut section = section;
            section.word_count = word_count;
            
            content.sections.push(section);
            content.metadata.current_word_count += word_count;
            
            progress_bar.inc(1);
        }
        
        progress_bar.finish_with_message("Multi-plot saga generation completed!");
    }
    
    Ok(())
}

async fn generate_multiplot_chapter(
    content: &Content,
    chapter_num: usize,
    current_plot: &PlotThread,
    world_state: &WorldState,
    model: &str
) -> Result<String> {
    let ollama_client = OllamaClient::new("http://localhost:11434".to_string())?;
    
    // Build context from world state and other plots
    let plot_context = build_multiplot_context(content, current_plot, world_state, chapter_num)?;
    
    let prompt = format!(
        "You are writing Chapter {} of a multi-plot saga titled '{}'. 
        
        PLOT FOCUS: This chapter focuses on the '{}' storyline.
        Plot Type: {:?}
        Theme: {}
        
        CHARACTERS IN THIS CHAPTER: {}
        LOCATIONS: {}
        CURRENT STAGE: {:?}
        
        WORLD CONTEXT:
        {}
        
        WRITING INSTRUCTIONS:
        - Write approximately 2000-3000 words
        - Focus primarily on the {} plot thread
        - Maintain consistency with the established world and other storylines
        - Include references or hints to other ongoing plots when natural
        - Advance this specific plot thread meaningfully
        - Use proper chapter formatting with scene breaks if needed
        
        Begin Chapter {}:",
        
        chapter_num,
        content.title,
        current_plot.name,
        current_plot.plot_type,
        current_plot.theme,
        current_plot.main_characters.join(", "),
        current_plot.key_locations.join(", "),
        current_plot.current_stage,
        plot_context,
        current_plot.name,
        chapter_num
    );
    
    ollama_client.generate_text(model, &prompt, 2500, 0.7).await
}

fn build_multiplot_context(
    content: &Content,
    current_plot: &PlotThread,
    world_state: &WorldState,
    chapter_num: usize
) -> Result<String> {
    let mut context = String::new();
    
    // Add premise and world info
    context.push_str(&format!("Overall Story: {}\n", content.premise));
    context.push_str(&format!("Genre: {}\n", content.genre));
    context.push_str(&format!("Narrative Style: {:?}\n\n", world_state.narrative_style));
    
    // Add info about other active plots
    context.push_str("OTHER ONGOING STORYLINES:\n");
    for plot in &world_state.plot_threads {
        if plot.id != current_plot.id {
            context.push_str(&format!("- {}: {} (Characters: {})\n", 
                plot.name, 
                plot.description, 
                plot.main_characters.join(", ")
            ));
        }
    }
    context.push('\n');
    
    // Add recent events from timeline
    context.push_str("RECENT EVENTS:\n");
    for event in world_state.timeline.events.iter().take(5) {
        if event.affected_plots.contains(&current_plot.id) {
            context.push_str(&format!("- {}: {}\n", event.time_marker, event.description));
        }
    }
    context.push('\n');
    
    // Add character relationships relevant to this plot
    context.push_str("CHARACTER RELATIONSHIPS:\n");
    for character in &world_state.characters {
        if current_plot.main_characters.contains(&character.name) {
            for relationship in &character.relationships {
                context.push_str(&format!("- {} has {} relationship with {}\n",
                    character.name,
                    format!("{:?}", relationship.relationship_type).to_lowercase(),
                    relationship.other_character_id
                ));
            }
        }
    }
    
    // Add previous chapters context if any
    if chapter_num > 1 {
        context.push_str("\nPREVIOUS CHAPTER SUMMARIES:\n");
        let recent_chapters = content.sections.iter()
            .rev()
            .take(3)
            .collect::<Vec<_>>();
        
        for section in recent_chapters.iter().rev() {
            if let Some(ref plot_thread) = section.plot_thread {
                let plot_name = world_state.plot_threads
                    .iter()
                    .find(|p| p.id == *plot_thread)
                    .map(|p| &p.name)
                    .unwrap_or(plot_thread);
                
                context.push_str(&format!("- Chapter {}: {} storyline\n", 
                    section.number, plot_name));
            }
        }
    }
    
    Ok(context)
}

fn generate_chapter_title(plot: &PlotThread, chapter_num: usize) -> Result<String> {
    let title = match plot.plot_type {
        PlotType::MainStory => format!("{} - Part {}", plot.theme.to_title_case(), 
            ((chapter_num - 1) / 3) + 1),
        PlotType::Subplot => format!("{} Unfolds", plot.name),
        PlotType::Backstory => format!("Echoes of {}", plot.theme.to_title_case()),
        PlotType::Parallel => format!("Meanwhile: {}", plot.name),
        _ => format!("{} Continues", plot.name),
    };
    Ok(title)
}

// Placeholder functions for continuation and management
async fn continue_multiplot_book() -> Result<()> {
    println!("🚧 Multi-plot continuation feature coming soon!");
    println!("This will allow you to load existing multi-plot books and continue their storylines.");
    Ok(())
}

async fn manage_multiplot_world() -> Result<()> {
    println!("🚧 World management feature coming soon!");
    println!("This will allow you to edit characters, locations, plot threads, and world rules.");
    Ok(())
}

// Helper trait for title case conversion
trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for str {
    fn to_title_case(&self) -> String {
        self.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// Generate opening variation guidance to prevent repetitive chapter beginnings
fn generate_opening_variation_guidance(content: &Content, current_section: usize, section_type: &SectionType) -> String {
    if content.sections.is_empty() {
        return String::new();
    }
    
    // Analyze previous openings to detect patterns
    let previous_openings = extract_previous_openings(content, current_section, 5); // Look at last 5 sections
    let detected_patterns = analyze_opening_patterns(&previous_openings);
    
    if detected_patterns.is_empty() {
        return generate_general_opening_variety_guidance(section_type);
    }
    
    // Generate specific guidance based on detected patterns
    let mut guidance = vec![];
    guidance.push("🎨 OPENING VARIATION REQUIREMENTS:".to_string());
    
    // Add pattern-specific avoidance instructions
    for pattern in &detected_patterns {
        match pattern.pattern_type {
            OpeningPatternType::RepetitivePhrase => {
                guidance.push(format!("- AVOID starting with '{}' - this phrase was used {} times recently", 
                    pattern.phrase, pattern.count));
                guidance.push(format!("- NEVER begin with variations of '{}' or similar temporal/descriptive phrases", 
                    pattern.phrase));
            },
            OpeningPatternType::SimilarStructure => {
                guidance.push(format!("- AVOID the sentence structure: {} (used {} times recently)", 
                    pattern.structure_description, pattern.count));
            },
            OpeningPatternType::SameTense => {
                guidance.push(format!("- Change narrative tense - avoid starting with {} tense (overused recently)", 
                    pattern.tense));
            }
        }
    }
    
    // Add creative alternative suggestions
    guidance.push("".to_string());
    guidance.push("✨ CREATIVE OPENING ALTERNATIVES:".to_string());
    
    let opening_styles = get_varied_opening_styles(content, section_type);
    for (i, style) in opening_styles.iter().enumerate().take(3) {
        guidance.push(format!("{}. {}", i + 1, style));
    }
    
    guidance.join("\n")
}

// Extract the first 1-3 sentences from previous sections to analyze patterns
fn extract_previous_openings(content: &Content, current_section: usize, look_back: usize) -> Vec<String> {
    let mut openings = Vec::new();
    
    let start_idx = if current_section > look_back { current_section - look_back } else { 1 };
    
    for section in &content.sections {
        if section.number >= start_idx && section.number < current_section {
            if !section.content.trim().is_empty() {
                // Extract first 2 sentences or first 150 characters, whichever is shorter
                let first_part = extract_opening_sentences(&section.content, 2);
                let opening = if first_part.chars().count() > 150 {
                    let truncated: String = first_part.chars().take(150).collect();
                    format!("{}...", truncated)
                } else {
                    first_part
                };
                
                if !opening.trim().is_empty() {
                    openings.push(opening);
                }
            }
        }
    }
    
    openings
}

// Extract first N sentences from text
fn extract_opening_sentences(text: &str, max_sentences: usize) -> String {
    let sentences: Vec<&str> = text.split(&['.', '!', '?'][..])
        .filter(|s| !s.trim().is_empty())
        .collect();
    
    let selected = sentences.into_iter()
        .take(max_sentences)
        .collect::<Vec<_>>()
        .join(". ");
    
    if selected.ends_with('.') { selected } else { format!("{}.", selected) }
}

#[derive(Debug, Clone)]
enum OpeningPatternType {
    RepetitivePhrase,
    SimilarStructure,
    SameTense,
}

#[derive(Debug, Clone)]
struct DetectedPattern {
    pattern_type: OpeningPatternType,
    phrase: String,
    structure_description: String,
    tense: String,
    count: usize,
}

// Analyze openings for repetitive patterns
fn analyze_opening_patterns(openings: &[String]) -> Vec<DetectedPattern> {
    let mut patterns = Vec::new();
    
    // Check for repetitive opening phrases (first 2-3 words)
    let mut phrase_counts = std::collections::HashMap::new();
    let mut tense_counts = std::collections::HashMap::new();
    
    for opening in openings {
        if let Some(words) = extract_opening_phrase(opening, 3) {
            if !words.is_empty() {
                *phrase_counts.entry(words.clone()).or_insert(0) += 1;
                
                // Simple tense detection
                let tense = detect_opening_tense(&words);
                *tense_counts.entry(tense.clone()).or_insert(0) += 1;
            }
        }
    }
    
    // Report phrases used more than once
    for (phrase, count) in phrase_counts {
        if count > 1 {
            patterns.push(DetectedPattern {
                pattern_type: OpeningPatternType::RepetitivePhrase,
                phrase: phrase.clone(),
                structure_description: String::new(),
                tense: String::new(),
                count,
            });
        }
    }
    
    // Report overused tenses
    for (tense, count) in tense_counts {
        if count > 2 && !tense.is_empty() {
            patterns.push(DetectedPattern {
                pattern_type: OpeningPatternType::SameTense,
                phrase: String::new(),
                structure_description: String::new(),
                tense,
                count,
            });
        }
    }
    
    patterns
}

// Extract first N words as opening phrase
fn extract_opening_phrase(text: &str, word_count: usize) -> Option<String> {
    let words: Vec<&str> = text.split_whitespace().take(word_count).collect();
    if words.len() >= 2 {
        Some(words.join(" "))
    } else {
        None
    }
}

// Basic tense detection for opening phrases
fn detect_opening_tense(phrase: &str) -> String {
    let lower = phrase.to_lowercase();
    
    if lower.starts_with("as the") || lower.starts_with("when the") || lower.starts_with("while the") {
        "temporal-descriptive".to_string()
    } else if lower.contains("was ") || lower.contains("were ") || lower.contains("had ") {
        "past".to_string()
    } else if lower.contains("is ") || lower.contains("are ") {
        "present".to_string()
    } else if lower.starts_with("the ") && lower.contains("ed ") {
        "past-descriptive".to_string()
    } else {
        String::new()
    }
}

// Generate varied opening style suggestions based on content type and previous usage
fn get_varied_opening_styles(_content: &Content, section_type: &SectionType) -> Vec<String> {
    let mut styles = Vec::new();
    
    match section_type {
        SectionType::Chapter => {
            styles.extend(vec![
                "Start with dialogue - let characters speak first to immediately engage readers".to_string(),
                "Begin with action or movement - someone doing something specific and purposeful".to_string(), 
                "Open with a concrete sensory detail - what someone sees, hears, feels, or smells".to_string(),
                "Start mid-scene with tension or conflict already in motion".to_string(),
                "Begin with an internal thought or realization from a character".to_string(),
                "Open with a specific object, document, or discovery that drives the plot".to_string(),
                "Start with weather or environment only if it directly affects the action".to_string(),
                "Begin with a character making a decision or taking a specific action".to_string(),
            ]);
        },
        SectionType::Scene => {
            styles.extend(vec![
                "Start with visual action - what the camera sees first".to_string(),
                "Begin with dialogue that reveals character or conflict".to_string(),
                "Open with a close-up on a significant object or detail".to_string(),
                "Start with movement - characters entering, leaving, or in motion".to_string(),
                "Begin with sound or lack of sound to set the mood".to_string(),
            ]);
        },
        _ => {
            styles.extend(vec![
                "Begin with a specific fact, example, or concrete detail".to_string(),
                "Start with a question or problem that needs solving".to_string(),
                "Open with a quote, statement, or declaration".to_string(),
                "Begin with a comparison or contrast".to_string(),
                "Start with a direct statement about the main topic".to_string(),
            ]);
        }
    }
    
    // Add some universal creative alternatives
    styles.extend(vec![
        "Try starting in the middle of a conversation or action".to_string(),
        "Begin with a character's internal conflict or dilemma".to_string(), 
        "Open with something unexpected or surprising".to_string(),
        "Start with a specific moment in time rather than general description".to_string(),
    ]);
    
    styles
}

// Generate general variety guidance when no specific patterns are detected
fn generate_general_opening_variety_guidance(section_type: &SectionType) -> String {
    format!(
        "🎨 OPENING VARIETY GUIDANCE:\n\
        - Vary your opening style from previous {}s\n\
        - Avoid repetitive temporal phrases like 'As the...', 'When the...', 'While the...'\n\
        - Start with action, dialogue, or specific details rather than general description\n\
        - Make each opening feel fresh and distinct from previous sections",
        section_type_name(section_type).to_lowercase()
    )
}

// Freeform Writing Mode - New feature requested
async fn interactive_freeform_writing() -> Result<()> {
    println!("{}", console::style("✨ Freeform Writing Mode").bold().magenta());
    println!("═══════════════════════════════════════");
    println!();
    println!("Describe what you want to create in your own words.");
    println!("You can include files for context, reference, or continuation.");
    println!("The AI will analyze your description and create content in the most appropriate format.");
    println!();

    // File management phase - using the existing ContinuationProject system
    let mut project = crate::continuation::ContinuationProject::new();
    let mut has_files = false;

    loop {
        // Show current files if any
        if !project.files.is_empty() {
            println!("\n📂 Current reference files:");
            for (i, file) in project.files.iter().enumerate() {
                let status = if file.exists { "✅" } else { "❌" };
                println!("  {}. {} {}", i + 1, status, file.display_info());
            }
            println!("Total words: {}\n", project.total_word_count());
            has_files = true;
        }

        // File management menu
        let mut options = vec![
            "📁 Add reference file",
        ];

        if has_files {
            options.push("📝 Review files");
            options.push("🗑️ Remove a file");
        }
        
        options.push("✅ Continue to writing prompt");
        options.push("← Back to main menu");

        let choice = Select::new()
            .with_prompt("File Management")
            .items(&options)
            .default(if has_files { 3 } else { 1 })
            .interact()?;

        match choice {
            0 => {
                // Add file
                if let Err(e) = add_file_to_freeform_project(&mut project).await {
                    println!("❌ Error adding file: {}", e);
                }
            },
            i if has_files && options[i] == "📝 Review files" => {
                review_freeform_files(&project)?;
            },
            i if has_files && options[i] == "🗑️ Remove a file" => {
                if let Err(e) = remove_file_from_freeform_project(&mut project) {
                    println!("❌ Error removing file: {}", e);
                }
            },
            i if options[i] == "✅ Continue to writing prompt" => {
                break;
            },
            _ => {
                // Back to main menu
                return Ok(());
            }
        }
    }

    // Freeform prompt phase
    println!("\n📝 Describe what you want to create:");
    println!("Be as specific or general as you like. Examples:");
    println!("• 'A business report analyzing our Q3 sales data from the spreadsheet'");
    println!("• 'Continue the story from my novel draft as a screenplay'"); 
    println!("• 'A technical manual for the API docs I uploaded'");
    println!("• 'Transform these meeting notes into action items'");
    println!();

    let freeform_prompt: String = Input::new()
        .with_prompt("Your writing request")
        .interact_text()?;

    if freeform_prompt.trim().is_empty() {
        println!("❌ Empty prompt. Returning to main menu.");
        return Ok(());
    }

    // Analyze the prompt and determine the best format
    let analysis = analyze_freeform_prompt(&freeform_prompt, &project.files);
    
    println!("\n🎯 Analysis Results:");
    println!("   Detected format: {}", analysis.detected_format);
    println!("   Confidence: {}%", (analysis.confidence * 100.0) as u8);
    println!("   Content type: {:?}", analysis.content_type);
    if !analysis.key_requirements.is_empty() {
        println!("   Key requirements: {}", analysis.key_requirements.join(", "));
    }
    println!();

    let proceed = Confirm::new()
        .with_prompt(&format!("Proceed with creating {} content?", analysis.detected_format))
        .default(true)
        .interact()?;

    if !proceed {
        println!("📝 You can refine your prompt or go back to file management.");
        return Ok(());
    }

    // Get title and author for the content
    let title: String = Input::new()
        .with_prompt("Content title")
        .default(format!("{} Content", analysis.detected_format))
        .interact_text()?;
    
    let author: String = Input::new()
        .with_prompt("Author name")
        .default("Pundit Writer".to_string())
        .interact_text()?;

    // Create a minimal config for the generation
    let temp_config = Config::default();

    // Generate the content
    generate_freeform_content(&project, &freeform_prompt, &analysis, &title, &author, &temp_config).await?;

    Ok(())
}

#[derive(Debug)]
struct FreeformAnalysis {
    detected_format: String,
    content_type: ContentType,
    confidence: f32,
    key_requirements: Vec<String>,
    estimated_length: usize,
    target_sections: usize,
}

async fn add_file_to_freeform_project(project: &mut ContinuationProject) -> anyhow::Result<()> {
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
                let file = project.files.last().expect("File should have been added to project");
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

fn review_freeform_files(project: &ContinuationProject) -> anyhow::Result<()> {
    if project.files.is_empty() {
        println!("No files in the project.");
        return Ok(());
    }

    println!("\n📋 Freeform Project Review");
    println!("==========================");

    if let Some(primary) = &project.primary_file {
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
                    if content.chars().count() > preview_length {
                        println!("\nContent preview (first {} characters):", preview_length);
                        let truncated: String = content.chars().take(preview_length).collect();
                        println!("{}", truncated);
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

fn remove_file_from_freeform_project(project: &mut ContinuationProject) -> anyhow::Result<()> {
    if project.files.is_empty() {
        return Err(anyhow::anyhow!("No files to remove"));
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

fn analyze_freeform_prompt(prompt: &str, files: &[crate::continuation::ContinuationFile]) -> FreeformAnalysis {
    let prompt_lower = prompt.to_lowercase();
    let mut confidence = 0.0;
    let mut detected_format = String::new();
    let mut content_type = ContentType::Document;
    let mut key_requirements = Vec::new();
    let mut estimated_length = 1000;
    let mut target_sections = 3;

    // Analyze for different content types with confidence scoring
    let mut format_scores = std::collections::HashMap::new();

    // Business/Professional formats
    if prompt_lower.contains("business report") || prompt_lower.contains("executive summary") || 
       prompt_lower.contains("quarterly report") || prompt_lower.contains("financial analysis") {
        format_scores.insert("Business Report", 0.9);
        content_type = ContentType::TechnicalDoc;
        estimated_length = 2000;
        target_sections = 5;
        key_requirements.push("Executive Summary".to_string());
        key_requirements.push("Data Analysis".to_string());
        key_requirements.push("Recommendations".to_string());
    }

    // Creative writing formats
    if prompt_lower.contains("novel") || prompt_lower.contains("story") || prompt_lower.contains("chapter") {
        format_scores.insert("Novel/Story", 0.85);
        content_type = ContentType::Book;
        estimated_length = 5000;
        target_sections = 8;
        key_requirements.push("Character Development".to_string());
        key_requirements.push("Plot Progression".to_string());
        key_requirements.push("Narrative Flow".to_string());
    }

    // Screenplay/Script formats
    if prompt_lower.contains("script") || prompt_lower.contains("screenplay") || 
       prompt_lower.contains("tv script") || prompt_lower.contains("movie") {
        format_scores.insert("Screenplay", 0.9);
        content_type = ContentType::Screenplay;
        estimated_length = 3000;
        target_sections = 6;
        key_requirements.push("Scene Structure".to_string());
        key_requirements.push("Character Dialogue".to_string());
        key_requirements.push("Visual Direction".to_string());
    }

    // Technical documentation
    if prompt_lower.contains("documentation") || prompt_lower.contains("manual") || 
       prompt_lower.contains("guide") || prompt_lower.contains("tutorial") {
        format_scores.insert("Technical Documentation", 0.8);
        content_type = ContentType::TechnicalDoc;
        estimated_length = 2500;
        target_sections = 6;
        key_requirements.push("Step-by-step Instructions".to_string());
        key_requirements.push("Examples".to_string());
        key_requirements.push("Troubleshooting".to_string());
    }

    // Marketing/Advertising
    if prompt_lower.contains("marketing") || prompt_lower.contains("advertisement") || 
       prompt_lower.contains("campaign") || prompt_lower.contains("promotional") {
        format_scores.insert("Marketing Content", 0.85);
        content_type = ContentType::MarketingAd;
        estimated_length = 1500;
        target_sections = 4;
        key_requirements.push("Target Audience".to_string());
        key_requirements.push("Call to Action".to_string());
        key_requirements.push("Value Proposition".to_string());
    }

    // Academic/Research
    if prompt_lower.contains("research") || prompt_lower.contains("academic") || 
       prompt_lower.contains("analysis") || prompt_lower.contains("study") {
        format_scores.insert("Research Report", 0.8);
        content_type = ContentType::ResearchReport;
        estimated_length = 3500;
        target_sections = 7;
        key_requirements.push("Literature Review".to_string());
        key_requirements.push("Methodology".to_string());
        key_requirements.push("Findings".to_string());
        key_requirements.push("Conclusions".to_string());
    }

    // Blog/Article
    if prompt_lower.contains("blog") || prompt_lower.contains("article") || 
       prompt_lower.contains("editorial") || prompt_lower.contains("opinion") {
        format_scores.insert("Blog/Article", 0.75);
        content_type = ContentType::BlogPost;
        estimated_length = 1200;
        target_sections = 4;
        key_requirements.push("Engaging Introduction".to_string());
        key_requirements.push("Supporting Evidence".to_string());
        key_requirements.push("Clear Conclusion".to_string());
    }

    // Find the highest confidence format
    if let Some((format, score)) = format_scores.iter().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) {
        detected_format = format.to_string();
        confidence = *score;
    } else {
        // Default fallback based on general indicators
        detected_format = "General Document".to_string();
        confidence = 0.5;
        key_requirements.push("Clear Structure".to_string());
        key_requirements.push("Coherent Content".to_string());
    }

    // Adjust estimates based on file content if available
    let total_input_words: usize = files.iter().map(|f| f.word_count).sum();
    if total_input_words > 0 {
        estimated_length = std::cmp::max(estimated_length, total_input_words / 2);
    }

    FreeformAnalysis {
        detected_format,
        content_type,
        confidence,
        key_requirements,
        estimated_length,
        target_sections,
    }
}

async fn generate_freeform_content(
    project: &ContinuationProject,
    prompt: &str,
    analysis: &FreeformAnalysis,
    title: &str,
    author: &str,
    config: &Config,
) -> anyhow::Result<()> {
    println!("\n🎯 Generating {} content...", analysis.detected_format);
    println!("Confidence: {:.0}%", analysis.confidence * 100.0);
    println!("Target length: ~{} words", analysis.estimated_length);
    println!("Sections: {}", analysis.target_sections);
    
    if !analysis.key_requirements.is_empty() {
        println!("Key elements to include:");
        for req in &analysis.key_requirements {
            println!("  • {}", req);
        }
    }

    // Get combined content from files
    let file_context = match project.get_combined_content() {
        Ok(content) => content,
        Err(_) => String::new(), // No files or unreadable files
    };

    // Create enhanced prompt that combines user request with file context
    let enhanced_prompt = if file_context.is_empty() {
        format!(
            "Create a {} with the following specifications:\n\n{}\n\nFormat: {}\nTarget length: {} words\nSections: {}\n\nEnsure the content follows proper {} formatting and structure.",
            analysis.detected_format.to_lowercase(),
            prompt,
            analysis.detected_format,
            analysis.estimated_length,
            analysis.target_sections,
            analysis.detected_format
        )
    } else {
        format!(
            "Using the provided file content as reference/context, create a {} with the following specifications:\n\n{}\n\nReference Content:\n{}\n\nFormat: {}\nTarget length: {} words\nSections: {}\n\nEnsure the content follows proper {} formatting and structure while incorporating insights from the reference materials.",
            analysis.detected_format.to_lowercase(),
            prompt,
            file_context,
            analysis.detected_format,
            analysis.estimated_length,
            analysis.target_sections,
            analysis.detected_format
        )
    };

    // Create content object for generation
    let mut content = crate::content::Content::new_document(
        title.to_string(),
        author.to_string(),
        "Freeform Generated Content".to_string(),
        enhanced_prompt.clone(),
        analysis.estimated_length,
        crate::content::DocumentFormat::Educational,
        "freeform".to_string(),
    );

    content.content_type = analysis.content_type.clone();
    content.metadata.target_word_count = Some(analysis.estimated_length);

    // Generate the content using the appropriate method based on content type
    let result = generate_freeform_content_internal(&mut content, &enhanced_prompt, config).await;

    match result {
        Ok(_) => {
            println!("✅ Freeform content generated successfully!");
            
            // Save the generated content
            let filename = format!("freeform_{}_{}.txt", 
                title.replace(" ", "_").to_lowercase(),
                chrono::Local::now().format("%Y%m%d_%H%M%S")
            );
            
            let output_path = std::path::Path::new(&filename);
            let content_text = content.to_text();
            std::fs::write(output_path, content_text.as_bytes())?;
            
            println!("📄 Content saved to: {}", output_path.display());
        },
        Err(e) => {
            println!("❌ Error generating content: {}", e);
        }
    }

    Ok(())
}

async fn generate_freeform_content_internal(
    content: &mut Content,
    prompt: &str,
    _config: &Config,
) -> anyhow::Result<()> {
    let target_sections = content.metadata.target_sections;
    let target_words = content.metadata.target_word_count.unwrap_or(1000);
    
    // Create a simple client using the default pattern from existing code
    let use_local = std::env::var("USE_LOCAL_OLLAMA").is_ok();
    let ollama_url = std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".to_string());
    
    let client = if use_local {
        let ollama_client = OllamaClient::new(ollama_url)?;
        AIClient::Ollama(ollama_client)
    } else {
        // Try to get API key from environment or config
        let api_key = std::env::var("HUGGINGFACE_API_KEY").ok();
        let model = "microsoft/DialoGPT-medium".to_string(); // Default model
        let hf_client = HuggingFaceClient::new(model, api_key)?;
        AIClient::HuggingFace(hf_client)
    };
    
    // Generate content sections progressively
    for section_num in 0..target_sections {
        let section_prompt = if section_num == 0 {
            format!("{}\n\nThis is section {} of {}. Start with a compelling opening.", 
                prompt, section_num + 1, target_sections)
        } else if section_num == target_sections - 1 {
            format!("{}\n\nThis is the final section {} of {}. Provide a satisfying conclusion.\n\nPrevious content:\n{}", 
                prompt, section_num + 1, target_sections, content.get_context_for_next_section())
        } else {
            format!("{}\n\nThis is section {} of {}. Continue the narrative/content progression.\n\nPrevious content:\n{}", 
                prompt, section_num + 1, target_sections, content.get_context_for_next_section())
        };

        let section_target_words = target_words / target_sections;
        let max_tokens = section_target_words * 6; // Conservative estimate

        // Generate section content
        let generated_text = match &client {
            AIClient::HuggingFace(hf_client) => {
                hf_client.generate_text(&section_prompt, max_tokens as u32, 0.8).await?
            },
            AIClient::Ollama(ollama_client) => {
                let default_model = "llama2"; // Default model for Ollama
                ollama_client.generate_text(default_model, &section_prompt, max_tokens as i32, 0.8).await?
            }
        };

        // Create section with correct structure
        let section = Section {
            number: section_num + 1,
            title: format!("Section {}", section_num + 1),
            content: generated_text.trim().to_string(),
            word_count: count_words(&generated_text),
            outline: format!("Generated section {} content", section_num + 1),
            section_type: SectionType::Section, // Use the generic Section type
            created_at: Utc::now(),
            completed: true,
            plot_thread: None,
            narrative_context: None,
        };

        content.add_section(section);
        content.metadata.current_word_count += count_words(&generated_text);

        println!("✅ Generated section {}/{} ({} words)", section_num + 1, target_sections, count_words(&generated_text));
    }

    Ok(())
}

// Helper function to create AI client
fn create_ai_client(use_local: bool, model: String, api_key: Option<String>, endpoint: Option<String>) -> Result<AIClient> {
    if use_local {
        let ollama_url = endpoint.unwrap_or_else(|| "http://localhost:11434".to_string());
        let ollama_client = OllamaClient::new(ollama_url)?;
        Ok(AIClient::Ollama(ollama_client))
    } else {
        let hf_client = HuggingFaceClient::new(model, api_key)?;
        Ok(AIClient::HuggingFace(hf_client))
    }
}

// Generate comprehensive educational textbook content
async fn generate_educational_textbook_content(client: &AIClient, prompt: &str, topic: &str) -> Result<String> {
    println!("\n📚 Generating textbook content...");
    
    // Estimate sections based on scope (this could be made more sophisticated)
    let estimated_sections = if prompt.contains("Single Chapter") {
        1
    } else if prompt.contains("Short Course") {
        4
    } else if prompt.contains("Semester Course") {
        10
    } else if prompt.contains("Full Year Course") {
        16
    } else {
        20 // Comprehensive Reference
    };
    
    let mut full_content = String::new();
    
    // Generate table of contents first
    let toc_prompt = format!("{}\n\nFirst, create a detailed table of contents with {} main sections. Each section should have clear learning objectives and subsections.", prompt, estimated_sections);
    
    let toc = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&toc_prompt, 2000, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text("llama2", &toc_prompt, 2000, 0.7).await?
        }
    };
    
    full_content.push_str(&format!("# {}\n\n", topic));
    full_content.push_str("## Table of Contents\n\n");
    full_content.push_str(&toc);
    full_content.push_str("\n\n---\n\n");
    
    // Generate each section
    for section_num in 1..=estimated_sections {
        println!("📖 Generating section {}/{}...", section_num, estimated_sections);
        
        let section_prompt = format!(
            "{}\n\nNow create Section {} of the textbook. Make it comprehensive with:\n\
            - Clear introduction to the section topic\n\
            - Detailed explanations with examples\n\
            - Practice problems or exercises\n\
            - Key terms and definitions\n\
            - Section summary\n\
            - Connection to previous and next sections\n\n\
            Previous content for context:\n{}",
            prompt, section_num, 
            if full_content.len() > 1000 {
                // Use last 1000 characters for context
                full_content.chars().skip(full_content.chars().count().saturating_sub(1000)).collect::<String>()
            } else {
                full_content.clone()
            }
        );
        
        let section_content = match client {
            AIClient::HuggingFace(hf_client) => {
                hf_client.generate_text(&section_prompt, 3000, 0.8).await?
            },
            AIClient::Ollama(ollama_client) => {
                ollama_client.generate_text("llama2", &section_prompt, 3000, 0.8).await?
            }
        };
        
        full_content.push_str(&format!("## Section {}\n\n", section_num));
        full_content.push_str(&section_content);
        full_content.push_str("\n\n---\n\n");
        
        // Small delay to prevent overwhelming the API
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    // Add conclusion/appendix
    let conclusion_prompt = format!("{}\n\nFinally, create a conclusion section that:\n\
        - Summarizes key learning outcomes\n\
        - Provides additional resources for further study\n\
        - Includes a glossary of important terms\n\
        - Suggests next steps for continued learning", prompt);
    
    let conclusion = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&conclusion_prompt, 2000, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text("llama2", &conclusion_prompt, 2000, 0.7).await?
        }
    };
    
    full_content.push_str("## Conclusion and Resources\n\n");
    full_content.push_str(&conclusion);
    
    println!("✅ Textbook generation complete! ({} words)", count_words(&full_content));
    
    Ok(full_content)
}

// Helper functions for AI output validation and corruption prevention
fn create_isolated_generation_prompt(content_type: &ContentType, genre: &str, context: &str, outline: &str, target_words: usize) -> String {
    // Create a clean, isolated prompt with clear structure
    format!(
        "===== CREATIVE WRITING TASK =====
Content Type: {:?}
Genre: {}
Target Length: {} words

Context:
{}

Writing Instructions:
{}

===== IMPORTANT RULES =====
1. Write ONLY the story content requested
2. Do NOT include chapter numbers, titles, or headers in your output
3. Do NOT include any meta-commentary or explanations
4. Do NOT reference this prompt or previous instructions
5. Write in narrative prose only
6. Maintain consistency with the provided context

===== BEGIN WRITING =====",
        content_type, genre, target_words, 
        sanitize_context_for_ai(context),
        sanitize_outline_for_ai(outline)
    )
}

fn sanitize_context_for_ai(context: &str) -> String {
    context
        .lines()
        .filter(|line| {
            let line_lower = line.to_lowercase();
            // Filter out lines that might confuse the AI
            !line_lower.contains("context:") &&
            !line_lower.contains("question:") &&
            !line_lower.contains("answer:") &&
            !line_lower.contains("user:") &&
            !line_lower.contains("assistant:") &&
            !line_lower.contains("ai:") &&
            !line_lower.contains("http") &&
            !line_lower.contains("www.") &&
            !line_lower.contains("<!--") &&
            !line_lower.contains("-->") &&
            !line.trim().starts_with("#") &&
            !line.trim().starts_with("*")
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn sanitize_outline_for_ai(outline: &str) -> String {
    outline
        .lines()
        .map(|line| {
            // Remove any formatting that might confuse the AI
            line.replace("*", "")
                .replace("#", "")
                .replace("**", "")
                .replace("__", "")
                .trim()
                .to_string()
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_prompt_corrupted(prompt: &str) -> bool {
    let prompt_lower = prompt.to_lowercase();
    
    // Check for signs of prompt corruption or contamination
    prompt_lower.contains("context:") && prompt_lower.contains("question:") ||
    prompt_lower.contains("user:") && prompt_lower.contains("assistant:") ||
    prompt_lower.contains("training data") ||
    prompt_lower.contains("example output") ||
    prompt_lower.contains("dataset") ||
    prompt_lower.contains("model training") ||
    prompt.len() > 50000 || // Unreasonably long prompt
    prompt.matches("Context").count() > 10 || // Too many "Context" keywords
    prompt.contains("$") && prompt.contains("http") // Suspicious URL patterns
}

fn is_ai_output_corrupted(output: &str) -> bool {
    let output_lower = output.to_lowercase();
    
    // Check for signs of corrupted AI output
    if output.len() < 10 {
        return true; // Too short to be valid content
    }
    
    // Check for training data contamination patterns
    let contamination_patterns = [
        "context:",
        "question:",
        "answer:",
        "user:",
        "assistant:",
        "questionnaire",
        "questioneering",
        "contexting",
        "problem textiles",
        "questionary",
        "academic writing: ishikaga",
        "context and solutions",
        "translate all this",
        "input=yoga",
        "user=user:",
        "$$",
        "http://",
        "www.",
        ".com",
        ".org",
        "because ihops",
        "atlasica",
        "ravennaise",
        "ninejustice.org",
        "growthshoe",
    ];
    
    let contamination_count = contamination_patterns.iter()
        .filter(|pattern| output_lower.contains(*pattern))
        .count();
    
    if contamination_count >= 2 {
        return true; // Multiple contamination patterns detected
    }
    
    // Check for fragments of training examples
    if (output_lower.contains("question") && output_lower.contains("answer")) ||
       (output_lower.contains("context") && output_lower.contains("documentation")) ||
       (output_lower.contains("user") && output_lower.contains("ai")) ||
       output_lower.contains("training") && output_lower.contains("model") {
        return true;
    }
    
    // Check for broken text patterns
    if output.matches("{").count() != output.matches("}").count() ||
       output.matches("[").count() != output.matches("]").count() ||
       output.contains("$") && output.matches("$").count() > 3 {
        return true;
    }
    
    false
}

fn truncate_for_error(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}

pub async fn non_stop_writing_mode(
    genre: Genre,
    style: WritingStyle,
    size: BookSize,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
    sections: usize,
    buffer_size: usize,
    auto_continue: bool,
) -> Result<()> {
    println!("🚀 Initializing Non-Stop Writing Mode...");
    
    let output_path = output.unwrap_or_else(|| {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        PathBuf::from(format!("NonStop_{}_{}.txt", genre, timestamp))
    });

    println!("📁 Output file: {}", output_path.display());
    
    let config = Config::load()?;
    
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };

    check_model_availability(&client, &model).await?;

    let mut writing_session = create_resilient_writing_session(
        output_path.clone(),
        buffer_size,
        sections,
    );
    
    writing_session.enable_auto_continue(auto_continue);
    
    println!("🎯 Generating creative premise for {} {} story...", genre, style);
    
    let premise = generate_creative_premise(&client, &genre, &style, &model).await?;
    println!("💡 Premise: {}", premise);
    
    let result = writing_session.run(|section_num| {
        generate_section_content(
            &client,
            &model,
            &genre,
            &style,
            &size,
            &premise,
            section_num,
            sections,
        )
    }).await?;

    println!("✅ Non-stop writing completed!");
    println!("📊 Final statistics:");
    println!("   Total words: {}", count_words(&result));
    println!("   Total sections: {}", sections);
    println!("   Output file: {}", output_path.display());
    
    Ok(())
}

async fn generate_creative_premise(
    client: &AIClient,
    genre: &Genre,
    style: &WritingStyle,
    model: &str,
) -> Result<String> {
    let premise_prompt = format!(
        "Create a compelling and original premise for a {} story written in {} style. \
         The premise should be 2-3 sentences that establish the main character, conflict, \
         and unique hook. Make it creative and engaging.",
        genre, style
    );

    let premise = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&premise_prompt, 150u32, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &premise_prompt, 150i32, 0.8).await?
        }
    };

    Ok(premise.trim().to_string())
}

async fn generate_section_content(
    client: &AIClient,
    model: &str,
    genre: &Genre,
    style: &WritingStyle,
    size: &BookSize,
    premise: &str,
    section_num: usize,
    total_sections: usize,
) -> Result<String> {
    let words_per_section = match size {
        BookSize::ShortStory => 200,
        BookSize::Short => 400,
        BookSize::Medium => 600,
        BookSize::Large => 800,
        BookSize::VeryLarge => 1200,
        BookSize::Epic => 1000,
        BookSize::Unlimited => 1500,
    };

    let section_prompt = format!(
        "Continue this {} story in {} style. This is section {} of {}.\n\n\
         Premise: {}\n\n\
         Write approximately {} words that:\n\
         - Advances the plot naturally\n\
         - Develops characters\n\
         - Maintains consistent tone and style\n\
         - Creates engaging narrative flow\n\
         - Ends with natural transition to next section\n\n\
         Section content:",
        genre, style, section_num, total_sections, premise, words_per_section
    );

    let section_content = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&section_prompt, words_per_section as u32, 0.7).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &section_prompt, words_per_section as i32, 0.7).await?
        }
    };

    let cleaned_content = clean_ai_output(&section_content);
    
    Ok(format!("\n## Section {}\n\n{}\n", section_num, cleaned_content))
}

fn clean_ai_output(content: &str) -> String {
    content
        .trim()
        .lines()
        .filter(|line| {
            !line.trim().is_empty() && 
            !line.starts_with("Section") &&
            !line.starts_with("##") &&
            !line.contains("Section content:")
        })
        .collect::<Vec<&str>>()
        .join("\n")
        .trim()
        .to_string()
}

pub async fn enhanced_intelligent_writing_mode(
    genre: Genre,
    style: WritingStyle,
    size: BookSize,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
    chapters: usize,
    buffer_size: usize,
    auto_continue: bool,
    interruption_recovery: bool,
) -> Result<()> {
    println!("🧠 Enhanced Intelligent Writing Mode Initializing...");
    println!("   🎯 Advanced creativity optimization enabled");
    println!("   ⏰ Temporal continuity engine active");
    println!("   📊 Intelligent progression tracking online");
    
    let session_id = uuid::Uuid::new_v4().to_string();
    let output_path = output.unwrap_or_else(|| {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        PathBuf::from(format!("Enhanced_{}_{}.txt", genre, timestamp))
    });

    // Initialize advanced systems
    let mut temporal_engine = TemporalEngine::new();
    let mut creativity_engine = AdvancedCreativityEngine::new(&genre, &style, &ContentType::Book);
    let mut progression_tracker = IntelligentProgressionTracker::new(session_id.clone(), chapters);

    // Setup AI client
    let config = Config::load()?;
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };

    check_model_availability(&client, &model).await?;

    // Generate story premise with enhanced creativity
    println!("🎨 Generating enhanced creative premise...");
    let premise = generate_enhanced_creative_premise(&client, &model, &genre, &style, &mut creativity_engine).await?;
    println!("💡 Enhanced Premise: {}", premise);

    let mut content_buffer = String::with_capacity(buffer_size);
    let mut full_story_context = String::new();

    // Initialize content structure - using placeholder values for now
    let mut content = Content::new(
        "Enhanced Story".to_string(),        // title
        "AI Writer".to_string(),             // author
        genre.to_string(),                   // genre
        style.to_string(),                   // writing_style
        premise.clone(),                     // premise
        size.to_string(),                    // target_size
        None,                               // target_word_count
        chapters,                           // target_chapters
        model.clone()                       // model
    );

    // Chapter generation loop with intelligent systems
    for chapter_num in 1..=chapters {
        println!("\n📖 === CHAPTER {} GENERATION === ", chapter_num);
        
        // Prepare chapter with intelligent analysis
        let generation_context = progression_tracker
            .prepare_for_chapter(chapter_num, interruption_recovery && chapter_num == 3)?;
        
        // Analyze temporal context
        let temporal_context = temporal_engine
            .analyze_chapter_context(chapter_num, &content)?;
        
        // Generate creative chapter plan
        let creative_plan = creativity_engine
            .generate_creative_chapter_plan(chapter_num, &full_story_context, interruption_recovery && chapter_num == 3)?;

        println!("   🧭 Story Analysis: {:.1}% progress, {} active threads", 
            generation_context.story_analysis.progress_percentage,
            generation_context.story_analysis.active_thread_count);
        
        println!("   🎭 Creative Focus: {:?}", creative_plan.creative_focus);
        println!("   ⏱️  Temporal Context: Chapter {} timeframe", chapter_num);

        // Generate enhanced prompt
        let base_prompt = create_enhanced_chapter_prompt(&premise, chapter_num, chapters, &size, &genre, &style)?;
        
        // Layer on temporal enhancements
        let temporal_enhanced_prompt = temporal_engine
            .generate_temporal_prompt_enhancement(chapter_num, &base_prompt, &temporal_context)?;
        
        // Layer on creativity enhancements
        let fully_enhanced_prompt = creativity_engine
            .enhance_prompt_with_creativity(&temporal_enhanced_prompt, &creative_plan)?;

        println!("   📝 Enhanced prompt prepared ({} characters)", fully_enhanced_prompt.len());

        // Generate chapter with resilience
        let generation_start = std::time::Instant::now();
        let mut generation_metrics = GenerationMetrics {
            generation_duration: chrono::Duration::seconds(0),
            interruptions: vec![],
        };

        let chapter_content = match generate_chapter_with_resilience(
            &client,
            &model,
            &fully_enhanced_prompt,
            chapter_num,
            &size,
        ).await {
            Ok(content) => content,
            Err(e) => {
                println!("   ⚠️  Generation error: {}", e);
                
                // Handle interruption intelligently
                let interruption_plan = progression_tracker
                    .handle_interruption(InterruptionType::SystemError, chapter_num)?;
                
                println!("   🔄 Applying intelligent recovery...");
                
                // Retry with recovery context
                generate_chapter_with_resilience(
                    &client,
                    &model,
                    &fully_enhanced_prompt,
                    chapter_num,
                    &size,
                ).await?
            }
        };

        let elapsed = generation_start.elapsed();
        generation_metrics.generation_duration = chrono::Duration::from_std(elapsed).unwrap_or(chrono::Duration::seconds(0));

        // Process generated chapter
        let processing_result = progression_tracker
            .process_generated_chapter(chapter_num, &chapter_content, generation_metrics)?;

        // Update temporal engine
        temporal_engine.update_after_chapter(chapter_num, &chapter_content)?;

        // Add to content buffer and full context
        let formatted_chapter = format!("\n## Chapter {}\n\n{}\n", chapter_num, chapter_content);
        content_buffer.push_str(&formatted_chapter);
        full_story_context.push_str(&formatted_chapter);
        
        // Maintain context window size
        if full_story_context.len() > 50000 {
            let words: Vec<&str> = full_story_context.split_whitespace().collect();
            let keep_words = words.len().saturating_sub(10000);
            full_story_context = words[keep_words..].join(" ");
        }

        println!("   ✅ Chapter {} completed", chapter_num);
        println!("   📊 Quality: {:.1}/10 | Innovation: {:.1}/10 | Coherence: {:.1}/10", 
            processing_result.quality_assessment.overall_score * 10.0,
            processing_result.quality_assessment.originality_score * 10.0,
            processing_result.quality_assessment.narrative_coherence * 10.0);

        // Auto-save progress
        if chapter_num % 3 == 0 {
            fs::write(&output_path, &content_buffer)?;
            println!("   💾 Progress auto-saved");
        }

        // Pause for user input if not auto-continue
        if !auto_continue && chapter_num < chapters {
            println!("   ⏸️  Press Enter to continue or Ctrl+C to stop...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
        }
    }

    // Final save
    fs::write(&output_path, &content_buffer)?;

    // Generate completion report
    println!("\n🎉 === ENHANCED WRITING COMPLETED === ");
    println!("📈 Final Statistics:");
    println!("   📖 Total chapters: {}", chapters);
    println!("   📝 Total words: {}", count_words(&content_buffer));
    println!("   📁 Saved to: {}", output_path.display());
    println!("   🧠 Advanced systems used: Temporal Engine, Creativity Optimizer, Progression Tracker");
    
    Ok(())
}

async fn generate_enhanced_creative_premise(
    client: &AIClient,
    model: &str,
    genre: &Genre,
    style: &WritingStyle,
    creativity_engine: &mut AdvancedCreativityEngine,
) -> Result<String> {
    let base_premise_prompt = format!(
        "Create an exceptionally compelling and original premise for a {} story written in {} style. \
         The premise should be 3-4 sentences that establish:\n\
         1. A unique protagonist with clear motivation\n\
         2. An original central conflict with high stakes\n\
         3. A distinctive story world or setting\n\
         4. An intriguing hook that promises depth and complexity\n\n\
         Make it creative, engaging, and avoid common tropes.",
        genre, style
    );

    // Generate creative plan for premise
    let premise_plan = creativity_engine.generate_creative_chapter_plan(0, "", false)?;
    let enhanced_premise_prompt = creativity_engine.enhance_prompt_with_creativity(&base_premise_prompt, &premise_plan)?;

    let premise = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&enhanced_premise_prompt, 200u32, 0.9).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &enhanced_premise_prompt, 200i32, 0.9).await?
        }
    };

    Ok(premise.trim().to_string())
}

fn create_enhanced_chapter_prompt(
    premise: &str,
    chapter_num: usize,
    total_chapters: usize,
    size: &BookSize,
    genre: &Genre,
    style: &WritingStyle,
) -> Result<String> {
    let words_per_chapter = match size {
        BookSize::ShortStory => 300,
        BookSize::Short => 600,
        BookSize::Medium => 800,
        BookSize::Large => 1000,
        BookSize::VeryLarge => 1400,
        BookSize::Epic => 1200,
        BookSize::Unlimited => 1600,
    };

    let chapter_context = if chapter_num == 1 {
        "This is the opening chapter. Establish the protagonist, setting, and central conflict while creating an engaging hook."
    } else if chapter_num == 3 {
        "This is a crucial chapter that should deepen the conflict and reveal important information that changes the reader's understanding. This chapter often determines if readers continue."
    } else if chapter_num == total_chapters {
        "This is the final chapter. Resolve the main conflict, provide emotional satisfaction, and conclude character arcs meaningfully."
    } else if chapter_num > total_chapters / 2 {
        "This chapter is in the latter half of the story. Escalate tension, develop character relationships, and build toward the climax."
    } else {
        "This chapter should develop the plot, deepen characterization, and maintain narrative momentum."
    };

    Ok(format!(
        "Write Chapter {} of a {} {} story in {} style.\n\n\
         PREMISE: {}\n\n\
         CHAPTER CONTEXT: {}\n\n\
         TARGET LENGTH: Approximately {} words\n\n\
         WRITING REQUIREMENTS:\n\
         - Write in engaging, immersive prose that matches the {} style\n\
         - Develop characters through action and dialogue, not exposition\n\
         - Advance the main plot while developing subplots\n\
         - Create vivid, sensory-rich scenes that draw readers in\n\
         - End with a natural transition or compelling hook for the next chapter\n\
         - Maintain consistent tone and voice throughout\n\
         - Show don't tell - let events and dialogue reveal character and plot\n\n\
         Write only the chapter content, no titles or meta-commentary:",
        chapter_num,
        genre,
        size.to_string(),
        style,
        premise,
        chapter_context,
        words_per_chapter,
        style
    ))
}

async fn generate_chapter_with_resilience(
    client: &AIClient,
    model: &str,
    prompt: &str,
    chapter_num: usize,
    size: &BookSize,
) -> Result<String> {
    let words_per_chapter = match size {
        BookSize::ShortStory => 300,
        BookSize::Short => 600,
        BookSize::Medium => 800,
        BookSize::Large => 1000,
        BookSize::VeryLarge => 1400,
        BookSize::Epic => 1200,
        BookSize::Unlimited => 1600,
    };

    let max_retries = 3;
    let mut last_error = None;

    for attempt in 1..=max_retries {
        match client {
            AIClient::HuggingFace(hf_client) => {
                match hf_client.generate_text(prompt, words_per_chapter as u32, 0.8).await {
                    Ok(content) => {
                        let cleaned = clean_ai_output(&content);
                        if cleaned.split_whitespace().count() > 50 {
                            return Ok(cleaned);
                        } else {
                            last_error = Some(anyhow!("Generated content too short"));
                        }
                    },
                    Err(e) => last_error = Some(e),
                }
            },
            AIClient::Ollama(ollama_client) => {
                match ollama_client.generate_text(model, prompt, words_per_chapter as i32, 0.8).await {
                    Ok(content) => {
                        let cleaned = clean_ai_output(&content);
                        if cleaned.split_whitespace().count() > 50 {
                            return Ok(cleaned);
                        } else {
                            last_error = Some(anyhow!("Generated content too short"));
                        }
                    },
                    Err(e) => last_error = Some(e),
                }
            }
        }

        if attempt < max_retries {
            println!("   🔄 Retry attempt {} for Chapter {}", attempt + 1, chapter_num);
            tokio::time::sleep(Duration::from_secs(2 * attempt as u64)).await;
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow!("Failed to generate chapter after {} attempts", max_retries)))
}

pub async fn historical_persona_writing_mode(
    genre: Genre,
    style: WritingStyle,
    size: BookSize,
    output: Option<PathBuf>,
    model: String,
    api_key: Option<String>,
    use_local: bool,
    ollama_url: String,
    chapters: usize,
    writer: Option<String>,
    era: Option<String>,
    auto_continue: bool,
    language_enhancement: String,
) -> Result<()> {
    use crate::historical_writer_personas::{HistoricalWriterPersonas, LiteraryEra};
    use crate::period_language_engine::{PeriodLanguageEngine, LanguageEnhancementLevel};
    use crate::creative_block_recovery::{CreativeBlockRecovery, CreativeBlockType};
    use crate::professional_formatting::{ProfessionalFormatter, PublishingType, ChapterTitleStyle, SceneBreakStyle, ChapterCreativeElement};

    println!("🎭 Historical Writer Persona Mode Initializing...");
    println!("   📚 Loading literary masters database...");
    println!("   🕰️  Period language systems online...");
    
    let session_id = uuid::Uuid::new_v4().to_string();
    let output_path = output.unwrap_or_else(|| {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let persona_name = writer.as_ref().unwrap_or(&"HistoricalMaster".to_string()).clone();
        PathBuf::from(format!("Persona_{}_{}.txt", persona_name, timestamp))
    });

    // Initialize persona systems
    let persona_system = HistoricalWriterPersonas::new();
    let language_engine = PeriodLanguageEngine::new();
    
    // Parse language enhancement level
    let enhancement_level = match language_enhancement.to_lowercase().as_str() {
        "none" => LanguageEnhancementLevel::None,
        "subtle" => LanguageEnhancementLevel::Subtle,
        "moderate" => LanguageEnhancementLevel::Moderate,
        "strong" => LanguageEnhancementLevel::Strong,
        "authentic" => LanguageEnhancementLevel::Authentic,
        _ => LanguageEnhancementLevel::Moderate,
    };

    // Select writer persona
    let selected_persona = if let Some(writer_name) = &writer {
        persona_system.get_persona_by_name(writer_name)
            .unwrap_or_else(|| {
                println!("⚠️  Writer '{}' not found. Using Hemingway as default.", writer_name);
                persona_system.get_persona_by_name("hemingway").unwrap()
            })
    } else if let Some(era_name) = &era {
        let literary_era = match era_name.as_str() {
            "classical" => LiteraryEra::Classical,
            "medieval" => LiteraryEra::Medieval,
            "renaissance" => LiteraryEra::Renaissance,
            "enlightenment" => LiteraryEra::Enlightenment,
            "romantic" => LiteraryEra::Romantic,
            "victorian" => LiteraryEra::Victorian,
            "modernist" => LiteraryEra::Modernist,
            "contemporary" => LiteraryEra::Contemporary,
            _ => LiteraryEra::Victorian,
        };
        persona_system.get_persona_by_era(&literary_era)
            .unwrap_or_else(|| persona_system.get_persona_by_name("dickens").unwrap())
    } else {
        // Default to appropriate persona for the genre
        match genre {
            Genre::Adventure | Genre::Action => persona_system.get_persona_by_name("hemingway").unwrap(),
            Genre::Horror | Genre::Mystery => persona_system.get_persona_by_name("poe").unwrap(),
            Genre::Romance | Genre::Drama => persona_system.get_persona_by_name("austen").unwrap(),
            Genre::Literary | Genre::Experimental => persona_system.get_persona_by_name("woolf").unwrap(),
            _ => persona_system.get_persona_by_name("dickens").unwrap(),
        }
    };

    println!("   🎨 Selected Persona: {} ({})", 
        selected_persona.writer_name, 
        selected_persona.era.to_string());
    println!("   📖 Literary Movement: {:?}", selected_persona.movement);
    println!("   ✍️  Philosophy: {}", selected_persona.writing_philosophy.core_beliefs.first().unwrap_or(&"Literary truth through authentic expression".to_string()));

    // Initialize temporal and creativity engines
    let mut temporal_engine = TemporalEngine::new();
    let mut creativity_engine = AdvancedCreativityEngine::new(&genre, &style, &ContentType::Book);
    let mut block_recovery = CreativeBlockRecovery::new(&genre);
    let formatter = ProfessionalFormatter::new();
    
    // Track generation metrics for block detection
    let mut recent_content: Vec<String> = Vec::new();
    let mut generation_times: Vec<Duration> = Vec::new();
    let mut error_count = 0;

    // Setup AI client
    let config = Config::load()?;
    let client = if use_local {
        println!("🏠 Using local Ollama server at: {}", ollama_url);
        let ollama_client = OllamaClient::new(ollama_url.clone())?;
        AIClient::Ollama(ollama_client)
    } else {
        let effective_api_key = api_key.or_else(|| config.get_effective_api_key());
        let hf_client = HuggingFaceClient::new(model.clone(), effective_api_key)?;
        AIClient::HuggingFace(hf_client)
    };

    check_model_availability(&client, &model).await?;

    // Generate persona-enhanced story premise
    println!("🌟 Generating story premise in {} style...", selected_persona.writer_name);
    let premise = generate_persona_premise(&client, &model, &genre, &style, &selected_persona).await?;
    println!("💡 Premise: {}", premise);

    let mut content_buffer = String::with_capacity(100 * 1024 * 1024); // 100MB buffer
    let mut full_story_context = String::new();

    // Initialize content structure  
    let mut content = Content::new(
        format!("{} Style Story", selected_persona.writer_name), // title
        format!("AI in the style of {}", selected_persona.writer_name), // author
        genre.to_string(),                   // genre
        style.to_string(),                   // writing_style
        premise.clone(),                     // premise
        size.to_string(),                    // target_size
        None,                               // target_word_count
        chapters,                           // target_chapters
        model.clone()                       // model
    );

    // Chapter generation loop
    for chapter_num in 1..=chapters {
        println!("\n🎭 === CHAPTER {} GENERATION ({} STYLE) ===", chapter_num, selected_persona.writer_name);
        
        // Analyze temporal context
        let temporal_context = temporal_engine.analyze_chapter_context(chapter_num, &content)?;
        
        // Generate creative chapter plan with persona influence
        let creative_plan = creativity_engine.generate_creative_chapter_plan(
            chapter_num, 
            &full_story_context, 
            false
        )?;

        println!("   📚 Writing in {} tradition", selected_persona.movement);
        println!("   🎯 Chapter focus: {:?}", creative_plan.creative_focus);

        // Create persona-enhanced prompt
        let base_prompt = create_persona_chapter_prompt(
            &premise, 
            chapter_num, 
            chapters, 
            &size, 
            &genre, 
            &style,
            &selected_persona
        )?;
        
        // Layer on temporal enhancements
        let temporal_enhanced_prompt = temporal_engine.generate_temporal_prompt_enhancement(
            chapter_num, 
            &base_prompt, 
            &temporal_context
        )?;
        
        // Layer on creativity enhancements
        let creativity_enhanced_prompt = creativity_engine.enhance_prompt_with_creativity(
            &temporal_enhanced_prompt, 
            &creative_plan
        )?;

        // Apply persona-specific prompt enhancements
        let fully_enhanced_prompt = persona_system.enhance_prompt_with_persona(
            &creativity_enhanced_prompt,
            &selected_persona
        )?;

        println!("   📝 Persona-enhanced prompt ready ({} characters)", fully_enhanced_prompt.len());

        // Generate chapter content with persona influence
        let generation_start = std::time::Instant::now();
        
        let raw_chapter_content = match generate_chapter_with_resilience(
            &client,
            &model,
            &fully_enhanced_prompt,
            chapter_num,
            &size
        ).await {
            Ok(content) => content,
            Err(e) => {
                error_count += 1;
                println!("   ⚠️  Generation error encountered: {}", e);
                
                // Check for creative block and apply recovery if needed
                if let Some(block_type) = block_recovery.detect_creative_block(
                    &recent_content,
                    &generation_times,
                    error_count,
                    chapter_num
                )? {
                    println!("   🚫 Creative block detected: {:?}", block_type);
                    
                    // Initiate sleep reflection mode
                    let reflection_result = block_recovery.initiate_sleep_reflection(
                        &full_story_context,
                        chapter_num
                    )?;
                    
                    // Apply renewal strategy
                    let renewal_strategies = block_recovery.renewal_strategies.clone();
                    if let Some(strategy) = renewal_strategies.first() {
                        let renewal_outcome = block_recovery.apply_renewal_strategy(
                            strategy,
                            &full_story_context,
                            chapter_num
                        )?;
                        
                        // Enhance prompt with renewal insights
                        let renewed_prompt = format!(
                            "{}\n\nREFLECTIVE RENEWAL: {}\n\nCreative Direction: {}",
                            fully_enhanced_prompt,
                            renewal_outcome.renewed_prompt_enhancement,
                            reflection_result.morning_renewal.narrative_direction
                        );
                        
                        // Retry with renewed approach
                        generate_chapter_with_resilience(
                            &client,
                            &model,
                            &renewed_prompt,
                            chapter_num,
                            &size
                        ).await?
                    } else {
                        return Err(e);
                    }
                } else {
                    return Err(e);
                }
            }
        };

        let generation_duration = generation_start.elapsed();
        generation_times.push(generation_duration);
        recent_content.push(raw_chapter_content.chars().take(500).collect::<String>());
        
        // Keep only recent content for block detection
        if recent_content.len() > 5 {
            recent_content.remove(0);
        }
        if generation_times.len() > 5 {
            generation_times.remove(0);
        }

        println!("   ⏱️  Generation took: {:.1}s", generation_duration.as_secs_f64());

        // Apply period language enhancement
        println!("   🎨 Applying {} era language enhancement...", selected_persona.era.to_string());
        let enhanced_content = language_engine.enhance_text_with_period_language(
            &raw_chapter_content,
            &selected_persona.era,
            &enhancement_level
        )?;

        // Apply persona-specific style enhancements
        let styled_content = persona_system.apply_persona_style_enhancements(
            &enhanced_content,
            &selected_persona
        )?;

        // Apply professional formatting
        let chapter_header = formatter.create_chapter_header(
            chapter_num,
            Some(&format!("Chapter {}", chapter_num)),
            &ChapterTitleStyle::Descriptive,
            &vec![ChapterCreativeElement::Ornamental("❦ ❦ ❦".to_string())]
        );
        
        let formatted_content = formatter.apply_creative_scene_breaks(
            &styled_content,
            &SceneBreakStyle::TripleStar,
            &genre
        );

        let final_chapter_content = format!("{}{}", chapter_header, formatted_content);

        println!("   ✨ Enhanced with {} period characteristics", selected_persona.era.to_string());
        println!("   📊 Chapter {} complete: {} words", 
            chapter_num, 
            final_chapter_content.split_whitespace().count());

        // Add to content buffer and context
        content_buffer.push_str(&format!("\n\n=== CHAPTER {} ===\n\n{}", chapter_num, final_chapter_content));
        full_story_context.push_str(&format!(" Chapter {}: {}", chapter_num, 
            final_chapter_content.chars().take(200).collect::<String>()));

        // Write to file after each chapter for safety
        std::fs::write(&output_path, &content_buffer)?;
        println!("   💾 Progress saved to {}", output_path.display());

        // Auto-continue check
        if !auto_continue && chapter_num < chapters {
            println!("\n⏸️  Continue to Chapter {} (y/N)?", chapter_num + 1);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().to_lowercase().starts_with('y') {
                println!("📖 Story generation paused. Current progress saved.");
                break;
            }
        }

        // Brief pause between chapters for system stability
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Apply final professional manuscript formatting
    println!("📖 Applying final manuscript formatting for publishing...");
    let formatted_manuscript = formatter.format_manuscript(
        &content_buffer,
        &genre,
        Some(&selected_persona.era),
        &PublishingType::Literary
    )?;

    // Write professionally formatted version
    let formatted_path = output_path.with_extension("formatted.txt");
    std::fs::write(&formatted_path, &formatted_manuscript.content)?;

    println!("\n🎉 Historical Persona Writing Complete!");
    println!("   📄 Final output: {}", output_path.display());
    println!("   📄 Formatted manuscript: {}", formatted_path.display());
    println!("   📊 Total size: {} KB", content_buffer.len() / 1024);
    println!("   📏 Publishing compliance: {:.1}%", formatted_manuscript.compliance_score * 100.0);
    println!("   ⭐ Presentation quality: {:.1}%", formatted_manuscript.presentation_quality * 100.0);
    println!("   🎭 Written in the style of: {}", selected_persona.writer_name);
    println!("   🏛️  Literary tradition: {:?}", selected_persona.movement);
    
    Ok(())
}

async fn generate_persona_premise(
    client: &AIClient,
    model: &str,
    genre: &Genre,
    style: &WritingStyle,
    persona: &crate::historical_writer_personas::HistoricalWriterPersona,
) -> Result<String> {
    let base_premise_prompt = format!(
        "Generate a compelling story premise for a {} {} story that would be written by {}.\n\
         Consider {}'s writing philosophy: {}\n\
         Key techniques to incorporate: {:?}\n\
         Literary movement: {:?}\n\
         Era: {}\n\n\
         Create a premise that captures {}'s distinctive voice, thematic concerns, and storytelling approach.\n\
         The premise should be 2-3 sentences that establish the central conflict and protagonist in {}'s characteristic style.\n\
         Premise:",
        genre, style, persona.writer_name,
        persona.writer_name, persona.writing_philosophy.core_beliefs.first().unwrap_or(&"Literary authenticity".to_string()),
        persona.signature_techniques.iter().take(3).collect::<Vec<_>>(),
        persona.movement,
        persona.era.to_string(),
        persona.writer_name,
        persona.writer_name
    );

    let premise = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_text(&base_premise_prompt, 150u32, 0.8).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_text(model, &base_premise_prompt, 150i32, 0.8).await?
        }
    };

    Ok(premise.trim().to_string())
}

fn create_persona_chapter_prompt(
    premise: &str,
    chapter_num: usize,
    total_chapters: usize,
    size: &BookSize,
    genre: &Genre,
    style: &WritingStyle,
    persona: &crate::historical_writer_personas::HistoricalWriterPersona,
) -> Result<String> {
    let words_per_chapter = match size {
        BookSize::ShortStory => 400,
        BookSize::Short => 700,
        BookSize::Medium => 900,
        BookSize::Large => 1100,
        BookSize::VeryLarge => 1500,
        BookSize::Epic => 1300,
        BookSize::Unlimited => 1800,
    };

    let chapter_context = if chapter_num == 1 {
        format!("This is the opening chapter. In {}'s style, establish the protagonist and setting with rich detail and atmosphere. Use {} techniques to create an engaging hook.", persona.writer_name, persona.signature_techniques[0].to_string())
    } else if chapter_num == total_chapters {
        format!("This is the final chapter. Conclude in {}'s distinctive style, providing resolution that reflects their philosophical outlook: {}", persona.writer_name, persona.writing_philosophy.core_beliefs.first().unwrap_or(&"Authentic human truth".to_string()))
    } else {
        format!("This chapter should develop the narrative using {}'s characteristic approach. Employ {} techniques while maintaining the {} literary tradition.", persona.writer_name, persona.signature_techniques.iter().take(2).map(|t| t.to_string()).collect::<Vec<_>>().join(" and "), persona.movement)
    };

    Ok(format!(
        "Write Chapter {} of a {} {} story in the distinctive style of {}.\n\n\
         PREMISE: {}\n\n\
         PERSONA GUIDANCE:\n\
         - Writer: {} ({})\n\
         - Philosophy: {}\n\
         - Key Techniques: {:?}\n\
         - Literary Movement: {:?}\n\
         - Era Characteristics: {}\n\n\
         CHAPTER CONTEXT: {}\n\n\
         TARGET LENGTH: Approximately {} words\n\n\
         STYLE REQUIREMENTS:\n\
         - Emulate {}'s distinctive voice and narrative approach\n\
         - Incorporate period-appropriate language and sensibilities\n\
         - Use {} signature techniques throughout\n\
         - Maintain thematic consistency with {} philosophical outlook\n\
         - Reflect {} literary movement characteristics\n\n\
         Write the chapter with authenticity to {}'s style while creating engaging, original content:",
        chapter_num, genre, style, persona.writer_name,
        premise,
        persona.writer_name, persona.era.to_string(),
        persona.writing_philosophy.core_beliefs.first().unwrap_or(&"Literary truth".to_string()),
        persona.signature_techniques,
        persona.movement,
        persona.era.to_string(),
        chapter_context,
        words_per_chapter,
        persona.writer_name,
        persona.writer_name,
        persona.writer_name,
        persona.movement,
        persona.writer_name
    ))
}