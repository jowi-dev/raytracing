use crate::vector3::Vector3;
pub struct Color{
    pub r: u16,
    pub g: u16,
    pub b: u16
}

impl Color{
    pub fn new(vec: &Vector3<f64>) -> Color {
        let pixel_color : Vector3<f64> = *vec * (255.999 as f64) ;

        return Color{
    r:  ( pixel_color.x) as u16,
    g:  ( pixel_color.y) as u16,
    b:  (pixel_color.z) as u16
        }
    }
}


pub trait Print<T> {

    fn print(_arg: &T) { }
}
impl Print<Self> for Color{
    fn print(color: &Color) {
        println!("{} {} {}", color.r, color.g, color.b);
    }


}

impl Print<Vector3<f64>> for Color {
    fn print(vec: &Vector3<f64>){
        let color : Color = Color::new(vec);

        Color::print(&color);
    }
}
