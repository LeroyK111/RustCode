// num_cpus crate 用于确定物理CPU内核的数量或可以在系统上有效执行的并行任务的数量。
fn main() {
    println!("Logical CPUs: {}", num_cpus::get());
    println!("Physical CPUs: {}", num_cpus::get_physical());
}