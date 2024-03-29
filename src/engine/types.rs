use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
};

macro_rules! impl_op {
    ($op:ident, $opfn:ident, $operator:tt) => {
        impl $op for Vector2 {
            type Output = Vector2;

            fn $opfn(self, other: Self) -> Self::Output {
                Vector2 {
                    x: self.x $operator other.x,
                    y: self.y $operator other.y,
                }
            }
        }
    };
}

macro_rules! impl_assign_op {
    ($op:ident, $opfn:ident, $operator:tt) => {
        impl $op for Vector2 {
            fn $opfn(&mut self, other: Self) {
                self.x $operator other.x;
                self.y $operator other.y;
            }
        }
    };
}



#[derive(Clone, Copy, Default, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}


// Impl for vector2
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {
            x,
            y
        }
    }

    pub fn arr(self) -> [f32; 2] {
        <[f32; 2]>::from(self)
    }

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Vector2 {
        if self.len() == 0.0 {
            return Vector2::new(0.0, 0.0);
        }

        self.div(self.len())
    }

    pub fn add(self, val: f32) -> Vector2 {
        Vector2 {
            x: self.x + val,
            y: self.y + val,
        }
    }

    pub fn sub(self, val: f32) -> Vector2 {
        Vector2 {
            x: self.x - val,
            y: self.y - val,
        }
    }

    pub fn mul(self, val: f32) -> Vector2 {
        Vector2 {
            x: self.x * val,
            y: self.y * val,
        }
    }

    pub fn div(self, val: f32) -> Vector2 {
        Vector2 {
            x: self.x / val,
            y: self.y / val,
        }
    }
}


impl From<(f32, f32)> for Vector2 {
    fn from(touple: (f32, f32)) -> Self {
        Vector2 {
            x: touple.0,
            y: touple.1,
        }
    }
}

impl From<Vector2> for (f32, f32) {
    fn from(vector: Vector2) -> Self {
        (vector.x, vector.y)
    }
}

impl From<Vector2> for [f32; 2] {
    fn from(vector: Vector2) -> Self {
        [vector.x, vector.y]
    }
}


impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);


impl_assign_op!(AddAssign, add_assign, +=);
impl_assign_op!(SubAssign, sub_assign, -=); 
impl_assign_op!(MulAssign, mul_assign, *=);
impl_assign_op!(DivAssign, div_assign, /=);
