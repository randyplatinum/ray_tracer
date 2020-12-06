const IMAGE_HEIGHT: u8=u8::MAX;
const IMAGE_WIDTH: u8=u8::MAX;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_HEIGHT, IMAGE_WIDTH);
    for i in (0..IMAGE_HEIGHT).rev()
    {
    	//println!("?");
    	for j in 0..IMAGE_WIDTH
    	{
    		let r = i as f64 / (IMAGE_WIDTH as f64 - 1f64);
    		let g = j as f64 / (IMAGE_HEIGHT as f64 - 1f64);
    		let b = 0.25f64;

    		let ir = (255.999 * r) as u32;
    		let ig = (255.999 * g) as u32;
    		let ib = (255.999 * b) as u32;

    		println!("{} {} {}", ir, ig, ib);
    	}
    }
}
