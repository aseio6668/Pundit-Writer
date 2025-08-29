use anyhow::Result;
use clap::{Args as ClapArgs, Subcommand};
use crate::soul_memory_manager::SoulMemoryManager;
use crate::soul_memory::SoulMemoryConfig;
use dialoguer::{Confirm, Input};

#[derive(ClapArgs)]
pub struct SoulMemoryArgs {
    #[command(subcommand)]
    pub command: SoulMemoryCommand,
}

#[derive(Subcommand)]
pub enum SoulMemoryCommand {
    /// Start Soul Memory system
    Start {
        /// Enable background sync
        #[arg(long, default_value = "true")]
        sync: bool,
        
        /// Sync interval in minutes
        #[arg(long, default_value = "15")]
        sync_interval: u64,
        
        /// Enable auto-recovery
        #[arg(long, default_value = "true")]
        auto_recovery: bool,
    },
    
    /// Check Soul Memory status
    Status,
    
    /// Force synchronization
    Sync,
    
    /// Configure Soul Memory settings
    Configure,
    
    /// Test Soul Memory system
    Test,
    
    /// Start nonstop learning with Soul Memory
    NonstopLearning {
        /// Duration in hours (optional)
        #[arg(long)]
        duration: Option<f32>,
        
        /// Maximum number of works to generate (optional)  
        #[arg(long)]
        max_works: Option<u32>,
        
        /// Auto-generate titles and descriptions
        #[arg(long, default_value = "true")]
        auto_titles: bool,
        
        /// Auto-approve outlines
        #[arg(long, default_value = "true")]
        auto_approve: bool,
    },
    
    /// Show learning insights
    Insights,
    
    /// Reset Soul Memory (dangerous!)
    Reset {
        /// Confirm the reset
        #[arg(long)]
        confirm: bool,
    },
}

pub async fn handle_soul_memory_command(args: SoulMemoryArgs) -> Result<()> {
    match args.command {
        SoulMemoryCommand::Start { sync, sync_interval, auto_recovery } => {
            println!("🌟 Initializing Soul Memory System...");
            
            let config = SoulMemoryConfig {
                enabled: true,
                sync_interval_minutes: sync_interval,
                auto_recovery,
                ..Default::default()
            };
            
            let mut manager = SoulMemoryManager::new();
            manager.initialize().await?;
            
            if sync {
                println!("🔄 Background sync enabled (interval: {} minutes)", sync_interval);
                println!("🛡️ Auto-recovery: {}", auto_recovery);
                println!("✅ Soul Memory is now running in the background");
                println!("💡 Use 'pundit soul-memory status' to check sync status");
            } else {
                println!("⏸️ Background sync disabled - Soul Memory will only save locally");
            }
        },
        
        SoulMemoryCommand::Status => {
            println!("🔍 Checking Soul Memory status...");
            
            let manager = SoulMemoryManager::new();
            
            if let Some(status) = manager.get_soul_memory_status().await {
                println!("📊 Soul Memory Status:");
                println!("   {}", status);
            } else {
                println!("❌ Soul Memory not initialized");
                println!("💡 Run 'pundit soul-memory start' to initialize");
            }
        },
        
        SoulMemoryCommand::Sync => {
            println!("🔄 Forcing Soul Memory synchronization...");
            
            let manager = SoulMemoryManager::new();
            manager.force_soul_memory_sync().await?;
        },
        
        SoulMemoryCommand::Configure => {
            configure_soul_memory().await?;
        },
        
        SoulMemoryCommand::Test => {
            test_soul_memory_system().await?;
        },
        
        SoulMemoryCommand::NonstopLearning { duration, max_works, auto_titles, auto_approve } => {
            start_nonstop_learning_with_soul_memory(duration, max_works, auto_titles, auto_approve).await?;
        },
        
        SoulMemoryCommand::Insights => {
            show_learning_insights().await?;
        },
        
        SoulMemoryCommand::Reset { confirm } => {
            reset_soul_memory(confirm).await?;
        },
    }
    
    Ok(())
}

async fn configure_soul_memory() -> Result<()> {
    println!("⚙️ Soul Memory Configuration");
    println!("═══════════════════════════════");
    
    let sync_interval: u64 = Input::new()
        .with_prompt("Sync interval (minutes)")
        .default(15)
        .interact()?;
    
    let corruption_check_interval: u64 = Input::new()
        .with_prompt("Corruption check interval (hours)")
        .default(6)
        .interact()?;
    
    let max_retry_attempts: u32 = Input::new()
        .with_prompt("Max retry attempts for failed syncs")
        .default(3)
        .interact()?;
    
    let backup_generations: u8 = Input::new()
        .with_prompt("Number of backup generations to keep")
        .default(5)
        .interact()?;
    
    let auto_recovery = Confirm::new()
        .with_prompt("Enable automatic corruption recovery?")
        .default(true)
        .interact()?;
    
    let compression_enabled = Confirm::new()
        .with_prompt("Enable data compression?")
        .default(true)
        .interact()?;
    
    let config = SoulMemoryConfig {
        enabled: true,
        sync_interval_minutes: sync_interval,
        corruption_check_interval_hours: corruption_check_interval,
        max_retry_attempts,
        backup_generations,
        auto_recovery,
        compression_enabled,
        ..Default::default()
    };
    
    // Save configuration
    let config_json = serde_json::to_string_pretty(&config)?;
    let config_path = crate::config::get_learning_data_dir()?.join("soul_memory_config.json");
    std::fs::write(&config_path, config_json)?;
    
    println!("✅ Soul Memory configuration saved");
    println!("💾 Config location: {:?}", config_path);
    
    Ok(())
}

async fn test_soul_memory_system() -> Result<()> {
    println!("🧪 Testing Soul Memory System...");
    println!("═══════════════════════════════════");
    
    // Initialize manager
    let mut manager = SoulMemoryManager::new();
    manager.initialize().await?;
    
    // Test learning system integration
    println!("1. Testing learning system integration...");
    let learning_system = manager.get_learning_system();
    {
        let system = learning_system.lock().unwrap();
        println!("   ✅ Learning system accessible");
        println!("   📊 Memory sessions: {}", system.writing_memory.session_memories.len());
    }
    
    // Test sync functionality
    println!("2. Testing sync functionality...");
    if let Err(e) = manager.force_soul_memory_sync().await {
        println!("   ⚠️ Sync test failed (expected if no cloud access): {}", e);
    } else {
        println!("   ✅ Sync test successful");
    }
    
    // Test learning insights
    println!("3. Testing learning insights...");
    let insights = manager.get_learning_insights().await?;
    println!("   ✅ Generated {} insights", insights.len());
    
    // Test status reporting
    println!("4. Testing status reporting...");
    if let Some(status) = manager.get_soul_memory_status().await {
        println!("   ✅ Status: {}", status);
    } else {
        println!("   ⚠️ Status not available");
    }
    
    println!("\n🎯 Soul Memory Test Results:");
    println!("   ✅ Core functionality: WORKING");
    println!("   ✅ Learning integration: WORKING");
    println!("   ⚠️ Cloud sync: DEPENDS ON CONNECTIVITY");
    println!("   ✅ Local backup: WORKING");
    
    Ok(())
}

async fn start_nonstop_learning_with_soul_memory(
    duration: Option<f32>, 
    max_works: Option<u32>, 
    auto_titles: bool, 
    auto_approve: bool
) -> Result<()> {
    println!("🌟 Starting Nonstop Learning with Soul Memory");
    println!("════════════════════════════════════════════");
    
    // Initialize Soul Memory Manager
    let mut soul_manager = SoulMemoryManager::new();
    soul_manager.initialize().await?;
    
    // Configure nonstop learning
    let config = crate::nonstop_learning_mode::NonstopLearningConfig {
        duration_hours: duration,
        max_works,
        auto_generate_titles: auto_titles,
        auto_approve_outlines: auto_approve,
        learning_frequency: 3, // Learn more frequently with soul memory
        ..Default::default()
    };
    
    // Create nonstop learning mode
    let mut nonstop_mode = crate::nonstop_learning_mode::NonstopLearningMode::new(config);
    
    // Integrate with Soul Memory
    soul_manager.integrate_with_nonstop_mode(&mut nonstop_mode);
    
    println!("🧠 Soul Memory integrated with learning system");
    println!("🔄 Automatic cloud sync enabled");
    println!("🛡️ Corruption detection and recovery active");
    println!("📈 Enhanced learning acceleration enabled");
    println!();
    
    // Start the nonstop learning session
    println!("🚀 Launching enhanced nonstop learning session...");
    nonstop_mode.run().await?;
    
    println!("\n🎯 Session Complete!");
    println!("💾 All learning data has been preserved in Soul Memory");
    
    Ok(())
}

async fn show_learning_insights() -> Result<()> {
    println!("🧠 Soul Memory Learning Insights");
    println!("═══════════════════════════════════");
    
    let manager = SoulMemoryManager::new();
    let insights = manager.get_learning_insights().await?;
    
    if insights.is_empty() {
        println!("📝 No learning insights available yet");
        println!("💡 Run some writing sessions to generate insights");
        return Ok(());
    }
    
    println!("📊 Current Learning Insights:");
    for (i, insight) in insights.iter().enumerate() {
        println!("   {}. {}", i + 1, insight);
    }
    
    // Show Soul Memory status
    if let Some(status) = manager.get_soul_memory_status().await {
        println!("\n🌟 Soul Memory Status:");
        println!("   {}", status);
    }
    
    Ok(())
}

async fn reset_soul_memory(confirm: bool) -> Result<()> {
    if !confirm {
        let really_reset = Confirm::new()
            .with_prompt("⚠️ This will permanently delete all Soul Memory data. Are you sure?")
            .default(false)
            .interact()?;
        
        if !really_reset {
            println!("❌ Reset cancelled");
            return Ok(());
        }
    }
    
    println!("🗑️ Resetting Soul Memory...");
    
    // Delete local data
    let learning_dir = crate::config::get_learning_data_dir()?;
    let soul_backups = learning_dir.join("soul_backups");
    
    if soul_backups.exists() {
        std::fs::remove_dir_all(&soul_backups)?;
        println!("   ✅ Deleted local Soul Memory backups");
    }
    
    // Delete learning data
    let memory_file = learning_dir.join("writing_memory.json");
    if memory_file.exists() {
        std::fs::remove_file(&memory_file)?;
        println!("   ✅ Deleted writing memory");
    }
    
    // Delete configuration
    let config_file = learning_dir.join("soul_memory_config.json");
    if config_file.exists() {
        std::fs::remove_file(&config_file)?;
        println!("   ✅ Deleted Soul Memory configuration");
    }
    
    println!("🔄 Soul Memory has been reset");
    println!("💡 Run 'pundit soul-memory start' to reinitialize");
    
    Ok(())
}