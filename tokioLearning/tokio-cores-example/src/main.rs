

use core_affinity::CoreId;

/// 获取应用程序使用的CPU内核
/// 如果没有指定范围，它将使用所有可用的内核
pub fn get_cpu_cores(range: Option<&str>) -> anyhow::Result<Vec<CoreId>> {
    let available_cores =
        core_affinity::get_core_ids().ok_or(anyhow::anyhow!("Failed to get available cores"))?;

    // 记录可用的核数
    for core in &available_cores {
        println!("可用内核: {}", core.id);
    }

    match range.map(parse_range_usize) {
        None => Ok(available_cores),
        Some(Err(err)) => Err(err),
        Some(Ok(range)) => {
            let cores = available_cores
                .into_iter()
                .filter(|core| range.contains(&core.id))
                .collect::<Vec<CoreId>>();
            Ok(cores)
        }
    }
}

/// 将一个范围字符串解析为一个usize向量
///
/// # 参数
/// - range_str: &str - 要解析的范围字符串
///
/// # 返回
/// - Result<Vec<usize>, anyhow::Error> - 解析范围
///
/// # 例子
/// ```
/// use notpu::utils::parse_range_usize;
///
/// let range = parse_range_usize("0-3").unwrap();
/// assert_eq!(range, vec![0, 1, 2]);
///
/// let range = parse_range_usize("0,1,2,3").unwrap();
/// assert_eq!(range, vec![0, 1, 2, 3]);
/// ```
pub fn parse_range_usize(range_str: &str) -> anyhow::Result<Vec<usize>> {
    // 解析两种格式：0-3或0,1,2,3
    if range_str.contains('-') {
        let mut range = range_str.split('-');
        let start = range
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid range"))?;
        let end = range
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid range"))?;
        let start = start
            .parse::<usize>()
            .map_err(|_| anyhow::anyhow!("Invalid range"))?;
        let end = end
            .parse::<usize>()
            .map_err(|_| anyhow::anyhow!("Invalid range"))?;

        Ok((start..end).collect::<Vec<usize>>())
    } else {
        let range = range_str
            .split(',')
            .map(|s| {
                s.parse::<usize>()
                    .map_err(|_| anyhow::anyhow!("Invalid range"))
            })
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(range)
    }
}

fn main() -> anyhow::Result<()> {
    // 使用CPU内核
    let args = Some("0-7");
    let cpu_cores: Vec<CoreId> = get_cpu_cores(args)?;

    // 让我们构建tokio运行时
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(cpu_cores.len().max(32))
        .on_thread_start(move || {
            // 这里我们使用core_affinity为worker随机选择一个核心
            use rand::seq::SliceRandom;
            // 选择一个CPU内核来运行工作线程
            let mut rng = rand::thread_rng();
            let core = cpu_cores.choose(&mut rng).unwrap();
            if core_affinity::set_for_current(*core) {
                println!("将工作线程放在 {} 内核上运行", core.id);
            } else {
                println!("将工作线程放在 {} 在内核上失败", core.id);
            }
        })
        .enable_all()
        .build()?;

    // 进入运行时
    let _guard = tokio_runtime.enter();

    // run
    tokio_runtime.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    Ok(())
}