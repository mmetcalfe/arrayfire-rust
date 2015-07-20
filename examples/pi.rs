extern crate arrayfire as af;
extern crate time;

use time::PreciseTime;
use af::Dim4;
use af::Aftype;

#[allow(unused_must_use)]
#[allow(unused_variables)]
fn main() {
    af::set_device(0);
    af::info();
    let samples = 20_000_000;
    let dims = Dim4::new(&[samples, 1, 1, 1]);

    let x = &af::randu(dims, Aftype::F32).unwrap();
    let y = &af::randu(dims, Aftype::F32).unwrap();

    let start = PreciseTime::now();

    for bench_iter in 0..100 {
        let pi_val = af::add(&(x*x), &(y*y))
            .and_then( |z| af::sqrt(&z) )
            .and_then( |z| af::le(&z, &af::constant(1, dims).unwrap()) )
            .and_then( |z| af::sum_all(&z) )
            .map( |z| z.0 * 4.0/(samples as  f64) )
            .unwrap();
    }

    let end = PreciseTime::now();

    println!("Estimated Pi Value in {} seconds", start.to(end) / 100);
}