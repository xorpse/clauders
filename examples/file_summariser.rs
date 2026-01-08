use std::io::{self, Write};
use std::path::PathBuf;

use clauders::{Client, Options, Responses};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("failed to get current directory"));

    println!("Scanning directory: {}", dir.display());
    println!();

    let files = list_files(&dir)?;
    if files.is_empty() {
        println!("No files found in directory.");
        return Ok(());
    }

    println!("Files:");
    for (i, file) in files.iter().enumerate() {
        println!("  [{}] {}", i + 1, file.display());
    }
    println!();

    let selection = prompt_selection(files.len())?;
    let selected_file = &files[selection - 1];
    println!();
    println!("Summarising: {}", selected_file.display());
    println!();

    let client = Client::new(Options::new()).await?;

    let prompt = format!(
        "Please read and provide a brief summary of the file at: {}\n\
         Focus on:\n\
         - What the file does or contains\n\
         - Key functions, types, or structures\n\
         - Any notable patterns or dependencies",
        selected_file.display()
    );

    client.query(&prompt).await?;

    let mut stream = std::pin::pin!(client.receive());
    let mut responses = Vec::new();

    while let Some(result) = stream.next().await {
        let response = result?;

        if let Some(text) = response.as_text() {
            print!("{}", text.content());
            io::stdout().flush()?;
        }

        if let Some(err) = response.as_error() {
            eprintln!("\nError: {}", err.message());
        }

        responses.push(response);
    }

    println!();

    let responses = Responses::new(responses);
    if let Some(complete) = responses.completion() {
        println!();
        println!("---");
        println!(
            "Completed in {:.2}s | {} turns",
            complete.duration_ms() as f64 / 1000.0,
            complete.num_turns()
        );
        if let Some(cost) = complete.total_cost_usd() {
            println!("Cost: ${:.4}", cost);
        }
    }

    Ok(())
}

fn list_files(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }

    files.sort();
    Ok(files)
}

fn prompt_selection(max: usize) -> io::Result<usize> {
    loop {
        print!("Select a file (1-{}): ", max);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= max => return Ok(n),
            _ => println!(
                "Invalid selection. Please enter a number between 1 and {}.",
                max
            ),
        }
    }
}
