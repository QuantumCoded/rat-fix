use image::{ImageBuffer, Rgba};

fn main() -> main_error::MainResult {
    let img_a = image::open("0001.png")?.into_rgba8();
    let img_b = image::open("0002.png")?.into_rgba8(); // to fix

    let mut a = img_a.pixels();
    let b = img_b.pixels();

    let x: ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::from_raw(
        img_a.width(),
        img_a.height(),
        b.map(|pix| {
            if pix
                == a.next()
                    .expect("you're stupid and the images are different sizes, moron")
            {
                Rgba([0u8, 0u8, 0u8, 0u8]).0
            } else {
                pix.0
            }
        })
        .flatten()
        .collect::<Vec<_>>(),
    )
    .expect("failed to build new image ya dingus");

    x.save("new.png")?;

    Ok(())
}
