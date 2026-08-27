#[cfg(test)]
mod image_test {
    use ayanami_cv::{
        Gray, Image,
        image::{ImageViewLike, ImageViewMutLike, image_view::ImageView},
    };

    #[test]
    fn test_new_row_iter() {
        let mut m = Image::<Gray>::new(640, 480);

        for mut r in m.row_iter_mut() {
            for j in 0..r.len() {
                r[j].gray += 1;
            }
        }

        for r in m.row_iter() {
            for j in 0..r.len() {
                assert_eq!(1, r[j].gray)
            }
        }
    }

    #[test]
    fn test_new_pixel_iter() {
        let mut m = Image::<Gray>::new(640, 480);
        for p in m.pixel_iter_mut() {
            p.gray += 1;
        }
        for p in m.pixel_iter() {
            assert_eq!(1, p.gray)
        }
    }

    #[test]
    fn test_new_view() {
        let m = Image::<Gray>::new(640, 480);
        let n = ImageView::<Gray>::new_from_image(&m);
        for i in 0..m.height() {
            for j in 0..m.width() {
                assert_eq!(m.at((i, j)).gray, n.at((i, j)).gray);
            }
        }
    }
}
