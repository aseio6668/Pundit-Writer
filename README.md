# Pundit - AI Content Creator

A comprehensive cross-platform CLI tool for generating professional content using AI models. Pundit can create complete books, screenplays, stage plays, TV scripts, audio dramas, game narratives, and business documents from start to finish without interruption, adapting to various genres, writing styles, and content specifications.

## 🚀 Features

### 📚 **Complete Content Creation Suite**
- **📖 Books**: Complete novels from simple prompts (fiction, non-fiction, any genre)
- **🎬 Screenplays**: Professional movie scripts with industry-standard formatting
- **🎭 Stage Plays**: Theater scripts with stage directions and character formatting
- **📺 TV Scripts**: Television episodes for sitcoms, dramas, mini-series, and anthologies
- **🎧 Audio Scripts**: Podcast scripts, audio dramas, radio plays, and documentaries
- **🎮 Game Scripts**: Interactive narratives with branching dialogue for video games
- **📄 Professional Documents**: Business plans, technical manuals, reports, proposals, and more

### 🔧 **Advanced Capabilities**
- **Local & Cloud Models**: Use local Ollama models OR HuggingFace API
- **Content-Specific Formatting**: Industry-standard formatting for each content type
- **Interactive Creation Mode**: Guided setup through interactive prompts for all content types
- **File Continuation**: Continue working on existing documents and add to previous work
- **Flexible Sizing**: From brief documents to extensive manuscripts
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Offline Capable**: Download models locally for offline content generation
- **Progress Tracking**: Real-time progress bars and section completion tracking
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

## 📖 Usage

### 🎭 **Interactive Mode (Recommended for Beginners)**

The easiest way to create any type of content:

```bash
pundit interactive
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

### 🏠 **Direct Commands (All Content Types)**

#### 📖 **Books**
```bash
# Traditional book writing
pundit book --genre mystery --style formal --size short --local --model llama3.2
```

#### 🎬 **Screenplays**
```bash
# Movie screenplay
pundit screenplay --genre horror --style dramatic --length feature --local
```

#### 🎭 **Stage Plays** 
```bash
# Theater script
pundit play --genre comedy --style humorous --length full --local
```

#### 📺 **TV Scripts**
```bash
# Television episode
pundit tv-script --show-type drama --genre crime --episodes 1 --local
```

#### 🎧 **Audio Scripts**
```bash
# Podcast or audio drama
pundit audio-script --audio-type audio-drama --genre mystery --duration 30 --local
```

#### 🎮 **Game Scripts**
```bash
# Interactive game narrative
pundit game-script --genre rpg --style dramatic --characters 10 --local
```

#### 📄 **Professional Documents**
```bash
# Business documents
pundit document --doc-type business-plan --style formal --length comprehensive --local
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

#### 📖 **Fantasy Novel**
```bash
pundit book \
  --genre fantasy \
  --style descriptive \
  --size medium \
  --local \
  --model llama3.2 \
  --output ./my-content
```

#### 🎬 **Horror Screenplay**
```bash
pundit screenplay \
  --genre horror \
  --style dramatic \
  --length feature \
  --local \
  --model mistral:7b
```

#### 📺 **Crime Drama Series**
```bash
pundit tv-script \
  --show-type drama \
  --genre crime \
  --episodes 6 \
  --style dramatic \
  --local
```

#### 🎧 **True Crime Podcast**
```bash
pundit audio-script \
  --audio-type podcast \
  --genre crime \
  --duration 45 \
  --style journalistic \
  --local
```

#### 🎮 **Fantasy RPG Script**
```bash
pundit game-script \
  --genre fantasy \
  --style creative \
  --characters 15 \
  --local \
  --model llama3.2
```

#### 📄 **Business Plan**
```bash
pundit document \
  --doc-type business-plan \
  --style formal \
  --length comprehensive \
  --local
```

### 🔐 **Environment Variables**

You can set your Hugging Face API key as an environment variable:

```bash
export HF_API_KEY=your-api-key-here
pundit screenplay --genre thriller --style dramatic
```

## 🎯 Available Options

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

### ✍️ **Writing Styles** (Content-Adaptive)
- **Narrative Styles**: Conversational, Descriptive, Narrative
- **Academic**: Formal, Academic, Technical, Expository
- **Creative**: Creative, Poetic, Humorous, Dramatic
- **Perspective**: First Person, Third Person, Omniscient
- **Tone**: Casual, Minimalist, Verbose, Persuasive
- **Professional**: Journalistic, Technical, Formal
- **Format**: Epistolary, Stream of Consciousness

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
  "generation_settings": {
    "temperature": 0.8,
    "context_window": 3,
    "words_per_chapter": 2500,
    "max_retries": 3,
    "retry_delay_seconds": 5
  },
  "auto_save": true,
  "save_frequency": 1
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

### 📄 **Professional Documents**
- **Text File**: `document_Title_20240816_143022.txt`
- **Markdown**: `document_Title_20240816_143022.md`

### 💾 **Progress Files**
- **Progress File**: `content-uuid.json` (for resuming interrupted sessions)

## 🤖 How It Works

1. **Content Analysis**: Pundit analyzes the requested content type and applies appropriate formatting rules
2. **Outline Generation**: Creates detailed outlines (chapters for books, scenes for screenplays, acts for plays, etc.)
3. **Progressive Creation**: Each section is written using context from previous sections, maintaining continuity
4. **Format-Specific Structure**: Applies industry-standard formatting for each content type:
   - **Screenplays**: Scene headings, character names, action lines, parentheticals
   - **Stage Plays**: Character dialogue, stage directions, act divisions
   - **Audio Scripts**: Sound effects, music cues, narrator instructions
   - **Game Scripts**: Branching dialogue, choice options, conditional logic
   - **Documents**: Professional structure, headings, bullet points
5. **Context Management**: Maintains consistency across all sections/chapters
6. **Natural Conclusion**: Determines appropriate ending points based on content type and target length

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

## 🔧 Troubleshooting

### Common Issues

#### "Authentication failed (401)" Error

This happens when using models that require API authentication. Solutions:

1. **Use a model that doesn't require authentication:**
   ```bash
   pundit write --genre fiction --style narrative --model gpt2
   ```

2. **Get a free Hugging Face API key:**
   - Visit https://huggingface.co/settings/tokens
   - Create a new token
   - Use it with: `--api-key YOUR_TOKEN`

3. **Set environment variable:**
   ```bash
   export HF_API_KEY=your_token_here
   pundit write --genre fiction --style narrative
   ```

#### "Model is currently unavailable (503)" Error

The model might be overloaded. Try:

1. **Use a different model:**
   ```bash
   pundit write --genre fiction --style narrative --model gpt2-medium
   ```

2. **Wait and retry:** Models sometimes become available again quickly

3. **Try smaller models:** `distilgpt2` is often more available than larger models

#### Slow Generation

- Smaller models (`gpt2`, `distilgpt2`) are faster than larger ones
- Using an API key generally provides faster access
- Network speed affects generation time

## 🛠 Development

### Building for Development

```bash
cargo run -- write --genre fiction --style narrative
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

### 📖 **Generate a Mystery Novel**
```bash
pundit book --genre mystery --style narrative --size medium --local
```

### 🎬 **Create a Sci-Fi Screenplay**
```bash
pundit screenplay --genre sci-fi --style dramatic --length feature --local
```

### 🎭 **Write a Comedy Stage Play**
```bash
pundit play --genre comedy --style humorous --length full --local
```

### 📺 **Develop a Crime TV Series**
```bash
pundit tv-script --show-type drama --genre crime --episodes 8 --local
```

### 🎧 **Create a History Podcast Script**
```bash
pundit audio-script --audio-type documentary --genre history --duration 45 --local
```

### 🎮 **Build an RPG Quest Script**
```bash
pundit game-script --genre rpg --style creative --characters 12 --local
```

### 📄 **Write a Technical Manual**
```bash
pundit document --doc-type technical-manual --style technical --length standard --local
```

### 🎨 **Interactive Content Creation**
```bash
# Let Pundit guide you through all options
pundit interactive
```

#### 📂 **Continue Existing Work Example**
```bash
# Start interactive mode
pundit interactive

# Select "Continue existing work"
# Add your files:
#   - my_novel_chapter1.txt (2,500 words, Book)
#   - my_novel_chapter2.txt (3,200 words, Book)
# Total project: 5,700 words
# Generate continuation to add Chapter 3
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

**Happy Writing with Pundit! 🖋️**