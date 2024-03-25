# Rust ClickHouse Project

This project demonstrates how to ingest data into a [ClickHouse](https://clickhouse.com/) database, perform queries and aggregations, and visualize the output using Rust.

## Project Requirements

This project meets the following requirements:

1. **Ingest data into Vector database**: The Rust program (main.rs) generates a random number of each value (1 through 10) and ingests this data into the ClickHouse database.

2. **Perform queries and aggregations**: The program performs a query to fetch all the values from the database.

3. **Visualize output**: The program uses the [Plotters](https://docs.rs/plotters/latest/plotters/) library to create a histogram of the values and saves it as a PNG image.

## How to Clone/Reproduce

1. Clone the repository: 
```
git clone https://gitlab.com/dukeaiml/IDS721/ds655_ids721_miniproject07
```

2. Navigate to the project directory: 
```
cd myrustproject
```

3. Build the project: 
```
cargo build
```

4. Run the project: 
```
cargo run
```

## How to Build the ClickHouse Server

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) for Windows from the official Docker website.

2. Open a command prompt or PowerShell window.

3. Pull the ClickHouse image from Docker Hub: 
```
docker pull yandex/clickhouse-server
```

4. Start a ClickHouse server container: 
```
docker run -d -p 8123:8123 -p 9000:9000 --name some-clickhouse-server --ulimit nofile=262144:262144 yandex/clickhouse-server
```

## Data Visualization

The program uses the [Plotters](https://docs.rs/plotters/latest/plotters/) library to visualize the data. After fetching the values from the ClickHouse database, it creates a histogram where the x-axis represents the values (1 through 10) and the y-axis represents the frequency of each value.

The histogram is then saved as a PNG image named `output.png` in the project's root directory.

Here's an example of what the output might look like:

![Histogram](output.png)

Please note that the actual output will vary because the program generates a random number of each value (1 through 10) each time it runs.

## Other Details

This project uses the `clickhouse-rs` crate for interacting with the ClickHouse database and the `plotters` crate for creating the histogram. The `tokio` crate is used for async programming, and the `rand` crate is used for generating random numbers.

Please ensure that the ClickHouse server is running and accessible at `localhost:9000` before running the program.