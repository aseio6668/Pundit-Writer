use clap::Parser;
use anyhow::Result;

mod cli_simple;

use cli_simple::{Args, Commands};

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
            println!("✍️  Starting book generation...");
            println!("📚 Genre: {}", genre);
            println!("✒️  Style: {}", style);
            println!("📖 Size: {:?}", size);
            println!("🤖 Model: {}", model);
            if local {
                println!("🏠 Using local Ollama at: {}", ollama_url);
            } else {
                println!("☁️  Using HuggingFace API");
            }
            if let Some(output_path) = output {
                println!("💾 Output: {:?}", output_path);
            }
            println!("✅ Basic parsing successful! (Full implementation not included in simple version)");
        },
        Commands::Interactive => {
            println!("🚀 Interactive mode would start here");
        },
    }
    
    Ok(())
}