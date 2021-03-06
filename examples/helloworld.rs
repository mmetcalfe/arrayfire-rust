#[macro_use(af_print)]
extern crate arrayfire as af;

use af::*;

#[allow(unused_must_use)]
fn main() {
    set_device(0);
    info();

    let num_rows: u64 = 5;
    let num_cols: u64 = 3;
    let values: &[f32] = &[1.0, 2.0, 3.0];
    let indices = Array::new(values, Dim4::new(&[3, 1, 1, 1])).unwrap();

    let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);

    let a = match randu::<f32>(dims) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };
    af_print!("Create a 5-by-3 matrix of random floats on the GPU", a);

    println!("Element-wise arithmetic");
    let  b = sin(&a)
        .and_then(|x| add(&x, &1.5, false))
        .unwrap();

    let b2 = sin(&a).
        and_then(|x| {
            cos(&a)
                .and_then(|y| add(&x, &y, false))
        })
        .unwrap();

    let b3 = ! &a;
    af_print!("sin(a) + 1.5 => ", b);
    af_print!("sin(a) + cos(a) => ", b2);
    af_print!("!a => ", b3);

    let test = &a + &b;
    af_print!("a + b", test);

    // Index array using sequences
    let seqs = &[Seq::new(1u32, 3, 1), Seq::default()];
    let sub = index(&a, seqs).unwrap();
    af_print!("a(seq(1,3,1), span)", sub);

    //Index array using array and sequence
    let seq4gen = Seq::new(0u32, 2, 1);

    let mut idxrs = match Indexer::new() {
        Ok(v) => v,
        Err(e) => panic!("{}",e),
    };
    idxrs.set_index(&indices, 0, None);
    idxrs.set_index(&seq4gen, 1, Some(false));

    let sub2 = index_gen(&a, idxrs).unwrap();
    af_print!("a(indices, seq(0, 2, 1))", sub2);

    // printf("Negate the first three elements of second column\n");
    // B(seq(0, 2), 1) = B(seq(0, 2), 1) * -1;
    // af_print(B);

    println!("Fourier transform the result");
    fft(&b, 1.0, 0).map(|x| print(&x));

    println!("Grab last row & col of the random matrix");
    print(&a);
    print(&row(&a, num_rows - 1).unwrap());
    print(&col(&a, num_cols - 1).unwrap());

    let r_dims = Dim4::new(&[3, 1, 1, 1]);
    let r_input: [f32; 3] = [1.0, 1.0, 1.0];
    let r = Array::new(&r_input, r_dims).unwrap();
    let ur = set_row(&a, &r, num_rows - 1).unwrap();
    af_print!("Set last row to 1's", ur);

    let d_dims = Dim4::new(&[2, 3, 1, 1]);
    let d_input: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let d = Array::new(&d_input, d_dims).unwrap();
    af_print!("Create 2-by-3 matrix from host data", d);

    // printf("Copy last column onto first\n");
    // D.col(0) = D.col(end);
    // af_print(D);

    // // Sort A
    println!("Sort A and print sorted array and corresponding indices");
    sort_index(&a, 0, true)
        .map(| x | {
            print(&x.0);
            print(&x.1);
        });

    let u8_cnst = &constant(1 as u8, dims).unwrap();
    af_print!("u8 constant array", u8_cnst);
    println!("Is u8_cnst array float precision type ? {}", u8_cnst.is_single().unwrap());
}
