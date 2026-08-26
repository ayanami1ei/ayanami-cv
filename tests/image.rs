#[cfg(test)]
mod image_test {
    use ayanami_cv::{Gray, Image, image::image_view::ImageView};

    #[test]
    fn test_new_index() {
        let mut m = Image::<Gray>::new(640, 480);
        for i in 0..m.height() {
            let row = m.row(i);
            for j in 0..row.len() {
                println!("m: {:?}", row[j]);
            }
        }

        let n=ImageView::<Gray>::new(m.width(), m.height(), m.data_mut());
        for i in 0..n.height() {
            let row = n.row(i);
            for j in 0..row.len() {
                println!("n: {:?}", row[j]);
            }
        }
    }
}
