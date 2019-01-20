#![feature(test)]
#[macro_use]
extern crate bencher;
use bencher::Bencher;
extern crate test;

extern crate image;
extern crate img_hash;

use std::path::Path;
use img_hash::{ImageHash, HashType};

fn bread(a: &mut Bencher) {

    a.iter(|| {
        let bread = image::open(&Path::new("Benchmarks/bread.png")).unwrap();
        let hash1 = ImageHash::hash(&bread, 8, HashType::Gradient);
        //println!("Image1 hash: {}", hash1.to_base64());
    });
}
fn grey(a: &mut Bencher) {

    a.iter(|| {
        let bread = image::open(&Path::new("Benchmarks/grey.png")).unwrap();
        let hash1 = ImageHash::hash(&bread, 8, HashType::Gradient);
        //println!("Image1 hash: {}", hash1.to_base64());
    });
}
fn colourful(a: &mut Bencher) {

    a.iter(|| {
        let bread = image::open(&Path::new("Benchmarks/colours.png")).unwrap();
        let hash1 = ImageHash::hash(&bread, 8, HashType::Gradient);
        //println!("Image1 hash: {}", hash1.to_base64());
    });
}

benchmark_group!(
    benches,
    bread,
    grey,
    colourful
);
benchmark_main!(benches);
