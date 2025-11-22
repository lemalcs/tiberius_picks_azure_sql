use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use std::env;
use azure_identity::ClientSecretCredential;
use azure_core::credentials::{Secret, TokenCredential, TokenRequestOptions};

pub async fn connect_to_azure_sql_database_with_password() -> anyhow::Result<Client<Compat<TcpStream>>> {
    let mut config=Config::new();
    config.authentication(AuthMethod::sql_server("theadmin","YOUR_FOREVER_SECRET_PASSWORD"));
    config.host("rainforest.database.windows.net");
    config.port(1433);
    config.database("nestdb");
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to database");

    Ok(client)
}

pub async fn connet_to_azure_sql_with_microsoft_entra() -> anyhow::Result<Client<Compat<TcpStream>>> {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID is missing");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET is missing");
    let tenant_id = env::var("TENANT_ID").expect("TENANT_ID is missing");

    let credential = ClientSecretCredential::new(
        tenant_id.as_str(),
        client_id,
        Secret::new(client_secret),
        Default::default(),
    )?;

    let token_response = credential.get_token(&["https://database.windows.net/.default"], Some(TokenRequestOptions::default())).await?;
    let access_token = token_response.token.secret();

    let mut config = Config::new();
    config.authentication(AuthMethod::AADToken(access_token.to_string()));
    config.host("rainforest.database.windows.net");
    config.port(1433);
    config.database("nestdb");
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to database with Azure AD token");

    Ok(client)
}