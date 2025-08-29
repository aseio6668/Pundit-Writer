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
mod self_healing_writer;
mod adaptive_learning_engine;
mod error_pattern_recognition;
mod enhanced_writer_system;
mod advanced_learning_system;
mod cognitive_writing_engine;
mod neural_creativity_enhancer;
mod master_intelligence_system;
mod nonstop_learning_mode;
mod soul_memory;
mod soul_memory_manager;
mod soul_memory_cli;
mod narrative_flow_monitor;
mod intelligent_pivot_engine;
mod flow_aware_writer;
mod metaphorical_writer;
mod emotional_writing_engine;
mod enhanced_writer_integration;

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
        
        Commands::Superintelligent { 
            genre, style, size, output, model, api_key, local, ollama_url, chapters,
            intelligence_level, learning_acceleration, creativity_enhancement,
            autonomous_improvement, meta_cognition, consciousness_level, continuous_learning
        } => {
            let parsed_genre = parse_genre(&genre)?;
            let parsed_style = parse_writing_style(&style)?;
            let parsed_size = parse_book_size(&size)?;
            
            println!("🚀 ACTIVATING MASTER INTELLIGENCE SYSTEM");
            println!("══════════════════════════════════════════");
            println!("   🧠 Intelligence Level: {:.1}%", intelligence_level * 100.0);
            println!("   ⚡ Learning Acceleration: {:.1}x", learning_acceleration);
            println!("   🎨 Creativity Enhancement: {:.1}x", creativity_enhancement);
            println!("   🤖 Autonomous Improvement: {}", autonomous_improvement);
            println!("   🤔 Meta-Cognition: {}", meta_cognition);
            println!("   🌟 Consciousness Level: {:.1}%", consciousness_level * 100.0);
            println!("   📚 Continuous Learning: {}", continuous_learning);
            println!("   📖 Target Chapters: {}", chapters);
            println!();
            
            use crate::master_intelligence_system::MasterIntelligenceSystem;
            let master_system = MasterIntelligenceSystem::new();
            
            // Display intelligence dashboard
            master_system.display_intelligence_dashboard().await?;
            
            // Start superintelligent writing mode
            println!("\n🚀 Initiating Super-Intelligent Writing Process...");
            
            // For now, call the enhanced writing mode with superintelligent parameters
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
                100 * 1024 * 1024, // 100MB buffer
                continuous_learning,
                true, // Enable all recovery features
            ).await?;
        },
        
        Commands::LearnOptimize { 
            focus, target_improvement, duration, experimental, detailed_report 
        } => {
            println!("🎓 LEARNING OPTIMIZATION SESSION");
            println!("═══════════════════════════════════");
            println!("   🎯 Target Improvement: {:.1}%", target_improvement);
            println!("   ⏱️  Duration: {} minutes", duration);
            println!("   🔬 Experimental Mode: {}", experimental);
            
            if let Some(focus_area) = &focus {
                println!("   🔍 Focus Area: {}", focus_area);
            }
            
            use crate::master_intelligence_system::MasterIntelligenceSystem;
            let master_system = MasterIntelligenceSystem::new();
            
            // Run adaptive learning session
            let learning_report = master_system.adaptive_learning_session().await?;
            
            println!("\n✅ Learning Session Complete!");
            println!("   📚 Capabilities Improved: {}", learning_report.capability_improvements.len());
            println!("   🔄 Systems Integrated: {}", learning_report.knowledge_integrations.systems_integrated);
            
            if detailed_report {
                println!("\n📊 DETAILED LEARNING REPORT");
                println!("═══════════════════════════════");
                
                println!("\n🎯 Capability Improvements:");
                for improvement in &learning_report.capability_improvements {
                    println!("   • {}", improvement);
                }
                
                println!("\n🔗 Knowledge Integrations:");
                println!("   • Cross-system learning connections established");
                println!("   • Pattern recognition enhanced");
                println!("   • Creative intelligence expanded");
                
                println!("\n📈 Next Learning Priorities:");
                for priority in &learning_report.next_learning_priorities {
                    println!("   • {}", priority);
                }
            }
            
            // Consciousness expansion if high target improvement
            if target_improvement > 20.0 {
                println!("\n🌟 High improvement target detected - initiating consciousness expansion...");
                let expansion_report = master_system.consciousness_expansion(0.8).await?;
                println!("   ✨ Consciousness expanded by {:.1}%", expansion_report.expansion_achieved * 100.0);
            }
        },
        
        Commands::SoulMemory { command } => {
            use crate::soul_memory_cli::{SoulMemoryArgs, handle_soul_memory_command};
            let soul_args = SoulMemoryArgs { command };
            handle_soul_memory_command(soul_args).await?;
        },
        
        Commands::EmotionalWrite { genre, style, content_type, output, model, theme, show_journey, ollama_url } => {
            use crate::emotional_writing_engine::EmotionalWritingEngine;
            
            let parsed_genre = parse_genre(&genre)?;
            let parsed_style = parse_writing_style(&style)?;
            
            println!("🎭 Starting Emotional Writing Session");
            println!("   Genre: {}", parsed_genre);
            println!("   Style: {}", parsed_style);
            println!("   Theme: {}", theme.as_deref().unwrap_or("Creative exploration"));
            println!("   Emotional journey logging: {}", show_journey);
            println!();
            
            let mut engine = EmotionalWritingEngine::new(&ollama_url)?;
            
            let project_desc = theme.as_deref().unwrap_or("A creative writing project");
            engine.begin_writing_session(&parsed_genre, &parsed_style, project_desc).await?;
            
            // Create a simple prompt based on content type
            let base_prompt = match content_type.as_str() {
                "scene" => format!("Write a compelling scene in the {} genre with {} style. Theme: {}", 
                    format!("{:?}", parsed_genre), 
                    format!("{:?}", parsed_style), 
                    project_desc),
                "chapter" => format!("Write an engaging chapter in the {} genre with {} style. Theme: {}", 
                    format!("{:?}", parsed_genre), 
                    format!("{:?}", parsed_style), 
                    project_desc),
                _ => format!("Write creatively in the {} genre with {} style. Theme: {}", 
                    format!("{:?}", parsed_genre), 
                    format!("{:?}", parsed_style), 
                    project_desc),
            };
            
            // Create minimal content context for the writing
            let content = crate::content::Content::new(
                format!("{:?}", parsed_genre),
                format!("{:?}", parsed_style),
                crate::cli_types::BookSize::Medium
            );
            
            let result = engine.write_with_soul(
                &base_prompt,
                &model,
                &parsed_genre,
                &parsed_style,
                &content,
                "Creative writing session",
                Some(1000)
            ).await?;
            
            println!("\n📖 Generated Content:");
            println!("────────────────────");
            println!("{}", result.final_content);
            
            if show_journey {
                println!("\n🎭 Emotional Journey:");
                println!("─────────────────────");
                for (i, thought) in result.emotional_journey.iter().enumerate() {
                    println!("{}. 💭 {}", i + 1, thought);
                }
                
                if !result.breaks_taken.is_empty() {
                    println!("\n🚶 Breaks Taken:");
                    for break_exp in &result.breaks_taken {
                        println!("   • {:?} - {}", break_exp.break_type, 
                                 break_exp.insights_gained.get(0).unwrap_or(&"Refreshed perspective".to_string()));
                    }
                }
                
                if !result.creative_insights.is_empty() {
                    println!("\n✨ Creative Insights:");
                    for insight in &result.creative_insights {
                        println!("   💡 {}", insight);
                    }
                }
            }
            
            let session_summary = engine.end_writing_session();
            println!("\n📊 Session Summary:");
            println!("   Satisfaction: {:.1}%", session_summary.session_satisfaction * 100.0);
            println!("   Creative energy remaining: {:.1}%", session_summary.creative_energy_remaining * 100.0);
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, &result.final_content)?;
                println!("💾 Content saved to: {:?}", output_path);
            }
        },
    }
    
    Ok(())
}
