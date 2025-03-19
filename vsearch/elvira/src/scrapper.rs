pub fn pull_page(url: &str) {
    let body = reqwest::get("https://www.theoutliers.org/")
        .await?
        .text()
        .await?;
} 
