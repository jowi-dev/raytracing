use std::ops;


#[derive(Copy, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T, 
    pub z: T, 
}

impl<T> Vector3<T> {
    pub fn new(x : T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Vector3::<T>::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }

}


impl<T: ops::Sub<Output = T>> ops::Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Vector3::<T>::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl<T: ops::Mul<Output = T>> ops::Mul for Vector3<T> {
    type Output = Self;
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        return Vector3::new(self.x * rhs.x, self.y*rhs.y, self.z*rhs.z)
    }
}


//impl ops::Mul<Vector3<f64>> for Vector3<f64> {
//    type Output = Vector3<f64>;
//    fn mul(self, rhs: Vector3<f64>) -> Self::Output {
//        return Vector3::new(self.x * rhs.x, self.y*rhs.y, self.z*rhs.z);
//    }
//}

impl ops::Div<f64> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn div(self, rhs: f64) -> Self::Output {
        return Vector3::new(self.x/rhs, self.y/rhs, self.z/rhs);
    }
}
