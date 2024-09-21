use bson::DateTime as BsonDateTime;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use std::str::FromStr;



pub fn convert_to_local_time(timezone_str: &str, utc_datetime: BsonDateTime) -> Option<DateTime<Tz>> {
    // Convertir bson::DateTime a chrono::DateTime<Utc>
    let utc_datetime_chrono: DateTime<Utc> = bson_to_chrono(utc_datetime);

    // Convertir la cadena de zona horaria en un objeto Tz
    let timezone: Tz = Tz::from_str(timezone_str).ok()?;

    // Convertir la fecha UTC a la fecha local de la zona horaria proporcionada
    let local_datetime = utc_datetime_chrono.with_timezone(&timezone);

    Some(local_datetime)
}


fn bson_to_chrono(bson_dt: BsonDateTime) -> DateTime<Utc> {
    let millis = bson_dt.timestamp_millis();
    let seconds = millis / 1000;
    let nanoseconds = ((millis % 1000) * 1_000_000) as u32;
    DateTime::from_timestamp(seconds, nanoseconds).unwrap()
}