use crate::filter::SourceFilter;
use crate::parser::{self, Book};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct BookContent {
    pub book: Book,
    pub files: HashMap<String, Vec<u8>>,
}

impl BookContent {
    pub fn new(book: Book) -> Self {
        Self {
            book,
            files: HashMap::new(),
        }
    }
    
    /// Add JSON content to the archive
    pub fn add_json(&mut self, path: &str, value: &Value) -> Result<()> {
        let json_str = serde_json::to_string_pretty(value)?;
        self.files.insert(path.to_string(), json_str.into_bytes());
        Ok(())
    }
    
    /// Add raw file content to the archive
    pub fn add_file(&mut self, path: &str, content: Vec<u8>) {
        self.files.insert(path.to_string(), content);
    }
}

/// Collect all content for a specific book
pub fn collect_book_content(book: &Book, repo_path: &Path) -> Result<BookContent> {
    let mut content = BookContent::new(book.clone());
    let source = &book.source;
    let data_dir = repo_path.join("data");
    
    // Add metadata
    let metadata = json!({
        "name": book.name,
        "id": book.id,
        "source": book.source,
        "group": book.group,
        "published": book.published,
        "author": book.author,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    content.add_json("metadata.json", &metadata)?;
    
    // Collect book narrative content
    collect_book_files(&mut content, &data_dir, "book", "book", source)?;
    
    // Collect adventure content
    collect_book_files(&mut content, &data_dir, "adventure", "adventure", source)?;
    
    // Collect bestiary content
    collect_book_files(&mut content, &data_dir, "bestiary", "bestiary", source)?;
    
    // Collect spell content
    collect_spell_files(&mut content, &data_dir, source)?;
    
    // Collect class content (filtered)
    collect_filtered_content(&mut content, &data_dir, "class", source)?;
    
    // Collect items (filtered)
    collect_filtered_items(&mut content, &data_dir, source)?;
    
    // Collect races (filtered)
    collect_filtered_races(&mut content, &data_dir, source)?;
    
    // Collect backgrounds (filtered)
    collect_filtered_backgrounds(&mut content, &data_dir, source)?;
    
    // Collect feats (filtered)
    collect_filtered_feats(&mut content, &data_dir, source)?;
    
    // Collect optional features (filtered)
    collect_filtered_optfeatures(&mut content, &data_dir, source)?;
    
    // Collect images
    collect_images(&mut content, repo_path, source)?;
    
    Ok(content)
}

/// Collect files for a specific type and source
fn collect_book_files(
    content: &mut BookContent,
    data_dir: &Path,
    dir_name: &str,
    file_prefix: &str,
    source: &str,
) -> Result<()> {
    let files = parser::get_matching_files(data_dir.parent().unwrap(), dir_name, file_prefix, source);
    
    for file_path in files {
        if let Ok(data) = parser::load_json_file(&file_path) {
            let relative_path = file_path
                .strip_prefix(data_dir)
                .unwrap_or(&file_path)
                .to_string_lossy();
            content.add_json(&relative_path, &data)?;
        }
    }
    
    Ok(())
}

/// Collect spell files with fluff
fn collect_spell_files(content: &mut BookContent, data_dir: &Path, source: &str) -> Result<()> {
    let spells_dir = data_dir.join("spells");
    if !spells_dir.exists() {
        return Ok(());
    }
    
    // Main spell file
    let spell_file = spells_dir.join(format!("spells-{}.json", source.to_lowercase()));
    if spell_file.exists() {
        if let Ok(data) = parser::load_json_file(&spell_file) {
            content.add_json(&format!("spells/spells-{}.json", source.to_lowercase()), &data)?;
        }
    }
    
    // Fluff spell file
    let fluff_file = spells_dir.join(format!("fluff-spells-{}.json", source.to_lowercase()));
    if fluff_file.exists() {
        if let Ok(data) = parser::load_json_file(&fluff_file) {
            content.add_json(&format!("spells/fluff-spells-{}.json", source.to_lowercase()), &data)?;
        }
    }
    
    Ok(())
}

/// Collect and filter content from a directory
fn collect_filtered_content(
    content: &mut BookContent,
    data_dir: &Path,
    dir_name: &str,
    source: &str,
) -> Result<()> {
    let dir = data_dir.join(dir_name);
    if !dir.exists() {
        return Ok(());
    }
    
    let mut all_items = Vec::new();
    
    // Read all files in directory
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(data) = parser::load_json_file(&path) {
                let filtered = data.filter_by_source(source);
                all_items.extend(filtered);
            }
        }
    }
    
    if !all_items.is_empty() {
        let result = json!({ dir_name: all_items });
        content.add_json(&format!("{}/{}.json", dir_name, source.to_lowercase()), &result)?;
    }
    
    Ok(())
}

/// Collect filtered items
fn collect_filtered_items(content: &mut BookContent, data_dir: &Path, source: &str) -> Result<()> {
    let items_file = data_dir.join("items.json");
    if !items_file.exists() {
        return Ok(());
    }
    
    let data = parser::load_json_file(&items_file)?;
    let filtered = parser::filter_by_source(&data, source, "item");
    
    if !filtered.is_empty() {
        let result = json!({ "item": filtered });
        content.add_json(&format!("items/{}.json", source.to_lowercase()), &result)?;
    }
    
    // Also check for item fluff
    let fluff_file = data_dir.join("fluff-items.json");
    if fluff_file.exists() {
        let data = parser::load_json_file(&fluff_file)?;
        let filtered = data.filter_by_source(source);
        if !filtered.is_empty() {
            let result = json!({ "itemFluff": filtered });
            content.add_json(&format!("items/fluff-{}.json", source.to_lowercase()), &result)?;
        }
    }
    
    Ok(())
}

/// Collect filtered races
fn collect_filtered_races(content: &mut BookContent, data_dir: &Path, source: &str) -> Result<()> {
    let races_file = data_dir.join("races.json");
    if !races_file.exists() {
        return Ok(());
    }
    
    let data = parser::load_json_file(&races_file)?;
    let races = parser::filter_by_source(&data, source, "race");
    let subraces = parser::filter_by_source(&data, source, "subrace");
    
    if !races.is_empty() || !subraces.is_empty() {
        let result = json!({ 
            "race": races,
            "subrace": subraces
        });
        content.add_json(&format!("races/{}.json", source.to_lowercase()), &result)?;
    }
    
    // Also check for race fluff
    let fluff_file = data_dir.join("fluff-races.json");
    if fluff_file.exists() {
        let data = parser::load_json_file(&fluff_file)?;
        let filtered = data.filter_by_source(source);
        if !filtered.is_empty() {
            let result = json!({ "raceFluff": filtered });
            content.add_json(&format!("races/fluff-{}.json", source.to_lowercase()), &result)?;
        }
    }
    
    Ok(())
}

/// Collect filtered backgrounds
fn collect_filtered_backgrounds(content: &mut BookContent, data_dir: &Path, source: &str) -> Result<()> {
    let bg_file = data_dir.join("backgrounds.json");
    if !bg_file.exists() {
        return Ok(());
    }
    
    let data = parser::load_json_file(&bg_file)?;
    let filtered = parser::filter_by_source(&data, source, "background");
    
    if !filtered.is_empty() {
        let result = json!({ "background": filtered });
        content.add_json(&format!("backgrounds/{}.json", source.to_lowercase()), &result)?;
    }
    
    // Also check for background fluff
    let fluff_file = data_dir.join("fluff-backgrounds.json");
    if fluff_file.exists() {
        let data = parser::load_json_file(&fluff_file)?;
        let filtered = data.filter_by_source(source);
        if !filtered.is_empty() {
            let result = json!({ "backgroundFluff": filtered });
            content.add_json(&format!("backgrounds/fluff-{}.json", source.to_lowercase()), &result)?;
        }
    }
    
    Ok(())
}

/// Collect filtered feats
fn collect_filtered_feats(content: &mut BookContent, data_dir: &Path, source: &str) -> Result<()> {
    let feats_file = data_dir.join("feats.json");
    if !feats_file.exists() {
        return Ok(());
    }
    
    let data = parser::load_json_file(&feats_file)?;
    let filtered = parser::filter_by_source(&data, source, "feat");
    
    if !filtered.is_empty() {
        let result = json!({ "feat": filtered });
        content.add_json(&format!("feats/{}.json", source.to_lowercase()), &result)?;
    }
    
    Ok(())
}

/// Collect filtered optional features
fn collect_filtered_optfeatures(content: &mut BookContent, data_dir: &Path, source: &str) -> Result<()> {
    let opt_file = data_dir.join("optionalfeatures.json");
    if !opt_file.exists() {
        return Ok(());
    }
    
    let data = parser::load_json_file(&opt_file)?;
    let filtered = parser::filter_by_source(&data, source, "optionalfeature");
    
    if !filtered.is_empty() {
        let result = json!({ "optionalfeature": filtered });
        content.add_json(&format!("optionalfeatures/{}.json", source.to_lowercase()), &result)?;
    }
    
    Ok(())
}

/// Collect images related to the book
fn collect_images(content: &mut BookContent, repo_path: &Path, source: &str) -> Result<()> {
    let img_dir = repo_path.join("img");
    if !img_dir.exists() {
        return Ok(());
    }
    
    // Collect cover image if specified
    if let Some(cover) = &content.book.cover {
        if cover.cover_type == "internal" {
            let cover_path = img_dir.join(&cover.path);
            if cover_path.exists() {
                if let Ok(data) = fs::read(&cover_path) {
                    content.add_file(&format!("img/{}", cover.path), data);
                }
            }
        }
    }
    
    // Collect book-specific images directory
    let book_img_dir = img_dir.join("book").join(source);
    if book_img_dir.exists() {
        collect_directory_recursive(content, &book_img_dir, &format!("img/book/{}", source))?;
    }
    
    // Collect adventure images
    let adventure_img_dir = img_dir.join("adventure").join(source);
    if adventure_img_dir.exists() {
        collect_directory_recursive(content, &adventure_img_dir, &format!("img/adventure/{}", source))?;
    }
    
    Ok(())
}

/// Recursively collect all files from a directory
fn collect_directory_recursive(
    content: &mut BookContent,
    dir: &Path,
    base_path: &str,
) -> Result<()> {
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let relative = entry.path().strip_prefix(dir).unwrap_or(entry.path());
            let target_path = format!("{}/{}", base_path, relative.to_string_lossy());
            
            if let Ok(data) = fs::read(entry.path()) {
                content.add_file(&target_path, data);
            }
        }
    }
    Ok(())
}