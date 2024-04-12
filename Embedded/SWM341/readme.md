# 为什么要在Swm341中运行Rust代码？
Swm341是一款非常优秀的国产单片机芯片，聚焦于显示控制，有着非常多的硬件资源可供使用，开发非常方便。另外Rust目前开始变得流行，执行效率也非常高，因此想在Swm341中尝试使用运行Rust开发。


## 一切从点灯开始
正如学习语言从打印hello world!开始，单片机的学习当然从点灯开始。在本工程中，将会打印一些日志，并且一秒闪烁一次。代码非常简单，在src/main中添加以下的代码即可。
```rust
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use swm341_pac as pac;

// 简单的毫秒延时函数
fn delay_ms(ms: u32) {
    for _ms in 0..ms {
        for _i in 0..20 {
            for _j in 0..10 {}
        }
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("swm341 led blinky example start...");

    let p = pac::Peripherals::take().unwrap();
    let sys_con = p.SYS;
    // 开启 gpion的时钟
    sys_con.clken0.write(|w| w.gpion().set_bit());
    let port = p.PORTN;
    // 设置portn.0 为 gpio 映射
    port.func0.write(|w| unsafe { w.pin0().bits(0) });
    // 设置 portn.0 上拉
    port.pullu.write(|w| w.pin0().set_bit());

    let gpio = p.GPION;
    // 设置 gpion.0 为输出方向
    gpio.dir.write(|w| w.pin0().set_bit());

    let mut count = 0u32;
    loop {
        // 翻转 gpion.0
        gpio.odr.modify(|r, w| w.pin0().bit(!r.pin0().bit()));
        // 打印计数值
        defmt::info!("{}", count);
        count += 1;
        delay_ms(500);
    }
}
```

然后直接执行cargo run --example blinky即可编译和下载并运行，下载非常快，不到1秒。