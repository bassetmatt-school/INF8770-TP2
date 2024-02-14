use image::{DynamicImage, GenericImageView, ImageFormat};
use nalgebra::{Matrix3, Matrix3x1};
use nshare::ToNdarray2;
macro_rules! load_img(
	($p:expr) => {
		image::load_from_memory_with_format(include_bytes!($p), ImageFormat::Png).unwrap()
	};
);

fn main() {
	let _images: [DynamicImage; 5] = [
		load_img!("../data/kodim01.png"),
		load_img!("../data/kodim02.png"),
		load_img!("../data/kodim05.png"),
		load_img!("../data/kodim13.png"),
		load_img!("../data/kodim23.png"),
	];
	for img in _images.iter() {
		println!("Image {:?}x{:?}", img.width(), img.height());
		println!("{:?}", img.pixels().nth(3).unwrap());
		println!("{:?}", img.to_rgb32f().pixels().nth(3).unwrap());
		let n_px = img.pixels().count() as f32;
		let mut image = vec![];
		let mut avg = img
			.to_rgb32f()
			.pixels()
			.fold(Matrix3x1::zeros(), |acc, px| {
				let px = Matrix3x1::new(px[0], px[1], px[2]);
				image.push(px);
				acc + px
			});
		avg /= n_px;
		let mut cov = image.iter().fold(Matrix3::zeros(), |acc, px| {
			acc + (px - avg) * (px - avg).transpose()
		});
		cov /= n_px;
	}
	println!("Hello, world!");
}
