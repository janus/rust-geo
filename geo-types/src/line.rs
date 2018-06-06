use {CoordinateType, Coordinate, Point};

#[cfg(feature = "spade")]
use algorithms::{BoundingBox, EuclideanDistance};

/// A line segment made up of exactly two [`Point`s](struct.Point.html)
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<T>
where
    T: CoordinateType,
{
    pub start: Coordinate<T>,
    pub end: Coordinate<T>,
}

impl<T> Line<T>
where
    T: CoordinateType,
{
    /// Creates a new line segment.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{Coordinate, Line};
    ///
    /// let line = Line::new(
    ///     Coordinate { x: 0., y: 0. },
    ///     Coordinate { x: 1., y: 2. },
    /// );
    ///
    /// assert_eq!(line.start, Coordinate { x: 0., y: 0. });
    /// assert_eq!(line.end, Coordinate { x: 1., y: 2. });
    /// ```
    pub fn new(start: Coordinate<T>, end: Coordinate<T>) -> Line<T> {
        Line {
            start,
            end
        }
   }

    pub fn to_points(&self) -> (Point<T>, Point<T>) {
        (Point(self.start), Point(self.end))
    }

    /// Calculate the difference in ‘x’ components (Δx).
    ///
    /// Equivalent to:
    ///
    /// ```txt
    /// line.end.x() - line.start.x()
    /// ```
    pub fn dx(&self) -> T {
        self.end.x - self.start.x
    }

    /// Calculate the difference in ‘y’ components (Δy).
    ///
    /// Equivalent to:
    ///
    /// ```txt
    /// line.end.y() - line.start.y()
    /// ```
    pub fn dy(&self) -> T {
        self.end.y - self.start.y
    }

    /// Calculate the slope (Δy/Δx).
    ///
    /// Equivalent to:
    ///
    /// ```txt
    /// line.dy() / line.dx()
    /// ```
    pub fn slope(&self) -> T {
        self.dy() / self.dx()
    }

    /// Calculate the [determinant] of the line.
    ///
    /// Equivalent to:
    ///
    /// ```txt
    /// line.start.x() * line.end.y() -
    ///     line.start.y() * line.end.x()
    /// ```
    ///
    /// [determinant]: https://en.wikipedia.org/wiki/Determinant
    pub fn determinant(&self) -> T {
        self.start.x * self.end.y - self.start.y * self.end.x
    }
}

impl<T: CoordinateType> From<[(T, T); 2]> for Line<T> {
    fn from(coord: [(T, T); 2]) -> Line<T> {
        Line::new(
            coord[0].into(),
            coord[1].into(),
        )
    }
}

#[cfg(feature = "spade")]
impl<T> ::spade::SpatialObject for Line<T>
where
    T: ::num_traits::Float + ::spade::SpadeNum + ::std::fmt::Debug,
{
    type Point = Point<T>;

    fn mbr(&self) -> ::spade::BoundingRect<Self::Point> {
        let bbox = self.bbox();
        ::spade::BoundingRect::from_corners(
            &Point::new(bbox.xmin, bbox.ymin),
            &Point::new(bbox.xmax, bbox.ymax),
        )
    }

    fn distance2(&self, point: &Self::Point) -> <Self::Point as ::spade::PointN>::Scalar {
        let d = self.euclidean_distance(point);
        if d == T::zero() {
            d
        } else {
            d.powi(2)
        }
    }
}
