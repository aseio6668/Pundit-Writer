use std::sync::Arc;
use std::collections::HashMap;
use anyhow::Result;

pub struct MemoryOptimizedBuffer {
    data: Vec<String>,
    max_size: usize,
    current_size: usize,
}

impl MemoryOptimizedBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: Vec::with_capacity(16),
            max_size,
            current_size: 0,
        }
    }

    pub fn push(&mut self, content: String) -> Result<()> {
        let content_size = content.len();
        
        if self.current_size + content_size > self.max_size {
            self.flush_oldest(content_size)?;
        }
        
        self.current_size += content_size;
        self.data.push(content);
        Ok(())
    }

    fn flush_oldest(&mut self, needed_space: usize) -> Result<()> {
        let mut freed = 0;
        while freed < needed_space && !self.data.is_empty() {
            if let Some(removed) = self.data.drain(0..1).next() {
                freed += removed.len();
                self.current_size -= removed.len();
            }
        }
        Ok(())
    }

    pub fn get_all(&self) -> String {
        self.data.join("\n")
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.current_size = 0;
    }
}

pub struct ContentCache {
    cache: HashMap<String, Arc<String>>,
    max_entries: usize,
}

impl ContentCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_entries),
            max_entries,
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<String>> {
        self.cache.get(key).cloned()
    }

    pub fn insert(&mut self, key: String, value: String) {
        if self.cache.len() >= self.max_entries {
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }
        self.cache.insert(key, Arc::new(value));
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

pub fn optimize_string_allocation(content: &str, estimated_final_size: usize) -> String {
    let mut result = String::with_capacity(estimated_final_size.max(content.len() * 2));
    result.push_str(content);
    result
}

pub fn batch_string_operations<F>(operations: Vec<F>) -> Result<Vec<String>> 
where
    F: FnOnce() -> Result<String>,
{
    let mut results = Vec::with_capacity(operations.len());
    
    for operation in operations {
        match operation() {
            Ok(result) => results.push(result),
            Err(e) => {
                eprintln!("Warning: Batch operation failed: {}", e);
                results.push(String::new());
            }
        }
    }
    
    Ok(results)
}