use cairo::{Format, ImageSurface};
use drm::control::ClipRect;
use std::{
    ops::Sub,
    path::{Path, PathBuf},
    str::FromStr,
    time::{Duration, SystemTime},
};
use video_rs::{Decoder, Location};

mod display;
mod fonts;

use display::DrmBackend;

fn main() {
    let mut drm = DrmBackend::open_card().unwrap();
    let (width, height) = (2008, 64);

    println!("{} {}", width, height);

    let mut surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();

    let mut decoder = Decoder::new(Location::File(PathBuf::from_str("./everyhousebar.mp4").unwrap()))
        .expect("failed to create decoder");

    let now = SystemTime::now();

    for frame_r in decoder.decode_iter() {
        if let Ok((time, frame)) = frame_r {
            let current_time = now.elapsed().unwrap();
            let diff = time.as_secs_f64() - current_time.as_secs_f64();
            println!("vtime: {:.2}, ctime: {:.2}", time.as_secs(), current_time.as_secs_f64());

            let mut data = surface.data().unwrap();

            let mut pixel_number: usize = 0;
            let mut offset: usize = 0;
            for _ in 0..(height * width) as usize {
                let x = pixel_number / height;
                let y = height - pixel_number % height - 1;

                let pixel = frame.slice(ndarray::s![y, x, ..]).to_slice().unwrap();
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                data[offset + 0] = b as u8; // blue
                data[offset + 1] = g as u8; // green
                data[offset + 2] = r as u8; // red
                                            // data[offset + 3] = a as u8;

                offset += 4;
                pixel_number += 1;
            }

            drm.map().unwrap().as_mut()[..data.len()].copy_from_slice(&data);

            drm.dirty(&[ClipRect::new(0, 0, height as u16, width as u16)])
                .unwrap();
        }
    }
}
