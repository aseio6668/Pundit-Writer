// Enhanced poetry generation with rhyme-aware prompting and emotional anchors
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EmotionalAnchor {
    pub emotion: String,
    pub metaphors: Vec<String>,
    pub imagery: Vec<String>,
    pub rhythm_style: String,
}

#[derive(Debug, Clone)]
pub struct RhymeHint {
    pub word: String,
    pub rhymes: Vec<String>,
    pub syllable_count: usize,
    pub phonetic_ending: String,
}

#[derive(Debug, Clone)]
pub struct EnhancedPoetryPrompt {
    pub base_theme: String,
    pub emotional_anchor: EmotionalAnchor,
    pub rhyme_hints: Vec<RhymeHint>,
    pub structural_guidance: String,
    pub coherence_requirements: Vec<String>,
}

impl EmotionalAnchor {
    pub fn new(emotion: &str) -> Self {
        let (metaphors, imagery, rhythm_style) = match emotion.to_lowercase().as_str() {
            "longing" => (
                vec![
                    "distant shores".to_string(),
                    "fading echoes".to_string(),
                    "empty spaces".to_string(),
                    "reaching arms".to_string(),
                    "unfinished songs".to_string(),
                ],
                vec![
                    "soft twilight".to_string(),
                    "whispered promises".to_string(),
                    "gentle breezes".to_string(),
                    "moonlit paths".to_string(),
                    "flowing streams".to_string(),
                ],
                "flowing with gentle pauses".to_string()
            ),
            "joy" => (
                vec![
                    "dancing light".to_string(),
                    "soaring birds".to_string(),
                    "blooming flowers".to_string(),
                    "bubbling laughter".to_string(),
                    "warm embraces".to_string(),
                ],
                vec![
                    "golden sunshine".to_string(),
                    "vibrant colors".to_string(),
                    "sparkling water".to_string(),
                    "open skies".to_string(),
                    "singing voices".to_string(),
                ],
                "upbeat and rhythmic".to_string()
            ),
            "melancholy" => (
                vec![
                    "autumn leaves".to_string(),
                    "silent rooms".to_string(),
                    "forgotten dreams".to_string(),
                    "fading photographs".to_string(),
                    "empty chairs".to_string(),
                ],
                vec![
                    "gray morning mist".to_string(),
                    "gentle rain".to_string(),
                    "shadowed corners".to_string(),
                    "quiet footsteps".to_string(),
                    "dimmed lights".to_string(),
                ],
                "slow and contemplative".to_string()
            ),
            "wonder" => (
                vec![
                    "vast galaxies".to_string(),
                    "hidden mysteries".to_string(),
                    "endless possibilities".to_string(),
                    "magical moments".to_string(),
                    "infinite horizons".to_string(),
                ],
                vec![
                    "starlit skies".to_string(),
                    "crystal clarity".to_string(),
                    "shimmering air".to_string(),
                    "ethereal beauty".to_string(),
                    "glowing pathways".to_string(),
                ],
                "expansive and awe-filled".to_string()
            ),
            "peace" => (
                vec![
                    "still waters".to_string(),
                    "gentle breath".to_string(),
                    "quiet moments".to_string(),
                    "soft earth".to_string(),
                    "calm presence".to_string(),
                ],
                vec![
                    "serene landscapes".to_string(),
                    "hushed whispers".to_string(),
                    "warm light".to_string(),
                    "floating clouds".to_string(),
                    "resting birds".to_string(),
                ],
                "calm and steady".to_string()
            ),
            _ => (
                vec!["flowing thoughts".to_string(), "gentle moments".to_string()],
                vec!["natural beauty".to_string(), "soft impressions".to_string()],
                "natural and flowing".to_string()
            )
        };

        EmotionalAnchor {
            emotion: emotion.to_string(),
            metaphors,
            imagery,
            rhythm_style,
        }
    }

    pub fn create_emotional_prompt(&self) -> String {
        format!(
            "Write with the feeling of {}. Use imagery that evokes {} and {}. \
            Let the rhythm feel {}. Draw upon metaphors of {} to create emotional resonance.",
            self.emotion,
            self.imagery.join(", "),
            self.metaphors[0],
            self.rhythm_style,
            self.metaphors.join(" and ")
        )
    }
}

impl RhymeHint {
    pub fn create_common_rhymes() -> HashMap<String, Vec<String>> {
        let mut rhymes = HashMap::new();
        
        // Common rhyme families with phonetic awareness
        rhymes.insert("love".to_string(), vec!["dove".to_string(), "above".to_string(), "shove".to_string()]);
        rhymes.insert("heart".to_string(), vec!["start".to_string(), "part".to_string(), "art".to_string(), "smart".to_string()]);
        rhymes.insert("night".to_string(), vec!["light".to_string(), "bright".to_string(), "sight".to_string(), "flight".to_string()]);
        rhymes.insert("day".to_string(), vec!["way".to_string(), "say".to_string(), "play".to_string(), "stay".to_string()]);
        rhymes.insert("dream".to_string(), vec!["stream".to_string(), "beam".to_string(), "team".to_string(), "gleam".to_string()]);
        rhymes.insert("time".to_string(), vec!["rhyme".to_string(), "climb".to_string(), "prime".to_string(), "chime".to_string()]);
        rhymes.insert("soul".to_string(), vec!["whole".to_string(), "goal".to_string(), "roll".to_string(), "toll".to_string()]);
        rhymes.insert("pain".to_string(), vec!["rain".to_string(), "gain".to_string(), "main".to_string(), "plain".to_string()]);
        rhymes.insert("free".to_string(), vec!["tree".to_string(), "sea".to_string(), "key".to_string(), "flee".to_string()]);
        rhymes.insert("fire".to_string(), vec!["desire".to_string(), "inspire".to_string(), "tire".to_string(), "wire".to_string()]);
        
        rhymes
    }

    pub fn get_syllable_count(word: &str) -> usize {
        // Simple syllable counting heuristic
        let vowels = "aeiouyAEIOUY";
        let mut count = 0;
        let mut prev_was_vowel = false;
        
        for ch in word.chars() {
            let is_vowel = vowels.contains(ch);
            if is_vowel && !prev_was_vowel {
                count += 1;
            }
            prev_was_vowel = is_vowel;
        }
        
        // Handle silent 'e' and minimum of 1 syllable
        if word.ends_with('e') && count > 1 {
            count -= 1;
        }
        
        count.max(1)
    }

    pub fn create_rhyme_guidance(words: &[&str]) -> Vec<RhymeHint> {
        let rhyme_map = Self::create_common_rhymes();
        let mut hints = Vec::new();
        
        for word in words {
            let word_str = word.to_lowercase();
            let rhymes = rhyme_map.get(&word_str).cloned().unwrap_or_else(|| {
                // Generate basic rhymes based on ending sounds
                if word_str.ends_with("ing") {
                    vec!["sing".to_string(), "ring".to_string(), "bring".to_string()]
                } else if word_str.ends_with("ight") {
                    vec!["light".to_string(), "bright".to_string(), "sight".to_string()]
                } else {
                    vec![]
                }
            });
            
            hints.push(RhymeHint {
                word: word_str.clone(),
                rhymes,
                syllable_count: Self::get_syllable_count(&word_str),
                phonetic_ending: word_str.chars().rev().take(2).collect::<String>().chars().rev().collect(),
            });
        }
        
        hints
    }
}

impl EnhancedPoetryPrompt {
    pub fn new(theme: &str, emotion: &str, poetry_style: &str) -> Self {
        let emotional_anchor = EmotionalAnchor::new(emotion);
        
        // Create rhyme hints based on theme words
        let theme_words: Vec<&str> = theme.split_whitespace().collect();
        let rhyme_hints = RhymeHint::create_rhyme_guidance(&theme_words);
        
        let structural_guidance = match poetry_style.to_lowercase().as_str() {
            "sonnet" => {
                "Create a 14-line sonnet with ABAB CDCD EFEF GG rhyme scheme. \
                Each line should have approximately 10 syllables (iambic pentameter). \
                The final couplet should provide a conclusion or twist.".to_string()
            },
            "haiku" => {
                "Create a traditional haiku with 5-7-5 syllable structure. \
                Focus on nature imagery and a moment of realization. \
                No need for rhyming, focus on imagery and emotion.".to_string()
            },
            "ballad" => {
                "Create a narrative ballad with ABAB or ABCB rhyme scheme. \
                Tell a story with emotional depth. Use a steady rhythm \
                that feels like a song. Each stanza should advance the narrative.".to_string()
            },
            "free verse" => {
                "Create free verse poetry without strict rhyme or meter. \
                Focus on natural speech rhythms and powerful imagery. \
                Use line breaks and spacing to create emphasis and flow.".to_string()
            },
            "limerick" => {
                "Create a humorous limerick with AABBA rhyme scheme. \
                Lines 1, 2, and 5 should have 7-10 syllables. \
                Lines 3 and 4 should have 5-7 syllables. Keep it playful!".to_string()
            },
            _ => {
                "Focus on creating natural, flowing verse with subtle rhymes \
                that enhance rather than force the meaning.".to_string()
            }
        };
        
        let coherence_requirements = vec![
            "Maintain thematic consistency throughout".to_string(),
            "Ensure each line contributes to the overall emotional arc".to_string(),
            "Use concrete imagery over abstract concepts".to_string(),
            "Create natural transitions between ideas".to_string(),
            "End with a sense of completion or resolution".to_string(),
        ];
        
        EnhancedPoetryPrompt {
            base_theme: theme.to_string(),
            emotional_anchor,
            rhyme_hints,
            structural_guidance,
            coherence_requirements,
        }
    }
    
    pub fn create_enhanced_prompt(&self, poem_title: &str) -> String {
        let emotional_guidance = self.emotional_anchor.create_emotional_prompt();
        
        let rhyme_guidance = if !self.rhyme_hints.is_empty() {
            let rhyme_suggestions: Vec<String> = self.rhyme_hints.iter()
                .map(|hint| {
                    if !hint.rhymes.is_empty() {
                        format!("Consider rhyming with '{}': {}", hint.word, hint.rhymes.join(", "))
                    } else {
                        format!("'{}' ({} syllables)", hint.word, hint.syllable_count)
                    }
                })
                .collect();
            
            format!("\n\nRhyme guidance:\n{}", rhyme_suggestions.join("\n"))
        } else {
            String::new()
        };
        
        format!(
            "Write a poem titled '{}' on the theme of {}.\n\n\
            EMOTIONAL DIRECTION:\n{}\n\n\
            STRUCTURAL GUIDANCE:\n{}\n\n\
            COHERENCE REQUIREMENTS:\n{}\n{}\n\n\
            Instead of forcing rhymes, let them emerge naturally from the emotional content. \
            Focus on creating authentic feeling over perfect technical execution. \
            Use soft, natural rhymes that feel inevitable rather than contrived.\n\n\
            Poem:",
            poem_title,
            self.base_theme,
            emotional_guidance,
            self.structural_guidance,
            self.coherence_requirements.join("\n"),
            rhyme_guidance
        )
    }
}

pub fn create_emotion_from_theme(theme: &str) -> String {
    // Map themes to appropriate emotions
    match theme.to_lowercase().as_str() {
        t if t.contains("love") || t.contains("romance") => "longing",
        t if t.contains("loss") || t.contains("grief") || t.contains("sad") => "melancholy", 
        t if t.contains("joy") || t.contains("happy") || t.contains("celebration") => "joy",
        t if t.contains("nature") || t.contains("beauty") || t.contains("wonder") => "wonder",
        t if t.contains("peace") || t.contains("calm") || t.contains("serenity") => "peace",
        t if t.contains("hope") || t.contains("dream") || t.contains("future") => "wonder",
        t if t.contains("memory") || t.contains("past") || t.contains("nostalgia") => "melancholy",
        _ => "wonder" // Default to wonder for unknown themes
    }.to_string()
}

pub fn post_process_poetry(generated_text: &str, expected_emotion: &str) -> String {
    // Basic post-processing to enhance emotional coherence
    let mut lines: Vec<String> = generated_text.lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    
    // Remove AI-generated meta-commentary
    lines.retain(|line| {
        !line.to_lowercase().contains("as an ai") &&
        !line.to_lowercase().contains("i cannot") &&
        !line.to_lowercase().contains("here is a") &&
        !line.to_lowercase().contains("this poem")
    });
    
    // Ensure the poem feels complete
    if !lines.is_empty() {
        let last_line = lines.last().unwrap();
        if last_line.ends_with(',') || last_line.ends_with(';') {
            // Fix incomplete ending
            if let Some(last) = lines.last_mut() {
                *last = last.trim_end_matches(',').trim_end_matches(';').to_string();
            }
        }
    }
    
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotional_anchor_creation() {
        let anchor = EmotionalAnchor::new("longing");
        assert_eq!(anchor.emotion, "longing");
        assert!(!anchor.metaphors.is_empty());
        assert!(!anchor.imagery.is_empty());
    }

    #[test]
    fn test_syllable_counting() {
        assert_eq!(RhymeHint::get_syllable_count("hello"), 2);
        assert_eq!(RhymeHint::get_syllable_count("beautiful"), 3);
        assert_eq!(RhymeHint::get_syllable_count("cat"), 1);
        assert_eq!(RhymeHint::get_syllable_count("love"), 1);
    }

    #[test]
    fn test_emotion_mapping() {
        assert_eq!(create_emotion_from_theme("love and loss"), "longing");
        assert_eq!(create_emotion_from_theme("nature's beauty"), "wonder");
        assert_eq!(create_emotion_from_theme("peaceful morning"), "peace");
    }
}