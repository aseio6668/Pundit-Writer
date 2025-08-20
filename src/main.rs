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
    }
    
    Ok(())
}
