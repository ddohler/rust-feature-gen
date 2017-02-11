use chrono::naive::datetime::NaiveDateTime;
use chrono::Datelike;
use chrono::Timelike;

#[derive(RustcDecodable)]
#[derive(Debug)]
/// Stores events as read from event CSVs
pub struct EventRecord {
    id: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
    category: String,
    x: f32,
    y: f32
}

#[derive(Debug)]
/// Stores events that have been quantized to occur in a spatiotemporal cell
pub struct QuantizedEvent {
    cell_t: u32, // Time slice (chronon)
    cell_x: i16, // Raster cell x coordinate
    cell_y: i16  // Raster cell y coordinate
}

impl QuantizedEvent {
    /// Construct a QuantizedEvent from an EventRecord
    pub fn from_event_record(record: &EventRecord) -> QuantizedEvent {
        // TODO: Support general transformations; these are
        // 3857 -> New Orleans @ 250m grid, 230x171 cells
        let x_transform = |x: f32| 0.004 * x + 40137.388552;
        let y_transform = |y: f32| 0.004 * y - 13945.687388;
        // Hour number of the event since 1/1/1 (proleptic Gregorian)
        let t_transform = |dt: NaiveDateTime| dt.num_days_from_ce() as u32 * 24 + dt.hour();
        QuantizedEvent {
            cell_t: t_transform(record.end),
            cell_x: x_transform(record.x) as i16,
            cell_y: y_transform(record.y) as i16
        }
    }
}