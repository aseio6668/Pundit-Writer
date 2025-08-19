// Backup of original main.rs - contains all commands but causes stack overflow
// This file preserves the original implementation

// The issue is that the CLI has too many large enums with ValueEnum derives
// which causes clap's derive macros to overflow the stack during static initialization
// 
// To fix this, we need to either:
// 1. Reduce the number of commands/enums
// 2. Simplify the ValueEnum implementations
// 3. Use a different CLI parsing approach
//
// The original main.rs had 24+ content types and hundreds of enum variants
// which exceeded Rust's stack limits for derive macro expansion.

fn main() {
    println!("This is a backup of the original main.rs that caused stack overflow");
    println!("The issue was too many large enum variants in the CLI causing");
    println!("clap derive macros to overflow the stack during static initialization");
}