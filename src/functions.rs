pub mod functions {
    use photon_rs::{ PhotonImage };
    use photon_rs::channels::alter_channel;

    // One rift on how to decrease image brighness based on
    // approch used in Photon RS inc_brightness.  However because
    // this needed to return a new image not favored as other effect calls
    // mutate the PhotoImage passed to the function. 
    #[allow(dead_code)]
    pub fn dec_brightness(photon_image: &mut PhotonImage, 
                          brightness: u8) -> PhotonImage {
        let end = photon_image.get_raw_pixels().len() - 4;
        let mut image_pixels = photon_image.get_raw_pixels();
        for i in (0..end).step_by(4) {
            let r_val = image_pixels[i];
            let g_val = image_pixels[i + 1];
            let b_val = image_pixels[i + 2];
    
            if r_val >= 0 + brightness {
                image_pixels[i] -= brightness;
            } else {
                image_pixels[i] = 0;
            }
            if g_val >= 0 + brightness {
                image_pixels[i + 1] -= brightness;
            } else {
                image_pixels[i+ 1] = 0
            }
    
            if b_val >= 0 + brightness {
                image_pixels[i + 2] -= brightness;
            } else {
                image_pixels[i + 2] = 0
            }
        }
        let image
            = PhotonImage::new(image_pixels,
                               photon_image.get_width(),
                               photon_image.get_height());
        image
    }

    // This approch keeps the calling
    // routine clean as a series of continuations. 
    pub fn dec_brightness_channel(photon_image: &mut PhotonImage, brightness: u8) {
        for c in 0..=2 {
            alter_channel(photon_image, c, brightness as i16 * -1);
        }
    }
}
