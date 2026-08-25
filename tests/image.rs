#[cfg(test)]
mod image_test {
    use ayanami_cv::{Gray, Image};

    #[test]
    fn test_new_index() {
        let m = Image::<Gray>::new(640, 480);
        for i in 0..m.height() {
            let row = m.row(i);
            for j in 0..row.len() {
                println!("{:?}", row[j]);
            }
        }
    }
}
