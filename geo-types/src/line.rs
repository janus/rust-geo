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

    pub fn delta_x(&self) -> T {
        self.end.x - self.start.x
    }

    pub fn delta_y(&self) -> T {
        self.end.y - self.start.y
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
