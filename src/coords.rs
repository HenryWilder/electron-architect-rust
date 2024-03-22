use std::convert::From;
use std::fmt::Display;

use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn default() -> Self {
        Self {
            x: i32::default(),
            y: i32::default(),
        }
    }

    /// Size in world units of the space between gridlines (including the infinitely thin gridline itself)
    pub const GRID_SIZE: f32 = 16.0;
    pub const GRID_RADIUS: f32 = Self::GRID_SIZE / 2.0;
    pub const GRID_SIZE_INV: f32 = 1.0 / Self::GRID_SIZE;
    pub const GRID_RADIUS_INV: f32 = 1.0 / Self::GRID_RADIUS;

    /// Converts grid coordinates to world vector
    pub fn into_position(self) -> Vector2 {
        Vector2 {
            x: self.x as f32 * Self::GRID_SIZE + Self::GRID_RADIUS,
            y: self.y as f32 * Self::GRID_SIZE + Self::GRID_RADIUS,
        }
    }

    /// Converts world vector to grid coordinates
    pub fn from_position(p: Vector2) -> Self {
        Self {
            x: ((p.x - Self::GRID_RADIUS) * Self::GRID_SIZE_INV).round() as i32,
            y: ((p.y - Self::GRID_RADIUS) * Self::GRID_SIZE_INV).round() as i32,
        }
    }

    pub fn is_intersecting_coords(&self, start: &Coords, end: &Coords) -> bool {
        // Inside the bounding rectangle of the line segment
        let is_in_bounds = (start.x.min(end.x) <= self.x && self.x <= start.x.max(end.x))
            && (start.y.min(end.y) <= self.y && self.y <= start.y.max(end.y));

        // Out of bounds CAN'T overlap
        if !is_in_bounds {
            return false;
        }

        // Vertical or horizontal line
        if start.x == end.x || start.y == end.y {
            assert_eq!(is_in_bounds, true);
            return true;
        }

        // Assume from here that the line must be diagonal and must be a 45 degree angle

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        assert_eq!((dx.abs(), dy.abs()), (1, 1));

        let m = dx * dy;
        let b = start.y - m * start.x;
        return self.y == m * self.x + b;
    }
}

impl From<Vector2> for Coords {
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x.round() as i32,
            y: value.y.round() as i32,
        }
    }
}

impl From<Coords> for Vector2 {
    fn from(value: Coords) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);
        write!(f, "({:width$}, {:width$})", self.x, self.y, width = width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod is_intersecting_coords {
        use super::*;

        #[test]
        fn test_zero_len() {
            let mut tests = 0;
            for x in -1..1 {
                for y in -1..1 {
                    let coords = Coords { x, y };

                    for x_offset in -1..1 {
                        for y_offset in -1..1 {
                            let expected = x_offset == 0 && y_offset == 0;

                            let test_coords = Coords {
                                x: x + x_offset,
                                y: y + y_offset,
                            };

                            let overlapping = test_coords.is_intersecting_coords(&coords, &coords);
                            println!("test: {test_coords:width$}, start: {coords:width$}, end: {coords:width$}, expected: {expected}, got: {overlapping}", width = 2);
                            assert_eq!(overlapping, expected, "test: {test_coords}, start: {coords}, end: {coords}, expected: {expected}, got: {overlapping}");

                            tests += 1;
                        }
                    }
                }
            }
            println!("Conducted {tests} tests.");
        }

        #[test]
        fn test_inclusive_endpoints() {
            for x1 in -1..1 {
                for y1 in -1..1 {
                    let start = Coords { x: x1, y: y1 };

                    for x2 in -1..1 {
                        for y2 in -1..1 {
                            let end = Coords { x: x2, y: y2 };

                            let inclusive_start = start.is_intersecting_coords(&start, &end);
                            assert_eq!(inclusive_start, true, "test: {start}, start: {start}, end: {end}, expected: true, got: {inclusive_start}");

                            let inclusive_end = end.is_intersecting_coords(&start, &end);
                            assert_eq!(inclusive_end, true, "test: {end}, start: {start}, end: {end}, expected: true, got: {inclusive_start}");
                        }
                    }
                }
            }
        }

        #[test]
        fn test_midpoint() {
            for x1 in -1..1 {
                for y1 in -1..1 {
                    let start = Coords { x: x1, y: y1 };

                    for x2 in -1..1 {
                        for y2 in -1..1 {
                            let end = Coords { x: x2, y: y2 };

                            let inclusive_start = start.is_intersecting_coords(&start, &end);
                            assert_eq!(inclusive_start, true, "test: {start}, start: {start}, end: {end}, expected: true, got: false");

                            let inclusive_end = end.is_intersecting_coords(&start, &end);
                            assert_eq!(inclusive_end, true, "test: {end}, start: {start}, end: {end}, expected: true, got: false");
                        }
                    }
                }
            }
        }
    }
}
