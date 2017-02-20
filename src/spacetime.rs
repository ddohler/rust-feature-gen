use std::cmp::Ordering;
use std::ops::Range;

/// Places limits on cells' spatial position
pub struct SpatialBoundary {
    pub along_x: Range<u16>,
    pub along_y: Range<u16>
}

impl SpatialBoundary {
    // TODO: Make this prettier when Range.contains stabilizes.
    fn x_in(&self, x: u16) -> bool {
        x >= self.along_x.start && x < self.along_x.end
    }
    
    fn y_in(&self, y: u16) -> bool {
        y >= self.along_y.start && y < self.along_x.end
    }
}

/// Currently unused, so commented out to avoid warnings.
// pub struct TemporalBoundary {
//     pub along_t: Range<u32>
// }

// impl TemporalBoundary {
//     fn t_in(&self, t: u32) -> bool {
//         t >= self.along_t.start && t < self.along_t.end
//     }
// }

/// Represents a quantized cell of space-time within the 3857 bounding box of a city's boundary.
/// Ordering is first by time, then by _row_, then by column. This is accomplished via #derive and
/// the ordering of the structs members, which is why cell_y appears first.
///
/// WARNING: Ordering of these fields matters and will change this struct's derived ordering.
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(Hash)]
pub struct SpaceTimeCell {
    pub cell_t: u32, // Time slice (chronon); Gregorian proleptic hour
    pub cell_y: u16,  // Raster cell y coordinate
    pub cell_x: u16 // Raster cell x coordinate; cells within rasterization of city boundary
}

impl SpaceTimeCell {
    /// Returns the next cell after this one, subject to bounds (needs to wrap at the borders)
    fn next_cell(&self, bounds: &SpatialBoundary) -> SpaceTimeCell {
        // Cell ordering is first t, then y, then x, so we increment in counting order,
        // that is, least-significant first (x, y, t).
        let new_x = if bounds.x_in(self.cell_x + 1) {
            self.cell_x + 1
        } else {
            bounds.along_x.start
        };
        
        // Determine y based on x.
        // If x has just rolled over then...
        let new_y = if new_x == bounds.along_x.start {
            // If incrementing y would stay in-bounds, do it
            if bounds.y_in(self.cell_y + 1) {
                self.cell_y + 1
            // Otherwise roll over y too.
            } else {
                bounds.along_y.start
            }
        // Otherwise y stays the same
        } else {
            self.cell_y
        };

        // Determine t based on y; t increments if y rolled over; t itself never rolls over.
        let new_t = if new_x == bounds.along_x.start && new_y == bounds.along_y.start {
            self.cell_t + 1
        } else {
            self.cell_t
        };

        SpaceTimeCell {
            cell_t: new_t,
            cell_y: new_y,
            cell_x: new_x
        }
    }
    
    /// Generate the next cell in time. No bounds necessary because wrapping in time doesn't
    /// make sense.
    fn next_time_cell(&self) -> SpaceTimeCell {
        SpaceTimeCell {
            cell_t: self.cell_t + 1,
            cell_y: self.cell_y,
            cell_x: self.cell_x
        }
    }
    // TODO: Implement Copy? Don't and use references instead?
    pub fn from_cell(other: &SpaceTimeCell) -> SpaceTimeCell {
        SpaceTimeCell {
            cell_x: other.cell_x,
            cell_y: other.cell_y,
            cell_t: other.cell_t
        }
    }
}

pub struct SpaceTimeCellGenerator<'a> {
    end: SpaceTimeCell,
    current: SpaceTimeCell,
    bounds: &'a SpatialBoundary
}

impl<'a> SpaceTimeCellGenerator<'a> {
    pub fn new(start: &SpaceTimeCell, end: &SpaceTimeCell, bounds: &'a SpatialBoundary) -> SpaceTimeCellGenerator<'a> {
        SpaceTimeCellGenerator {
            end: SpaceTimeCell::from_cell(end),
            current: SpaceTimeCell::from_cell(start),
            bounds: bounds
        }
    }
}

impl<'a> Iterator for SpaceTimeCellGenerator<'a> {
    type Item = SpaceTimeCell;

    fn next(&mut self) -> Option<SpaceTimeCell> {
        self.current = self.current.next_cell(&self.bounds);
        match self.current.cmp(&self.end) {
            Ordering::Less => Some(SpaceTimeCell::from_cell(&self.current)),
            _ => None
        }
    }
}

pub struct TimeCellGenerator {
    end: SpaceTimeCell,
    current: SpaceTimeCell
}

impl TimeCellGenerator {
    pub fn new(start: &SpaceTimeCell, end: &SpaceTimeCell) -> TimeCellGenerator {
        TimeCellGenerator {
            end: SpaceTimeCell::from_cell(end),
            current: SpaceTimeCell::from_cell(start)
        }
    }
}

impl Iterator for TimeCellGenerator {
    type Item = SpaceTimeCell;

    fn next(&mut self) -> Option<SpaceTimeCell> {
        self.current = self.current.next_time_cell();
        match self.current.cmp(&self.end) {
            Ordering::Less => Some(SpaceTimeCell::from_cell(&self.current)),
            _ => None
        }
    }
}