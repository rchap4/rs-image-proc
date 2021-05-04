extern crate photon_rs;
use photon_rs::effects;
use photon_rs::native::{ open_image, save_image };
use photon_rs::PhotonImage;
use photon_rs::transform::resize;
use structopt::StructOpt;

mod functions;
//use functions::functions::dec_brightness;
use functions::functions::dec_brightness_channel;

#[derive(Debug, StructOpt)]
#[structopt(name = "Photon Image Tool", about = "Simple image processing")]
struct CliOptions {
    
    /// Resize image
    #[structopt(long = "resize")]
    resize: bool,

    // orginal implementation of resize that took
    // new width x height input, current scale behavior
    // maintains aspect ratio
    //516 x 783
    /// New Width
    // #[structopt(short = "w", long = "width", 
    //             required_if("resize", "true"))]
    // new_width: Option<u32>,

    // /// New Height
    // #[structopt(short = "h", long = "height",
    //             required_if("resize", "true"))]
    // new_height: Option<u32>,

    /// Scale the image instead of resize
    #[structopt(short= "s", long = "scale",
                required_if("resize", "true"),
                help = "Percent to scale image")]
    scale: Option<f32>,
    
    /// Image correction
    #[structopt(long = "correct")]
    correct:bool,

    /// Brightness value
    #[structopt(short = "b", long = "brighten")]
    brightness: Option<u8>,

    /// Darken value
    #[structopt(short = "d", long = "darken")]
    darken: Option<u8>,

    /// Contrast value
    #[structopt(short = "c", long = "contrast")]
    contrast: Option<f32>,

    /// Input image
    #[structopt(parse(from_os_str))]
    input_image: std::path::PathBuf,

    /// Output image
    #[structopt(parse(from_os_str))]
    output_image: std::path::PathBuf

}

fn brighten_contrast(image: &mut PhotonImage,
                     brighten: Option<u8>,
                     darken: Option<u8>,
                     contrast: Option<f32>) {

    brighten.and_then(|b| {
        Some(effects::inc_brightness(image, b))
    });

    darken.and_then(|d| {
        Some(dec_brightness_channel(image,d))
    });

    contrast.and_then(|c| {
        Some(effects::adjust_contrast(image, c))
    });

}

// unneeded alternative approch to decrease brightness
// fn darken_image(image: &mut PhotonImage,
//                 darken: Option<u8>) -> Option<PhotonImage> {

//     darken.and_then(|b| {
//         let dark_image = dec_brightness(image,b);
//         Some(dark_image)
//     })
// }

fn resize_image(image: &mut PhotonImage,
                new_width: u32,
                new_height: u32) -> Option<PhotonImage> {
        //println!("Resize...");
        Some(resize(&image,
                    new_width,
                    new_height,
                    photon_rs::transform::SamplingFilter::Nearest))
}

fn image_scale_size(width:u32,
                    height:u32,
                    scale: f32) -> (u32,u32) {
    let img_width = (width as f32 * scale) as u32 ;
    let img_height = (height as f32 * scale) as u32;
    (img_width, img_height)
}

fn main() {
    let cli_options = CliOptions::from_args();

    if let Some(img) = cli_options.input_image.to_str() {
        let mut image = open_image(img).expect("Could not open image");

        if cli_options.correct {
            brighten_contrast(&mut image,
                              cli_options.brightness,
                              cli_options.darken,
                              cli_options.contrast);   
        }


        // unneeded function for another approch goes with darken_image
        // function above
        // let dark_image = match cli_options.darken {
        //         Some(d) => darken_image(&mut image, Some(d)),
        //         None => None
        // };

        let new_image = match cli_options.resize {
            true => {
                cli_options.scale.and_then(|s| {
                    let (w,h) =
                        image_scale_size(image.get_width(),
                                         image.get_height(),
                                         s);
                    //resize_image(&mut image, cli_options.new_width.unwrap(), cli_options.new_height.unwrap())
                    resize_image(&mut image, w, h)
                })
            },
            false => None
        };
        
        // unneeded experiments with cli options/resize/behavior
        //and_then = flatmap
        // let new_image = cli_options.resize.and_then(|b| {
        //         if b {
        //             let (w,h) =
        //                 image_scale_size(image.get_width(), image.get_height(), cli_options.scale.unwrap());
        //             resize_image(&mut image, w, h)
        //         } else {
        //             None
        //         }
        //     }
        // );
        
        // let new_image = match cli_options.resize {
        //     Some(b) => if b == true {
        //         Some(resize_image(&mut image, cli_options.new_width.unwrap(), cli_options.new_height.unwrap()))
        //     } else {
        //         None
        //     }
        //     None => None
        // };
        
        // Since structops shouldn't let you get this far without
        // an input and output path we'll expect/panic
        match new_image {
            Some(img) => save_image(img, cli_options.output_image
                                            .to_str()
                                            .expect("Output path not provided")),
            None => save_image(image, cli_options.output_image
                                            .to_str()
                                            .expect("Output path not provided"))
        }

    }
}
