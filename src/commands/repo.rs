use reqwest::Client;
use futures_util::StreamExt;
use std::fs::File as StdFile;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;


pub async fn repo_downloader(link: String, branch: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/archive/refs/heads/{}.zip", link, branch);

    let client = Client::new();
    let response = client.get(url).send().await? ;

    let filename = format!("{}.zip", branch);
    let mut file = File::create(filename).await?;

    let mut stream = response.bytes_stream();
        while let Some(chunk)=stream.next().await {

        let chunk = chunk?;

        file.write_all(&chunk)
            .await?;
    }


    let zip_file =
        StdFile::open(
            format!("{}.zip", branch)
        )?;

    zip_extract::extract(
        zip_file,
        std::path::Path::new(&branch),
        true
    )?;
    Ok(())
}