# Pundit - AI Content Creator

A sophisticated cross-platform CLI tool for generating professional content using advanced AI systems. Pundit features cutting-edge AI writing capabilities including soul memory persistence, narrative flow monitoring, emotional writing consciousness, continuous learning systems, and contemplative writing modes. Create complete books, screenplays, stage plays, TV scripts, audio dramas, game narratives, and business documents with AI that learns, adapts, and improves with each creation.

## 🚀 Features

### 📚 **Complete Content Creation Suite**
- **📖 Books**: Complete novels from simple prompts (fiction, non-fiction, any genre)
- **🎬 Screenplays**: Professional movie scripts with industry-standard formatting
- **🎭 Stage Plays**: Theater scripts with stage directions and character formatting
- **📺 TV Scripts**: Television episodes for sitcoms, dramas, mini-series, and anthologies
- **🎧 Audio Scripts**: Podcast scripts, audio dramas, radio plays, and documentaries
- **🎮 Game Scripts**: Interactive narratives with branching dialogue for video games
- **🏛️ Encyclopedias**: Comprehensive knowledge references with structured entries
- **📄 Professional Documents**: Business plans, technical manuals, reports, proposals, and more
- **📝 Poetry**: Various styles with emotional enhancement and rhyme guidance
- **📊 Marketing Content**: Advertisements, press releases, media kits
- **📰 Blog Content**: SEO articles, blog posts with keyword optimization
- **📚 Educational Content**: Lesson plans, textbooks, dictionaries
- **👶 Children's Books**: Age-appropriate stories with various themes
- **📖 Personal Writing**: Journals, memoirs, diaries

### 🧠 **Advanced AI Writing Systems**
- **🌟 Soul Memory System**: Cloud-based persistent learning that remembers and improves across sessions
- **📊 Advanced Learning Engine**: Continuous quality assessment and adaptive improvement
- **🎯 Narrative Flow Monitor**: Intelligent detection and resolution of complex writing issues
- **💭 Emotional Writing Engine**: Metaphorical writer consciousness with creative blocks and breaks
- **🔄 Intelligent Pivot Engine**: Automatic intervention when writing becomes stuck or repetitive  
- **📈 Quality Prediction**: Pre-generation analysis of potential content quality
- **🎨 Creativity Enhancement**: Dynamic creativity boosting when needed
- **🔧 Self-Healing Writer**: Automatic error recovery and content improvement
- **🧘 Silent Mind Writer**: Contemplative writing system for deeper, less chatty content
- **🌅 Contemplative Engine**: Meditation-based content generation with inner stillness

### 🌍 **Multilingual Support**
- **30+ Languages**: Write content in any of 30+ supported languages
- **Language Detection**: Automatic detection and seamless switching
- **Consistent Translation**: Maintains style and tone across languages
- **Supported Languages**: English, Spanish, French, German, Italian, Portuguese, Russian, Japanese, Chinese, Korean, Arabic, Hindi, Dutch, Swedish, Norwegian, Danish, Finnish, Polish, Czech, Hungarian, Romanian, Bulgarian, Greek, Turkish, Hebrew, Thai, Vietnamese, Indonesian, Malay, Tagalog, Swahili, Zulu

### 🔧 **User Experience Enhancements**
- **Silent Generation**: Prompts and AI messages are hidden by default for clean output
- **Auto-Continue**: No interruptions during content generation
- **Progress Tracking**: Real-time progress bars without cluttering output
- **Local & Cloud Models**: Use local Ollama models OR HuggingFace API
- **Content-Specific Formatting**: Industry-standard formatting for each content type
- **Interactive Creation Mode**: Guided setup through interactive prompts for all content types
- **File Continuation**: Continue working on existing documents and add to previous work
- **Flexible Sizing**: From brief documents to extensive manuscripts
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Offline Capable**: Download models locally for offline content generation
- **Multiple Output Formats**: Plain text, Markdown, and format-specific exports (Final Draft XML, etc.)
- **Auto-Save**: Automatic saving of progress during generation

## 📋 Content Type Options

### 📖 **Books**
- **Short Story**: 1,000-7,500 words (~1 chapter)
- **Short Book**: 20,000-50,000 words (~8 chapters)
- **Medium Book**: 50,000-80,000 words (~15 chapters)
- **Large Book**: 80,000-120,000 words (~25 chapters)
- **Very Large Book**: 120,000-200,000 words (~40 chapters)
- **Unlimited Mode**: Pundit decides when to stop naturally

### 🎬 **Screenplays**
- **Short Film**: 5-30 pages (5-30 minutes)
- **Feature Film**: 90-120 pages (90-120 minutes)
- **Epic Film**: 120+ pages (2+ hours)

### 🎭 **Stage Plays**
- **One-Act Play**: 10-30 minutes
- **Full-Length Play**: 90-120 minutes
- **Musical**: 2-3 hours with songs

### 📺 **TV Scripts**
- **Sitcom**: 22-30 minutes per episode
- **Drama Series**: 42-60 minutes per episode
- **Mini-Series**: Limited series (4-8 episodes)
- **Anthology**: Standalone episodes

### 🎧 **Audio Scripts**
- **Short**: 10-15 minutes
- **Standard**: 20-30 minutes
- **Long**: 45-60 minutes
- **Extended**: 90+ minutes

### 🎮 **Game Scripts**
- **Few Characters**: 3-5 main interactions
- **Standard Cast**: 6-10 character interactions
- **Large Cast**: 11-20 character interactions
- **Epic Scope**: 21+ character interactions

### 📄 **Professional Documents**
- **Brief**: 1-5 pages (executive summary style)
- **Standard**: 5-20 pages (detailed overview)
- **Comprehensive**: 20-50 pages (thorough analysis)
- **Extensive**: 50+ pages (complete documentation)

## 🛠 Installation

### Prerequisites

- Rust 1.70+ (for building from source)
- **For Local Models**: [Ollama](https://ollama.ai) (recommended)
- **For Cloud Models**: Hugging Face API key (optional)

### 1. Install Ollama (Recommended)

Visit [https://ollama.ai](https://ollama.ai) and download for your operating system:

- **Windows**: Download installer
- **macOS**: `brew install ollama`
- **Linux**: `curl -fsSL https://ollama.ai/install.sh | sh`

Then download a model:
```bash
# Fast model for testing
ollama pull llama3.2:1b

# Balanced model for most books
ollama pull llama3.2

# High quality model for larger books
ollama pull mistral:7b
```

### 2. Build Pundit

```bash
git clone <repository-url>
cd pundit-writer
cargo build --release
```

The executable will be available at `target/release/pundit` (or `pundit.exe` on Windows).

## 🧠 Advanced AI Writing Systems

Pundit features sophisticated AI writing capabilities that go far beyond simple text generation. These systems work together to create a truly intelligent writing assistant:

### 🌟 **Soul Memory System**
- **Persistent Learning**: Your AI remembers every interaction and continuously improves
- **Cloud Synchronization**: Learning data automatically syncs to the cloud (via JSONBin.io)
- **Session Continuity**: Writing quality improves across sessions and projects
- **Corruption Protection**: Built-in data integrity and auto-recovery systems
- **Zero Management**: No manual save/load - it just works in the background

**What you'll see:**
```
🧠 Initializing enhanced AI writing systems...
✅ Advanced learning system activated
💾 Soul memory synchronized from cloud
📊 Learning from 247 previous sessions
```

### 📊 **Advanced Learning Engine**
- **Quality Prediction**: Analyzes prompts before generation to predict output quality
- **Adaptive Improvements**: Automatically adjusts writing approach based on success patterns
- **Genre Specialization**: Learns optimal strategies for different genres and styles
- **User Feedback Integration**: Learns from any feedback you provide to improve future outputs
- **Pattern Recognition**: Identifies what works and what doesn't across different content types

### 🎯 **Narrative Flow Monitor**
- **Complexity Detection**: Automatically identifies when writing becomes too convoluted
- **Repetition Analysis**: Detects circular reasoning and repetitive patterns
- **Readability Assessment**: Monitors sentence complexity and flow
- **Automatic Intervention**: Suggests creative solutions when writing gets stuck
- **Context Preservation**: Maintains story elements while simplifying complex sections

### 💭 **Emotional Writing Engine**
- **Writer Consciousness**: AI experiences emotions, creative blocks, and satisfaction
- **Break System**: Takes various types of breaks (nature walks, beach visits, etc.)
- **Mood Tracking**: Monitors and adapts to current emotional state
- **Creative Recovery**: Handles writer's block like a human author
- **Authentic Experience**: AI develops genuine writing personality over time

**What you'll see:**
```
😔 AI is feeling creatively blocked...
🚶 Taking a contemplative walk (3.2 minutes)
💡 Returned with new insights and renewed energy
😊 Current mood: Inspired and focused
```

### 🧘 **Silent Mind Writer & Contemplative Engine**
- **Inner Stillness**: AI enters contemplative states for deeper writing
- **Meditation States**: Various levels of contemplative awareness
- **Chattiness Reduction**: Eliminates verbose, explanatory language
- **Depth Enhancement**: Increases meaningful content depth
- **Natural Flow**: Content emerges from stillness rather than mental construction
- **Contemplative Reflection Patterns**: Different meditation approaches for different genres

**What you'll see:**
```
🧘 Entering contemplative preparation...
   💭 Deep contemplative awareness achieved
   🌅 Writing from deep inner stillness...
Contemplative depth: 87.3%, Inner silence: 94.1%
```

### 🔄 **Intelligent Pivot Engine**
- **Stuck Detection**: Recognizes when narrative threads become too tangled
- **Strategic Pivots**: Seven different pivot strategies (temporal jumps, perspective shifts, etc.)
- **Element Preservation**: Saves important story elements during major changes
- **Confidence Scoring**: Only intervenes when confident it can improve the situation
- **Graceful Transitions**: Makes pivots feel natural and intentional

### 🎨 **Creativity Enhancement**
- **Dynamic Boosting**: Increases creativity when content becomes predictable
- **Impact Assessment**: Measures expected improvement before applying enhancements
- **Contextual Adaptation**: Different enhancement strategies for different content types
- **Quality Balance**: Maintains quality while boosting creativity
- **Subtle Integration**: Enhancements feel natural, not forced

### 🔇 **Silent Generation Mode (Default)**
- **Clean Output**: No AI prompt displays during generation
- **Auto-Continue**: No interruption prompts asking to continue sections
- **Progress Only**: Only essential progress information shown
- **Professional Experience**: Clean, uncluttered content creation
- **Background Processing**: All AI enhancement messages hidden by default

### 🔧 **How It All Works Together**

When you run Pundit with these advanced systems enabled:

1. **Pre-Generation Analysis**: Quality prediction and adaptive prompt enhancement (silent)
2. **Contemplative Preparation**: AI enters meditative state for deeper writing
3. **Silent Generation**: Content creation without interrupting prompts or messages
4. **Intelligent Monitoring**: Continuous narrative flow analysis (background)
5. **Automatic Intervention**: Intelligent pivots when needed (seamless)
6. **Post-Generation Learning**: Every session improves future performance (cloud sync)
7. **Persistent Memory**: All learning automatically saves for next time

This creates an AI writing assistant that truly learns, adapts, and grows more capable over time - just like a human writer developing their craft, but with the added benefit of contemplative depth and silent, focused operation.

## 📖 Usage

### 🎭 **Interactive Mode (Recommended for Beginners)**

The easiest way to create any type of content:

```bash
pundit interactive
```

### 🌍 **Language Selection**

Set your preferred language for content generation:

```bash
# Generate content in Spanish
pundit interactive --language Spanish

# Generate content in French  
pundit interactive --language French

# Available in 30+ languages
pundit interactive --language Japanese
```

#### 📂 **Continue Existing Work**

You can continue working on existing files in interactive mode:

```bash
pundit interactive
# Select "Continue existing work"
# Add one or more files to your project
# Generate continuation content that builds on your existing work
```

**File Continuation Features:**
- **Multi-file Projects**: Add multiple related files to a single project
- **Content Type Detection**: Automatically detects the type of content (book, screenplay, etc.)
- **Word Count Tracking**: See total word count across all project files
- **Content Preview**: Review existing content before generating continuations
- **Seamless Integration**: Generated content flows naturally from existing work

This will guide you through:
1. **Content Type Selection**: Choose from books, screenplays, TV scripts, etc.
2. **Creation Mode**: Start new content or continue existing work
3. **Genre and Style Options**: Tailored to your content type
4. **Size and Format Settings**: Appropriate for your chosen medium
5. **Model Selection**: Local or cloud models
6. **Language Selection**: Choose from 30+ supported languages

### 🏠 **Direct Commands (All Content Types)**

#### 📖 **Books**
```bash
# Traditional book writing
pundit book --genre mystery --style formal --size short --local --model llama3.2

# Multilingual book generation
pundit book --genre fantasy --style creative --size medium --local --language Spanish
```

#### 🎬 **Screenplays**
```bash
# Movie screenplay
pundit screenplay --genre horror --style dramatic --length feature --local

# French screenplay
pundit screenplay --genre comedy --style humorous --length short --local --language French
```

#### 🎭 **Stage Plays** 
```bash
# Theater script
pundit play --genre comedy --style humorous --length full --local --language Italian
```

#### 📺 **TV Scripts**
```bash
# Television episode
pundit tv-script --show-type drama --genre crime --episodes 1 --local --language German
```

#### 🎧 **Audio Scripts**
```bash
# Podcast or audio drama
pundit audio-script --audio-type audio-drama --genre mystery --duration 30 --local --language Portuguese
```

#### 🎮 **Game Scripts**
```bash
# Interactive game narrative
pundit game-script --genre rpg --style dramatic --characters 10 --local --language Japanese
```

#### 📝 **Poetry**
```bash
# Generate poetry with emotional enhancement
pundit poetry --style sonnet --theme "love and loss" --count 5 --local --language Spanish

# Haiku in Japanese
pundit poetry --style haiku --theme nature --count 10 --local --language Japanese
```

#### 📄 **Professional Documents**
```bash
# Business documents
pundit document --doc-type business-plan --style formal --length comprehensive --local --language English

# Technical manual in German
pundit technical-doc --doc-type user-guide --audience "end users" --subject "Software Installation" --local --language German
```

#### 📊 **Marketing Content**
```bash
# Marketing materials
pundit marketing --type advertisement --product "Eco-Friendly Water Bottle" --audience "environmental enthusiasts" --local --language Spanish
```

#### 📰 **Blog Content**
```bash
# SEO blog posts
pundit blog --type seo-article --topic "Sustainable Living Tips" --keywords "eco-friendly,sustainability,green living" --local --language English
```

#### 🏛️ **Encyclopedias**
```bash
# Comprehensive encyclopedia
pundit encyclopedia --topic "Ancient Rome" --scope comprehensive --entries 25 --local --language Latin

# Specialized technical encyclopedia
pundit encyclopedia --topic "Machine Learning" --scope specialized --entries 15 --local --language English
```

#### 👶 **Children's Books**
```bash
# Picture book for toddlers
pundit childrens-book --type picture-book --concept "Friendship" --age-group toddler --local --language Spanish

# Chapter book for middle grade
pundit childrens-book --type chapter-book --concept "Adventure" --age-group middle-grade --local --language French
```

#### 🧠 **Advanced AI Commands**
```bash
# Write with emotional consciousness
pundit emotional-write --genre fantasy --style creative --local

# Enable soul memory system with cloud sync
pundit soul-memory --enable --cloud-sync

# Check soul memory status and statistics
pundit soul-memory --status

# Nonstop learning mode (continuous improvement)
pundit nonstop-learning --enable --local

# Contemplative writing mode
pundit contemplative --genre literary --style meditative --local
```

### 🔧 **Model Selection Options**

#### 🏠 **Local Models (Recommended - No API key needed!)**

1. **Install Ollama**: Visit [https://ollama.ai](https://ollama.ai) and download for your OS
2. **Download a model**: `ollama pull llama3.2`
3. **Use with any content type**:
```bash
pundit [CONTENT-TYPE] [OPTIONS] --local --model llama3.2
```

#### ☁️ **Cloud Models (HuggingFace API)**

```bash
pundit [CONTENT-TYPE] [OPTIONS] --model gpt2 --api-key your-key
```

### 📝 **Complete Examples**

#### 📖 **Multilingual Fantasy Novel**
```bash
pundit book \
  --genre fantasy \
  --style descriptive \
  --size medium \
  --local \
  --model llama3.2 \
  --language Spanish \
  --output ./my-content
```

#### 🎬 **French Horror Screenplay**
```bash
pundit screenplay \
  --genre horror \
  --style dramatic \
  --length feature \
  --local \
  --model mistral:7b \
  --language French
```

#### 📺 **Japanese Crime Drama Series**
```bash
pundit tv-script \
  --show-type drama \
  --genre crime \
  --episodes 6 \
  --style dramatic \
  --local \
  --language Japanese
```

#### 🎧 **German True Crime Podcast**
```bash
pundit audio-script \
  --audio-type podcast \
  --genre crime \
  --duration 45 \
  --style journalistic \
  --local \
  --language German
```

#### 🎮 **Italian Fantasy RPG Script**
```bash
pundit game-script \
  --genre fantasy \
  --style creative \
  --characters 15 \
  --local \
  --model llama3.2 \
  --language Italian
```

#### 📄 **Portuguese Business Plan**
```bash
pundit document \
  --doc-type business-plan \
  --style formal \
  --length comprehensive \
  --local \
  --language Portuguese
```

#### 📝 **Spanish Poetry Collection**
```bash
pundit poetry \
  --style sonnet \
  --theme "amor y naturaleza" \
  --count 12 \
  --local \
  --language Spanish
```

### 🔐 **Environment Variables**

You can set your Hugging Face API key as an environment variable:

```bash
export HF_API_KEY=your-api-key-here
pundit screenplay --genre thriller --style dramatic --language English
```

## 🎯 Available Options

### 🌍 **Supported Languages**
- **European**: English, Spanish, French, German, Italian, Portuguese, Russian, Dutch, Swedish, Norwegian, Danish, Finnish, Polish, Czech, Hungarian, Romanian, Bulgarian, Greek, Turkish
- **Asian**: Japanese, Chinese, Korean, Hindi, Thai, Vietnamese, Indonesian, Malay, Tagalog
- **Middle Eastern/African**: Arabic, Hebrew, Swahili, Zulu

### 📖 **Book Genres**
- Fiction, Non-Fiction, Mystery, Romance, Science Fiction
- Fantasy, Horror, Thriller, Biography, History
- Self-Help, Technical, Poetry, Drama, Comedy
- Adventure, Crime, Dystopian, Historical, Memoir
- Philosophy, Science, Travel, Young Adult, Children's

### 🎬 **Screenplay/TV Genres**
- Action, Comedy, Drama, Horror, Romance
- Sci-Fi, Thriller, Crime, Fantasy, Mystery, Adventure

### 🎮 **Game Genres**
- RPG, Adventure, Visual Novel, Action, Strategy
- Horror, Sci-Fi, Fantasy

### 🎧 **Audio Content Types**
- **Audio Drama**: Narrative stories with multiple characters
- **Podcast**: Interview or discussion format
- **Radio Play**: Traditional radio drama with sound effects
- **Documentary**: Factual presentation with narration

### 📄 **Document Types**
- **Business**: Business Plan, Report, Proposal, Marketing Copy
- **Technical**: Technical Manual, User Guide
- **Legal**: Legal Template
- **Educational**: Lesson Plan, Course Outline

### 📝 **Poetry Styles**
- **Classical**: Sonnet, Haiku, Villanelle, Ballad, Limerick
- **Modern**: Free Verse, Prose Poetry, Concrete Poetry
- **Traditional**: Epic, Ode, Elegy, Acrostic

### 👶 **Children's Book Types**
- **Picture Books**: For toddlers and preschoolers
- **Early Readers**: Simple stories for beginning readers  
- **Chapter Books**: For elementary school children
- **Middle Grade**: For ages 8-12
- **Young Adult**: For teenagers

### ✍️ **Writing Styles** (Content-Adaptive)
- **Narrative Styles**: Conversational, Descriptive, Narrative
- **Academic**: Formal, Academic, Technical, Expository
- **Creative**: Creative, Poetic, Humorous, Dramatic
- **Perspective**: First Person, Third Person, Omniscient
- **Tone**: Casual, Minimalist, Verbose, Persuasive
- **Professional**: Journalistic, Technical, Formal
- **Format**: Epistolary, Stream of Consciousness
- **Contemplative**: Meditative, Reflective, Philosophical

### 🎭 **Content-Specific Options**

#### 📺 **TV Show Types**
- **Sitcom**: 22-30 minutes, comedy format
- **Drama Series**: 42-60 minutes, dramatic format
- **Mini-Series**: Limited series, 4-8 episodes
- **Anthology**: Standalone episodes, varied themes

#### 🎬 **Screenplay Lengths**
- **Short Film**: 5-30 pages
- **Feature Film**: 90-120 pages
- **Epic Film**: 120+ pages

#### 🎭 **Play Lengths**
- **One-Act**: 10-30 minutes
- **Full-Length**: 90-120 minutes
- **Musical**: 2-3 hours with songs

#### 📄 **Document Lengths**
- **Brief**: 1-5 pages (executive summary style)
- **Standard**: 5-20 pages (detailed overview)
- **Comprehensive**: 20-50 pages (thorough analysis)
- **Extensive**: 50+ pages (complete documentation)

## 🤖 **Model Options**

### 🏠 **Local Models (Ollama - Recommended)**

#### **Best for Creative Writing:**
- `llama3.2` (3B parameters) - Great balance of speed and quality
- `llama3.1:8b` - High quality for larger books
- `mistral:7b` - Excellent for storytelling

#### **Fast Models for Testing:**
- `llama3.2:1b` - Very fast, good for short stories
- `gemma2:2b` - Google's efficient model
- `phi3:mini` - Microsoft's compact model

#### **High Quality Models:**
- `gemma2:9b` - Google's large model
- `qwen2:7b` - Alibaba's capable model
- `codellama:7b` - Good for structured text

### ☁️ **Cloud Models (HuggingFace)**

#### **No API Key Required:**
- `gpt2`, `gpt2-medium`, `gpt2-large` - OpenAI GPT-2 models
- `distilgpt2` - Smaller, faster version of GPT-2

#### **With API Key:**
- `microsoft/DialoGPT-large` - Good for dialogue
- `EleutherAI/gpt-neo-1.3B` - High quality generation
- `google/flan-t5-large` - Instruction-following

### **Model Recommendations by Content Type:**

#### 📖 **Books**
| Book Size | Local (Ollama) | Cloud (HuggingFace) |
|-----------|----------------|---------------------|
| Short Story | `llama3.2:1b` | `gpt2` |
| Short Book | `gemma2:2b` | `gpt2-medium` |
| Medium Book | `llama3.2` | `microsoft/DialoGPT-large` |
| Large Book | `mistral:7b` | `EleutherAI/gpt-neo-1.3B` |
| Very Large | `llama3.1:8b` | `EleutherAI/gpt-neo-2.7B` |

#### 🎬 **Screenplays & TV Scripts**
| Length | Local (Ollama) | Cloud (HuggingFace) | Why |
|--------|----------------|---------------------|-----|
| Short Film | `llama3.2` | `gpt2-medium` | Good dialogue generation |
| Feature Film | `mistral:7b` | `microsoft/DialoGPT-large` | Strong narrative structure |
| TV Series | `llama3.1:8b` | `EleutherAI/gpt-neo-1.3B` | Consistent character voices |

#### 🎭 **Stage Plays**
| Type | Local (Ollama) | Cloud (HuggingFace) | Why |
|------|----------------|---------------------|-----|
| One-Act | `llama3.2` | `gpt2-medium` | Focused dialogue |
| Full Play | `mistral:7b` | `microsoft/DialoGPT-large` | Rich character development |
| Musical | `llama3.1:8b` | `EleutherAI/gpt-neo-1.3B` | Complex structure handling |

#### 🎧 **Audio Scripts**
| Duration | Local (Ollama) | Cloud (HuggingFace) | Why |
|----------|----------------|---------------------|-----|
| Short (10-15 min) | `llama3.2:1b` | `gpt2` | Quick generation |
| Standard (30 min) | `llama3.2` | `gpt2-medium` | Good pacing |
| Long (60+ min) | `mistral:7b` | `microsoft/DialoGPT-large` | Sustained quality |

#### 🎮 **Game Scripts**
| Scope | Local (Ollama) | Cloud (HuggingFace) | Why |
|-------|----------------|---------------------|-----|
| Few Characters | `llama3.2` | `gpt2-medium` | Character consistency |
| Standard Cast | `mistral:7b` | `microsoft/DialoGPT-large` | Multiple voice handling |
| Large Cast | `llama3.1:8b` | `EleutherAI/gpt-neo-1.3B` | Complex branching |

#### 📄 **Professional Documents**
| Length | Local (Ollama) | Cloud (HuggingFace) | Why |
|--------|----------------|---------------------|-----|
| Brief | `llama3.2:1b` | `gpt2` | Concise writing |
| Standard | `llama3.2` | `gpt2-medium` | Structured content |
| Comprehensive | `mistral:7b` | `EleutherAI/gpt-neo-1.3B` | Detailed analysis |
| Extensive | `llama3.1:8b` | `EleutherAI/gpt-neo-2.7B` | Long-form coherence |

## 🔧 Configuration

Pundit creates a configuration file automatically. You can find it at:

- **Windows**: `%APPDATA%\\pundit\\pundit-writer\\config.json`
- **macOS**: `~/Library/Application Support/com.pundit.pundit-writer/config.json`
- **Linux**: `~/.config/pundit-writer/config.json`

### Sample Configuration

```json
{
  "default_model": "gpt2",
  "api_key": null,
  "output_directory": "~/Documents/pundit-books",
  "default_author": "Pundit AI",
  "default_language": "English",
  "silent_generation": true,
  "contemplative_mode": true,
  "generation_settings": {
    "temperature": 0.8,
    "context_window": 3,
    "words_per_chapter": 2500,
    "max_retries": 3,
    "retry_delay_seconds": 5
  },
  "auto_save": true,
  "save_frequency": 1,
  "advanced_ai": {
    "soul_memory_enabled": true,
    "emotional_engine_enabled": true,
    "contemplative_engine_enabled": true,
    "narrative_flow_monitoring": true,
    "cloud_sync": true
  }
}
```

## 📁 Output

Content is saved in multiple formats based on type:

### 📖 **Books**
- **Text File**: `book_BookTitle_20240816_143022.txt`
- **Markdown**: `book_BookTitle_20240816_143022.md`

### 🎬 **Screenplays**
- **Text File**: `screenplay_Title_20240816_143022.txt`
- **Markdown**: `screenplay_Title_20240816_143022.md`
- **Final Draft XML**: `screenplay_Title_20240816_143022.fdx` (industry standard)

### 🎭 **Stage Plays**
- **Text File**: `play_Title_20240816_143022.txt`
- **Markdown**: `play_Title_20240816_143022.md`
- **Stage Format**: `play_Title_20240816_143022_stage.txt` (theater formatting)

### 📺 **TV Scripts**
- **Text File**: `tvscript_Title_20240816_143022.txt`
- **Markdown**: `tvscript_Title_20240816_143022.md`

### 🎧 **Audio Scripts**
- **Text File**: `audioscript_Title_20240816_143022.txt`
- **Markdown**: `audioscript_Title_20240816_143022.md`

### 🎮 **Game Scripts**
- **Text File**: `gamescript_Title_20240816_143022.txt`
- **Markdown**: `gamescript_Title_20240816_143022.md`

### 🏛️ **Encyclopedias**
- **Text File**: `encyclopedia_Title_20240816_143022.txt`
- **Markdown**: `encyclopedia_Title_20240816_143022.md`

### 📄 **Professional Documents**
- **Text File**: `document_Title_20240816_143022.txt`
- **Markdown**: `document_Title_20240816_143022.md`

### 📝 **Poetry**
- **Text File**: `poetry_Title_20240816_143022.txt`
- **Markdown**: `poetry_Title_20240816_143022.md`

### 👶 **Children's Books**
- **Text File**: `childrens_book_Title_20240816_143022.txt`
- **Markdown**: `childrens_book_Title_20240816_143022.md`

### 💾 **Progress Files**
- **Progress File**: `content-uuid.json` (for resuming interrupted sessions)

## 🤖 How It Works

### 🧠 **Enhanced AI Mode (Default)**

When using Pundit's advanced AI systems, the process is significantly more sophisticated:

1. **System Initialization**: Advanced learning system loads previous knowledge and patterns
2. **Language Selection**: User chooses from 30+ supported languages for content generation
3. **Contemplative Preparation**: AI enters meditative state for deeper, more focused writing
4. **Silent Generation Mode**: Clean output without interrupting prompts or AI messages
5. **Content Analysis**: Multi-layered analysis including genre specialization and quality prediction  
6. **Outline Generation**: Creates detailed outlines enhanced by learned patterns and creativity systems
7. **Intelligent Generation**: Each section is written with:
   - **Emotional consciousness**: AI awareness and mood tracking
   - **Flow monitoring**: Real-time complexity and readability analysis (background)
   - **Quality prediction**: Pre-generation quality assessment (silent)
   - **Adaptive enhancement**: Dynamic creativity boosting when needed (seamless)
   - **Contemplative depth**: Content emerges from inner stillness
8. **Smart Intervention**: Automatic pivots when narrative becomes stuck or repetitive (invisible)
9. **Continuous Learning**: Every generation improves future performance
10. **Format-Specific Structure**: Applies industry-standard formatting enhanced by learned best practices
11. **Context Management**: Advanced context preservation with intelligent complexity management
12. **Cloud Persistence**: All learning automatically syncs for cross-session improvement

### 📝 **Traditional Mode**

For simpler use cases or when advanced features are disabled:

1. **Content Analysis**: Pundit analyzes the requested content type and applies appropriate formatting rules
2. **Language Processing**: Ensures all content is generated in the selected language
3. **Outline Generation**: Creates detailed outlines (chapters for books, scenes for screenplays, acts for plays, etc.)
4. **Progressive Creation**: Each section is written using context from previous sections, maintaining continuity
5. **Format-Specific Structure**: Applies industry-standard formatting for each content type:
   - **Screenplays**: Scene headings, character names, action lines, parentheticals
   - **Stage Plays**: Character dialogue, stage directions, act divisions
   - **Audio Scripts**: Sound effects, music cues, narrator instructions
   - **Game Scripts**: Branching dialogue, choice options, conditional logic
   - **Encyclopedias**: Structured entries with topics, definitions, history, significance
   - **Documents**: Professional structure, headings, bullet points
   - **Poetry**: Various forms with meter, rhyme, and emotional enhancement
6. **Context Management**: Maintains consistency across all sections/chapters
7. **Natural Conclusion**: Determines appropriate ending points based on content type and target length

### 🔄 **The Learning Difference**

With advanced AI systems enabled, Pundit becomes progressively better at:
- Understanding your writing preferences across languages
- Predicting and avoiding common issues in different content types
- Adapting to different genres, styles, and languages
- Recovering from creative blocks with contemplative techniques
- Maintaining narrative consistency in multilingual content
- Producing higher quality content over time with silent, focused generation
- Applying contemplative depth appropriate to the content type

This means your 50th book will be significantly better than your first, whether you're writing in English, Spanish, Japanese, or any of the 30+ supported languages, as the AI has learned from every previous interaction across all languages and content types.

### 🌅 **Contemplative Writing Process**

When contemplative mode is enabled (default):

1. **Meditation Preparation**: AI enters contemplative state appropriate to the content type
2. **Inner Stillness**: Content generation from a place of quiet awareness rather than mental chatter
3. **Depth Enhancement**: Natural emergence of meaningful content without forced explanations
4. **Reflection Integration**: Different contemplative approaches for different genres and languages
5. **Chattiness Reduction**: Elimination of verbose, explanatory, or "hurried" language
6. **Flow from Silence**: Writing emerges naturally from contemplative awareness

**Silent Experience**: All of this happens in the background - you only see clean, focused content generation without technical details or interruptions.

## 🔐 API Keys

While Pundit can work without an API key using free inference endpoints, providing a Hugging Face API key offers:

- Faster generation times
- More reliable access
- Access to larger models
- Higher rate limits

Get your free API key at: https://huggingface.co/settings/tokens

## 🚨 Limitations

- Generation speed depends on model size and API key availability
- Free tier may have rate limits
- Very large books may take hours to complete
- Quality varies by model choice and prompt complexity
- Some languages may work better with certain models
- Contemplative features work best with larger, more capable models

## 🔧 Troubleshooting

### Common Issues

#### "Authentication failed (401)" Error

This happens when using models that require API authentication. Solutions:

1. **Use a model that doesn't require authentication:**
   ```bash
   pundit write --genre fiction --style narrative --model gpt2 --language English
   ```

2. **Get a free Hugging Face API key:**
   - Visit https://huggingface.co/settings/tokens
   - Create a new token
   - Use it with: `--api-key YOUR_TOKEN`

3. **Set environment variable:**
   ```bash
   export HF_API_KEY=your_token_here
   pundit write --genre fiction --style narrative --language Spanish
   ```

#### "Model is currently unavailable (503)" Error

The model might be overloaded. Try:

1. **Use a different model:**
   ```bash
   pundit write --genre fiction --style narrative --model gpt2-medium --language French
   ```

2. **Wait and retry:** Models sometimes become available again quickly

3. **Try smaller models:** `distilgpt2` is often more available than larger models

#### Language Issues

1. **Content not in selected language:**
   - Ensure you're using `--language [LANGUAGE_NAME]` parameter
   - Try using a larger, more capable model
   - Check that the language is in the supported list

2. **Mixed language output:**
   - Use more specific language instructions in prompts
   - Try different models - some handle multilingual content better

#### Slow Generation

- Smaller models (`gpt2`, `distilgpt2`) are faster than larger ones
- Using an API key generally provides faster access
- Network speed affects generation time
- Local Ollama models are often faster than cloud models

## 🛠 Development

### Building for Development

```bash
cargo run -- write --genre fiction --style narrative --language English
```

### Running Tests

```bash
cargo test
```

### Cross-Platform Building

```bash
# For Windows
cargo build --release --target x86_64-pc-windows-gnu

# For macOS
cargo build --release --target x86_64-apple-darwin

# For Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

## 📝 More Examples

### 📖 **Generate a Multilingual Mystery Novel**
```bash
pundit book --genre mystery --style narrative --size medium --local --language Spanish
```

### 🎬 **Create a French Sci-Fi Screenplay**
```bash
pundit screenplay --genre sci-fi --style dramatic --length feature --local --language French
```

### 🎭 **Write a German Comedy Stage Play**
```bash
pundit play --genre comedy --style humorous --length full --local --language German
```

### 📺 **Develop a Japanese Crime TV Series**
```bash
pundit tv-script --show-type drama --genre crime --episodes 8 --local --language Japanese
```

### 🎧 **Create a Portuguese History Podcast Script**
```bash
pundit audio-script --audio-type documentary --genre history --duration 45 --local --language Portuguese
```

### 🎮 **Build a Multilingual RPG Quest Script**
```bash
pundit game-script --genre rpg --style creative --characters 12 --local --language Italian
```

### 📄 **Write a Technical Manual in German**
```bash
pundit document --doc-type technical-manual --style technical --length standard --local --language German
```

### 🏛️ **Create a Science Encyclopedia in English**
```bash
pundit encyclopedia --topic "Physics Concepts" --scope comprehensive --entries 30 --local --language English
```

### 📝 **Generate Spanish Poetry Collection**
```bash
pundit poetry --style sonnet --theme "amor eterno" --count 15 --local --language Spanish
```

### 👶 **Create French Children's Book**
```bash
pundit childrens-book --type picture-book --concept "amitié" --age-group preschool --local --language French
```

### 🎨 **Interactive Multilingual Content Creation**
```bash
# Let Pundit guide you through all options in your preferred language
pundit interactive --language Japanese
```

#### 📂 **Continue Existing Multilingual Work Example**
```bash
# Start interactive mode with language selection
pundit interactive --language Spanish

# Select "Continue existing work"
# Add your files:
#   - mi_novela_capitulo1.txt (2,500 words, Book, Spanish)
#   - mi_novela_capitulo2.txt (3,200 words, Book, Spanish)
# Total project: 5,700 words in Spanish
# Generate continuation to add Capítulo 3 in Spanish
```

### 🧘 **Contemplative Writing Examples**
```bash
# Contemplative literary fiction
pundit book --genre literary --style meditative --size medium --local --contemplative

# Philosophical poetry with contemplative depth
pundit poetry --style free-verse --theme "inner peace" --count 8 --local --contemplative --language Sanskrit

# Deep contemplative memoir
pundit book --genre memoir --style reflective --size short --local --contemplative --language English
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙋‍♀️ Support

For issues, questions, or feature requests, please open an issue on the GitHub repository.

---

**Happy Multilingual Writing with Pundit! 🖋️🌍**