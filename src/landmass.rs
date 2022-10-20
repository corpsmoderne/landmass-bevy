use rand::prelude::{Rng,StdRng,SeedableRng};
use noise::{NoiseFn,Perlin};
use image::{ImageBuffer, Rgba, RgbaImage};

type Vec2f = (f32, f32);

pub struct Landmass {
    pub seed: u64,
    pub size: u32,
    pub data: Vec<f32>,
    pub img: image::RgbaImage,
    pub palette: image::RgbaImage
}

impl Landmass {
    pub fn with_palette(size: u32, seed: u64,
			palette: image::RgbaImage) -> Landmass {
	let mut lm = Landmass { seed, size, data: vec![],
				img: RgbaImage::new(size, size),
				palette  };
	lm.update_map(&(0.0, 0.0), 1.0);	
	lm
    }

    #[allow(dead_code)]    
    pub fn new(size: u32, seed: u64) -> Landmass {
	let palette = gen_palette();
	Landmass::with_palette(size, seed, palette)
    }

    #[allow(dead_code)] 
    fn save_palette(&self) {
	_ = self.palette.save("palette.png");
    }
    
    pub fn update_map(&mut self, pos: &Vec2f, scale: f32) {
	self.data = self.gen_land(pos, scale);
	
	self.img = ImageBuffer::from_fn(self.size, self.size, |x, y| {
	    let i = (x + y * self.size) as usize;
	    self.palette[(0, 255-self.data[i].max(0.0).min(255.0) as u32)]
	});
    }
    
    fn gen_land(&self, p: &Vec2f, scale: f32) -> Vec<f32> {
	let mut rng = StdRng::seed_from_u64(self.seed);
	let noise = Perlin::new(self.seed as u32);
	let a = 50.0 + rng.gen::<f32>() * 75.0;
	let b = rng.gen::<f32>() * 40.0;
	let pos = *p;
	let sea_level : f64 = rng.gen::<f64>() * 240.0;
	
	(0..(self.size*self.size)).into_iter()
	    .map(|i| {
		let x = - ((self.size/2) as f32) + (i % self.size) as f32;
		let y = - ((self.size/2) as f32) + (i / self.size) as f32;
		let p = ((pos.0 + x * scale) / a,
			 (pos.1 + y * scale) / a);
		let v : f64 = noise.get([p.0 as f64, p.1 as f64]).powf(3.0);
		let v1 = (v + 1.0) * 128.0;
		
		let p2 = ((pos.0 + x * scale) / b,
			  (pos.1 + y * scale) / b);
		let w : f64 = noise.get([p2.0 as f64, p2.1 as f64]);
		let v2 = (w + 1.0) * b as f64;
		(v1 + v2 - sea_level).max(0.0) as f32
	    }).collect()
    }

}


fn gen_palette() -> image::RgbaImage {
    struct Level {
	height: u32,
	color: image::Rgba<u8>
    }

    let levels =
	vec![ Level { height: 0,   color: Rgba([ 10,  10, 80, 255]) },
	      Level { height: 5,   color: Rgba([150, 150, 50,  255]) },
	      Level { height: 40,  color: Rgba([ 50, 150, 50,  255]) },
	      Level { height: 80,  color: Rgba([ 30, 120, 30,  255]) },
	      Level { height: 130, color: Rgba([150,  75, 30,  255]) },
	      Level { height: 160, color: Rgba([130,  55, 20,  255]) },
	];

    ImageBuffer::from_fn(256, 256, |_x, y| {
	for lvl in &levels {
	    if y >= 255-lvl.height {
		return lvl.color;
	    }
	}
	image::Rgba([255, 255, 255, 255])
    })
}
