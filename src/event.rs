use chrono::naive::datetime::NaiveDateTime;
use chrono::Datelike;
use chrono::Timelike;

use spacetime;

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
/// Note that currently there's no concept of "types" of events; we assume that
/// all events are the same type.
pub struct QuantizedEvent {
    pub location: spacetime::SpaceTimeCell
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
            location: spacetime::SpaceTimeCell {
                cell_t: t_transform(record.end),
                cell_x: x_transform(record.x) as u16,
                cell_y: y_transform(record.y) as u16
            }
        }
    }
}