/*
 * An image of the ppm file format.
 * `width`, `height`: of the image, in pixels.
 * `max_color_val`: the maximum color depth of each pixel.
 * `pixmap`: A 2D vector of `PPMPixel`s of type `T` to represent the image.
 */
struct PPMImage<T>
{
    width: u32,
    height: u32,
    max_color_val: u16,
    pixmap: Vec<Vec <PPMPixel <T>>>,
}

/*
 * A single uncompressed pixel of the ppm file format.
 * `red`, `green`, `blue`: A RGB representation of the given color,
 * stored as type `T` (expected `u8` or `u16`).
*/
struct PPMPixel<T>
{
    red: T,
    green: T,
    blue: T,
}
