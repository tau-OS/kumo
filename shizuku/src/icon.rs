#[derive(serde::Deserialize, Clone, Debug)]
pub struct ImageData(u32, u32, i32, bool, i32, i32, Vec<u8>);

pub enum Icon {
    Rgb(image::RgbImage),
    Rgba(image::RgbaImage),
}

impl From<ImageData> for Icon {
    fn from(value: ImageData) -> Self {
        assert_eq!(value.4, 8, "Not 8 bits per sample");
        // value.2 rowstride?
        // value.5 no. channels?
        if value.3 {
            Self::Rgba(image::RgbaImage::from_vec(value.0, value.1, value.6).unwrap())
        } else {
            Self::Rgb(image::RgbImage::from_vec(value.0, value.1, value.6).unwrap())
        }
    }
}
