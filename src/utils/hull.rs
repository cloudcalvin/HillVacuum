//=======================================================================//
// IMPORTS
//
//=======================================================================//

use std::ops::{Add, AddAssign, RangeInclusive, Sub, SubAssign};

use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::utils::math::AroundEqual;

//=======================================================================//
// TYPES
//
//=======================================================================//

/// A rectangle with sides parallel to the x and y axis encompassing a region of bidimensional
/// space.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hull
{
    /// The y coordinate of the top side.
    top:    f32,
    /// The y coordinate of the bottom side.
    bottom: f32,
    /// The x coordinate of the left side.
    left:   f32,
    /// The x coordinate of the right side.
    right:  f32
}

impl Add<Vec2> for Hull
{
    type Output = Self;

    /// Generates a new [`Hull`] with the same dimensions of `self` but displaced by `rhs`.
    #[inline]
    #[must_use]
    fn add(self, rhs: Vec2) -> Self
    {
        Self {
            top:    self.top + rhs.y,
            bottom: self.bottom + rhs.y,
            left:   self.left + rhs.x,
            right:  self.right + rhs.x
        }
    }
}

impl AddAssign<Vec2> for Hull
{
    /// Moves the [`Hull`] by `rhs`.
    #[inline]
    fn add_assign(&mut self, rhs: Vec2)
    {
        self.top += rhs.y;
        self.bottom += rhs.y;
        self.left += rhs.x;
        self.right += rhs.x;
    }
}

impl Sub<Vec2> for Hull
{
    type Output = Self;

    /// Generates a new [`Hull`] with the same dimensions of `self` but displaced by `-rhs`.
    #[inline]
    #[must_use]
    fn sub(self, rhs: Vec2) -> Self
    {
        Self {
            top:    self.top - rhs.y,
            bottom: self.bottom - rhs.y,
            left:   self.left - rhs.x,
            right:  self.right - rhs.x
        }
    }
}

impl SubAssign<Vec2> for Hull
{
    /// Moves the [`Hull`] by `-rhs`.
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2)
    {
        self.top -= rhs.y;
        self.bottom -= rhs.y;
        self.left -= rhs.x;
        self.right -= rhs.x;
    }
}

impl AroundEqual for Hull
{
    #[inline]
    #[must_use]
    fn around_equal(&self, other: &Self) -> bool
    {
        self.top.around_equal(&other.top) &&
            self.bottom.around_equal(&other.bottom) &&
            self.left.around_equal(&other.left) &&
            self.right.around_equal(&other.right)
    }

    #[inline]
    #[must_use]
    fn around_equal_narrow(&self, other: &Self) -> bool
    {
        self.top.around_equal_narrow(&other.top) &&
            self.bottom.around_equal_narrow(&other.bottom) &&
            self.left.around_equal_narrow(&other.left) &&
            self.right.around_equal_narrow(&other.right)
    }
}

impl Hull
{
    //==============================================================
    // New

    /// Returns a new [`Hull`].
    /// # Panics
    /// Panics if `bottom` is greater than `top` or `left` is greater than `right`.
    #[inline]
    #[must_use]
    pub(crate) fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self
    {
        assert!(
            top >= bottom && right >= left,
            "Invalid Hull values: top {top} bottom {bottom} left {left} right {right}"
        );

        Self {
            top,
            bottom,
            left,
            right
        }
    }

    /// Returns the [`Hull`] encompassing all the points contained in `points`.
    /// Returns None if `points` contained no elements.
    #[inline]
    #[must_use]
    pub(crate) fn from_points(points: impl ExactSizeIterator<Item = Vec2>) -> Option<Self>
    {
        if points.len() == 0
        {
            return None;
        }

        let (mut top, mut bottom, mut left, mut right) = (f32::MIN, f32::MAX, f32::MAX, f32::MIN);

        for vx in points
        {
            if vx.y > top
            {
                top = vx.y;
            }

            if vx.y < bottom
            {
                bottom = vx.y;
            }

            if vx.x < left
            {
                left = vx.x;
            }

            if vx.x > right
            {
                right = vx.x;
            }
        }

        Some(Hull::new(top, bottom, left, right))
    }

    //==============================================================
    // Info

    /// The y coordinate of the top side.
    #[inline]
    #[must_use]
    pub const fn top(&self) -> f32 { self.top }

    /// The y coordinate of the bottom side.
    #[inline]
    #[must_use]
    pub const fn bottom(&self) -> f32 { self.bottom }

    /// The x coordinate of the left side.
    #[inline]
    #[must_use]
    pub const fn left(&self) -> f32 { self.left }

    /// The x coordinate of the right side.
    #[inline]
    #[must_use]
    pub const fn right(&self) -> f32 { self.right }

    /// Returns the point representing the top left corner.
    #[inline]
    #[must_use]
    pub const fn top_left(&self) -> Vec2 { Vec2::new(self.left, self.top) }

    /// Returns the point representing the top right corner.
    #[inline]
    #[must_use]
    pub const fn top_right(&self) -> Vec2 { Vec2::new(self.right, self.top) }

    /// Returns the point representing the bottom left corner.
    #[inline]
    #[must_use]
    pub const fn bottom_left(&self) -> Vec2 { Vec2::new(self.left, self.bottom) }

    /// Returns the point representing the bottom right corner.
    #[inline]
    #[must_use]
    pub const fn bottom_right(&self) -> Vec2 { Vec2::new(self.right, self.bottom) }

    /// Returns the width.
    #[inline]
    #[must_use]
    pub fn width(&self) -> f32 { self.right - self.left }

    /// Returns the height.
    #[inline]
    #[must_use]
    pub fn height(&self) -> f32 { self.top - self.bottom }

    /// Returns the he half width.
    #[inline]
    #[must_use]
    pub fn half_width(&self) -> f32 { self.width() / 2f32 }

    /// Returns he half height.
    #[inline]
    #[must_use]
    pub fn half_height(&self) -> f32 { self.height() / 2f32 }

    /// Returns the width and height.
    #[inline]
    #[must_use]
    pub fn dimensions(&self) -> (f32, f32) { (self.width(), self.height()) }

    /// Returns the center.
    #[inline]
    #[must_use]
    pub fn center(&self) -> Vec2
    {
        Vec2::new(self.left + self.half_width(), self.bottom + self.half_height())
    }

    /// Returns the horizontal and vertical ranges.
    #[inline]
    #[must_use]
    pub const fn range(&self) -> (RangeInclusive<f32>, RangeInclusive<f32>)
    {
        (self.left..=self.right, self.bottom..=self.top)
    }
}

//=======================================================================//
// UI
//
//=======================================================================//

#[cfg(feature = "ui")]
pub(crate) mod ui_mod
{
    //=======================================================================//
    // TRAITS
    //
    //=======================================================================//

    /// A trait for entity which are characterized by a bidimensional size.
    pub trait EntityHull
    {
        /// Returns the [`Hull`] representing the dimensions of the entity.
        #[must_use]
        fn hull(&self) -> Hull;
    }

    //=======================================================================//
    // ENUMS
    //
    //=======================================================================//

    use std::{cmp::Ordering, fmt::Display};

    use arrayvec::ArrayVec;
    use glam::Vec2;
    use hill_vacuum_proc_macros::{EnumFromUsize, EnumIter, EnumSize};

    use crate::{
        utils::{
            math::{
                lines_and_segments::point_to_segment_distance_squared,
                points::rotate_point_around_origin,
                AroundEqual
            },
            misc::{next, next_n_steps, prev, PointInsideUiHighlight, VX_HGL_SIDE_SQUARED}
        },
        Hull
    };

    /// The orientation of the rectangle triangle generated by `Hull`. The values of the
    /// enum represent on which vertex the 90 degrees angle of the triangle is
    /// located.
    #[derive(Clone, Copy, Default, Debug, PartialEq)]
    pub(crate) enum TriangleOrientation
    {
        /// Rectangle angle up left.
        #[default]
        TopLeft,
        /// Rectangle angle down left.
        BottomLeft,
        /// Rectangle angle up right.
        TopRight,
        /// Rectangle angle down right.
        BottomRight
    }

    impl Display for TriangleOrientation
    {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            match self
            {
                TriangleOrientation::TopLeft => write!(f, "Top left"),
                TriangleOrientation::BottomLeft => write!(f, "Bottom left"),
                TriangleOrientation::TopRight => write!(f, "Top right"),
                TriangleOrientation::BottomRight => write!(f, "Bottom right")
            }
        }
    }

    impl TriangleOrientation
    {
        /// Returns a [`TrianglOrientation`] based on the relative position of `pos` with respect to
        /// `hull`.    ______
        /// |*   |     |    /
        /// |    |     |   /
        /// |    | ==> |  /
        /// |    |     | /
        /// |    |     |/
        /// ------
        #[inline]
        #[must_use]
        pub fn new(pos: Vec2, hull: &Hull) -> Self
        {
            assert!(hull.contains_point(pos), "Hull {hull:?} does not contain {pos}.");

            let center = hull.center();
            let orientation = u8::from(pos.y < center.y) + u8::from(pos.x > center.x) * 2;

            match orientation
            {
                0 => TriangleOrientation::TopLeft,
                1 => TriangleOrientation::BottomLeft,
                2 => TriangleOrientation::TopRight,
                3 => TriangleOrientation::BottomRight,
                _ => unreachable!()
            }
        }
    }

    //=======================================================================//

    /// The four corners of a rectangle.
    #[derive(Clone, Copy, Debug, EnumSize, EnumIter, EnumFromUsize)]
    pub(crate) enum Corner
    {
        /// The top right corner.
        TopRight,
        /// The top left corner.
        TopLeft,
        /// The bottom left corner.
        BottomLeft,
        /// The bottom right corner.
        BottomRight
    }

    impl Corner
    {
        /// Returns the opposite corner.
        #[inline]
        #[must_use]
        pub fn opposite_corner(self) -> Self
        {
            Self::from(next_n_steps(self as usize, 2, Self::SIZE))
        }

        /// Returns the next cw corner.
        #[inline]
        pub fn next(self) -> Self { Self::from(prev(self as usize, Self::SIZE)) }

        /// Returns the next ccw corner
        #[inline]
        pub fn previous(self) -> Self { Self::from(next(self as usize, Self::SIZE)) }

        /// Returns the horizontal specular corner.
        #[inline]
        #[must_use]
        pub const fn horizontal_specular(self) -> Self
        {
            match self
            {
                Self::TopRight => Self::TopLeft,
                Self::TopLeft => Self::TopRight,
                Self::BottomLeft => Self::BottomRight,
                Self::BottomRight => Self::BottomLeft
            }
        }

        /// Returns the vertical specular corner.
        #[inline]
        #[must_use]
        pub const fn vertical_specular(self) -> Self
        {
            match self
            {
                Self::TopRight => Self::BottomRight,
                Self::TopLeft => Self::BottomLeft,
                Self::BottomLeft => Self::TopLeft,
                Self::BottomRight => Self::TopRight
            }
        }
    }

    //=======================================================================//

    /// The four sides of a rectangle.
    #[derive(Clone, Copy, Debug, EnumIter)]
    pub(crate) enum Side
    {
        /// The top side.
        Top,
        /// The left side.
        Left,
        /// The bottom side.
        Bottom,
        /// The right side.
        Right
    }

    //=======================================================================//

    /// The way a [`Hull`] should be flipped.
    #[derive(Clone, Copy, Debug)]
    pub(crate) enum Flip
    {
        /// Above.
        Above(f32),
        /// Below,
        Below(f32),
        /// Left.
        Left(f32),
        /// Right.
        Right(f32)
    }

    impl Flip
    {
        /// Returns the value wrapped by all enum arms.
        #[inline]
        #[must_use]
        pub const fn mirror(self) -> f32
        {
            let (Self::Above(m) | Self::Below(m) | Self::Left(m) | Self::Right(m)) = self;
            m
        }
    }

    //=======================================================================//

    /// How the [`Hull`] was scaled.
    pub(crate) enum ScaleResult
    {
        /// No scaling.
        None,
        /// Scaled.
        Scale(Hull),
        /// Flipped and maybe scaled.
        Flip(ArrayVec<Flip, 2>, Hull)
    }

    //=======================================================================//

    impl<T: ExactSizeIterator<Item = Vec2>> From<T> for Hull
    {
        #[inline]
        #[must_use]
        fn from(value: T) -> Self { Self::from_points(value).unwrap() }
    }

    impl Hull
    {
        //==============================================================
        // New

        /// Returns a new [`Hull`] from two points used as opposite vertexes of a rectangular shape.
        #[inline]
        #[must_use]
        pub(crate) fn from_opposite_vertexes(a: Vec2, b: Vec2) -> Option<Self>
        {
            if a.around_equal_narrow(&b)
            {
                return None;
            }

            Some(Self::new(a.y.max(b.y), a.y.min(b.y), a.x.min(b.x), a.x.max(b.x)))
        }

        /// Returns the [`Hull`] encompassing all the [`Hull`]s contained in `iter`.
        /// Returns None if `iter` contains no elements.
        #[inline]
        #[must_use]
        pub(crate) fn from_hulls_iter<I: Iterator<Item = Hull>>(iter: I) -> Option<Self>
        {
            let mut result = None;

            for hull in iter
            {
                match &mut result
                {
                    None => result = (hull.top, hull.bottom, hull.left, hull.right).into(),
                    Some((top, bottom, left, right)) =>
                    {
                        *top = f32::max(*top, hull.top);
                        *bottom = f32::min(*bottom, hull.bottom);
                        *left = f32::min(*left, hull.left);
                        *right = f32::max(*right, hull.right);
                    }
                };
            }

            result.map(|(top, bottom, left, right)| Hull::new(top, bottom, left, right))
        }

        //==============================================================
        // Info

        /// Whether the [`Hull`] contains `p`.
        #[inline]
        #[must_use]
        pub(crate) fn contains_point(&self, p: Vec2) -> bool
        {
            let range = self.range();
            range.0.contains(&p.x) && range.1.contains(&p.y)
        }

        /// Whether the [`Hull`] contains `other`.
        #[inline]
        #[must_use]
        pub(crate) fn contains_hull(&self, other: &Self) -> bool
        {
            let range = self.range();

            range.0.contains(&other.left) &&
                range.0.contains(&other.right) &&
                range.1.contains(&other.bottom) &&
                range.1.contains(&other.top)
        }

        /// Whether the [`Hull`] overlaps `other`.
        #[inline]
        #[must_use]
        pub(crate) fn overlaps(&self, other: &Self) -> bool
        {
            self.left < other.right &&
                self.right > other.left &&
                self.top > other.bottom &&
                self.bottom < other.top
        }

        /// Whether the [`Hull`] intersects another one.
        #[inline]
        #[must_use]
        pub(crate) fn intersects(&self, other: &Self) -> bool
        {
            let (x_range, y_range) = other.range();

            (other.left < self.right &&
                other.right > self.left &&
                (y_range.contains(&self.top) || y_range.contains(&self.bottom))) ||
                (other.bottom < self.top &&
                    other.top > self.bottom &&
                    (x_range.contains(&self.left) || x_range.contains(&self.right)))
        }

        //==============================================================
        // Update

        /// Breaks the [`Hull`] into its components: top, bottom, left, right.
        #[inline]
        #[must_use]
        pub(crate) const fn decompose(self) -> (f32, f32, f32, f32)
        {
            (self.top, self.bottom, self.left, self.right)
        }

        /// Returns the [`Hull`] encompassing this and `other`.
        #[inline]
        #[must_use]
        pub(crate) fn merged(&self, other: &Hull) -> Self
        {
            Hull::new(
                f32::max(self.top, other.top),
                f32::min(self.bottom, other.bottom),
                f32::min(self.left, other.left),
                f32::max(self.right, other.right)
            )
        }

        /// Extends the horizontal and vertical dimensions by `2f32 * bump` while maintaining the
        /// same center.
        #[inline]
        #[must_use]
        pub(crate) fn bumped(&self, bump: f32) -> Self
        {
            Hull::new(self.top + bump, self.bottom - bump, self.left - bump, self.right + bump)
        }

        #[inline]
        #[must_use]
        pub(crate) fn transformed<F: Fn(Vec2) -> Vec2>(&self, f: F) -> Self
        {
            Self::from_points(self.vertexes().map(f)).unwrap()
        }

        /// Returns the vector representing the overlap of the [`Hull`] and `other`.
        #[inline]
        #[must_use]
        pub(crate) fn overlap_vector(&self, other: &Hull) -> Option<Vec2>
        {
            let (l_1, r_1) = (self.left, self.right);
            let (b_1, t_1) = (self.bottom, self.top);
            let mut overlap_vector = None;

            /// Checks the overlap between the hulls at the coordinates.
            macro_rules! overlap {
                ($vx:expr, $x:ident, $y:expr) => {
                    let vx = $vx;

                    if self.contains_point(vx)
                    {
                        let o = {
                            let v_x = $x - vx.x;
                            let v_y = $y - vx.y;

                            if v_x.abs() < v_y.abs()
                            {
                                Vec2::new(v_x, 0f32)
                            }
                            else
                            {
                                Vec2::new(0f32, v_y)
                            }
                        };

                        match overlap_vector
                        {
                            None => overlap_vector = o.into(),
                            Some(o_v) if o_v.length_squared() < o.length_squared() =>
                            {
                                overlap_vector = o_v.into()
                            },
                            _ => ()
                        };
                    }
                };
            }

            overlap!(other.top_left(), r_1, b_1);
            overlap!(other.top_right(), l_1, b_1);
            overlap!(other.bottom_left(), r_1, t_1);
            overlap!(other.bottom_right(), l_1, t_1);

            overlap_vector
        }

        /// Returns the corner coordinates closets to `p`.
        #[inline]
        #[must_use]
        pub(crate) fn nearest_corner_to_point(&self, p: Vec2) -> Vec2
        {
            let mut vx = Vec2::new(self.left, self.top);

            if p.x > self.right - self.width() / 2f32
            {
                vx.x = self.right;
            }

            if p.y < self.bottom + self.height() / 2f32
            {
                vx.y = self.bottom;
            }

            vx
        }

        //==============================================================
        // Polygons

        /// Generates an array of three vertexes representing a triangle from the [`Hull`] based on
        /// `orientation`.
        #[inline]
        #[must_use]
        pub(crate) const fn triangle(&self, orientation: TriangleOrientation) -> [Vec2; 3]
        {
            match orientation
            {
                TriangleOrientation::TopLeft =>
                {
                    [self.top_right(), self.top_left(), self.bottom_left()]
                },
                TriangleOrientation::TopRight =>
                {
                    [self.top_right(), self.top_left(), self.bottom_right()]
                },
                TriangleOrientation::BottomLeft =>
                {
                    [self.top_left(), self.bottom_left(), self.bottom_right()]
                },
                TriangleOrientation::BottomRight =>
                {
                    [self.top_right(), self.bottom_left(), self.bottom_right()]
                },
            }
        }

        /// Generates an array of four vertexes representing a rectangle with dimensions equal to
        /// [`Hull`]'s.
        #[inline]
        #[must_use]
        pub(crate) const fn rectangle(&self) -> [Vec2; 4]
        {
            [
                self.top_right(),
                self.top_left(),
                self.bottom_left(),
                self.bottom_right()
            ]
        }

        /// Returns an iterator to the vertexes of a circle-like polygon inscribed in [`Hull`] with
        /// `resolution` sides.
        #[inline]
        #[must_use]
        pub(crate) fn circle(&self, resolution: u8) -> CircleIterator
        {
            CircleIterator::new(resolution, self)
        }

        //==============================================================
        // Corner & Side

        /// The coordinates of `corner`.
        #[inline]
        #[must_use]
        pub(crate) const fn corner_vertex(&self, corner: Corner) -> Vec2
        {
            match corner
            {
                Corner::TopRight => self.top_right(),
                Corner::TopLeft => self.top_left(),
                Corner::BottomLeft => self.bottom_left(),
                Corner::BottomRight => self.bottom_right()
            }
        }

        /// Returns an iterator to all four [`Corner`]s and relative coordinates of the [`Hull`].
        #[inline]
        pub(crate) fn corners(&self) -> impl ExactSizeIterator<Item = (Corner, Vec2)> + '_
        {
            Corner::iter().map(|corner| (corner, self.corner_vertex(corner)))
        }

        /// Returns an iterator to the coordinates of the four [`Corner`]s.
        #[inline]
        pub(crate) fn vertexes(&self) -> impl ExactSizeIterator<Item = Vec2> + '_
        {
            Corner::iter().map(|corner| self.corner_vertex(corner))
        }

        /// Returns the coordinates of the [`Corner`] nearby `cursor_pos`, if any.
        #[inline]
        #[must_use]
        pub(crate) fn nearby_corner(&self, cursor_pos: Vec2, camera_scale: f32) -> Option<Corner>
        {
            self.corners().find_map(|(corner, vx)| {
                vx.is_point_inside_ui_highlight(cursor_pos, camera_scale)
                    .then_some(corner)
            })
        }

        /// The coordinates of the vertexes representing `side`.
        #[inline]
        #[must_use]
        pub(crate) const fn side_segment(&self, side: Side) -> [Vec2; 2]
        {
            match side
            {
                Side::Top => [self.top_right(), self.top_left()],
                Side::Left => [self.top_left(), self.bottom_left()],
                Side::Bottom => [self.bottom_left(), self.bottom_right()],
                Side::Right => [self.bottom_right(), self.top_right()]
            }
        }

        /// Returns an Iterator to all four [`Side`]s and relative coordinates of the [`Hull`].
        #[inline]
        pub(crate) fn sides(&self) -> impl Iterator<Item = (Side, [Vec2; 2])> + '_
        {
            Side::iter().map(|side| (side, self.side_segment(side)))
        }

        /// Returns the [`Side`] closet to `cursor_pos`, if any.
        #[allow(clippy::missing_panics_doc)]
        #[inline]
        #[must_use]
        pub(crate) fn nearby_side(&self, cursor_pos: Vec2, camera_scale: f32) -> Option<Side>
        {
            let mut distance = f32::MAX;
            let mut result = None;
            let max_distance = VX_HGL_SIDE_SQUARED * camera_scale;

            for (side, [vx_j, vx_i]) in self.sides()
            {
                let temp = point_to_segment_distance_squared(vx_j, vx_i, cursor_pos);

                if temp <= max_distance && temp < distance
                {
                    result = side.into();
                    distance = temp;
                }
            }

            result
        }

        //==============================================================
        // Scale

        /// Flips the [`Hull`] with respect to the `flip_queue` elements.
        #[inline]
        #[must_use]
        pub(crate) fn flipped(&self, flip_queue: impl Iterator<Item = Flip>) -> Self
        {
            let mut hull = *self;
            let width = hull.width();
            let height = hull.height();

            for flip in flip_queue
            {
                match flip
                {
                    Flip::Above(_) =>
                    {
                        hull.top += height;
                        hull.bottom += height;
                    },
                    Flip::Below(_) =>
                    {
                        hull.top -= height;
                        hull.bottom -= height;
                    },
                    Flip::Left(_) =>
                    {
                        hull.left -= width;
                        hull.right -= width;
                    },
                    Flip::Right(_) =>
                    {
                        hull.left += width;
                        hull.right += width;
                    }
                };
            }

            hull
        }

        /// Returns a [`Hull`] scaled according to the new position of the moved [`Corner`].
        #[inline]
        #[must_use]
        pub(crate) fn scaled(
            &self,
            selected_corner: &mut Corner,
            new_corner_position: Vec2
        ) -> ScaleResult
        {
            use arrayvec::ArrayVec;
            use hill_vacuum_shared::return_if_none;

            /// Checks a flip above the pivot.
            macro_rules! check_flip_higher {
                ($(($name:ident, $xy:ident, $mirror:ident, $flip:ident, $hor_ver:ident)),+) => { paste::paste! { $(
                    #[inline]
                    fn [< check_flip_ $name >] (
                        hull: &Hull,
                        selected_corner: &mut Corner,
                        new_corner_position: Vec2,
                        flip_queue: &mut ArrayVec<Flip, 2>
                    )
                    {
                        if new_corner_position.$xy <= hull.$mirror
                        {
                            return;
                        }

                        flip_queue.push(Flip::$flip(hull.$mirror));
                        *selected_corner = selected_corner.[< $hor_ver _specular >]();
                    }
                )+ }};
            }

            /// Checks a flip below the pivot.
            macro_rules! check_flip_lower {
                ($(($name:ident, $xy:ident, $mirror:ident, $flip:ident, $hor_ver:ident)),+) => { paste::paste! { $(
                    #[inline]
                    fn [< check_flip_ $name >] (
                        hull: &Hull,
                        selected_corner: &mut Corner,
                        new_corner_position: Vec2,
                        flip_queue: &mut ArrayVec<Flip, 2>
                    )
                    {
                        if new_corner_position.$xy >= hull.$mirror
                        {
                            return;
                        }

                        flip_queue.push(Flip::$flip(hull.$mirror));
                        *selected_corner = selected_corner.[< $hor_ver _specular >]();
                    }
                )+ }};
            }

            check_flip_higher!(
                (above, y, top, Above, vertical),
                (right, x, right, Right, horizontal)
            );
            check_flip_lower!(
                (below, y, bottom, Below, vertical),
                (left, x, left, Left, horizontal)
            );

            if new_corner_position.around_equal_narrow(&self.corner_vertex(*selected_corner))
            {
                return ScaleResult::None;
            }

            let mut flip_queue = ArrayVec::<_, 2>::new();
            let opposite_vertex = self.corner_vertex(selected_corner.opposite_corner());
            let mut new_selected_corner = *selected_corner;

            /// Checks whether the scaling process leads to a flip.
            macro_rules! check_flips {
                ($funcs:expr) => {
                    for func in $funcs
                    {
                        func(self, &mut new_selected_corner, new_corner_position, &mut flip_queue);
                    }
                };
            }

            match selected_corner
            {
                Corner::TopRight => check_flips!([check_flip_left, check_flip_below]),
                Corner::TopLeft => check_flips!([check_flip_right, check_flip_below]),
                Corner::BottomLeft => check_flips!([check_flip_right, check_flip_above]),
                Corner::BottomRight => check_flips!([check_flip_left, check_flip_above])
            };

            let hull = return_if_none!(
                Hull::from_opposite_vertexes(new_corner_position, opposite_vertex),
                ScaleResult::None
            );

            if hull.width() == 0f32 || hull.height() == 0f32
            {
                return ScaleResult::None;
            }

            *selected_corner = new_selected_corner;

            match flip_queue.len()
            {
                0 => ScaleResult::Scale(hull),
                1 | 2 => ScaleResult::Flip(flip_queue, hull),
                _ => unreachable!()
            }
        }
    }

    /// An iterator that generates the coordinates of an oval/circular shape.
    #[derive(Clone, Copy)]
    pub(crate) struct CircleIterator
    {
        /// The first coordinate.
        starting_point: Vec2,
        /// The center of the shape.
        center:         Vec2,
        /// The angle of the slices in which the circular shape is divides.
        slice_angle:    f32,
        /// The index of the coordinate to be returned.
        left:           usize,
        /// The total amount of sides.
        right:          usize,
        /// The multiplier needed to generate the coordinates of elleptical shapes.
        multi:          f32,
        /// The function generating the coordinates.
        generator:      fn(Vec2, f32) -> Vec2
    }

    impl ExactSizeIterator for CircleIterator
    {
        #[must_use]
        fn len(&self) -> usize { self.right - self.left }
    }

    #[allow(clippy::copy_iterator)]
    impl Iterator for CircleIterator
    {
        type Item = Vec2;

        #[must_use]
        fn next(&mut self) -> Option<Self::Item>
        {
            if self.left == self.right
            {
                return None;
            }

            let vx = self.next_vertex();
            self.left += 1;
            Some(vx)
        }
    }

    impl CircleIterator
    {
        /// Returns a new [`CircleIterator`].
        #[inline]
        fn new(resolution: u8, hull: &Hull) -> Self
        {
            let u_resolution = resolution as usize;
            let (width, height) = hull.dimensions();

            let (multi, generator): (f32, fn(Vec2, f32) -> Vec2) =
                match width.partial_cmp(&height).unwrap()
                {
                    Ordering::Equal => (0f32, Self::actual_circle_vx_generator),
                    Ordering::Greater => (width / height, Self::oval_vx_greater_width_generator),
                    Ordering::Less => (height / width, Self::oval_vx_greater_height_generator)
                };

            Self {
                starting_point: Vec2::new(0f32, width.min(height) / 2f32),
                center: hull.center(),
                slice_angle: std::f32::consts::TAU / f32::from(resolution),
                left: 0,
                right: u_resolution,
                multi,
                generator
            }
        }

        /// Returns the first point.
        #[inline]
        #[must_use]
        pub fn starting_point(&self) -> Vec2 { self.starting_point + self.center }

        /// Reverts the iteration to the previous element.
        #[inline]
        pub fn regress(&mut self) { self.left -= 1; }

        /// Generates the next vertex of the shape.
        #[inline]
        #[must_use]
        fn next_vertex(&self) -> Vec2
        {
            #[allow(clippy::cast_precision_loss)]
            let mut vx = rotate_point_around_origin(
                self.starting_point,
                self.slice_angle * self.left as f32
            );
            vx = (self.generator)(vx, self.multi);
            vx += self.center;
            vx
        }

        /// Function that modifies `pos` to generate a circular shape.
        #[inline]
        #[must_use]
        const fn actual_circle_vx_generator(pos: Vec2, _: f32) -> Vec2 { pos }

        /// Function that modifies `pos` to generate an oval shape with width greater than height.
        #[inline]
        #[must_use]
        fn oval_vx_greater_width_generator(mut pos: Vec2, multi: f32) -> Vec2
        {
            pos.x *= multi;
            pos
        }

        /// Function that modifies `pos` to generate an oval shape with height greater than width.
        #[inline]
        #[must_use]
        fn oval_vx_greater_height_generator(mut pos: Vec2, multi: f32) -> Vec2
        {
            pos.y *= multi;
            pos
        }
    }
}

#[cfg(feature = "ui")]
pub(crate) use ui_mod::*;
