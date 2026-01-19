use clauders::{Client, Model, Options};
use schemars::JsonSchema;
use serde::Deserialize;

/// The structured output from sentiment analysis.
#[derive(Debug, Deserialize, JsonSchema)]
struct SentimentResult {
    /// The sentiment classification (e.g., "positive", "negative", "neutral")
    classification: String,
    /// Confidence score between 0.0 and 1.0
    confidence: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sentence = std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");

    if sentence.is_empty() {
        eprintln!("Usage: sentiment_analysis <sentence>");
        eprintln!("Example: sentiment_analysis I love this product!");
        std::process::exit(1);
    }

    println!("Analyzing: \"{sentence}\"");
    println!();

    let client = Client::new(
        Options::new()
            .model(Model::Haiku)
            .with_json_schema::<SentimentResult>(),
    )
    .await?;

    let prompt = format!(
        "Analyze the sentiment of the following sentence and provide a classification \
         (positive, negative, or neutral) along with a confidence score between 0.0 and 1.0.\n\n\
         DO NOT use any tools to answer this query.\n\n\
         Sentence: \"{sentence}\""
    );

    let (result, responses) = client.query_once_as::<SentimentResult>(&prompt).await?;

    println!("Classification: {}", result.classification);
    println!("Confidence: {:.1}%", result.confidence * 100.0);

    if let Some(complete) = responses.completion() {
        println!();
        println!("---");
        println!(
            "Completed in {:.2}s",
            complete.duration_ms() as f64 / 1000.0
        );
        if let Some(cost) = complete.total_cost_usd() {
            println!("Cost: ${:.6}", cost);
        }
    }

    Ok(())
}
