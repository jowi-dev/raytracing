use std::ops;
//use cgmath::Vector3;


#[derive(Copy, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T, 
    pub z: T, 
}

impl<T> Vector3<T>{
    pub fn new(x : T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}


impl ops::Add<Vector3<f64>> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn add(self, rhs: Vector3<f64>) -> Vector3<f64> {
        return Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl ops::Sub<Vector3<f64>> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn sub(self, rhs: Vector3<f64>) -> Self::Output {
        return Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl ops::Mul<f64> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn mul(self, rhs: f64) -> Self::Output {
        return Vector3::new(self.x*rhs, self.y*rhs, self.z*rhs);
    }
}


impl ops::Mul<Vector3<f64>> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn mul(self, rhs: Vector3<f64>) -> Self::Output {
        return Vector3::new(self.x * rhs.x, self.y*rhs.y, self.z*rhs.z);
    }
}

impl ops::Div<f64> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn div(self, rhs: f64) -> Self::Output {
        return Vector3::new(self.x/rhs, self.y/rhs, self.z/rhs);
    }
}
