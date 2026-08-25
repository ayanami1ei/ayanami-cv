#[cfg(test)]
mod image_test{
    use ayanami_cv::{color_space::GrayChannel, image::Image};

    #[test]
    fn test_new_index(){
        let m=Image::<GrayChannel>::new(640, 480);
        for i in 0..m.height(){
            let row=m.row(i);
            for j in 0..row.len(){
                println!("{:?}", row[j]);
            }
        }
    }
}