use crate::vector3::Vector3;

pub struct Ray<T>{
    pub origin: Vector3<T>,
    pub direction: Vector3<T>,

}

impl<T>  Ray<T> {
    pub fn new(origin: Vector3<T>, direction: Vector3<T>) -> Ray<T> {
        return Ray{ origin, direction }
    }
}
