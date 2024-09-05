use std::fs;
use scylla::{Session};

async fn run_query(session: &Session, query: &str) {
    if !query.trim().is_empty() {
        if let Err(e) = session.query_unpaged(query, &[]).await {
            eprintln!("Failed to execute query: {}. Error: {:?}", query, e);
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