mod connection;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_to_azure_sql_with_password() {
        let conn = connection::connect_to_azure_sql_database_with_password().await;

        // In case of failure, uncomment this line to see the error details
        //dbg!(&conn);
        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn test_connection_to_azure_sql_with_valid_credentials() {
        let conn = connection::connet_to_azure_sql_with_microsoft_entra().await;

        // In case of failure, uncomment this line to see the error details
        dbg!(&conn);

        assert!(conn.is_ok());
    }
}