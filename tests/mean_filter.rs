#[cfg(test)]
mod mean_filter_test {
    use std::{fs, time::{Duration, Instant}};

    use ayanami_cv::{
        Gray, Rgb, Image, ImageViewLike, ImageViewMutLike,
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

    fn fill_random(v: &mut [u8], mut x: u32) {
        for b in v {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            *b = x as u8;
        }
    }

    fn bench_once<const SIZE: usize>(src: &Image<Gray>, dst: &mut Image<Gray>, f: &mut MeanFilter) -> Duration {
        let start = Instant::now();
        f.filter::<_, _, _, SIZE>(src, dst).unwrap();
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
        let mut f = MeanFilter::new();

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
        let mut f = MeanFilter::new();

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

        let mut f = MeanFilter::new();
        f.filter::<_, _, _, 3>(&src, &mut dst).unwrap();

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

        let mut f = MeanFilter::new();
        f.filter::<_, _, _, 3>(&src, &mut dst).unwrap();

        assert!(collect(&dst).iter().all(|&v| v == 0));
        assert_eq!(0, dst.at((2, 2)).gray);
    }

    #[test]
    fn test_border_padding() {
        let src = Image::<Gray>::new_from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut dst = Image::<Gray>::new(3, 3);

        let mut f = MeanFilter::new();
        f.filter::<_, _, _, 3>(&src, &mut dst).unwrap();

        // 3x3 窗口，超出边界的像素按 0 补零，分母固定为 3*3=9
        let expect = vec![1, 2, 1, 3, 5, 3, 2, 4, 3];
        assert_eq!(expect, collect(&dst));
    }

    #[test]
    fn test_reuse_across_images() {
        // 图 A 5x5 滤波后，句柄状态停留在 last_index=3*5+4=19；
        // 图 B 取 19 列宽，内区第一个像素 index=1*19+1=20==last_index+1，
        // 若不做 reset 会误判为连续滑动，用 A 的 sum 增量更新 → 结果错。
        let a_src = Image::<Gray>::new_from_vec(5, 5, vec![0u8; 25]);
        let b_src = Image::<Gray>::new_from_vec(19, 5, {
            let mut data = vec![0u8; 19 * 5];
            fill_random(&mut data, 0xabcdef12);
            data
        });

        let mut f = MeanFilter::new();
        let mut da = Image::<Gray>::new(5, 5);
        f.filter::<_, _, _, 3>(&a_src, &mut da).unwrap();
        let mut db = Image::<Gray>::new(19, 5);
        f.filter::<_, _, _, 3>(&b_src, &mut db).unwrap();

        // 独立句柄单独滤波 B，结果必须一致
        let mut ffresh = MeanFilter::new();
        let mut db_fresh = Image::<Gray>::new(19, 5);
        ffresh.filter::<_, _, _, 3>(&b_src, &mut db_fresh).unwrap();

        assert_eq!(collect(&db), collect(&db_fresh));
    }

    #[test]
    fn test_rgb_uniform_interior() {
        // RGB 三通道各自独立求均值（泛化改造目标：同一帧代码同时支持 Gray/RGB）
        let mut src = Image::<Rgb>::new(5, 5);
        for i in 0..5 {
            for j in 0..5 {
                let p = src.at_mut((i, j));
                p.r = 10;
                p.g = 20;
                p.b = 30;
            }
        }
        let mut dst = Image::<Rgb>::new(5, 5);

        let mut f = MeanFilter::new();
        f.filter::<_, _, _, 3>(&src, &mut dst).unwrap();

        for i in 1..4 {
            for j in 1..4 {
                assert_eq!(10, dst.at((i, j)).r);
                assert_eq!(20, dst.at((i, j)).g);
                assert_eq!(30, dst.at((i, j)).b);
            }
        }
    }

    #[test]
    fn test_rgb_slides_incrementally() {
        // 与灰度图同款构造：A(5x5) 滤波后句柄停在 last_index=19，
        // B(19x5) 内区首像素 index=20==last_index+1，强制 RGB 走滚动分支
        let a_src = Image::<Rgb>::new(5, 5);
        let b_src = Image::<Rgb>::new_from_vec(19, 5, {
            let mut data = vec![0u8; 19 * 5 * 3];
            // 按 R,G,B 三通道分别填充不同随机种子，验证每通道独立滚动
            for k in 0..3 {
                fill_random(&mut data[k..], 0xfeed_beef ^ (k as u32));
            }
            data
        });

        let mut f = MeanFilter::new();
        let mut da = Image::<Rgb>::new(5, 5);
        f.filter::<_, _, _, 3>(&a_src, &mut da).unwrap();

        let mut db = Image::<Rgb>::new(19, 5);
        f.filter::<_, _, _, 3>(&b_src, &mut db).unwrap();

        let mut fb = MeanFilter::new();
        let mut db_fresh = Image::<Rgb>::new(19, 5);
        fb.filter::<_, _, _, 3>(&b_src, &mut db_fresh).unwrap();

        for i in 0..5 {
            for j in 0..19 {
                assert_eq!(db.at((i, j)).r, db_fresh.at((i, j)).r);
                assert_eq!(db.at((i, j)).g, db_fresh.at((i, j)).g);
                assert_eq!(db.at((i, j)).b, db_fresh.at((i, j)).b);
            }
        }
    }

    #[test]
    fn test_even_window_returns_error() {
        let src = Image::<Gray>::new(5, 5);
        let mut dst = Image::<Gray>::new(5, 5);

        let mut f = MeanFilter::new();
        let res = f.filter::<_, _, _, 2>(&src, &mut dst);

        assert!(matches!(res, Err(Error::WindowSizeMustBeOdd)));
    }
}