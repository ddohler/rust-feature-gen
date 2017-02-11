extern crate csv;
extern crate chrono;
extern crate rustc_serialize;

mod event;

fn main() {
    let mut reader = match csv::Reader::from_file("small.csv") {
        Ok(reader) => reader,
        Err(_) => panic!("Couldn't open file")
    };

    for row in reader.decode::<event::EventRecord>() {
        match row {
            //Ok(row) => println!("{:?}", row),
            Ok(row) => println!("{:?}", event::QuantizedEvent::from_event_record(&row)),
            Err(_) => println!("Couldn't parse row")
        }
    }
}
