#[cfg(test)]
mod mean_filter_test {
    use ayanami_cv::{
        Gray, Image, ImageViewLike, ImageViewMutLike,
        algorithm::neighborhood::{error::Error, filters::MeanFilter},
    };

    fn collect(img: &Image<Gray>) -> Vec<u8> {
        img.pixel_iter().map(|p| p.gray).collect()
    }

    #[test]
    fn test_uniform_image_interior() {
        let src = Image::<Gray>::new_from_vec(5, 5, vec![10u8; 25]);
        let mut dst = Image::<Gray>::new(5, 5);

        let f = MeanFilter;
        f.filter::<_, _, 3>(&src, &mut dst).unwrap();

        for i in 1..4 {
            for j in 1..4 {
                assert_eq!(10, dst.at((i, j)).gray);
            }
        }
    }

    #[test]
    fn test_single_hot_pixel() {
        let mut src = Image::<Gray>::new_from_vec(5, 5, vec![0u8; 25]);
        src.at_mut((2, 2)).gray = 5;
        let mut dst = Image::<Gray>::new(5, 5);

        let f = MeanFilter;
        f.filter::<_, _, 3>(&src, &mut dst).unwrap();

        assert!(collect(&dst).iter().all(|&v| v == 0));
        assert_eq!(0, dst.at((2, 2)).gray);
    }

    #[test]
    fn test_border_padding() {
        let src = Image::<Gray>::new_from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut dst = Image::<Gray>::new(3, 3);

        let f = MeanFilter;
        f.filter::<_, _, 3>(&src, &mut dst).unwrap();

        // 3x3 窗口，超出边界的像素按 0 补零，分母固定为 3*3=9
        let expect = vec![1, 2, 1, 3, 5, 3, 2, 4, 3];
        assert_eq!(expect, collect(&dst));
    }

    #[test]
    fn test_even_window_returns_error() {
        let src = Image::<Gray>::new(5, 5);
        let mut dst = Image::<Gray>::new(5, 5);

        let f = MeanFilter;
        let res = f.filter::<_, _, 2>(&src, &mut dst);

        assert!(matches!(res, Err(Error::WindowSizeMustBeOdd)));
    }
}