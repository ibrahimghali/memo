use std::env;

mod ai;
mod models;
mod storage;

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenvy::dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a word.");
                return;
            }

            let word = args[2..].join(" ");

            let api_key = std::env::var("GEMINI_API_KEY")
                .expect("Please set GEMINI_API_KEY environment variable");

            println!("Generating entry for '{}'", word);

            match ai::generate_entry(&word, &api_key).await {
                Ok(entry) => {
                    println!("Word: {}", entry.word);
                    println!("English: {}", entry.translation_en);
                    let reshaper = arabic_reshaper::ArabicReshaper::default();
                    let reshaped = reshaper.reshape(&entry.translation_ar);
                    let ar_display: String = reshaped.chars().rev().collect();
                    println!("Arabic: {}", ar_display);
                    println!("Example: {}", entry.example);
                    println!("Part of speech: {}", entry.part_of_speech);
                    println!("Register: {}", entry.register);
                    println!("Synonyms: {}", entry.synonyms);
                    println!("Notes: {}", entry.notes);
                    
                    if let Err(e) = storage::add_word(&entry).await {
                        println!("Failed to save to Supabase: {}", e);
                    }

                    println!("*******************************************");
                    
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        "list" => {
            let words = match storage::load_words().await {
                Ok(w) => w,
                Err(e) => {
                    println!("Error loading from Supabase: {}", e);
                    return;
                }
            };

            if words.is_empty() {
                println!("No vocabulary stored yet.");
                return;
            }

            let reshaper = arabic_reshaper::ArabicReshaper::default();
            for w in words {
                println!("Word: {}", w.word);
                println!("English: {}", w.translation_en);
                let ar_display: String = reshaper.reshape(&w.translation_ar).chars().rev().collect();
                println!("Arabic: {}", ar_display);
                println!("Example: {}", w.example);
                println!("Part of speech: {}", w.part_of_speech);
                println!("Register: {}", w.register);
                println!("Synonyms: {}", w.synonyms);
                println!("Notes: {}", w.notes);
                println!("---------------------------");
            }
        }

        "review" => {
            let words = match storage::load_words().await {
                Ok(w) => w,
                Err(e) => {
                    println!("Error loading from Supabase: {}", e);
                    return;
                }
            };

            if words.is_empty() {
                println!("No words to review.");
                return;
            }

            for w in words {
                println!("Word: {}", w.word);
                println!("Press ENTER to see the answer...");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                println!("English: {}", w.translation_en);
                let reshaper = arabic_reshaper::ArabicReshaper::default();
                let ar_display: String = reshaper.reshape(&w.translation_ar).chars().rev().collect();
                println!("Arabic: {}", ar_display);
                println!("Example: {}", w.example);
                println!("Part of speech: {}", w.part_of_speech);
                println!("Register: {}", w.register);
                println!("Synonyms: {}", w.synonyms);
                println!("Notes: {}", w.notes);
                println!("---------------------------");
            }
        }

        _ => print_help(),
    }
}

fn print_help() {
    println!("Vocabulary CLI");
    println!();
    println!("Commands:");
    println!("  add <word>    Add a new word (prints the result immediately)");
    println!("  list          List saved words");
    println!("  review        Review vocabulary");
}