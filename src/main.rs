use anyhow::Result;
use clap::Parser;

mod cli_types;
mod content;
mod models;
mod ollama;
mod writer;
mod config;
mod simple_cli;
mod poetry_enhancements;
mod continuation;
mod dynamic_length;
mod creative_enhancement;
mod memory_utils;
mod resilient_writer;
mod temporal_engine;
mod advanced_creativity_engine;
mod intelligent_progression_tracker;
mod historical_writer_personas;
mod period_language_engine;
mod creative_block_recovery;
mod professional_formatting;

use simple_cli::{Args, Commands, parse_genre, parse_writing_style, parse_book_size, parse_screenplay_length, parse_play_length};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    match args.command {
        Commands::Book { genre, style, size, output, model, api_key, local, ollama_url } => {
            let parsed_genre = parse_genre(&genre)?;
            let parsed_style = parse_writing_style(&style)?;
            let parsed_size = parse_book_size(&size)?;
            
            println!("🚀 Generating {} {} book in {} style...", parsed_size.to_string(), parsed_genre, parsed_style);
            
            writer::write_book(
                parsed_genre,
                parsed_style,
                parsed_size,
                output,
                model,
                api_key,
                local,
                ollama_url,
            ).await?;
        },
        
        Commands::Screenplay { genre, style, length, output, model, api_key, local, ollama_url } => {
            let parsed_genre = parse_genre(&genre)?;
            let parsed_style = parse_writing_style(&style)?;
            let parsed_length = parse_screenplay_length(&length)?;
            
            println!("🎬 Generating {} screenplay in {} style...", parsed_genre, parsed_style);
            
            writer::write_screenplay(
                parsed_genre,
                parsed_style,
                parsed_length,
                output,
                model,
                api_key,
                local,
                ollama_url,
            ).await?;
        },
        
        Commands::Play { genre, style, length, output, model, api_key, local, ollama_url } => {
            let parsed_genre = parse_genre(&genre)?;
            let parsed_style = parse_writing_style(&style)?;
            let parsed_length = parse_play_length(&length)?;
            
            println!("🎭 Generating {} play in {} style...", parsed_genre, parsed_style);
            
            writer::write_play(
                parsed_genre,
                parsed_style,
                parsed_length,
                output,
                model,
                api_key,
                local,
                ollama_url,
            ).await?;
        },
        
        Commands::Interactive => {
            println!("🚀 Pundit Writer - Interactive Mode");
            writer::interactive_mode().await?;
        },
        
        Commands::NonStop { genre, style, size, output, model, api_key, local, ollama_url, sections, buffer_size, auto_continue } => {
            let parsed_genre = parse_genre(&genre)?;
            let parsed_style = parse_writing_style(&style)?;
            let parsed_size = parse_book_size(&size)?;
            
            println!("🌟 Starting Non-Stop Writing Mode...");
            println!("   Genre: {}", parsed_genre);
            println!("   Style: {}", parsed_style);
            println!("   Size: {}", parsed_size.to_string());
            println!("   Sections: {}", sections);
            println!("   Auto-continue: {}", auto_continue);
            
            writer::non_stop_writing_mode(
                parsed_genre,
                parsed_style,
                parsed_size,
                output,
                model,
                api_key,
                local,
                ollama_url,
                sections,
                buffer_size * 1024 * 1024, // Convert MB to bytes
                auto_continue,
            ).await?;
        },
        
        Commands::Enhanced { genre, style, size, output, model, api_key, local, ollama_url, chapters, buffer_size, auto_continue, interruption_recovery } => {
            let parsed_genre = parse_genre(&genre)?;
            let parsed_style = parse_writing_style(&style)?;
            let parsed_size = parse_book_size(&size)?;
            
            println!("🧠 Starting Enhanced Intelligent Writing Mode...");
            println!("   Genre: {}", parsed_genre);
            println!("   Style: {}", parsed_style);
            println!("   Size: {}", parsed_size.to_string());
            println!("   Chapters: {}", chapters);
            println!("   Advanced AI Systems: ENABLED");
            println!("   Interruption Recovery: {}", interruption_recovery);
            
            writer::enhanced_intelligent_writing_mode(
                parsed_genre,
                parsed_style,
                parsed_size,
                output,
                model,
                api_key,
                local,
                ollama_url,
                chapters,
                buffer_size * 1024 * 1024, // Convert MB to bytes
                auto_continue,
                interruption_recovery,
            ).await?;
        },
        
        Commands::Persona { genre, style, size, output, model, api_key, local, ollama_url, chapters, writer, era, auto_continue, language_enhancement } => {
            let parsed_genre = parse_genre(&genre)?;
            let parsed_style = parse_writing_style(&style)?;
            let parsed_size = parse_book_size(&size)?;
            
            println!("🎭 Starting Historical Writer Persona Mode...");
            println!("   Genre: {}", parsed_genre);
            println!("   Style: {}", parsed_style);
            println!("   Size: {}", parsed_size.to_string());
            println!("   Chapters: {}", chapters);
            
            if let Some(ref w) = writer {
                println!("   Target Writer: {}", w);
            }
            if let Some(ref e) = era {
                println!("   Target Era: {}", e);
            }
            println!("   Language Enhancement: {}", language_enhancement);
            println!("   🎨 Historical persona system: ENABLED");
            
            writer::historical_persona_writing_mode(
                parsed_genre,
                parsed_style,
                parsed_size,
                output,
                model,
                api_key,
                local,
                ollama_url,
                chapters,
                writer,
                era,
                auto_continue,
                language_enhancement,
            ).await?;
        },
    }
    
    Ok(())
}
