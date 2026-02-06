//! Example demonstrating the conversation API for multi-turn interactions.
//!
//! Run with:
//! ```sh
//! cargo run --example conversation
//! ```

use std::io::{self, Write};

use clauders::{Client, Options};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Conversation API Demo");
    println!("=====================");
    println!();

    let client = Client::new(Options::new()).await?;
    let mut conv = client.conversation();

    println!("Part 1: Simple multi-turn conversation");
    println!("--------------------------------------");

    let response1 = conv.say("What is Rust? Answer in one sentence.").await?;
    println!("Q: What is Rust?");
    println!("A: {}", response1);
    println!();

    let response2 = conv
        .say("What is its key feature that makes it unique? One sentence please.")
        .await?;
    println!("Q: What is its key feature?");
    println!("A: {}", response2);
    println!();

    println!("Conversation has {} turns.", conv.history().len());
    println!();

    println!("Part 2: Streaming with callback");
    println!("-------------------------------");
    print!("Streaming: ");
    io::stdout().flush()?;

    let _text = conv
        .turn("Give me a very short poem about programming (4 lines max).")
        .on_text(|chunk| {
            print!("{}", chunk);
            io::stdout().flush().ok();
        })
        .send_text()
        .await?;

    println!();
    println!();

    println!("Part 3: Full control with callbacks");
    println!("-----------------------------------");

    let responses = conv
        .turn("What is 2 + 2? Just give the number.")
        .on_text(|t| {
            print!("{}", t);
            io::stdout().flush().ok();
        })
        .on_thinking(|t| {
            eprintln!("[thinking] {}", t);
        })
        .send()
        .await?;

    println!();

    if responses.has_error()
        && let Some(err) = responses.first_error()
    {
        eprintln!("Error: {}", err.message());
    }

    if let Some(complete) = responses.completion() {
        println!();
        println!("---");
        println!(
            "Completed in {:.2}s | {} turns",
            complete.duration_ms() as f64 / 1000.0,
            complete.num_turns()
        );
        if let Some(cost) = complete.total_cost_usd() {
            println!("Cost: ${:.6}", cost);
        }
    }

    println!();
    println!("Conversation Summary");
    println!("--------------------");
    println!("Total turns: {}", conv.history().len());
    for (i, turn) in conv.history().iter().enumerate() {
        let preview = turn.text().chars().take(50).collect::<String>();
        let ellipsis = if turn.text().len() > 50 { "..." } else { "" };
        println!("Turn {}: {} -> {}{}", i + 1, turn.prompt, preview, ellipsis);
    }

    Ok(())
}
