# Blender与Bevy工作流

继续增强Blender和Bevy的工作流，目前的插件在GitHub上可用

- • GPU 驱动渲染(GPU-Driven Rendering)  
    ECS 关系(ECS Relationships)
    
- • `no_std` 支持(no_std Support)
    
- • 程序化大气散射(Procedural Atmospheric Scattering)
    
- • 贴花(Decals)
    
- • 遮挡剔除(Occlusion Culling)
    
- • 改进的生成 API(Improved Spawn API)
    
- • 统一的错误处理(Unified Error Handling)
    
- • 更快的坐标变换传播(Faster Transform Propagation)

- **GPU 驱动的渲染**：Bevy 现在会尽可能地在 GPU 上执行更多的渲染工作，大幅提升在处理大型复杂场景时速度。
    
- **过程性大气散射**：以较低的性能开销模拟逼真的、基于物理原理的类似地球的天空，且可在一天中的任何时刻进行模拟。
    
- **贴图**：将纹理动态地分层到已渲染的网格上。
    
- **遮挡剔除**：通过不渲染被其他物体遮挡的对象来提升性能。
    
- **ECS 关系**：ECS 最热门的功能之一终于来了：让你能够轻松且稳健地对实体与实体之间的连接进行建模并处理相关工作。虽然存在一些不完美的地方，但我们很高兴今天能为用户提供一个简单而可靠的解决方案。
    
- **改进的 Spawn API**：现在生成实体层级结构要容易得多了！
    
- **统一的错误处理**：Bevy 现在具备一流的错误处理支持，使其既简单易用、灵活且符合人体工程学，同时也让调试变得更加容易！
    
- **no_std 支持**：Bevy 本身以及我们的大量子 crate 不再依赖于 Rust 的标准库，这让你可以在从现代游戏主机到 Gameboy Advance 等各种设备上使用相同的引擎。
    
- **更快的变换传播**：我们显著提升了同时处理更多对象时的变换传播性能，尤其是在这些对象为静态的情况下。



