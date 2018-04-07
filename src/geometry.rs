use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0 };

    #[inline]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    #[inline]
    pub fn scaled(&self, multiplier: f64) -> Point {
        Point::new(self.x * multiplier, self.y * multiplier)
    }

    #[inline]
    pub fn round(&self) -> Point {
        Point::new(self.x.round(), self.y.round())
    }
}

impl Size {
    pub const ZERO: Size = Size {
        width: 0.0,
        height: 0.0,
    };

    #[inline]
    pub fn new(width: f64, height: f64) -> Size {
        Size {
            width: width,
            height: height,
        }
    }

    #[inline]
    pub fn scaled(&self, multiplier: f64) -> Size {
        Size::new(self.width * multiplier, self.height * multiplier)
    }

    #[inline]
    pub fn round(&self) -> Size {
        Size::new(self.width.round(), self.height.round())
    }
}

impl Rect {
    pub const ZERO: Rect = Rect {
        origin: Point::ZERO,
        size: Size::ZERO,
    };

    #[inline]
    pub fn new(origin: Point, size: Size) -> Rect {
        Rect {
            origin: origin,
            size: size,
        }
    }

    #[inline]
    pub fn scaled(&self, multiplier: f64) -> Rect {
        Rect::new(self.origin.scaled(multiplier), self.size.scaled(multiplier))
    }

    #[inline]
    pub fn round(&self) -> Rect {
        Rect::new(self.origin.round(), self.size.round())
    }

    #[inline]
    pub fn is_point_visible(&self, point: Point) -> bool {
        point.x >= self.origin.x && point.y >= self.origin.y && point.x < self.max_x()
            && point.y < self.max_y()
    }

    #[inline]
    pub fn is_rect_visible(&self, rect: Rect) -> bool {
        self.is_point_visible(rect.origin)
            && (rect.size.width + rect.origin.x + self.origin.x) <= self.size.width
            && (rect.size.height + rect.origin.y + self.origin.y) <= self.size.height
    }

    #[inline]
    pub fn max_x(&self) -> f64 {
        self.origin.x + self.size.width
    }

    #[inline]
    pub fn max_y(&self) -> f64 {
        self.origin.y + self.size.height
    }

    pub fn iter_point(&self, point: Point) -> Option<Point> {
        if point.y + 1.0 < self.max_y() {
            Some(Point::new(point.x, point.y + 1.0))
        } else if point.x + 1.0 < self.max_x() {
            Some(Point::new(point.x + 1.0, self.origin.y))
        } else {
            None
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.origin, self.size)
    }
}
