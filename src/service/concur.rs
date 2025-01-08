use crate::service::util;
use reqwest::Client;
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use rand::Rng;
use tokio::task;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use futures::future::join_all;

#[derive(Debug, Serialize, Deserialize)]
pub struct Concur {
    pub id: i32,
}

pub async fn get_concur(State(_state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    // Generate a random duration between 1 to 3 seconds
    let duration = rand::thread_rng().gen_range(1..=3);
    // Sleep for the generated duration
    sleep(Duration::from_secs(duration)).await;
    let concur:Concur = Concur {
        id,
    };
    util::resp_success(concur)
}

pub async fn reqwest_url(
    State(_state): State<AppState>,
) -> impl IntoResponse {

    info!("reqwest_url !!");
    // Initialize an HTTP client
    let client = Client::builder().timeout(Duration::from_secs(2)).build().expect("set time error");

    // Define the URLs to fetch
    let urls = vec![
        "http://127.0.0.1:3000/concur/1".to_string(),
        "http://127.0.0.1:3000/concur/2".to_string(),
    ];

    // Spawn multiple concurrent tasks to fetch each URL
    let fetches = urls.into_iter().map(|url| {
        let client = client.clone();
        task::spawn(async move {
            let response = client.get(&url).send().await?;
            let body = response.text().await?;
            info!("body: {}", body);
            Ok::<_, reqwest::Error>(body) // Return the response's body
        })
    });

    // Collect the results of all tasks
    let results = join_all(fetches).await;

    // Process and log the results
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(Ok(body)) => {
                info!("Response {}: {}", i + 1, body);
            }
            Ok(Err(err)) => {
                warn!("Request {} failed with error: {}", i + 1, err);
            }
            Err(err) => {
                warn!("Task {} panicked with error: {:?}", i + 1, err);
            }
        }
    }

}
