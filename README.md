# Pundit - AI Book Writer

A cross-platform CLI tool for generating entire books using Hugging Face models. Pundit can write complete books from start to finish without interruption, adapting to various genres, writing styles, and book lengths.

## 🚀 Features

- **Complete Book Generation**: Write entire books from a simple prompt
- **Local & Cloud Models**: Use local Ollama models OR HuggingFace API
- **Multiple Genres**: Fiction, Non-Fiction, Mystery, Romance, Sci-Fi, Fantasy, Horror, and many more
- **Diverse Writing Styles**: From casual to academic, humorous to dramatic
- **Flexible Book Sizes**: Short stories to unlimited-length books
- **Interactive Narration Mode**: Guided setup through interactive prompts
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Offline Capable**: Download models locally for offline book generation
- **Progress Tracking**: Real-time progress bars and chapter completion tracking
- **Multiple Output Formats**: Plain text and Markdown
- **Auto-Save**: Automatic saving of progress during generation

## 📋 Book Size Options

- **Short Story**: 1,000-7,500 words (~1 chapter)
- **Short Book**: 20,000-50,000 words (~8 chapters)
- **Medium Book**: 50,000-80,000 words (~15 chapters)
- **Large Book**: 80,000-120,000 words (~25 chapters)
- **Very Large Book**: 120,000-200,000 words (~40 chapters)
- **Unlimited Mode**: Pundit decides when to stop naturally

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

### 🏠 **Local Models (Recommended - No API key needed!)**

1. **Install Ollama**: Visit [https://ollama.ai](https://ollama.ai) and download for your OS
2. **Download a model**: `ollama pull llama3.2:1b`
3. **Generate your book**:
```bash
pundit write --genre mystery --style formal --size short --local --model llama3.2:1b
```

### ☁️ **Cloud Models (HuggingFace API)**

```bash
pundit write --genre mystery --style formal --size short --model gpt2 --api-key your-key
```

### 🎭 **Interactive Mode (Supports Both Local & Cloud)**

```bash
pundit narrate
```

This will ask you to choose between local and cloud models, then guide you through all the options.

### Complete Examples

#### Local Ollama Model:
```bash
pundit write \
  --genre fantasy \
  --style descriptive \
  --size medium \
  --local \
  --model llama3.2 \
  --output ./my-books
```

#### HuggingFace API:
```bash
pundit write \
  --genre fantasy \
  --style descriptive \
  --size medium \
  --model gpt2-large \
  --api-key your-hf-api-key
```

### Environment Variables

You can set your Hugging Face API key as an environment variable:

```bash
export HF_API_KEY=your-api-key-here
pundit write --genre mystery --style thriller
```

## 🎯 Available Options

### Genres
- Fiction, Non-Fiction, Mystery, Romance, Science Fiction
- Fantasy, Horror, Thriller, Biography, History
- Self-Help, Technical, Poetry, Drama, Comedy
- Adventure, Crime, Dystopian, Historical, Memoir
- Philosophy, Science, Travel, Young Adult, Children's

### Writing Styles
- **Narrative Styles**: Conversational, Descriptive, Narrative
- **Academic**: Formal, Academic, Technical, Expository
- **Creative**: Creative, Poetic, Humorous, Dramatic
- **Perspective**: First Person, Third Person, Omniscient
- **Tone**: Casual, Minimalist, Verbose, Persuasive
- **Format**: Epistolary, Stream of Consciousness

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

### **Model Recommendations by Book Size:**

| Book Size | Local (Ollama) | Cloud (HuggingFace) |
|-----------|----------------|---------------------|
| Short Story | `llama3.2:1b` | `gpt2` |
| Short Book | `gemma2:2b` | `gpt2-medium` |
| Medium Book | `llama3.2` | `microsoft/DialoGPT-large` |
| Large Book | `mistral:7b` | `EleutherAI/gpt-neo-1.3B` |
| Very Large | `llama3.1:8b` | `EleutherAI/gpt-neo-2.7B` |

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

Books are saved in multiple formats:

- **Text File**: `BookTitle_20240816_143022.txt`
- **Markdown**: `BookTitle_20240816_143022.md`
- **Progress File**: `book-uuid.json` (for resuming interrupted sessions)

## 🤖 How It Works

1. **Outline Generation**: Pundit first creates a detailed chapter outline
2. **Progressive Writing**: Each chapter is written using context from previous chapters
3. **Context Management**: Maintains story continuity across chapters
4. **Natural Conclusion**: In unlimited mode, Pundit determines natural ending points

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

## 📝 Examples

### Generate a Mystery Novel

```bash
pundit write --genre mystery --style narrative --size medium
```

### Write a Technical Guide

```bash
pundit write --genre technical --style academic --size short
```

### Create an Unlimited Fantasy Epic

```bash
pundit write --genre fantasy --style descriptive --size unlimited
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