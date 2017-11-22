/// geom.rs	Simple geometry stuff.
///
/// Copyright (c) 2017  Douglas P Lau
use std::fmt;
use std::ops;

// Declare floating point type to use
type Float = f32;

/// 2-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
	pub x: Float,
	pub y: Float,
}

/// 3-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
	pub x: Float,
	pub y: Float,
	pub z: Float,
}

/// Pos trait allows point lookup by handle
pub trait Pos {
	fn pos(&self, hnd: u32) -> Vec3;
}

/// Bounding box
#[derive(Clone, Copy)]
pub struct BBox {
	pub center: Vec3,
	pub half_len: f32,
}

impl fmt::Debug for Vec2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({},{})", self.x, self.y)
	}
}

impl ops::Add for Vec2 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Vec2::new(self.x + other.x, self.y + other.y)
	}
}

impl ops::Sub for Vec2 {
	type Output = Self;

	fn sub(self, other: Self) -> Self{
		Vec2::new(self.x - other.x, self.y - other.y)
	}
}

impl ops::Mul<Float> for Vec2 {
	type Output = Self;

	fn mul(self, s: Float) -> Self {
		Vec2::new(self.x * s, self.y * s)
	}
}

impl ops::Mul for Vec2 {
	type Output = Float;

	/// Calculate the cross product of two Vec2
	fn mul(self, other: Self) -> Float {
		self.x * other.y - self.y * other.x
	}
}

impl ops::Div<Float> for Vec2 {
	type Output = Self;

	fn div(self, s: Float) -> Self {
		Vec2::new(self.x / s, self.y / s)
	}
}

impl ops::Neg for Vec2 {
	type Output = Self;

	fn neg(self) -> Self {
		Vec2::new(-self.x, -self.y)
	}
}

impl Vec2 {
	/// Create a new Vec2
	pub fn new(x: Float, y: Float) -> Self {
		Vec2 { x: x, y: y }
	}

	/// Create a zero Vec2
	pub fn zero() -> Self {
		Vec2::new(0 as Float, 0 as Float)
	}

	/// Get the magnitude of a Vec2
	pub fn mag(self) -> Float {
		self.x.hypot(self.y)
	}

	/// Normalize a Vec2
	pub fn normalize(self) -> Self {
		let m = self.mag();
		if m > 0 as Float {
			self / m
		} else {
			Vec2::zero()
		}
	}

	/// Calculate the distance squared between two Vec2
	pub fn dist_sq(self, other: Self) -> Float {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		dx * dx + dy * dy
	}

	/// Calculate the distance between two Vec2
	pub fn dist(self, other: Self) -> Float {
		self.dist_sq(other).sqrt()
	}

	/// Find the midpoint between two Vec2
	pub fn midpoint(self, other: Self) -> Self {
		let x = (self.x + other.x) / 2 as Float;
		let y = (self.y + other.y) / 2 as Float;
		Vec2::new(x, y)
	}

	/// Create a left-hand perpendicular Vec2
	pub fn left(self) -> Self {
		Vec2::new(-self.y, self.x)
	}

	/// Create a right-hand perpendicular Vec2
	pub fn right(self) -> Self {
		Vec2::new(self.y, -self.x)
	}

	/// Calculate winding order for two Vec2.
	///
	/// The Vec2 should be initialized as edges pointing toward the same vertex.
	/// Returns true if the winding order is widdershins (counter-clockwise).
	pub fn widdershins(self, other: Self) -> bool {
		// Cross product (with Z zero) is used to determine the winding order.
		(self.x * other.y) > (other.x * self.y)
	}

	/// Calculate linear interpolation of two Vec2
	pub fn lerp(self, other: Self, t: Float) -> Self {
		let x = float_lerp(self.x, other.x, t);
		let y = float_lerp(self.y, other.y, t);
		Vec2::new(x, y)
	}
}

/// Calculate linear interpolation of two values
///
/// The t value should be between 0 and 1.
pub fn float_lerp(a: Float, b: Float, t: Float) -> Float {
	b + (a - b) * t
}

/// Calculate intersection point of two lines.
///
/// Returns None if the lines are colinear.
pub fn intersection(a0: Vec2, a1: Vec2, b0: Vec2, b1: Vec2) -> Option<Vec2> {
	let av = a0 - a1;
	let bv = b0 - b1;
	let den = av * bv;
	if den != 0 as Float {
		let ca = a0 * a1;
		let cb = b0 * b1;
		let xn = bv.x * ca - av.x * cb;
		let yn = bv.y * ca - av.y * cb;
		Some(Vec2::new(xn / den, yn / den))
	} else {
		None
	}
}

impl fmt::Debug for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({},{},{})", self.x, self.y, self.z)
	}
}

impl Vec3 {
	/// Create a new Vec3
	pub fn new(x: Float, y: Float, z: Float) -> Self {
		Vec3 { x: x, y: y, z: z }
	}

	/// Find the minimum ordinal value
	fn min_p(self) -> f32 {
		self.x.min(self.y).min(self.z)
	}

	/// Find the maximum ordinal value
	fn max_p(self) -> f32 {
		self.x.max(self.y).max(self.z)
	}

	/// Find the midpoint between two Vec3
	pub fn midpoint(self, other: Self) -> Self {
		let x = (self.x + other.x) / 2 as Float;
		let y = (self.y + other.y) / 2 as Float;
		let z = (self.z + other.z) / 2 as Float;
		Vec3::new(x, y, z)
	}

	/// Calculate the distance squared between two Vec3
	pub fn dist_sq(self, other: Self) -> f32 {
		let dx = other.x - self.x;
		let dy = other.y - self.y;
		let dz = other.z - self.z;
		dx * dx + dy * dy + dz * dz
	}
}

impl fmt::Debug for BBox {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}±{}", self.center, self.half_len)
	}
}

impl BBox {
	pub fn empty() -> BBox {
		BBox { center: Vec3::new(0.0, 0.0, 0.0), half_len: -1.0 }
	}

	pub fn new(p: Vec3) -> BBox {
		BBox { center: p, half_len: 1.0 }
	}

	fn min_p(&self) -> f32 {
		if self.half_len > 0.0 {
			self.center.min_p() - self.half_len
		} else {
			self.center.min_p()
		}
	}

	fn max_p(&self) -> f32 {
		if self.half_len > 0.0 {
			self.center.max_p() + self.half_len
		} else {
			self.center.max_p()
		}
	}

	pub fn extend(&mut self, p: Vec3) {
		self.center = self.move_center(p);
		self.half_len *= 2.0;
	}

	fn move_center(&self, p: Vec3) -> Vec3 {
		let min_p = self.min_p();
		if p.min_p() < min_p {
			return Vec3::new(min_p, min_p, min_p);
		} else {
			let max_p = self.max_p();
			return Vec3::new(max_p, max_p, max_p);
		}
	}

	pub fn contains(&self, p: Vec3) -> bool {
		let Vec3 { x, y, z } = self.center;
		let hl = self.half_len;
		(p.x >= x - hl) &&
		(p.x <  x + hl) &&
		(p.y >= y - hl) &&
		(p.y <  y + hl) &&
		(p.z >= z - hl) &&
		(p.z <  z + hl)
	}
}

#[test]
fn test_vec2() {
	let a = Vec2::new(2f32, 1f32);
	let b = Vec2::new(3f32, 4f32);
	assert!(a + b == Vec2::new(5f32, 5f32));
	assert!(b - a == Vec2::new(1f32, 3f32));
	assert!(a * 2f32 == Vec2::new(4f32, 2f32));
	assert!(a / 2f32 == Vec2::new(1f32, 0.5f32));
	assert!(-a == Vec2::new(-2f32, -1f32));
	assert!(b.mag() == 5f32);
	assert!(a.normalize() == Vec2::new(0.8944272f32, 0.4472136f32));
	assert!(a.dist_sq(b) == 10f32);
	assert!(b.dist(Vec2::new(0f32, 0f32)) == 5f32);
	assert!(a.midpoint(b) == Vec2::new(2.5f32, 2.5f32));
	assert!(a.left() == Vec2::new(-1f32, 2f32));
	assert!(a.right() == Vec2::new(1f32, -2f32));
}
