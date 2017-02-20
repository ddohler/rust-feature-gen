use std::collections::HashMap;

use spacetime::SpatialBoundary;
use spacetime::SpaceTimeCell;
use spacetime;
use event as evt;

trait FeatureAction {
    fn target(&self) -> &SpaceTimeCell;
}
/// Add 1 to the target feature value
struct FeatureIncrement {
    cell: SpaceTimeCell
}

/// Stop incrementing the target feature value
struct FeatureClose {
    cell: SpaceTimeCell
}

impl FeatureAction for FeatureIncrement {
    fn target(&self) -> &SpaceTimeCell { &self.cell }
}

impl FeatureAction for FeatureClose {
    fn target(&self) -> &SpaceTimeCell { &self.cell }
}

/////////////////////////////////////////////////////////////////
// CLOSERS: Things that are Iterator<Item=FeatureAction::Close>
/////////////////////////////////////////////////////////////////
struct CloseCellsBetween<'b> {
    cells: spacetime::SpaceTimeCellGenerator<'b>
}

impl<'b> CloseCellsBetween<'b> {
    fn new(start_event: &evt::QuantizedEvent, end_event: &evt::QuantizedEvent, bounds: &'b SpatialBoundary) ->
    CloseCellsBetween<'b> {
        CloseCellsBetween {
            cells: spacetime::SpaceTimeCellGenerator::new(&start_event.location, &end_event.location, bounds)
        }
    }
}

impl<'b> Iterator for CloseCellsBetween<'b> {
    type Item = FeatureClose;
    fn next(&mut self) -> Option<FeatureClose> {
        match self.cells.next() {
            Some(cell) => Some(FeatureClose { cell: cell }),
            None => None
        }
    }
}

//////////////////////////////////////////////////////////////////////////
// INCREMENTERS: Things that are Iterator<Item=FeatureAction::Increment>
//////////////////////////////////////////////////////////////////////////
/// Increment cells by time only
struct FutureSpanIncrementer {
    cells: spacetime::TimeCellGenerator
}

impl FutureSpanIncrementer {
    fn new(event: &evt::QuantizedEvent, span_hrs: u32) -> FutureSpanIncrementer {
        let start_cell = &event.location;
        let end_cell = spacetime::SpaceTimeCell {
            cell_x: start_cell.cell_x,
            cell_y: start_cell.cell_y,
            cell_t: start_cell.cell_t + span_hrs
        };
        FutureSpanIncrementer {
            cells: spacetime::TimeCellGenerator::new(&start_cell, &end_cell)
        }
    }
}

// TODO: Refactor as per https://doc.rust-lang.org/book/enums.html
impl Iterator for FutureSpanIncrementer {
    type Item = FeatureIncrement;

    fn next(&mut self) -> Option<FeatureIncrement> {
        match self.cells.next() {
            Some(cell) => Some(FeatureIncrement { cell: cell }),
            None => None
        }
    }
}

/// Wrapper for Close and Increment generators to handle all outcomes of an event
pub trait Feature {
    fn accumulate_and_close(&mut self, event: evt::QuantizedEvent);
    // TODO: Add finalize() function that closes all cells up to an end cell.
}

/// Actually generate a prior364 feature (which I'm calling Future364 because in practice it broadcasts events
/// into the future rather than looking them up from the past.
/// This is required to provide a 
pub struct FutureSpanFeature {
    bounds: SpatialBoundary,
    span_hrs: u32,
    prev_event: Option<evt::QuantizedEvent>,
    feature_map: HashMap<spacetime::SpaceTimeCell, u8>
}

impl FutureSpanFeature {
    pub fn new(bounds: SpatialBoundary, span: u32) -> FutureSpanFeature {
        FutureSpanFeature {
            bounds: bounds,
            span_hrs: span,
            prev_event: None,
            feature_map: HashMap::new()
        }
    }
}

impl Feature for FutureSpanFeature {
    fn accumulate_and_close(&mut self, event: evt::QuantizedEvent) -> () {
        //println!("Starting event {:?}", event);
        let increments = FutureSpanIncrementer::new(&event, self.span_hrs);
        let closes = match self.prev_event {
            Some(ref prev) => CloseCellsBetween::new(prev, &event, &self.bounds),
            None => CloseCellsBetween::new(&event, &event, &self.bounds)
        };
        //println!("Incrementing features...");
        //let mut count = 0;
        for inc in increments {
            let cell = inc.target();
            let value = self.feature_map.entry(SpaceTimeCell::from_cell(cell)).or_insert(0);
            *value += 1;
            //count += 1;
        }
        //println!("Incremented {} features", count);
        //println!("Closing features...");
        //count = 0;
        for close in closes {
            let cell = close.target();
            let value = match self.feature_map.get(&cell) {
                Some(val) => *val,
                None => 0
            };
            self.feature_map.remove(&cell);
            //count += 1;
            //println!("Value at {:?} is {}", cell, value);
        }
        //println!("Closed {} features", count);
        //println!("Done with event {:?}", event);
        self.prev_event = Some(event);
    }
}