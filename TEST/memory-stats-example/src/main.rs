use memory_stats::memory_stats;
use thousands::Separable;

fn main() {
    show_mem();

    println!("         字节          物理内存       虚拟内存  ");
    check_mem(10000);
    check_mem(100000);
    check_mem(1000000);
    check_mem(10000000);
    check_mem(100000000);
    check_mem(1000000000);
    check_mem(10000000000);
}

fn check_mem(bytes: usize) {
    let before = memory_stats().unwrap();
    let _text = "x".repeat(bytes);
    let after = memory_stats().unwrap();

    let physical_mem = after.physical_mem - before.physical_mem;
    let virtual_mem = after.virtual_mem - before.virtual_mem;
    println!(
        "{:>15} {:>15} {:>15}",
        bytes.separate_with_commas(),
        physical_mem.separate_with_commas(),
        virtual_mem.separate_with_commas()
    )
}

fn show_mem() {
    if let Some(usage) = memory_stats() {
        println!(
            "物理内存使用: {:>15}",
            usage.physical_mem.separate_with_commas()
        );
        println!(
            "虚拟内存使用:  {:>15}",
            usage.virtual_mem.separate_with_commas()
        );
    } else {
        println!("Couldn't get the current memory usage :(");
    }
}