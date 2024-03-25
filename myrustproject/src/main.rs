use clickhouse_rs::{types::Block, Pool, errors::Error};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ddl = r"
        CREATE TABLE IF NOT EXISTS payment (
            customer_id  UInt32,
            amount       UInt32,
            account_name Nullable(FixedString(3))
        ) Engine=Memory";

    let pool = Pool::new("tcp://localhost:9000");

    let mut client = pool.get_handle().await?;
    client.execute(ddl).await?;

    let block = Block::new()
        .column("customer_id", vec![1_u32, 3, 5, 7, 9])
        .column("amount", vec![2_u32, 4, 6, 8, 10])
        .column("account_name", vec![Some("foo"), None, None, None, Some("bar")]);

    client.insert("payment", block).await?;

    let block = client.query("SELECT * FROM payment").fetch_all().await?;
    for row in block.rows() {
        let id: u32 = row.get("customer_id")?;
        let amount: u32 = row.get("amount")?;
        let name: Option<&str> = row.get("account_name")?;
        println!("Found payment {}: {} {:?}", id, amount, name);
    }
    Ok(())
}