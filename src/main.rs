extern crate csv;
extern crate chrono;
extern crate rustc_serialize;

mod event;
mod spacetime;
mod feature;

use feature::Feature;
use feature::FutureSpanFeature;

const SPAN_364_DAYS_HRS: u32 = 364 * 24;
const SPAN_3_DAYS_HRS: u32 = 3 * 24;

fn main() {
    println!("Reading");
    let mut reader = match csv::Reader::from_file("small.csv") {
        Ok(reader) => reader,
        Err(_) => panic!("Couldn't open file")
    };
    println!("Sorting");
    let mut sorted_events: Vec<event::QuantizedEvent> =
        reader.decode::<event::EventRecord>() // Read CSV
              .flat_map(|r| r.ok()) // Remove any errors
              .map(|e| event::QuantizedEvent::from_event_record(&e)) // Quantize
              .collect(); // Convert to Vector
    sorted_events.sort_by_key(|qe| spacetime::SpaceTimeCell::from_cell(&qe.location));
    let bounds = spacetime::SpatialBoundary {
        along_x: (0..230),
        along_y: (0..171)
    };
    let mut future_364_feature = FutureSpanFeature::new(bounds, SPAN_364_DAYS_HRS);
    println!("First event: {:?}", sorted_events.as_slice().first());
    println!("Last event: {:?}", sorted_events.as_slice().last());
    println!("Calculating...");
    for event in sorted_events {
        future_364_feature.accumulate_and_close(event);
    }
    // TODO: Write a finalize stage to print out anything that has been accumulated but not written yet,
    // out to whatever time we want our training to finish.
}
