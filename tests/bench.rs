#[cfg(test)]
mod mean_filter_bench {
    use std::{
        fs,
        time::{Duration, Instant},
    };

    use ayanami_cv::{Gray, Image, algorithm::neighborhood::filters::MeanFilter};

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
                    println!("内存:{}", v.trim());
                    break;
                }
            }
        }

        if cfg!(debug_assertions) {
            println!("crate: ayanami-cv {} (debug profile)", env!("CARGO_PKG_VERSION"));
        } else {
            println!("crate: ayanami-cv {} (release profile)", env!("CARGO_PKG_VERSION"));
        }
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

    #[test]
    fn bench_mean_filter_7x7_1920x1080() {
        print_system_info();

        const WIDTH: usize = 1920;
        const HEIGHT: usize = 1080;
        const SIZE: usize = 7;

        let mut data = vec![0u8; WIDTH * HEIGHT];
        fill_random(&mut data, 0x12345678);

        let src = Image::<Gray>::new_from_vec(WIDTH, HEIGHT, data);
        let mut dst = Image::<Gray>::new(WIDTH, HEIGHT);

        let mut f = MeanFilter::new();
        let start = Instant::now();
        f.filter::<_, _, _, SIZE>(&src, &mut dst).unwrap();
        let elapsed: Duration = start.elapsed();

        let secs = elapsed.as_secs_f64();
        println!(
            "1920x1080, 核 7x7, 滤波 1 次: {} ms ({} s), 像素数 {} ({} MP/s)",
            elapsed.as_millis(),
            format!("{secs:.3}"),
            WIDTH * HEIGHT,
            format!("{:.1}", (WIDTH * HEIGHT) as f64 / secs / 1e6)
        );
    }
}