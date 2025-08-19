# File Continuation Guide

This guide covers Pundit's file continuation feature, which allows you to continue working on existing documents and build upon previous content.

## 🎯 Overview

The continuation feature enables you to:
- **Continue existing work**: Add new content to partially completed books, screenplays, documents, etc.
- **Combine multiple files**: Work with projects spanning multiple files
- **Maintain consistency**: Generate content that flows naturally from existing work
- **Track progress**: Monitor word counts and project scope across all files

## 🚀 Getting Started

### Access Continuation Mode

```bash
pundit interactive
```

From the main menu, select:
```
📂 Continue existing work
```

### Workflow Overview

1. **Project Setup**: Add files to your continuation project
2. **File Management**: Review, add, or remove files as needed
3. **Content Review**: Preview existing content and project statistics
4. **Generation**: Create continuation content that builds on existing work
5. **Output**: Save new content as markdown files

## 📁 File Management

### Adding Files

When you select "📁 Add file to project", you'll be prompted to enter file paths:

```
Enter file path (or 'back' to return): ./my_novel/chapter1.txt
✅ Added: ./my_novel/chapter1.txt (2,500 words)
Add another file? (y/N)
```

**Supported File Types:**
- `.txt` - Plain text files
- `.md` - Markdown files
- `.doc` - Word documents (text content only)
- Any text-based file format

**File Path Examples:**
```bash
# Relative paths
./my_book/chapter1.txt
../documents/story.md

# Absolute paths
/home/user/writing/novel.txt
C:\Users\User\Documents\screenplay.txt

# Files that don't exist yet (will be created)
./new_chapter.txt
```

### File Status Indicators

Files are displayed with status indicators:
- **✅** - File exists and was successfully read
- **❌** - File not found (will be created during generation)

### Removing Files

Select "🗑️ Remove a file" to remove files from your project:

```
Which file would you like to remove?
1. ./chapter1.txt (2,500 words, Book)
2. ./chapter2.txt (3,200 words, Book)
← Cancel
```

## 📊 Project Management

### Project Information

The system tracks:
- **Primary File**: The first file added (determines project type)
- **Project Type**: Automatically detected content type (Book, Screenplay, etc.)
- **Total Files**: Number of files in the project
- **Total Words**: Combined word count across all files

### Content Type Detection

Pundit automatically detects content types based on file content:

| Content Type | Detection Keywords |
|--------------|-------------------|
| **Book** | "chapter", "novel", "book" |
| **Screenplay** | "scene", "fade in", screenplay formatting |
| **Stage Play** | "act", "scene", "stage" |
| **Poetry** | "stanza", "verse", "poem" |
| **Technical Doc** | "# ", "## " (markdown headers) |
| **Meeting Notes** | "meeting", "agenda" |
| **Marketing** | "marketing", "campaign" |
| **Blog Post** | "blog", "article" |
| **Strategic Doc** | "strategic", "business plan" |
| **Research** | "research", "white paper" |

### Project Review

Select "📝 Review project files" to see detailed project information:

```
📋 Project Review
================
Primary file: ./my_novel/chapter1.txt
Detected type: Book
Total files: 3
Total words: 8,700

Files:
  1. ./my_novel/chapter1.txt (2,500 words, Book) - ✅ Exists
  2. ./my_novel/chapter2.txt (3,200 words, Book) - ✅ Exists
  3. ./my_novel/chapter3.txt (3,000 words, Book) - ✅ Exists

Would you like to preview the combined content? (y/N)
```

**Content Preview**: Shows the first 500 characters of combined content to help you review what will be used as context for generation.

## 🔄 Navigation and Controls

### Back Navigation

You can navigate back at any point:
- **Selection Menus**: Choose "← Back to main menu"
- **Text Inputs**: Type `back` to return to the previous step
- **Any Point**: Use Ctrl+C to exit completely

### Input Controls

- **File Paths**: Enter relative or absolute paths
- **Confirmation**: Most actions require confirmation before proceeding
- **Validation**: Invalid paths or operations show helpful error messages

## ⚙️ Generation Process

### Content Context

When generating continuations, Pundit uses:
- **Existing Content**: Full content from all project files
- **Content Type**: Detected or manually specified type
- **Style Consistency**: Maintains writing style from existing work
- **Narrative Flow**: Ensures smooth continuation from where you left off

### Generation Settings

The system automatically:
- Detects appropriate writing style from existing content
- Maintains character consistency (for fiction)
- Follows established formatting patterns
- Continues plot/argument threads naturally

### Output Generation

Generated content is saved as:
```
continuation_[ProjectName]_[Timestamp].md
```

Example:
```
continuation_MyNovel_20240819_143022.md
```

## 📝 Best Practices

### File Organization

**Recommended Structure:**
```
my_project/
├── chapter1.txt
├── chapter2.txt
├── chapter3.txt
└── notes/
    ├── outline.txt
    └── characters.txt
```

**Project Setup Tips:**
1. **Start with primary file**: Add your main content file first
2. **Related files only**: Include files that are part of the same work
3. **Consistent formatting**: Use similar formatting across files
4. **Clear naming**: Use descriptive filenames that indicate content order

### Content Preparation

**Before Adding Files:**
- Ensure files are saved in a readable text format
- Check that content is complete up to the continuation point
- Review content for consistency in style and tone
- Remove any incomplete sentences or thoughts at the end

**File Content Guidelines:**
- **Books**: Include complete chapters or scenes
- **Screenplays**: Include complete scenes with proper formatting
- **Documents**: Include complete sections or chapters
- **Poetry**: Include complete poems or verses

### Continuation Strategy

**Effective Continuation:**
1. **Clear stopping point**: End existing content at a natural break
2. **Context clues**: Include enough context for the AI to understand the direction
3. **Style consistency**: Maintain consistent formatting and style
4. **Logical progression**: Ensure the continuation request makes narrative sense

## 🛠️ Troubleshooting

### Common Issues

#### File Not Found
```
❌ Error: No such file or directory
```
**Solutions:**
- Check file path spelling and case sensitivity
- Use absolute paths if relative paths aren't working
- Ensure the file exists and is readable

#### Permission Denied
```
❌ Error: Permission denied reading file
```
**Solutions:**
- Check file permissions
- Run Pundit with appropriate user permissions
- Move files to a location with read access

#### Content Type Mismatch
```
⚠️ Warning: Mixed content types detected
```
**Solutions:**
- Review files to ensure they're all the same type of content
- Remove unrelated files from the project
- Consider creating separate projects for different content types

#### Large File Handling
```
⚠️ Warning: Large file detected (50,000+ words)
```
**Solutions:**
- Consider breaking large files into smaller sections
- Ensure adequate system memory for processing
- Use more powerful AI models for complex continuations

### Error Recovery

**If Generation Fails:**
1. Check that all files are accessible
2. Verify content type detection is correct
3. Ensure adequate disk space for output
4. Try with a smaller subset of files first

**If Output is Inconsistent:**
1. Review existing content for clarity
2. Ensure files are in logical order
3. Check that content ends at appropriate stopping points
4. Consider using fewer files for better consistency

## 🔧 Advanced Features

### Multi-Format Projects

You can include files in different formats:
```bash
./novel/
├── chapter1.txt      # Plain text
├── chapter2.md       # Markdown
└── notes.txt         # Research notes
```

### Content Type Override

If automatic detection is incorrect, you can:
1. Modify file content to include appropriate keywords
2. Start a new project with correctly detected content
3. Manually adjust the primary file to influence detection

### Batch Processing

For large projects:
1. Add all related files to one project
2. Review combined word count and scope
3. Generate continuations in manageable sections
4. Use the output as input for subsequent continuations

## 📈 Performance Tips

### File Size Optimization

- **Optimal**: 1,000-10,000 words per file
- **Large**: 10,000-50,000 words (may be slower)
- **Very Large**: 50,000+ words (consider splitting)

### Generation Efficiency

- **Local Models**: Faster for smaller projects
- **Cloud Models**: Better for complex continuations
- **Model Selection**: Choose appropriate model size for content length

### Memory Management

- **Small Projects**: Any model will work
- **Large Projects**: Use more powerful models
- **Very Large**: Consider breaking into smaller continuation sessions

---

**Note**: The continuation feature is designed to work seamlessly with all content types supported by Pundit. Whether you're continuing a novel, adding scenes to a screenplay, or expanding a technical document, the system adapts to maintain consistency with your existing work.