use std::sync::{Arc, Mutex};
use chrono::Utc;
use anyhow::Result;
use crate::cli_types::{Genre, WritingStyle, BookSize};
use crate::self_healing_writer::{SelfHealingWriter, GenerationPhase, PausePoint};
use crate::adaptive_learning_engine::{AdaptiveLearningEngine, UserFeedback, ContentIssue, QualityPredictions};
use crate::error_pattern_recognition::{ErrorPatternRecognition, ErrorPrediction};
use dialoguer::{Input, Select, Confirm, MultiSelect};

#[derive(Debug, Clone)]
pub struct EnhancedWriterSystem {
    pub self_healing: Arc<Mutex<SelfHealingWriter>>,
    pub adaptive_learning: Arc<Mutex<AdaptiveLearningEngine>>,
    pub error_recognition: Arc<Mutex<ErrorPatternRecognition>>,
    pub enhancement_enabled: bool,
    pub auto_improvement_threshold: f32,
    pub user_feedback_frequency: u32,
}

impl EnhancedWriterSystem {
    pub fn new() -> Self {
        Self {
            self_healing: Arc::new(Mutex::new(SelfHealingWriter::new())),
            adaptive_learning: Arc::new(Mutex::new(AdaptiveLearningEngine::new())),
            error_recognition: Arc::new(Mutex::new(ErrorPatternRecognition::new())),
            enhancement_enabled: true,
            auto_improvement_threshold: 0.7,
            user_feedback_frequency: 5, // Request feedback every 5 generations
        }
    }

    pub async fn enhanced_generation_with_learning(
        &self,
        client: &crate::writer::AIClient,
        model: &str,
        prompt: &str,
        chapter_num: usize,
        size: &BookSize,
        genre: &Genre,
        style: &WritingStyle,
        context: &str,
    ) -> Result<String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let generation_start = std::time::Instant::now();

        // Phase 1: Pre-generation analysis and prediction
        println!("🔍 Analyzing potential issues...");
        let error_prediction = {
            let error_recognition = self.error_recognition.lock().unwrap();
            error_recognition.predict_potential_errors(genre, style, size, &GenerationPhase::ChapterWriting, prompt)
        };

        self.display_error_predictions(&error_prediction)?;

        // Phase 2: Adaptive prompt enhancement
        let enhanced_prompt = {
            let learning_engine = self.adaptive_learning.lock().unwrap();
            learning_engine.adapt_writing_approach(genre, style, prompt)
        };

        if enhanced_prompt != prompt {
            println!("✨ Applied adaptive improvements to prompt");
        }

        // Phase 3: Self-healing generation attempt
        let generation_result = {
            let mut healing_writer = self.self_healing.lock().unwrap();
            crate::writer::self_healing_chapter_generation(
                client,
                model,
                &enhanced_prompt,
                chapter_num,
                size,
                &mut *healing_writer,
                context,
            ).await
        };

        let generation_duration = generation_start.elapsed().as_secs_f32();

        // Phase 4: Post-generation analysis and learning
        match &generation_result {
            Ok(content) => {
                let quality_score = self.assess_content_quality(content).await;
                
                // Record successful generation
                {
                    let mut learning_engine = self.adaptive_learning.lock().unwrap();
                    learning_engine.record_generation_outcome(
                        genre.clone(),
                        style.clone(),
                        size.clone(),
                        GenerationPhase::ChapterWriting,
                        None,
                        true,
                        quality_score,
                        generation_duration,
                    );
                }

                // Check if we should request user feedback
                if self.should_request_feedback().await {
                    self.collect_user_feedback(&session_id, content, chapter_num).await?;
                }

                println!("✅ Chapter {} generated successfully with quality score: {:.1}/10", chapter_num, quality_score);
                self.display_learning_insights().await?;
            }
            Err(e) => {
                // Analyze the error for learning
                {
                    let mut error_recognition = self.error_recognition.lock().unwrap();
                    let error_analysis = error_recognition.analyze_error(
                        &e.to_string(),
                        context,
                        genre,
                        style,
                        size,
                        &GenerationPhase::ChapterWriting,
                        &self.categorize_error(&e.to_string()),
                    );
                    
                    println!("🧠 Error pattern analysis complete:");
                    println!("   Pattern recognized: {}", error_analysis.pattern_recognized);
                    println!("   Severity: {:?}", error_analysis.severity);
                    println!("   Confidence: {:.0}%", error_analysis.confidence * 100.0);
                }

                // Record failed generation
                {
                    let mut learning_engine = self.adaptive_learning.lock().unwrap();
                    learning_engine.record_generation_outcome(
                        genre.clone(),
                        style.clone(),
                        size.clone(),
                        GenerationPhase::ChapterWriting,
                        None,
                        false,
                        0.0,
                        generation_duration,
                    );
                }
            }
        }

        generation_result
    }

    pub async fn collect_user_feedback(&self, session_id: &str, content: &str, chapter_num: usize) -> Result<()> {
        println!("\n🎯 Quality Assessment for Chapter {}", chapter_num);
        println!("To help Pundit learn and improve, please provide feedback:");

        // Content preview
        println!("\n📖 Content Preview:");
        let preview = content.chars().take(300).collect::<String>();
        println!("{}{}", preview, if content.len() > 300 { "..." } else { "" });

        // Rating
        let rating_options = vec![
            "1 - Poor", "2 - Below Average", "3 - Fair", "4 - Average", "5 - Good",
            "6 - Above Average", "7 - Very Good", "8 - Excellent", "9 - Outstanding", "10 - Perfect"
        ];
        
        let rating_index = Select::new()
            .with_prompt("Rate the overall quality")
            .items(&rating_options)
            .interact()?;
        let rating = (rating_index + 1) as u8;

        // Specific issues (multi-select)
        let issue_options = vec![
            "Poor Character Development",
            "Weak Dialogue",
            "Inconsistent Tone",
            "Plot Holes",
            "Pacing Issues",
            "Grammar Errors",
            "Repetitive Content",
            "Lack of Depth",
            "Unrealistic Scenarios",
            "Poor Transitions",
            "No specific issues"
        ];

        let selected_issues = MultiSelect::new()
            .with_prompt("Select any specific issues (use Space to select, Enter to confirm)")
            .items(&issue_options)
            .interact()?;

        let issues: Vec<ContentIssue> = selected_issues
            .into_iter()
            .filter_map(|i| match i {
                0 => Some(ContentIssue::PoorCharacterDevelopment),
                1 => Some(ContentIssue::WeakDialogue),
                2 => Some(ContentIssue::InconsistentTone),
                3 => Some(ContentIssue::PlotHoles),
                4 => Some(ContentIssue::PacingIssues),
                5 => Some(ContentIssue::GrammarErrors),
                6 => Some(ContentIssue::RepetitiveContent),
                7 => Some(ContentIssue::LackOfDepth),
                8 => Some(ContentIssue::UnrealisticScenarios),
                9 => Some(ContentIssue::PoorTransitions),
                _ => None,
            })
            .collect();

        // Optional text feedback
        let text_feedback: String = Input::new()
            .with_prompt("Additional comments (optional)")
            .allow_empty(true)
            .interact()?;

        let feedback = UserFeedback {
            session_id: session_id.to_string(),
            content_id: format!("chapter_{}", chapter_num),
            rating,
            feedback_text: if text_feedback.trim().is_empty() { None } else { Some(text_feedback) },
            specific_issues: issues,
            timestamp: Utc::now(),
        };

        // Record feedback for learning
        {
            let mut learning_engine = self.adaptive_learning.lock().unwrap();
            learning_engine.record_user_feedback(feedback);
        }

        println!("🙏 Thank you for your feedback! Pundit will learn from this to improve future writing.");
        Ok(())
    }

    pub async fn display_learning_insights(&self) -> Result<()> {
        println!("\n🧠 Learning Insights:");
        
        let learning_insights = {
            let learning_engine = self.adaptive_learning.lock().unwrap();
            learning_engine.get_learning_insights()
        };

        for insight in learning_insights {
            println!("   {}", insight);
        }

        let improvement_suggestions = {
            let learning_engine = self.adaptive_learning.lock().unwrap();
            learning_engine.generate_improvement_suggestions()
        };

        if !improvement_suggestions.is_empty() {
            println!("\n💡 Improvement Suggestions:");
            for suggestion in improvement_suggestions.iter().take(3) {
                println!("   • {}", suggestion);
            }
        }

        // Error intelligence
        let error_report = {
            let error_recognition = self.error_recognition.lock().unwrap();
            error_recognition.get_error_intelligence_report()
        };

        if error_report.len() > 2 {
            println!("\n{}", error_report.join("\n"));
        }

        Ok(())
    }

    pub async fn get_quality_predictions(&self, genre: &Genre, style: &WritingStyle) -> QualityPredictions {
        let learning_engine = self.adaptive_learning.lock().unwrap();
        learning_engine.get_writing_quality_predictions(genre, style)
    }

    pub async fn should_request_feedback(&self) -> bool {
        let healing_writer = self.self_healing.lock().unwrap();
        healing_writer.success_metrics.total_generations % self.user_feedback_frequency == 0
    }

    pub async fn assess_content_quality(&self, content: &str) -> f32 {
        let word_count = content.split_whitespace().count();
        let sentence_count = content.matches('.').count() + content.matches('!').count() + content.matches('?').count();
        
        // Basic quality heuristics
        let mut quality_score: f32 = 5.0; // Base score
        
        // Word count factor
        if word_count > 100 && word_count < 2000 {
            quality_score += 1.0;
        }
        
        // Sentence variety
        if sentence_count > 0 {
            let avg_sentence_length = word_count as f32 / sentence_count as f32;
            if avg_sentence_length > 8.0 && avg_sentence_length < 25.0 {
                quality_score += 1.0;
            }
        }
        
        // Dialogue presence
        if content.contains('"') {
            quality_score += 0.5;
        }
        
        // Paragraph structure
        let paragraph_count = content.split("\n\n").count();
        if paragraph_count > 1 && paragraph_count < 20 {
            quality_score += 0.5;
        }
        
        // Descriptive language
        let descriptive_words = ["beautiful", "dark", "bright", "mysterious", "ancient", "vibrant"];
        let descriptive_count = descriptive_words.iter()
            .map(|word| content.to_lowercase().matches(word).count())
            .sum::<usize>();
        
        if descriptive_count > 0 {
            quality_score += 0.5;
        }
        
        quality_score.min(10.0f32)
    }

    fn display_error_predictions(&self, prediction: &ErrorPrediction) -> Result<()> {
        if !prediction.predicted_errors.is_empty() {
            println!("⚠️ Potential Issues Detected:");
            for (i, error) in prediction.predicted_errors.iter().take(3).enumerate() {
                println!(
                    "   {}. {:?} ({}% probability)",
                    i + 1,
                    error.error_type,
                    (error.probability * 100.0) as u32
                );
            }
            
            if !prediction.prevention_strategies.is_empty() {
                println!("🛡️ Prevention strategies ready");
            }
            
            let should_continue = Confirm::new()
                .with_prompt("Continue with generation despite potential issues?")
                .default(true)
                .interact()?;
                
            if !should_continue {
                return Err(anyhow::anyhow!("User chose to abort generation"));
            }
        } else {
            println!("✅ No significant issues predicted");
        }
        
        Ok(())
    }

    fn categorize_error(&self, error_message: &str) -> crate::self_healing_writer::ErrorCategory {
        let msg_lower = error_message.to_lowercase();
        
        if msg_lower.contains("memory") || msg_lower.contains("overflow") {
            crate::self_healing_writer::ErrorCategory::MemoryOverflow
        } else if msg_lower.contains("creative") || msg_lower.contains("block") {
            crate::self_healing_writer::ErrorCategory::CreativeBlock
        } else if msg_lower.contains("timeout") {
            crate::self_healing_writer::ErrorCategory::SystemTimeout
        } else if msg_lower.contains("parse") || msg_lower.contains("parsing") {
            crate::self_healing_writer::ErrorCategory::ParsingError
        } else if msg_lower.contains("network") || msg_lower.contains("connection") {
            crate::self_healing_writer::ErrorCategory::NetworkError
        } else {
            crate::self_healing_writer::ErrorCategory::Unknown
        }
    }

    pub async fn export_learning_data(&self) -> Result<String> {
        let healing_data = {
            let healing_writer = self.self_healing.lock().unwrap();
            serde_json::to_string_pretty(&*healing_writer)?
        };

        let learning_data = {
            let learning_engine = self.adaptive_learning.lock().unwrap();
            format!(
                "Learning Insights:\n{}\n\nImprovement Suggestions:\n{}\n",
                learning_engine.get_learning_insights().join("\n"),
                learning_engine.generate_improvement_suggestions().join("\n")
            )
        };

        let error_intelligence = {
            let error_recognition = self.error_recognition.lock().unwrap();
            error_recognition.get_error_intelligence_report().join("\n")
        };

        Ok(format!(
            "PUNDIT LEARNING DATA EXPORT\n\
            ===========================\n\n\
            {}\n\n\
            LEARNING ENGINE DATA:\n\
            {}n\n\
            ERROR INTELLIGENCE:\n\
            {}\n",
            healing_data,
            learning_data,
            error_intelligence
        ))
    }

    pub async fn display_system_status(&self) -> Result<()> {
        println!("\n🔧 Enhanced Writer System Status");
        println!("════════════════════════════════");
        
        let (healing_stats, learning_stats, error_stats) = {
            let healing_writer = self.self_healing.lock().unwrap();
            let learning_engine = self.adaptive_learning.lock().unwrap();
            let error_recognition = self.error_recognition.lock().unwrap();
            
            (
                healing_writer.success_metrics.clone(),
                learning_engine.user_feedback_history.len(),
                error_recognition.error_signatures.len()
            )
        };

        println!("📊 Self-Healing Statistics:");
        println!("   • Total generations: {}", healing_stats.total_generations);
        println!("   • Successful completions: {}", healing_stats.successful_completions);
        println!("   • Auto-healed errors: {}", healing_stats.auto_healed_errors);
        println!("   • Success rate: {:.1}%", healing_stats.improvement_rate * 100.0);
        
        println!("\n🧠 Learning Statistics:");
        println!("   • Feedback sessions: {}", learning_stats);
        println!("   • Error patterns recognized: {}", error_stats);
        println!("   • Enhancement status: {}", if self.enhancement_enabled { "Enabled" } else { "Disabled" });
        
        Ok(())
    }
}

impl Default for EnhancedWriterSystem {
    fn default() -> Self {
        Self::new()
    }
}