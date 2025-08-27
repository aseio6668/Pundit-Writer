use anyhow::{Result, anyhow};
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::time::{sleep, timeout};
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingSession {
    pub id: String,
    pub started_at: DateTime<Utc>,
    pub last_checkpoint: DateTime<Utc>,
    pub total_words: usize,
    pub sections_completed: usize,
    pub current_section: usize,
    pub output_path: PathBuf,
    pub auto_save_enabled: bool,
}

impl WritingSession {
    pub fn new(output_path: PathBuf) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            started_at: now,
            last_checkpoint: now,
            total_words: 0,
            sections_completed: 0,
            current_section: 0,
            output_path,
            auto_save_enabled: true,
        }
    }

    pub fn save_checkpoint(&mut self, content: &str) -> Result<()> {
        if !self.auto_save_enabled {
            return Ok(());
        }

        let checkpoint_path = self.get_checkpoint_path();
        fs::write(&checkpoint_path, content)?;
        self.last_checkpoint = Utc::now();
        
        let session_path = self.get_session_path();
        let session_json = serde_json::to_string_pretty(self)?;
        fs::write(&session_path, session_json)?;
        
        Ok(())
    }

    pub fn restore_from_checkpoint(&self) -> Result<Option<String>> {
        let checkpoint_path = self.get_checkpoint_path();
        if checkpoint_path.exists() {
            Ok(Some(fs::read_to_string(checkpoint_path)?))
        } else {
            Ok(None)
        }
    }

    fn get_checkpoint_path(&self) -> PathBuf {
        let mut path = self.output_path.clone();
        path.set_extension("checkpoint");
        path
    }

    fn get_session_path(&self) -> PathBuf {
        let mut path = self.output_path.clone();
        path.set_extension("session");
        path
    }

    pub fn cleanup_checkpoint(&self) -> Result<()> {
        let checkpoint_path = self.get_checkpoint_path();
        let session_path = self.get_session_path();
        
        if checkpoint_path.exists() {
            fs::remove_file(checkpoint_path)?;
        }
        if session_path.exists() {
            fs::remove_file(session_path)?;
        }
        
        Ok(())
    }
}

pub struct ResilientWriter {
    session: WritingSession,
    retry_attempts: usize,
    base_delay: Duration,
    max_delay: Duration,
    timeout_duration: Duration,
    content_buffer: crate::memory_utils::MemoryOptimizedBuffer,
}

impl ResilientWriter {
    pub fn new(output_path: PathBuf, buffer_size: usize) -> Self {
        Self {
            session: WritingSession::new(output_path),
            retry_attempts: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            timeout_duration: Duration::from_secs(120),
            content_buffer: crate::memory_utils::MemoryOptimizedBuffer::new(buffer_size),
        }
    }

    pub fn with_retry_config(mut self, attempts: usize, base_delay: Duration, max_delay: Duration) -> Self {
        self.retry_attempts = attempts;
        self.base_delay = base_delay;
        self.max_delay = max_delay;
        self
    }

    pub fn with_timeout(mut self, timeout_duration: Duration) -> Self {
        self.timeout_duration = timeout_duration;
        self
    }

    pub async fn write_with_resilience<F, Fut>(&mut self, operation: F) -> Result<String>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<String>> + Send,
    {
        let mut last_error = None;
        let mut delay = self.base_delay;

        for attempt in 1..=self.retry_attempts {
            match timeout(self.timeout_duration, operation()).await {
                Ok(Ok(result)) => {
                    self.content_buffer.push(result.clone())?;
                    self.session.save_checkpoint(&self.content_buffer.get_all())?;
                    return Ok(result);
                },
                Ok(Err(e)) => {
                    last_error = Some(e);
                    eprintln!("Attempt {} failed: {:?}", attempt, last_error);
                },
                Err(_) => {
                    last_error = Some(anyhow!("Operation timed out after {:?}", self.timeout_duration));
                    eprintln!("Attempt {} timed out", attempt);
                }
            }

            if attempt < self.retry_attempts {
                eprintln!("Retrying in {:?}...", delay);
                sleep(delay).await;
                delay = std::cmp::min(delay * 2, self.max_delay);
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("All retry attempts failed")))
    }

    pub fn get_session(&self) -> &WritingSession {
        &self.session
    }

    pub fn get_session_mut(&mut self) -> &mut WritingSession {
        &mut self.session
    }

    pub fn get_buffered_content(&self) -> String {
        self.content_buffer.get_all()
    }

    pub fn clear_buffer(&mut self) {
        self.content_buffer.clear();
    }

    pub async fn finalize_session(&mut self) -> Result<()> {
        let final_content = self.content_buffer.get_all();
        fs::write(&self.session.output_path, final_content)?;
        self.session.cleanup_checkpoint()?;
        Ok(())
    }
}

pub struct NonStopWritingMode {
    writer: ResilientWriter,
    target_sections: usize,
    current_section: usize,
    auto_continue: bool,
    progress_callback: Option<Box<dyn Fn(usize, usize) -> Result<()> + Send + Sync>>,
}

impl NonStopWritingMode {
    pub fn new(writer: ResilientWriter, target_sections: usize) -> Self {
        Self {
            writer,
            target_sections,
            current_section: 0,
            auto_continue: true,
            progress_callback: None,
        }
    }

    pub fn with_progress_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, usize) -> Result<()> + Send + Sync + 'static,
    {
        self.progress_callback = Some(Box::new(callback));
        self
    }

    pub async fn run<F, Fut>(&mut self, section_generator: F) -> Result<String>
    where
        F: Fn(usize) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<String>> + Send,
    {
        println!("🚀 Starting non-stop writing mode ({} sections)...", self.target_sections);

        while self.current_section < self.target_sections {
            self.current_section += 1;
            
            if let Some(ref callback) = self.progress_callback {
                callback(self.current_section, self.target_sections)?;
            }

            let section_content = self.writer.write_with_resilience(|| {
                section_generator(self.current_section)
            }).await?;

            self.writer.get_session_mut().sections_completed += 1;
            self.writer.get_session_mut().total_words += section_content.split_whitespace().count();

            println!("✅ Section {} completed ({} words)", 
                self.current_section, 
                section_content.split_whitespace().count()
            );

            if !self.auto_continue && self.current_section < self.target_sections {
                println!("⏸️  Pausing... Press Enter to continue or Ctrl+C to stop");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
            }
        }

        self.writer.finalize_session().await?;
        Ok(self.writer.get_buffered_content())
    }

    pub fn enable_auto_continue(&mut self, enabled: bool) {
        self.auto_continue = enabled;
    }
}

pub fn create_resilient_writing_session(
    output_path: PathBuf,
    buffer_size: usize,
    target_sections: usize,
) -> NonStopWritingMode {
    let writer = ResilientWriter::new(output_path, buffer_size)
        .with_retry_config(
            5, 
            Duration::from_secs(2), 
            Duration::from_secs(60)
        )
        .with_timeout(Duration::from_secs(180));

    NonStopWritingMode::new(writer, target_sections)
        .with_progress_callback(|current, total| {
            println!("📝 Progress: {}/{} sections ({:.1}%)", 
                current, total, (current as f64 / total as f64) * 100.0);
            Ok(())
        })
}