use clickhouse_rs::{types::Block, Pool, errors::Error};
use plotters::prelude::*;
use tokio;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ddl = r"
        CREATE TABLE IF NOT EXISTS numbers (
            value UInt32
        ) Engine=Memory";

    let pool = Pool::new("tcp://localhost:9000");

    let mut client = pool.get_handle().await?;
    client.execute(ddl).await?;

    // Ingest data
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    for _ in 0..100 {
        let value: u32 = rng.gen_range(1..=10);
        data.push(value);
    }
    let block = Block::new().column("value", data);
    client.insert("numbers", block).await?;

    // Perform queries and aggregations
    let block = client.query("SELECT value FROM numbers").fetch_all().await?;
    let mut values = Vec::new();
    for row in block.rows() {
        let value: u32 = row.get(0)?;
        values.push(value);
    }

    // Visualize output
    let root = BitMapBackend::new("output.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).expect("Failed to fill the drawing area");

    let mut chart = ChartBuilder::on(&root)
        .caption("Histogram of Values", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0u32..10u32, 0..10)
        .expect("Failed to build the chart");

    chart.configure_mesh().draw().expect("Failed to draw the mesh");

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.filled())
            .data(values.iter().map(|x| (*x, 1))),
    ).expect("Failed to draw the series");

    root.present().expect("Failed to present the drawing");

    Ok(())
}