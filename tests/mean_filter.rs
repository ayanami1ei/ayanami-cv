#[cfg(test)]
mod mean_filter_test {
    use std::{fs, time::{Duration, Instant}};

    use ayanami_cv::{
        Gray, Image, ImageViewLike, ImageViewMutLike,
        algorithm::neighborhood::{error::Error, filters::MeanFilter},
    };

    fn collect(img: &Image<Gray>) -> Vec<u8> {
        img.pixel_iter().map(|p| p.gray).collect()
    }

    fn print_system_info() {
        println!("========== 系统信息 ==========");
        println!(
            "OS: {} {} (arch: {})",
            std::env::consts::OS,
            fs::read_to_string("/proc/sys/kernel/ostype")
                .or(fs::read_to_string("/proc/sys/kernel/osrelease"))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "unknown".into()),
            std::env::consts::ARCH
        );

        if let Ok(cpu) = fs::read_to_string("/proc/cpuinfo") {
            let mut model = None;
            let mut cores = 0;
            for line in cpu.lines() {
                if let Some(m) = line.strip_prefix("model name") {
                    model = m.split(':').nth(1).map(|s| s.trim().to_string());
                }
                if line.starts_with("processor") {
                    cores += 1;
                }
            }
            println!(
                "CPU: {} ({} 逻辑核心, 可用并行度 {})",
                model.unwrap_or_else(|| "unknown".into()),
                cores,
                std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0)
            );
        }

        if let Ok(mem) = fs::read_to_string("/proc/meminfo") {
            for line in mem.lines() {
                if let Some(v) = line.strip_prefix("MemTotal:") {
                    println!("内存:{} kB", v.trim());
                    break;
                }
            }
        }

        println!(
            "crate: ayanami-cv {} ({} profile)",
            env!("CARGO_PKG_VERSION"),
            if cfg!(debug_assertions) { "debug" } else { "release" }
        );
        println!("==============================");
    }

    fn fill_random(v: &mut Vec<u8>, mut x: u32) {
        for b in v {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            *b = x as u8;
        }
    }

    fn bench_once<const SIZE: usize>(src: &Image<Gray>, dst: &mut Image<Gray>, f: &mut MeanFilter) -> Duration {
        let start = Instant::now();
        f.filter::<_, _, SIZE>(src, dst).unwrap();
        start.elapsed()
    }

    #[test]
    fn speed_vs_kernel_size() {
        print_system_info();

        const WIDTH: usize = 1920;
        const HEIGHT: usize = 1080;

        let mut data = vec![0u8; WIDTH * HEIGHT];
        fill_random(&mut data, 0x12345678);
        let src = Image::<Gray>::new_from_vec(WIDTH, HEIGHT, data);
        let mut dst = Image::<Gray>::new(WIDTH, HEIGHT);
        let mut f = MeanFilter;

        let t3 = bench_once::<3>(&src, &mut dst, &mut f);
        let t5 = bench_once::<5>(&src, &mut dst, &mut f);
        let t7 = bench_once::<7>(&src, &mut dst, &mut f);

        println!("1920x1080 同尺寸, 核 3x3: {}ms, 5x5: {}ms, 7x7: {}ms", t3.as_millis(), t5.as_millis(), t7.as_millis());
        println!("速度比 3x3:5x5:7x7 = 1 : {:.2} : {:.2}", t5.as_secs_f64() / t3.as_secs_f64(), t7.as_secs_f64() / t3.as_secs_f64());

        // 核越大耗时越多，留 0.5 容差抗干扰
        assert!(t5.as_secs_f64() >= t3.as_secs_f64() * 0.5);
        assert!(t7.as_secs_f64() >= t5.as_secs_f64() * 0.5);
    }

    #[test]
    fn speed_vs_image_area() {
        print_system_info();

        const WIDTH: usize = 1920;
        const HEIGHT: usize = 1080;
        const MULTS: [usize; 5] = [1, 2, 4, 6, 8];

        let src = Image::<Gray>::new_from_vec(WIDTH, HEIGHT, {
            let mut data = vec![0u8; WIDTH * HEIGHT];
            fill_random(&mut data, 0x9abcdef0);
            data
        });
        let mut dst = Image::<Gray>::new(WIDTH, HEIGHT);
        let mut f = MeanFilter;

        let base = bench_once::<3>(&src, &mut dst, &mut f);

        println!("同核 3x3, 基准 1920x1080 = {}ms:", base.as_millis());
        let mut prev = base.as_secs_f64();
        for &m in &MULTS[1..] {
            let s = Image::<Gray>::new(WIDTH, HEIGHT * m);
            let mut d = Image::<Gray>::new(WIDTH, HEIGHT * m);
            let t = bench_once::<3>(&s, &mut d, &mut f);
            println!("  {}x 面积 ({}x{}): {}ms, 耗时比 {:.2}x, 预期 {:.2}x", m, WIDTH, HEIGHT * m, t.as_millis(), t.as_secs_f64() / base.as_secs_f64(), m);
            // 面积越大耗时越多，留 0.5 容差
            assert!(t.as_secs_f64() >= prev * 0.5);
            prev = t.as_secs_f64();
        }
    }

    #[test]
    fn test_uniform_image_interior() {
        let src = Image::<Gray>::new_from_vec(5, 5, vec![10u8; 25]);
        let mut dst = Image::<Gray>::new(5, 5);

        let mut f = MeanFilter;
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

        let mut f = MeanFilter;
        f.filter::<_, _, 3>(&src, &mut dst).unwrap();

        assert!(collect(&dst).iter().all(|&v| v == 0));
        assert_eq!(0, dst.at((2, 2)).gray);
    }

    #[test]
    fn test_border_padding() {
        let src = Image::<Gray>::new_from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut dst = Image::<Gray>::new(3, 3);

        let mut f = MeanFilter;
        f.filter::<_, _, 3>(&src, &mut dst).unwrap();

        // 3x3 窗口，超出边界的像素按 0 补零，分母固定为 3*3=9
        let expect = vec![1, 2, 1, 3, 5, 3, 2, 4, 3];
        assert_eq!(expect, collect(&dst));
    }

    #[test]
    fn test_even_window_returns_error() {
        let src = Image::<Gray>::new(5, 5);
        let mut dst = Image::<Gray>::new(5, 5);

        let mut f = MeanFilter;
        let res = f.filter::<_, _, 2>(&src, &mut dst);

        assert!(matches!(res, Err(Error::WindowSizeMustBeOdd)));
    }
}