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
        assert_ne!(dx.abs(), 0); // Non-zero width
        assert_ne!(dy.abs(), 0); // Non-zero height
        assert_eq!(dx.abs(), dy.abs()); // Equal width and height

        // Both are negative or both are positive = positive slope. One is negative and one is positive = negative slope.
        // Because the diagonal lines are ALWAYS at a 45 degree angle, the slope will ALWAYS have a magnitude of 1.
        let m = if dx == dy { 1 } else { -1 };
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
                    let start = Coords {
                        x: x1 * 2,
                        y: y1 * 2,
                    };

                    for x2 in -1..1 {
                        for y2 in -1..1 {
                            let end = Coords {
                                x: x2 * 2,
                                y: y2 * 2,
                            };

                            let midpoint = Coords {
                                x: (start.x + end.x) / 2,
                                y: (start.y + end.y) / 2,
                            };

                            let overlapping = midpoint.is_intersecting_coords(&start, &end);
                            assert_eq!(overlapping, true, "test: {midpoint}, start: {start}, end: {end}, expected: true, got: {overlapping}");

                            // Perpendicular point
                            let perp: Coords;

                            if x1 == x2 {
                                // vertical
                                perp = Coords {
                                    x: midpoint.x + 1,
                                    y: midpoint.y,
                                };
                            } else if y1 == y2 {
                                // horizontal
                                perp = Coords {
                                    x: midpoint.x,
                                    y: midpoint.y + 1,
                                };
                            } else {
                                // diagonal
                                perp = Coords {
                                    x: start.x,
                                    y: end.y,
                                };
                            }

                            let result = perp.is_intersecting_coords(&start, &end);
                            assert_eq!(result, false, "test: {perp}, start: {start}, end: {end}, expected: false, got: {result}");
                        }
                    }
                }
            }
        }

        #[test]
        fn test_beyond_segment() {
            for x1 in -1..1 {
                for y1 in -1..1 {
                    let start = Coords { x: x1, y: y1 };

                    for x2 in -1..1 {
                        for y2 in -1..1 {
                            let end = Coords { x: x2, y: y2 };

                            if start == end {
                                continue;
                            }

                            let after_end = Coords {
                                x: start.x + 2 * (end.x - start.x),
                                y: start.y + 2 * (end.y - start.y),
                            };

                            let overlapping1 = after_end.is_intersecting_coords(&start, &end);
                            assert_eq!(overlapping1, false, "test: {after_end}, start: {start}, end: {end}, expected: false, got: {overlapping1}");

                            let before_start = Coords {
                                x: end.x + 2 * (start.x - end.x),
                                y: end.y + 2 * (start.y - end.y),
                            };

                            let overlapping2 = before_start.is_intersecting_coords(&start, &end);
                            assert_eq!(overlapping2, false, "test: {before_start}, start: {start}, end: {end}, expected: false, got: {overlapping2}");
                        }
                    }
                }
            }
        }
    }
}
