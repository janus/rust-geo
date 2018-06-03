use std::iter::FromIterator;
use {Coordinate, CoordinateType, Line};

/// An ordered collection of two or more [`Coordinate`s](struct.Coordinate.html), representing a
/// path between locations.
///
/// # Examples
///
/// Create a `LineString` by calling it directly:
///
/// ```
/// use geo_types::{LineString, Coordinate};
///
/// let line_string = LineString(vec![
///     Coordinate { x: 0., y: 0. },
///     Coordinate { x: 10., y: 0. },
/// ]);
/// ```
///
/// Converting a `Vec` of `Coordinate`-like things:
///
/// ```
/// use geo_types::LineString;
///
/// let line_string: LineString<f32> = vec![
///     (0., 0.),
///     (10., 0.),
/// ].into();
/// ```
///
/// ```
/// use geo_types::LineString;
///
/// let line_string: LineString<f64> = vec![
///     [0., 0.],
///     [10., 0.],
/// ].into();
/// ```
//
/// Or `collect`ing from a `Coordinate` iterator
///
/// ```
/// use geo_types::{LineString, Coordinate};
///
/// let mut coords_iter = vec![
///     Coordinate { x: 0., y: 0. },
///     Coordinate { x: 10., y: 0. }
/// ].into_iter();
///
/// let line_string: LineString<f32> = coords_iter.collect();
/// ```
///
/// You can iterate over the coordintes in the `LineString`
///
/// ```
/// use geo_types::{LineString, Coordinate};
///
/// let line_string = LineString(vec![
///     Coordinate { x: 0., y: 0. },
///     Coordinate { x: 10., y: 0. },
/// ]);
///
/// for coord in line_string {
///     println!("Coordinate x = {}, y = {}", coord.x, coord.y);
/// }
/// ```
///
#[derive(PartialEq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LineString<T>(pub Vec<Coordinate<T>>)
where
    T: CoordinateType;

impl<T: CoordinateType> LineString<T> {
    /// Return an `Line` iterator that yields one `Line` for each line segment
    /// in the `LineString`.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{Line, LineString, Coordinate};
    ///
    /// let mut coords = vec![(0., 0.), (5., 0.), (7., 9.)];
    /// let line_string: LineString<f32> = coords.into_iter().collect();
    ///
    /// let mut lines = line_string.lines();
    /// assert_eq!(
    ///     Some(Line::new(Coordinate { x: 0., y: 0. }, Coordinate { x: 5., y: 0. })),
    ///     lines.next()
    /// );
    /// assert_eq!(
    ///     Some(Line::new(Coordinate { x: 5., y: 0. }, Coordinate { x: 7., y: 9. })),
    ///     lines.next()
    /// );
    /// assert!(lines.next().is_none());
    /// ```
    pub fn lines<'a>(&'a self) -> impl Iterator<Item = Line<T>> + 'a {
        self.0.windows(2).map(|w| unsafe {
            // As long as the LineString has at least two coordinates, we shouldn't
            // need to do bounds checking here.
            Line::new(*w.get_unchecked(0), *w.get_unchecked(1))
        })
    }
}

/// Turn a `Vec` of `Point`-ish objects into a `LineString`.
impl<T: CoordinateType, IC: Into<Coordinate<T>>> From<Vec<IC>> for LineString<T> {
    fn from(v: Vec<IC>) -> Self {
        LineString(v.into_iter().map(|c| c.into()).collect())
    }
}

/// Turn a `Point`-ish iterator into a `LineString`.
impl<T: CoordinateType, IC: Into<Coordinate<T>>> FromIterator<IC> for LineString<T> {
    fn from_iter<I: IntoIterator<Item = IC>>(iter: I) -> Self {
        LineString(iter.into_iter().map(|c| c.into()).collect())
    }
}

/// Iterate over all the [Coordinate](struct.Coordinates.html)s in this `LineString`.
impl<T: CoordinateType> IntoIterator for LineString<T> {
    type Item = Coordinate<T>;
    type IntoIter = ::std::vec::IntoIter<Coordinate<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
