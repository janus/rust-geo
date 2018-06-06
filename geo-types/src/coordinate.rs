use {Point, CoordinateType};

/// A primitive type which holds `x` and `y` position information
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coordinate<T>
where
    T: CoordinateType,
{
    pub x: T,
    pub y: T,
}

impl<T> Coordinate<T>
where
    T: CoordinateType,
{
    /// Returns the dot product of the two points:
    /// `dot = x1 * x2 + y1 * y2`
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::Coordinate;
    ///
    /// let coord = Coordinate { x: 1.5, y: 0.5 };
    /// let dot = coord.dot(&Coordinate { x: 2.0, y: 4.5 });
    ///
    /// assert_eq!(dot, 5.25);
    /// ```
    pub fn dot(&self, coord: &Coordinate<T>) -> T {
        self.x * coord.x + self.y * coord.y
    }

    /// Returns the cross product of 3 points. A positive value implies
    /// `self` → `point_b` → `point_c` is counter-clockwise, negative implies
    /// clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::Coordinate;
    ///
    /// let coord_a = Coordinate { x: 1., y: 2. };
    /// let coord_b = Coordinate { x: 3., y: 5. };
    /// let coord_c = Coordinate { x: 7., y: 12. };
    ///
    /// let cross = coord_a.cross_prod(&coord_b, &coord_c);
    ///
    /// assert_eq!(cross, 2.0)
    /// ```
    pub fn cross_prod(&self, coord_b: &Coordinate<T>, coord_c: &Coordinate<T>) -> T {
        (coord_b.x - self.x) * (coord_c.y - self.y)
            - (coord_b.y - self.y) * (coord_c.x - self.x)
    }
}

impl<T: CoordinateType> From<(T, T)> for Coordinate<T> {
    fn from(coords: (T, T)) -> Self {
        Coordinate {
            x: coords.0,
            y: coords.1,
        }
    }
}

impl<T: CoordinateType> From<[T; 2]> for Coordinate<T> {
    fn from(coords: [T; 2]) -> Self {
        Coordinate {
            x: coords[0],
            y: coords[1],
        }
    }
}

impl<T: CoordinateType> From<Point<T>> for Coordinate<T> {
    fn from(point: Point<T>) -> Self {
        Coordinate {
            x: point.x(),
            y: point.y(),
        }
    }
}
