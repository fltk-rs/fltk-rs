pub fn get_fl_name(txt: String) -> String {
    if txt == "Frame" {
        return String::from("Fl_Box");
    }
    if txt == "JpegImage" {
        return String::from("Fl_JPEG_Image");
    }
    if txt == "PngImage" {
        return String::from("Fl_PNG_Image");
    }
    if txt == "BmpImage" {
        return String::from("Fl_BMP_Image");
    }
    if txt == "SvgImage" {
        return String::from("Fl_SVG_Image");
    }
    if txt == "GifImage" {
        return String::from("Fl_GIF_Image");
    }
    if txt == "RgbImage" {
        return String::from("Fl_RGB_Image");
    }
    if txt == "XpmImage" {
        return String::from("Fl_XPM_Image");
    }
    if txt == "XbmImage" {
        return String::from("Fl_XBM_Image");
    }
    if txt == "PnmImage" {
        return String::from("Fl_PNM_Image");
    }

    let mut fl_name = String::from("Fl");
    for c in txt.chars() {
        if c.is_uppercase() {
            fl_name.push('_');
            fl_name.push(c);
        } else {
            fl_name.push(c);
        }
    }
    fl_name
}
