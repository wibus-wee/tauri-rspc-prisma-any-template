use crate::{
    errors::CoreError,
    prisma::{new_client_with_url, PrismaClient},
    utils::get_frpanel_dir,
};

pub async fn new_client() -> Result<PrismaClient, CoreError> {
    let frpanel_url = get_frpanel_dir().join("frpanel.db");

    log::info!(
        "Connecting to frpanel database at {}",
        frpanel_url.display()
    );
    tokio::fs::create_dir_all(frpanel_url.parent().unwrap()).await?;

    if !frpanel_url.exists() {
        tokio::fs::File::create(frpanel_url.clone()).await?;
    }

    let client =
        new_client_with_url(&("file:".to_string() + frpanel_url.to_str().unwrap())).await?;

    Ok(client)
}
