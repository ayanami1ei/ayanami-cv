#[cfg(test)]
mod image_test {
    use ayanami_cv::{Gray, Image, image::image_view::ImageView};

    #[test]
    fn test_new_index() {
        let m = Image::<Gray>::new(640, 480);
        for i in 0..m.height() {
            let row = m.row(i);
            for j in 0..row.len() {
                println!("m: {:?}", row[j]);
            }
        }
    }

    #[test]
    fn test_new_row_iter() {
        let m = Image::<Gray>::new(640, 480);
        for r in m.row_iter() {
            for j in 0..r.len() {
                println!("m: {:?}", r[j]);
            }
        }
    }

    #[test]
    fn test_new_view() {
        let m = Image::<Gray>::new(640, 480);
        for i in 0..m.height() {
            let row = m.row(i);
            for j in 0..row.len() {
                println!("m: {:?}", row[j]);
            }
        }

        let n=ImageView::<Gray>::new(m.width(), m.height(), m.data());
        for i in 0..n.height() {
            let row = n.row(i);
            for j in 0..row.len() {
                println!("n: {:?}", row[j]);
            }
        }
    }
}
