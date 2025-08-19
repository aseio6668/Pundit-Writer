// Test just content types
mod content;

fn main() {
    use content::ContentType;
    let ct = ContentType::Book;
    println!("Content type: {}", ct);
}