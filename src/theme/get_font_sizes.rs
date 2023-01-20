use array_macro::array;

pub fn get_font_sizes(base: f64) -> [FontSizePair; 10] {
    let mut font_sizes = [0.0; 10];

    for (index, item) in font_sizes.iter_mut().enumerate() {
        let i = (index as i64 - 1) as f64;
        let base_size = base * std::f64::consts::E.powf(i / 5.0);

        let mut int_size = base_size.ceil();
        if index > 1 {
            int_size = base_size.floor();
        }

        *item = (int_size / 2.0).floor() * 2.0;
    }

    font_sizes[1] = base;

    let mut pairs = array!(FontSizePair::default(); 10);

    for index in 0..font_sizes.len() {
        let size = font_sizes[index];
        let height = size + 8.0;
        pairs[index].size = size;
        pairs[index].line_height = height / size;
    }

    pairs
}

#[derive(Debug, Clone, Default)]
pub struct FontSizePair {
    pub size: f64,
    pub line_height: f64,
}

#[test]
fn it_works() {
    println!("{:#?}", get_font_sizes(12.0));

    println!("{:#?}", get_font_sizes(14.0));

    println!("{:#?}", get_font_sizes(24.0));
}
