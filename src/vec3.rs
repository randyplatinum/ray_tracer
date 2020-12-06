use std::ops::{Add, Sub, Neg, Index};

pub struct Vec3 {
	pub e:[f64; 3]
}

impl Vec3
{
	pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
		Self {e: [e0, e1, e2] }
	}
	
	pub fn x(&self) -> f64 {
		self.e[0]
	}

	pub fn y(&self) -> f64 {
		self.e[1]
	}

	pub fn z(&self) -> f64 {
		self.e[2]
	}

	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f64 {
		self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
	}

	pub fn zeros() -> Self {
		Self::new(0.0, 0.0, 0.0)
	}

	pub fn ones() -> Self {
		Self::new(1.0, 1.0, 1.0)
	}
}

impl Add for Vec3 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self { 
			e : [self.e[0] + other.e[0],
			     self.e[1] + other.e[1],
			     self.e[2] + other.e[2]]
		}
	}
}

impl Add for &Vec3 {
	type Output = Vec3;

	fn add(self, other: Self) -> Vec3 {
		Vec3 { 
			e : [self.e[0] + other.e[0],
			     self.e[1] + other.e[1],
			     self.e[2] + other.e[2]]
		}
	}
}

impl Add<&Vec3> for Vec3 {
	type Output = Self;

	fn add(self, other: &Self) -> Self {
		Self { 
			e : [self.e[0] + other.e[0],
			     self.e[1] + other.e[1],
			     self.e[2] + other.e[2]]
		}
	}
}

impl Add<Vec3> for &Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3 {
		Vec3 { 
			e : [self.e[0] + other.e[0],
			     self.e[1] + other.e[1],
			     self.e[2] + other.e[2]]
		}
	}
}

impl Add<f64> for Vec3 {
	type Output = Self;

	fn add(self, scalar: f64) -> Self {
		Self { 
			e : [self.e[0] + scalar,
			     self.e[1] + scalar,
			     self.e[2] + scalar]
		}
	}
}

impl Add<f64> for &Vec3 {
	type Output = Vec3;

	fn add(self, scalar: f64) -> Vec3 {
		Vec3 { 
			e : [self.e[0] + scalar,
			     self.e[1] + scalar,
			     self.e[2] + scalar]
		}
	}
}

impl Sub for Vec3 {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self { 
			e : [self.e[0] - other.e[0],
			   	 self.e[1] - other.e[1],
			     self.e[2] - other.e[2]]
		}
	}
}

impl Sub for &Vec3 {
	type Output = Vec3;

	fn sub(self, other: Self) -> Vec3 {
		Vec3 { 
			e : [self.e[0] - other.e[0],
			   	 self.e[1] - other.e[1],
			     self.e[2] - other.e[2]]
		}
	}
}

impl Sub<&Vec3> for Vec3 {
	type Output = Self;

	fn sub(self, other: &Self) -> Self {
		Self { 
			e : [self.e[0] - other.e[0],
			   	 self.e[1] - other.e[1],
			     self.e[2] - other.e[2]]
		}
	}
}

impl Sub<Vec3> for &Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3 {
		Vec3 { 
			e : [self.e[0] - other.e[0],
			   	 self.e[1] - other.e[1],
			     self.e[2] - other.e[2]]
		}
	}
}

impl Sub<f64> for Vec3 {
	type Output = Self;

	fn sub(self, scalar: f64) -> Self {
		Self { 
			e : [self.e[0] - scalar,
			   	 self.e[1] - scalar,
			     self.e[2] - scalar]
		}
	}
}

impl Sub<f64> for &Vec3 {
	type Output = Vec3;

	fn sub(self, scalar: f64) -> Vec3 {
		Vec3 { 
			e : [self.e[0] - scalar,
			   	 self.e[1] - scalar,
			     self.e[2] - scalar]
		}
	}
}

impl Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self {
		Self { e : [-self.e[0],  -self.e[1], -self.e[2]] }
	}
}
/*
impl Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, i: usize) -> &f64 {
		match i {
			0 => &mut self.e[0],
			1 => &mut self.e[1],
			2 => &mut self.e[2],
			_ => panic!("Index out of bounds: {}", i),
		}
	}
}

*/