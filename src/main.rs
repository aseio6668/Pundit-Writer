use clap::Parser;
use anyhow::Result;

mod cli;
mod content;
mod models;
mod ollama;
mod writer;
mod config;

use cli::{Args, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    match args.command {
        Commands::Book { 
            genre, 
            style, 
            size, 
            output, 
            model,
            api_key,
            local,
            ollama_url,
        } => {
            writer::write_book(genre, style, size, output, model, api_key, local, ollama_url).await?;
        },
        Commands::Screenplay {
            genre,
            style,
            length,
            output,
            model,
            api_key,
            local,
            ollama_url,
        } => {
            writer::write_screenplay(genre, style, length, output, model, api_key, local, ollama_url).await?;
        },
        Commands::Play {
            genre,
            style,
            length,
            output,
            model,
            api_key,
            local,
            ollama_url,
        } => {
            writer::write_play(genre, style, length, output, model, api_key, local, ollama_url).await?;
        },
        Commands::TvScript {
            show_type,
            genre,
            style,
            episodes,
            output,
            model,
            api_key,
            local,
            ollama_url,
        } => {
            writer::write_tv_script(show_type, genre, style, episodes, output, model, api_key, local, ollama_url).await?;
        },
        Commands::AudioScript {
            audio_type,
            genre,
            style,
            duration,
            output,
            model,
            api_key,
            local,
            ollama_url,
        } => {
            writer::write_audio_script(audio_type, genre, style, duration, output, model, api_key, local, ollama_url).await?;
        },
        Commands::GameScript {
            genre,
            style,
            characters,
            output,
            model,
            api_key,
            local,
            ollama_url,
        } => {
            writer::write_game_script(genre, style, characters, output, model, api_key, local, ollama_url).await?;
        },
        Commands::Document {
            doc_type,
            style,
            length,
            output,
            model,
            api_key,
            local,
            ollama_url,
        } => {
            writer::write_document(doc_type, style, length, output, model, api_key, local, ollama_url).await?;
        },
        Commands::Interactive => {
            writer::interactive_mode().await?;
        },
    }
    
    Ok(())
}
