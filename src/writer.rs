use crate::cli::{Genre, WritingStyle, BookSize, ScreenplayLength, PlayLength, TvShowType, AudioType, GameGenre, DocumentType, DocumentLength};
use crate::models::{HuggingFaceClient, get_model_recommendation};
use crate::ollama::{OllamaClient, get_ollama_recommendation, get_download_instructions};
use crate::content::{Content, Section, SectionType, ContentType, DocumentFormat, Book};
use crate::config::{Config, save_book_state};
use anyhow::Result;
use dialoguer::{Input, Select, Confirm};
use indicatif::{ProgressBar, ProgressStyle};
use console::Term;
use std::path::PathBuf;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

enum AIClient {
    HuggingFace(HuggingFaceClient),
    Ollama(OllamaClient),
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
    
    // Create book instance
    let mut book = Book::new(
        title,
        author,
        genre.to_string(),
        style.to_string(),
        premise,
        format!("{:?}", size),
        size.word_target(),
        size.chapter_target(),
        model.clone(),
    );
    
    println!("\n🎯 Book Configuration:");
    println!("   Genre: {}", book.genre);
    println!("   Style: {}", book.writing_style);
    println!("   Target: {:?}", size);
    if let Some(target) = book.metadata.target_word_count {
        println!("   Words: {} target", target);
    }
    println!("   Sections: {} planned", book.metadata.target_sections);
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
    let context = content.get_context_for_next_section();
    
    // Extract section outline from the content's overall outline
    let section_outline = extract_section_outline(&content.outline, section_number, &section_type)
        .unwrap_or_else(|| format!("{} {}: Continue the story", section_type_name(&section_type), section_number));
    
    progress_bar.set_message(format!("Writing {} {}...", section_type_name(&section_type), section_number));
    
    let target_words = content.metadata.generation_parameters.max_tokens_per_chapter as usize;
    
    // Generate section content
    let content_text = match client {
        AIClient::HuggingFace(hf_client) => {
            hf_client.generate_content_section(&content.content_type, &context, &section_outline, target_words).await?
        },
        AIClient::Ollama(ollama_client) => {
            ollama_client.generate_content_section(model, &content.content_type, &context, &section_outline, target_words).await?
        }
    };
    
    // Extract title from the generated content or use outline
    let title = extract_section_title(&content_text, &section_type)
        .unwrap_or_else(|| format!("{} {}", section_type_name(&section_type), section_number));
    
    let mut section = Section::new(section_number, title, section_outline, section_type);
    section.set_content(content_text.to_string());
    
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
    for line in full_outline.lines() {
        if line.trim().starts_with(&format!("{} {}:", section_name, section_number)) {
            return Some(line.trim().to_string());
        }
    }
    None
}

fn extract_section_title(content: &str, section_type: &SectionType) -> Option<String> {
    // Try to find a title in the first few lines based on content type
    for line in content.lines().take(5) {
        let line = line.trim();
        
        match section_type {
            SectionType::Scene => {
                if line.starts_with("INT.") || line.starts_with("EXT.") {
                    return Some(line.to_string());
                }
            },
            SectionType::Act => {
                if line.starts_with("ACT") || line.starts_with("Act") {
                    return Some(line.to_string());
                }
            },
            SectionType::Episode => {
                if line.starts_with("Episode") || line.starts_with("EPISODE") {
                    return Some(line.to_string());
                }
            },
            _ => {
                if line.starts_with(&format!("{}", section_type_name(section_type))) || line.starts_with("#") {
                    return Some(line.replace("#", "").trim().to_string());
                }
            }
        }
        
        if !line.is_empty() && line.len() < 100 {
            return Some(line.to_string());
        }
    }
    None
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
            .unwrap()
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
    };
    
    // Save as plain text
    let txt_path = output_dir.join(format!("{}_{}_{}.txt", content_type_name, safe_title, timestamp));
    fs::write(&txt_path, content.to_text())?;
    
    // Save as markdown
    let md_path = output_dir.join(format!("{}_{}_{}.md", content_type_name, safe_title, timestamp));
    fs::write(&md_path, content.to_markdown())?;
    
    // Save format-specific version for certain content types
    match content.content_type {
        ContentType::Screenplay => {
            let fdx_path = output_dir.join(format!("screenplay_{}_{}.fdx", safe_title, timestamp));
            fs::write(&fdx_path, format_as_final_draft(content))?;
            println!("   Final Draft: {}", fdx_path.display());
        },
        ContentType::Play => {
            let play_path = output_dir.join(format!("play_{}_{}.txt", safe_title, timestamp));
            fs::write(&play_path, format_as_stage_play(content))?;
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
    
    println!("{}", console::style("🎭 Pundit - Interactive Content Creator").bold().magenta());
    println!("═══════════════════════════════════════");
    println!();
    println!("Welcome to Pundit's interactive content creation mode!");
    println!("I'll help you choose the perfect content type and guide you through the creation process.");
    println!();
    
    // Content type selection
    let content_types = vec![
        "📚 Book - Traditional long-form narrative",
        "🎬 Screenplay - Movie script with professional formatting",
        "🎭 Stage Play - Theater script with stage directions",
        "📺 TV Script - Television episode or series",
        "🎧 Audio Script - Podcast, radio play, or audio drama",
        "🎮 Game Script - Interactive dialogue with branching choices",
        "📄 Business Document - Professional or technical document",
    ];
    
    let content_idx = Select::new()
        .with_prompt("What type of content would you like to create?")
        .items(&content_types)
        .default(0)
        .interact()?;
    
    match content_idx {
        0 => {
            // Redirect to existing narrate mode for books
            narrate_mode().await
        },
        1 => {
            // Screenplay creation
            interactive_screenplay_creation().await
        },
        2 => {
            // Stage play creation
            interactive_play_creation().await
        },
        3 => {
            // TV script creation
            interactive_tv_creation().await
        },
        4 => {
            // Audio script creation
            interactive_audio_creation().await
        },
        5 => {
            // Game script creation
            interactive_game_creation().await
        },
        6 => {
            // Document creation
            interactive_document_creation().await
        },
        _ => {
            println!("Invalid selection");
            Ok(())
        }
    }
}

async fn interactive_screenplay_creation() -> Result<()> {
    println!("\n🎬 Creating a Screenplay");
    println!("Let me ask you a few questions to create the perfect screenplay...\n");
    
    // Genre selection for screenplay
    let genres = vec!["Action", "Comedy", "Drama", "Horror", "Romance", "Sci-Fi", "Thriller"];
    let genre_idx = Select::new()
        .with_prompt("What genre is your screenplay?")
        .items(&genres)
        .default(2)
        .interact()?;
    
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
    ];
    
    let length_idx = Select::new()
        .with_prompt("What length screenplay would you like?")
        .items(&lengths)
        .default(1)
        .interact()?;
    
    let length = match length_idx {
        0 => ScreenplayLength::Short,
        1 => ScreenplayLength::Feature,
        2 => ScreenplayLength::Epic,
        _ => ScreenplayLength::Feature,
    };
    
    // Style selection
    let style = WritingStyle::Dramatic; // Default for screenplays
    
    // Model selection (simplified)
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama models? (otherwise will use cloud models)")
        .default(true)
        .interact()?;
    
    let model = if use_local {
        "llama3.2".to_string()
    } else {
        "gpt2".to_string()
    };
    
    // Create the screenplay
    write_screenplay(genre, style, length, None, model, None, use_local, "http://localhost:11434".to_string()).await
}

async fn interactive_play_creation() -> Result<()> {
    println!("\n🎭 Creating a Stage Play");
    println!("Let me ask you a few questions to create the perfect stage play...\n");
    
    // Genre selection for stage play
    let genres = vec![
        "Drama", "Comedy", "Musical", "Tragedy", "Romance", 
        "Historical", "Contemporary", "Experimental", "Children's"
    ];
    let genre_idx = Select::new()
        .with_prompt("What genre is your stage play?")
        .items(&genres)
        .default(0)
        .interact()?;
    
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
    ];
    
    let length_idx = Select::new()
        .with_prompt("What length play would you like?")
        .items(&lengths)
        .default(1)
        .interact()?;
    
    let length = match length_idx {
        0 => PlayLength::OneAct,
        1 => PlayLength::Full,
        2 => PlayLength::Musical,
        _ => PlayLength::Full,
    };
    
    // Style selection
    let styles = vec![
        "Dramatic", "Conversational", "Poetic", "Humorous", 
        "Formal", "Creative", "Minimalist"
    ];
    
    let style_idx = Select::new()
        .with_prompt("What writing style do you prefer?")
        .items(&styles)
        .default(0)
        .interact()?;
    
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
    
    // Model selection (simplified)
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama models? (otherwise will use cloud models)")
        .default(true)
        .interact()?;
    
    let model = if use_local {
        "llama3.2".to_string()
    } else {
        "gpt2".to_string()
    };
    
    // Create the stage play
    write_play(genre, style, length, None, model, None, use_local, "http://localhost:11434".to_string()).await
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
    
    // Model selection
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama models? (otherwise will use cloud models)")
        .default(true)
        .interact()?;
    
    let model = if use_local {
        "llama3.2".to_string()
    } else {
        "gpt2".to_string()
    };
    
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
        AudioType::AudioDrama | AudioType::RadioPlay => vec![
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
    };
    
    let genre_idx = Select::new()
        .with_prompt("What genre/topic is your audio content?")
        .items(&genres)
        .default(0)
        .interact()?;
    
    let genre = match audio_type {
        AudioType::AudioDrama | AudioType::RadioPlay => match genre_idx {
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
        AudioType::AudioDrama | AudioType::RadioPlay => vec![
            "Dramatic", "Conversational", "Descriptive", "Humorous", "Poetic"
        ],
        AudioType::Podcast => vec![
            "Conversational", "Formal", "Casual", "Journalistic", "Humorous"
        ],
        AudioType::Documentary => vec![
            "Formal", "Descriptive", "Journalistic", "Academic", "Narrative"
        ],
    };
    
    let style_idx = Select::new()
        .with_prompt("What writing style do you prefer?")
        .items(&styles)
        .default(0)
        .interact()?;
    
    let style = match audio_type {
        AudioType::AudioDrama | AudioType::RadioPlay => match style_idx {
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
    };
    
    // Model selection
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama models? (otherwise will use cloud models)")
        .default(true)
        .interact()?;
    
    let model = if use_local {
        "llama3.2".to_string()
    } else {
        "gpt2".to_string()
    };
    
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
    
    // Model selection
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama models? (otherwise will use cloud models)")
        .default(true)
        .interact()?;
    
    let model = if use_local {
        "llama3.2".to_string()
    } else {
        "gpt2".to_string()
    };
    
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
    
    // Model selection
    let use_local = Confirm::new()
        .with_prompt("Use local Ollama models? (otherwise will use cloud models)")
        .default(true)
        .interact()?;
    
    let model = if use_local {
        "llama3.2".to_string()
    } else {
        "gpt2".to_string()
    };
    
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
    
    // Save as plain text
    let txt_path = output_dir.join(format!("{}_{}.txt", safe_title, timestamp));
    fs::write(&txt_path, book.to_text())?;
    
    // Save as markdown
    let md_path = output_dir.join(format!("{}_{}.md", safe_title, timestamp));
    fs::write(&md_path, book.to_markdown())?;
    
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
    
    println!("{}", console::style("🎭 Pundit - Narration Mode").bold().magenta());
    println!("═══════════════════════════════════════");
    println!();
    println!("Welcome to Pundit's interactive narration mode!");
    println!("I'll ask you a few questions to understand what kind of book you'd like me to write.");
    println!();
    
    // Genre selection
    let genres = vec![
        "Fiction", "Non-Fiction", "Mystery", "Romance", "Science Fiction", 
        "Fantasy", "Horror", "Thriller", "Biography", "History", "Self-Help",
        "Technical", "Poetry", "Drama", "Comedy", "Adventure", "Crime"
    ];
    
    let genre_idx = Select::new()
        .with_prompt("What genre would you like your book to be?")
        .items(&genres)
        .default(0)
        .interact()?;
    
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
    let styles = vec![
        "Conversational", "Descriptive", "Narrative", "Creative", "Formal",
        "Casual", "Poetic", "Humorous", "Dramatic", "First Person", "Third Person"
    ];
    
    let style_idx = Select::new()
        .with_prompt("What writing style do you prefer?")
        .items(&styles)
        .default(0)
        .interact()?;
    
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
    let sizes = vec![
        "Short Story (1,000-7,500 words)",
        "Short Book (20,000-50,000 words)",
        "Medium Book (50,000-80,000 words)",
        "Large Book (80,000-120,000 words)",
        "Very Large Book (120,000-200,000 words)",
        "Unlimited (Let Pundit decide when to stop)"
    ];
    
    let size_idx = Select::new()
        .with_prompt("How long should your book be?")
        .items(&sizes)
        .default(2)
        .interact()?;
    
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
    println!("\n🤖 Model Selection:");
    let model_options = vec![
        "🏠 Local models (Ollama) - No API key needed, runs offline",
        "☁️  Cloud models (HuggingFace) - Requires API key or limited free tier"
    ];
    
    let model_type_idx = Select::new()
        .with_prompt("Which type of model would you like to use?")
        .items(&model_options)
        .default(0) // Default to local
        .interact()?;
    
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
    write_book(genre, style, size, None, model, api_key, use_local, "http://localhost:11434".to_string()).await
}

fn size_to_description(size: &BookSize) -> &'static str {
    match size {
        BookSize::ShortStory => "short story",
        BookSize::Short => "short",
        BookSize::Medium => "medium-length",
        BookSize::Large => "large",
        BookSize::VeryLarge => "very large",
        BookSize::Unlimited => "unlimited-length",
    }
}