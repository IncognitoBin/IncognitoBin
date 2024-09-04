use std::fs;
use scylla::{Session};

pub async fn run_query(session: &Session, query: &str) {
    if !query.trim().is_empty() {
        if let Err(e) = session.query(query, &[]).await {
            eprintln!("Failed to execute query: {}. Error: {:?}", query, e);
        } else {
            println!("Executed query: {} -------- Done", query);
        }
    }
}
pub async fn initialize_schema(session: &Session, filepath: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(filepath)?;
    let queries: Vec<&str> = content.split(';').collect();

    for query in queries {
        run_query(session, query).await;
    }

    Ok(())
}