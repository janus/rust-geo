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

    /// Calculate the difference in ‘x’ components (Δx).
    ///
    /// Equivalent to:
    ///
    /// ```rust
    /// # use geo_types::{Line, Coordinate, Point};
    /// # let line = Line::new(
    /// #     Point(Coordinate { x: 4., y: -12. }),
    /// #     Point(Coordinate { x: 0., y: 9. }),
    /// # );
    /// # assert_eq!(
    /// #     line.dx(),
    /// line.end.x() - line.start.x()
    /// # );
    /// ```
    pub fn dx(&self) -> T {
        self.end.x - self.start.x
    }

    /// Calculate the difference in ‘y’ components (Δy).
    ///
    /// Equivalent to:
    ///
    /// ```rust
    /// # use geo_types::{Line, Coordinate, Point};
    /// # let line = Line::new(
    /// #     Point(Coordinate { x: 4., y: -12. }),
    /// #     Point(Coordinate { x: 0., y: 9. }),
    /// # );
    /// # assert_eq!(
    /// #     line.dy(),
    /// line.end.y() - line.start.y()
    /// # );
    /// ```
    pub fn dy(&self) -> T {
        self.end.y - self.start.y
    }

    /// Calculate the slope (Δy/Δx).
    ///
    /// Equivalent to:
    ///
    /// ```rust
    /// # use geo_types::{Line, Coordinate, Point};
    /// # let line = Line::new(
    /// #     Point(Coordinate { x: 4., y: -12. }),
    /// #     Point(Coordinate { x: 0., y: 9. }),
    /// # );
    /// # assert_eq!(
    /// #     line.slope(),
    /// line.dy() / line.dx()
    /// # );
    /// ```
    ///
    /// Note that:
    ///
    /// ```rust
    /// # use geo_types::{Line, Coordinate, Point};
    /// # let a = Point(Coordinate { x: 4., y: -12. });
    /// # let b = Point(Coordinate { x: 0., y: 9. });
    /// # assert!(
    /// Line::new(a, b).slope() ==
    ///     Line::new(b, a).slope()
    /// # );
    /// ```
    ///
    /// Note that:
    ///
    /// ```txt
    /// Line::new(a, b).slope() ==
    ///     Line::new(b, a).slope()
    /// ```
    pub fn slope(&self) -> T {
        self.dy() / self.dx()
    }

    /// Calculate the [determinant](https://en.wikipedia.org/wiki/Determinant) of the line.
    ///
    /// Equivalent to:
    ///
    /// ```rust
    /// # use geo_types::{Line, Coordinate, Point};
    /// # let line = Line::new(
    /// #     Point(Coordinate { x: 4., y: -12. }),
    /// #     Point(Coordinate { x: 0., y: 9. }),
    /// # );
    /// # assert_eq!(
    /// #     line.determinant(),
    /// line.start.x() * line.end.y() -
    ///     line.start.y() * line.end.x()
    /// # );
    /// ```
    ///
    /// Note that:
    ///
    /// ```rust
    /// # use geo_types::{Line, Coordinate, Point};
    /// # let a = Point(Coordinate { x: 4., y: -12. });
    /// # let b = Point(Coordinate { x: 0., y: 9. });
    /// # assert!(
    /// Line::new(a, b).determinant() ==
    ///     -Line::new(b, a).determinant()
    /// # );
    /// ```
    ///
    /// Note that:
    ///
    /// ```txt
    /// Line::new(a, b).determinant() ==
    ///     -Line::new(b, a).determinant()
    /// ```
    ///
    /// [determinant]: https://en.wikipedia.org/wiki/Determinant
    pub fn determinant(&self) -> T {
        self.start.x * self.end.y - self.start.y * self.end.x
    }

    pub fn start_point(&self) -> Point<T> {
        Point(self.start)
    }

    pub fn end_point(&self) -> Point<T> {
        Point(self.end)
    }

    pub fn points(&self) -> (Point<T>, Point<T>) {
        (self.start_point(), self.end_point())
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
