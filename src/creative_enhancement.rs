// Creative enhancement system for filling gaps in short prompts and adding authentic creativity
use crate::content::ContentType;
use crate::cli_types::{Genre, WritingStyle};
use serde::{Deserialize, Serialize};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativePromptEnhancer {
    pub base_prompt: String,
    pub content_type: ContentType,
    pub genre: Option<Genre>,
    pub style: Option<WritingStyle>,
    pub target_sections: usize,
    pub creativity_level: CreativityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreativityLevel {
    Conservative,    // Stick close to prompt
    Moderate,       // Some creative interpretation
    High,           // Significant creative freedom
    Unlimited,      // Full creative liberty
}

#[derive(Debug, Clone)]
pub struct EnhancedPrompt {
    pub original_prompt: String,
    pub enhanced_prompt: String,
    pub narrative_elements: Vec<NarrativeElement>,
    pub creative_directions: Vec<String>,
    pub thematic_suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NarrativeElement {
    pub element_type: ElementType,
    pub description: String,
    pub importance: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub enum ElementType {
    Character,
    Setting,
    Plot,
    Theme,
    Conflict,
    Mood,
    Style,
    Perspective,
}

impl CreativePromptEnhancer {
    pub fn new(
        base_prompt: String,
        content_type: ContentType,
        genre: Option<Genre>,
        style: Option<WritingStyle>,
        target_sections: usize,
    ) -> Self {
        let creativity_level = Self::determine_creativity_level(&base_prompt, target_sections);
        
        Self {
            base_prompt,
            content_type,
            genre,
            style,
            target_sections,
            creativity_level,
        }
    }

    pub fn enhance_prompt(&self) -> EnhancedPrompt {
        let mut rng = StdRng::from_entropy();
        
        // Analyze the original prompt
        let prompt_analysis = self.analyze_prompt();
        
        // Generate creative elements based on analysis
        let narrative_elements = self.generate_narrative_elements(&prompt_analysis, &mut rng);
        
        // Create enhanced prompt
        let enhanced_prompt = self.build_enhanced_prompt(&prompt_analysis, &narrative_elements);
        
        // Generate creative directions for each section
        let creative_directions = self.generate_creative_directions(&narrative_elements, &mut rng);
        
        // Generate thematic suggestions
        let thematic_suggestions = self.generate_thematic_suggestions(&prompt_analysis, &mut rng);

        EnhancedPrompt {
            original_prompt: self.base_prompt.clone(),
            enhanced_prompt,
            narrative_elements,
            creative_directions,
            thematic_suggestions,
        }
    }

    fn determine_creativity_level(prompt: &str, target_sections: usize) -> CreativityLevel {
        let word_count = prompt.split_whitespace().count();
        let sections_per_word = target_sections as f32 / word_count.max(1) as f32;
        
        match (word_count, sections_per_word) {
            (1..=3, _) => CreativityLevel::Unlimited,
            (4..=10, ratio) if ratio > 5.0 => CreativityLevel::High,
            (11..=20, ratio) if ratio > 2.0 => CreativityLevel::Moderate,
            _ => CreativityLevel::Conservative,
        }
    }

    fn analyze_prompt(&self) -> PromptAnalysis {
        let words: Vec<&str> = self.base_prompt.split_whitespace().collect();
        let word_count = words.len();
        
        let has_characters = self.detect_characters(&words);
        let has_setting = self.detect_setting(&words);
        let has_plot = self.detect_plot(&words);
        let mood = self.detect_mood(&words);
        let themes = self.detect_themes(&words);
        
        PromptAnalysis {
            word_count,
            has_characters,
            has_setting,
            has_plot,
            mood,
            themes,
            gaps: self.identify_gaps(has_characters, has_setting, has_plot),
        }
    }

    fn detect_characters(&self, words: &[&str]) -> bool {
        let character_keywords = [
            "person", "man", "woman", "child", "hero", "protagonist", "character",
            "she", "he", "they", "someone", "girl", "boy", "friend", "enemy",
            "detective", "wizard", "knight", "princess", "king", "queen"
        ];
        words.iter().any(|word| character_keywords.contains(&word.to_lowercase().as_str()))
    }

    fn detect_setting(&self, words: &[&str]) -> bool {
        let setting_keywords = [
            "in", "at", "place", "world", "city", "town", "forest", "castle", "house",
            "school", "space", "planet", "kingdom", "village", "island", "mountain",
            "ocean", "desert", "future", "past", "medieval", "modern"
        ];
        words.iter().any(|word| setting_keywords.contains(&word.to_lowercase().as_str()))
    }

    fn detect_plot(&self, words: &[&str]) -> bool {
        let plot_keywords = [
            "adventure", "journey", "quest", "mystery", "romance", "war", "battle",
            "escape", "rescue", "discover", "find", "search", "fight", "love",
            "betrayal", "revenge", "survival", "competition", "challenge"
        ];
        words.iter().any(|word| plot_keywords.contains(&word.to_lowercase().as_str()))
    }

    fn detect_mood(&self, words: &[&str]) -> Mood {
        let happy_words = ["funny", "happy", "joy", "cheerful", "bright", "comedy"];
        let dark_words = ["dark", "scary", "horror", "sad", "tragic", "grim"];
        let mysterious_words = ["mystery", "secret", "hidden", "unknown", "strange"];
        let adventurous_words = ["adventure", "exciting", "thrilling", "bold", "brave"];
        
        for word in words {
            let lower = word.to_lowercase();
            if happy_words.contains(&lower.as_str()) { return Mood::Lighthearted; }
            if dark_words.contains(&lower.as_str()) { return Mood::Dark; }
            if mysterious_words.contains(&lower.as_str()) { return Mood::Mysterious; }
            if adventurous_words.contains(&lower.as_str()) { return Mood::Adventurous; }
        }
        
        // Default mood based on content type
        match self.content_type {
            ContentType::ChildrensBook => Mood::Lighthearted,
            ContentType::Poetry => Mood::Reflective,
            ContentType::TechnicalDoc => Mood::Neutral,
            _ => Mood::Neutral,
        }
    }

    fn detect_themes(&self, words: &[&str]) -> Vec<String> {
        let mut themes = Vec::new();
        
        let theme_map = [
            (["friendship", "friend", "loyalty"], "Friendship and Loyalty"),
            (["love", "romance", "heart"], "Love and Romance"),
            (["courage", "brave", "hero"], "Courage and Heroism"),
            (["family", "parent", "child"], "Family Bonds"),
            (["freedom", "escape", "liberty"], "Freedom and Liberation"),
            (["growth", "learn", "change"], "Personal Growth"),
            (["nature", "environment", "earth"], "Nature and Environment"),
            (["technology", "future", "robot"], "Technology and Progress"),
            (["justice", "right", "wrong"], "Justice and Morality"),
            (["mystery", "secret", "hidden"], "Mystery and Discovery"),
        ];
        
        for (keywords, theme) in theme_map {
            if words.iter().any(|word| keywords.contains(&word.to_lowercase().as_str())) {
                themes.push(theme.to_string());
            }
        }
        
        themes
    }

    fn identify_gaps(&self, has_characters: bool, has_setting: bool, has_plot: bool) -> Vec<CreativeGap> {
        let mut gaps = Vec::new();
        
        if !has_characters {
            gaps.push(CreativeGap::Characters);
        }
        if !has_setting {
            gaps.push(CreativeGap::Setting);
        }
        if !has_plot {
            gaps.push(CreativeGap::Plot);
        }
        
        gaps
    }

    fn generate_narrative_elements(&self, analysis: &PromptAnalysis, rng: &mut StdRng) -> Vec<NarrativeElement> {
        let mut elements = Vec::new();
        
        // Fill gaps with creative elements
        for gap in &analysis.gaps {
            match gap {
                CreativeGap::Characters => {
                    elements.extend(self.generate_characters(rng));
                },
                CreativeGap::Setting => {
                    elements.extend(self.generate_settings(rng));
                },
                CreativeGap::Plot => {
                    elements.extend(self.generate_plot_elements(rng));
                },
            }
        }
        
        // Add thematic elements
        elements.extend(self.generate_thematic_elements(analysis, rng));
        
        elements
    }

    fn generate_characters(&self, rng: &mut StdRng) -> Vec<NarrativeElement> {
        let character_archetypes = match self.content_type {
            ContentType::ChildrensBook => vec![
                "a curious young explorer",
                "a wise talking animal",
                "a magical helper",
                "a friendly neighborhood child",
                "a brave little adventurer"
            ],
            ContentType::Dictionary => vec!["the lexicographer", "word enthusiasts", "language scholars"],
            ContentType::EducationalLesson => vec!["the teacher", "eager students", "learning companions"],
            _ => vec![
                "a mysterious stranger",
                "a determined protagonist",
                "an unlikely hero",
                "a wise mentor",
                "a complex antagonist",
                "a loyal companion"
            ]
        };
        
        let selected = character_archetypes[rng.gen_range(0..character_archetypes.len())];
        
        vec![NarrativeElement {
            element_type: ElementType::Character,
            description: selected.to_string(),
            importance: 0.8,
        }]
    }

    fn generate_settings(&self, rng: &mut StdRng) -> Vec<NarrativeElement> {
        let settings = match self.content_type {
            ContentType::ChildrensBook => vec![
                "a colorful magical forest",
                "a cozy neighborhood",
                "a whimsical playground",
                "a fantastic dreamland",
                "a cheerful village"
            ],
            ContentType::Dictionary => vec![
                "a grand library of words",
                "the realm of language",
                "the etymology workshop",
                "the word laboratory"
            ],
            ContentType::EducationalLesson => vec![
                "an interactive classroom",
                "a learning adventure world",
                "the knowledge garden",
                "an educational journey"
            ],
            _ => vec![
                "a world on the brink of change",
                "a place where ordinary meets extraordinary",
                "a realm of hidden possibilities",
                "a landscape shaped by secrets",
                "a setting that challenges expectations"
            ]
        };
        
        let selected = settings[rng.gen_range(0..settings.len())];
        
        vec![NarrativeElement {
            element_type: ElementType::Setting,
            description: selected.to_string(),
            importance: 0.7,
        }]
    }

    fn generate_plot_elements(&self, rng: &mut StdRng) -> Vec<NarrativeElement> {
        let plot_elements = match self.content_type {
            ContentType::ChildrensBook => vec![
                "a journey of discovery and friendship",
                "learning an important life lesson",
                "solving a gentle mystery",
                "overcoming a small but meaningful challenge",
                "helping others and making new friends"
            ],
            ContentType::Dictionary => vec![
                "exploring the fascinating origins of words",
                "uncovering linguistic mysteries",
                "tracing word evolution through time",
                "discovering hidden word connections"
            ],
            ContentType::EducationalLesson => vec![
                "mastering new concepts through practice",
                "connecting learning to real-world applications",
                "building understanding step by step",
                "developing critical thinking skills"
            ],
            _ => vec![
                "uncovering hidden truths",
                "facing unexpected challenges",
                "making difficult choices",
                "discovering inner strength",
                "transforming through adversity"
            ]
        };
        
        let selected = plot_elements[rng.gen_range(0..plot_elements.len())];
        
        vec![NarrativeElement {
            element_type: ElementType::Plot,
            description: selected.to_string(),
            importance: 0.9,
        }]
    }

    fn generate_thematic_elements(&self, analysis: &PromptAnalysis, rng: &mut StdRng) -> Vec<NarrativeElement> {
        let mut elements = Vec::new();
        
        // Add mood element
        elements.push(NarrativeElement {
            element_type: ElementType::Mood,
            description: format!("{:?} atmosphere", analysis.mood),
            importance: 0.6,
        });
        
        // Add style element based on content type
        let style_desc = match self.content_type {
            ContentType::ChildrensBook => "warm, accessible, and age-appropriate",
            ContentType::Dictionary => "precise, informative, yet engaging",
            ContentType::EducationalLesson => "clear, instructional, and interactive",
            ContentType::Poetry => "lyrical, evocative, and rhythmic",
            _ => "authentic, engaging, and purposeful"
        };
        
        elements.push(NarrativeElement {
            element_type: ElementType::Style,
            description: style_desc.to_string(),
            importance: 0.5,
        });
        
        elements
    }

    fn build_enhanced_prompt(&self, analysis: &PromptAnalysis, elements: &[NarrativeElement]) -> String {
        let mut enhanced = String::new();
        
        // Start with original prompt
        enhanced.push_str(&format!("Based on the concept: '{}'\n\n", self.base_prompt));
        
        // Add creative interpretation note
        match self.creativity_level {
            CreativityLevel::Unlimited => {
                enhanced.push_str("CREATIVE FREEDOM: Use this as loose inspiration to create an original, imaginative work. Feel free to interpret creatively and add authentic, non-repetitive content.\n\n");
            },
            CreativityLevel::High => {
                enhanced.push_str("CREATIVE INTERPRETATION: Expand on this concept with significant creative liberty while maintaining thematic connection.\n\n");
            },
            CreativityLevel::Moderate => {
                enhanced.push_str("CREATIVE EXPANSION: Develop this concept with moderate creative interpretation and authentic details.\n\n");
            },
            CreativityLevel::Conservative => {
                enhanced.push_str("FAITHFUL DEVELOPMENT: Stay close to the original concept while adding necessary narrative elements.\n\n");
            },
        }
        
        // Add narrative elements
        enhanced.push_str("NARRATIVE ELEMENTS TO INCORPORATE:\n");
        for element in elements {
            if element.importance > 0.6 {
                enhanced.push_str(&format!("- {}: {}\n", 
                    format!("{:?}", element.element_type), element.description));
            }
        }
        
        enhanced.push_str("\nIMPORTANT GUIDELINES:\n");
        enhanced.push_str("- Create unique, non-repetitive content for each section\n");
        enhanced.push_str("- Avoid being literal about the original prompt - be creative and interpretive\n");
        enhanced.push_str("- Focus on authentic storytelling rather than explaining the concept\n");
        enhanced.push_str("- Vary pacing, tone, and focus throughout the work\n");
        enhanced.push_str("- Make each section contribute something new to the overall narrative\n");
        
        enhanced
    }

    fn generate_creative_directions(&self, elements: &[NarrativeElement], rng: &mut StdRng) -> Vec<String> {
        let mut directions = Vec::new();
        
        for i in 0..self.target_sections {
            let section_direction = match i {
                0 => self.generate_opening_direction(elements, rng),
                n if n == self.target_sections - 1 => self.generate_closing_direction(elements, rng),
                _ => self.generate_middle_direction(elements, rng, i),
            };
            directions.push(section_direction);
        }
        
        directions
    }

    fn generate_opening_direction(&self, elements: &[NarrativeElement], rng: &mut StdRng) -> String {
        let opening_approaches = match self.content_type {
            ContentType::ChildrensBook => vec![
                "Begin with wonder and curiosity",
                "Start with a relatable situation",
                "Open with gentle excitement",
                "Begin with a question or mystery"
            ],
            ContentType::Dictionary => vec![
                "Start with an intriguing word or phrase",
                "Begin with the joy of language discovery",
                "Open with a fascinating etymology",
                "Start with a word mystery"
            ],
            _ => vec![
                "Hook the reader with intrigue",
                "Start in the middle of action",
                "Begin with a compelling question",
                "Open with atmospheric description"
            ]
        };
        
        opening_approaches[rng.gen_range(0..opening_approaches.len())].to_string()
    }

    fn generate_middle_direction(&self, elements: &[NarrativeElement], rng: &mut StdRng, section: usize) -> String {
        let middle_approaches = vec![
            "Develop character relationships",
            "Explore the setting in detail",
            "Introduce a new challenge",
            "Reveal hidden information",
            "Deepen the emotional connection",
            "Add unexpected elements",
            "Focus on sensory details",
            "Create tension and conflict"
        ];
        
        let base = middle_approaches[rng.gen_range(0..middle_approaches.len())];
        format!("{} - Section {} should feel distinct and purposeful", base, section + 1)
    }

    fn generate_closing_direction(&self, elements: &[NarrativeElement], rng: &mut StdRng) -> String {
        let closing_approaches = match self.content_type {
            ContentType::ChildrensBook => vec![
                "End with warmth and satisfaction",
                "Conclude with a gentle lesson learned",
                "Finish with hope and happiness",
                "Close with a sense of accomplishment"
            ],
            ContentType::Dictionary => vec![
                "Conclude with linguistic appreciation",
                "End with the beauty of language",
                "Finish with word wisdom",
                "Close with etymological insight"
            ],
            _ => vec![
                "Bring resolution to main conflicts",
                "End with meaningful reflection",
                "Conclude with transformation",
                "Finish with satisfying closure"
            ]
        };
        
        closing_approaches[rng.gen_range(0..closing_approaches.len())].to_string()
    }

    fn generate_thematic_suggestions(&self, analysis: &PromptAnalysis, rng: &mut StdRng) -> Vec<String> {
        let mut suggestions = analysis.themes.clone();
        
        // Add content-type specific themes
        let additional_themes = match self.content_type {
            ContentType::ChildrensBook => vec![
                "Growing up and learning".to_string(),
                "Kindness and empathy".to_string(),
                "Imagination and wonder".to_string(),
            ],
            ContentType::Dictionary => vec![
                "The power of words".to_string(),
                "Language evolution".to_string(),
                "Communication and understanding".to_string(),
            ],
            ContentType::EducationalLesson => vec![
                "Knowledge and discovery".to_string(),
                "Learning through practice".to_string(),
                "Building confidence".to_string(),
            ],
            _ => vec![
                "Human resilience".to_string(),
                "Unexpected connections".to_string(),
                "Finding meaning".to_string(),
            ]
        };
        
        suggestions.extend(additional_themes);
        suggestions
    }
}

#[derive(Debug)]
struct PromptAnalysis {
    word_count: usize,
    has_characters: bool,
    has_setting: bool,
    has_plot: bool,
    mood: Mood,
    themes: Vec<String>,
    gaps: Vec<CreativeGap>,
}

#[derive(Debug)]
enum CreativeGap {
    Characters,
    Setting,
    Plot,
}

#[derive(Debug)]
enum Mood {
    Lighthearted,
    Dark,
    Mysterious,
    Adventurous,
    Reflective,
    Neutral,
}

pub fn enhance_short_prompt(
    prompt: &str,
    content_type: ContentType,
    genre: Option<Genre>,
    style: Option<WritingStyle>,
    target_sections: usize,
) -> EnhancedPrompt {
    let enhancer = CreativePromptEnhancer::new(
        prompt.to_string(),
        content_type,
        genre,
        style,
        target_sections,
    );
    
    enhancer.enhance_prompt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_prompt_enhancement() {
        let enhanced = enhance_short_prompt(
            "funny book",
            ContentType::Book,
            Some(Genre::Comedy),
            None,
            10,
        );
        
        assert!(enhanced.enhanced_prompt.len() > enhanced.original_prompt.len());
        assert!(!enhanced.narrative_elements.is_empty());
        assert_eq!(enhanced.creative_directions.len(), 10);
    }

    #[test]
    fn test_creativity_level_detection() {
        let level = CreativePromptEnhancer::determine_creativity_level("funny", 20);
        assert!(matches!(level, CreativityLevel::Unlimited));
        
        let level = CreativePromptEnhancer::determine_creativity_level("a detailed story about adventure", 5);
        assert!(matches!(level, CreativityLevel::Conservative));
    }

    #[test]
    fn test_childrens_book_enhancement() {
        let enhanced = enhance_short_prompt(
            "dragon",
            ContentType::ChildrensBook,
            None,
            None,
            5,
        );
        
        assert!(enhanced.enhanced_prompt.contains("CREATIVE FREEDOM"));
        assert!(!enhanced.creative_directions.is_empty());
    }
}