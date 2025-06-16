# 📝 Text Analyzer (Rust Project)

A simple **modular text analyzer** written in Rust.  
The program can:

- Read text from the terminal or from a file.
- Count total characters, characters without spaces, and words.
- Search for specific words and show their positions.

---

## 📂 Project Structure

```text
text_analyzer/
├── src/
│   ├── main.rs          # Entry point
│   ├── input/           # Handles user input (terminal or file)
│   │   ├── mod.rs
│   ├── analysis/        # Handles text analysis logic
│   │   ├── mod.rs
```
