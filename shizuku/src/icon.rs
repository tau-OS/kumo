use zvariant::Value;

// (iiibiiay)
// width, height, rowstride, has alpha, bits per sample, channels, image data
#[derive(Clone, Debug)]
pub struct ImageData(i32, i32, i32, bool, i32, i32, Vec<u8>);

impl From<&ImageData> for gtk::gdk_pixbuf::Pixbuf {
    fn from(value: &ImageData) -> Self {
        Self::from_bytes(
            &glib::Bytes::from(&value.6),
            gtk::gdk_pixbuf::Colorspace::Rgb,
            value.3,
            value.4,
            value.0,
            value.2,
            value.1,
        )
    }
}

impl From<&zvariant::Value<'_>> for ImageData {
    fn from(value: &zvariant::Value<'_>) -> Self {
        let Value::Structure(data) = value else {
            unreachable!()
        };
        let [Value::I32(w), Value::I32(h), Value::I32(r), Value::Bool(a), Value::I32(s), Value::I32(c), Value::Array(d)] =
            &data.fields()[..]
        else {
            unreachable!()
        };
        let d = d.iter().map(|v| {
            let Value::U8(v) = v else { unreachable!() };
            *v
        });
        Self {
            0: *w,
            1: *h,
            2: *r,
            3: *a,
            4: *s,
            5: *c,
            6: d.collect(),
        }
    }
}
