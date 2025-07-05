#[cfg(test)]
mod tests {
    use reqwest;
    use serde_json::Value;

    async fn my_async_call(url: &str) -> Result<Value, reqwest::Error> {
        let response = reqwest::get(url)
            .await?
            .error_for_status()?; // Ensures 4xx/5xx errors are caught early

        let json_response = response.json::<Value>().await?;
        Ok(json_response)
    }

    #[tokio::test]
    async fn tests_calls_async_fn() {
        // ✅ Use a stable and working API
        let api_url = "https://catfact.ninja/fact";

        match my_async_call(api_url).await {
            Ok(json) => {
                dbg!(json);
            }
            Err(e) => {
                panic!("Failed to make request: {:?}", e);
            }
        }
    }
}

