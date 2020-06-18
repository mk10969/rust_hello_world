use chrono::{DateTime, Utc};
use influxdb::{Client, Query, Timestamp};
use influxdb::InfluxDbWriteable;

#[tokio::test]
async fn my_test() {
    influx_write().await;
}

async fn influx_write() {
    let client = Client::new("http://localhost:8086", "test");

    #[derive(InfluxDbWriteable)]
    struct WeatherReading {
        time: DateTime<Utc>,
        humidity: i32,
        #[tag] wind_direction: String,
    }

    // Let's write some data into a measurement called `weather`
    let weather_reading = WeatherReading {
        time: Timestamp::Hours(1).into(),
        humidity: 30,
        wind_direction: String::from("north"),
    };

    let write_result = client
        .query(&weather_reading.into_query("weather"))
        .await;
    assert!(write_result.is_ok(), "Write result was not okay");

    // Let's see if the data we wrote is there
    let read_query = Query::raw_read_query("SELECT * FROM weather");

    let read_result = client.query(&read_query).await;
    assert!(read_result.is_ok(), "Read result was not ok");
    println!("{}", read_result.unwrap());
}