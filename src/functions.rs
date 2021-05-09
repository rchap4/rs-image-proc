/**
 * Copyright 2021 RChapman et. Al.  
 * This file is part of rs-image-proc.
 *
 *   rs-image-proc is free software: you can redistribute it and/or modify
 *   it under the terms of the Affero GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   rs-image-proc is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   Affero GNU General Public License for more details.
 *
 *   You should have received a copy of the Affero GNU General Public License
 *   along with rs-image-proc.  If not, see <https://www.gnu.org/licenses/>.
 */
use photon_rs::channels::alter_channel;
use photon_rs::PhotonImage;

// One rift on how to decrease image brighness based on
// approch used in Photon RS inc_brightness.  However because
// this needed to return a new image not favored as other effect calls
// mutate the PhotoImage passed to the function.
#[allow(dead_code)]
pub fn dec_brightness(photon_image: &mut PhotonImage, brightness: u8) -> PhotonImage {
    let end = photon_image.get_raw_pixels().len() - 4;
    let mut image_pixels = photon_image.get_raw_pixels();
    for i in (0..end).step_by(4) {
        let r_val = image_pixels[i];
        let g_val = image_pixels[i + 1];
        let b_val = image_pixels[i + 2];

        if r_val >= brightness {
            image_pixels[i] -= brightness;
        } else {
            image_pixels[i] = 0;
        }
        if g_val >= brightness {
            image_pixels[i + 1] -= brightness;
        } else {
            image_pixels[i + 1] = 0
        }

        if b_val >= brightness {
            image_pixels[i + 2] -= brightness;
        } else {
            image_pixels[i + 2] = 0
        }
    }
    PhotonImage::new(
        image_pixels,
        photon_image.get_width(),
        photon_image.get_height(),
    )
}

// This approch keeps the calling
// routine clean as a series of continuations.
pub fn dec_brightness_channel(photon_image: &mut PhotonImage, brightness: u8) {
    let amount = i16::from(brightness);
    for c in 0..=2 {
        alter_channel(photon_image, c, -amount);
    }
}
