# 微软：The Windows Kernel

例如，DWriteCore（DirectWrite的Windows应用程序SDK实现，用于高质量文本渲染和字体解析），这个项目大约包括了152000行Rust代码。

微软还在试验在Windows的GDI（图形设备接口）和Win32k组件中使用Rust。


# Figma：Multiplayer

全球知名的产品协同设计工具Figma，其中实时协作编辑功能“Multiplayer”，可以帮助用户以一种快速且轻松的方式远程处理一个共同的项目、共享文件和审查设计。

Figma的Multiplayer服务器最初是用TypeScript编写的，但随着Figma变得越来越流行，服务器无法应付。

单线程的TypeScript无法并行处理服务器操作。

Rust的低内存使用率和多线程功能极大地固定了多人服务器，因此峰值平均CPU使用率下降了6倍，峰值最坏情况下的文件保存时间加快了16.4倍。


# Coursera：对编程作业进行评分

由斯坦福大学教授发起的知名大型公开在线课程项目Coursera，在亚马逊EC2容器服务（ECS）管理的加固Docker容器中安全地对作业提交进行分级。

尽管ECS提供了自动化功能，Coursera仍需要对评分过程进行额外的协调。这涉及到存储在AmazonS3中的提交的安全处理以及分级容器中分级脚本的执行。

为了应对这些挑战，Coursera采用了Rust，因为它承诺对过程中遇到的许多安全漏洞具有免疫力。

# npm：授权服务

npm是世界上最大的软件注册中心，每天处理约13亿次软件包下载。          

npm的工程师发现，他们的授权服务（确保只有授权用户才能发布包）存在令人担忧的CPU性能限制。

他们使用Rust重新编写了这项服务，毫不奇怪，该服务已经运行了一年多，没有发出任何警报。非常幸福！

# Solana

Solana是一个使用Rust编写的快速、去中心化和超高效的区块链。

Solana速度极快，块时间为400毫秒，每秒处理大约3000个事务。

虽然这只有在测试网络条件下才能实现，但Solana每秒可能处理高达65000笔交易。

它要——

较比特币快10000倍         

较以太坊快4000倍

较Visa快2.5倍


Solana的创始人还选择了Rust（而不是以太坊区块链的流行语言Solidity），这样他们就可以吸引能够构建高质量可扩展程序（智能合约）的开发者，而不是复制粘贴现有的智能合约代码。


# 其他
其他一些在代码库中使用Rust的项目有：

Deliveroo，一种流行的送餐服务，可以在送餐网络中快速做出分配决定

1Password，一种密码管理服务，为其所有客户端应用程序的整个后端（加密、网络、数据库和业务逻辑）赋能

Atlassian，用于分析pb级的源代码服务

Cloudflare，用于边缘计算和安全服务

使用Rust的Cloudflare还开发了Pingora，这是一种新的HTTP代理，每天可处理超过1万亿的请求。

Yelp，在一个为实时A/B测试构建的框架中

Dropbox，在其核心文件存储系统中

Honeypot，在Searchspot，他们用于寻找顶尖科技人才的搜索引擎中

HuggingFace，在他们最新的开源机器学习框架Candle中

ntpd-rs, 是一个工具，用于同步你的计算机时钟，实现NTP和NTS协议。它是用Rust编写的，重点是安全性和稳定性。它支持客户端和服务器端。

aerugo,用Rust编写的面向安全应用的实时操作系统。该项目是欧洲航天局开发的，针对基于32位ARM Cortex-M7处理器的ATSAMV71Q21微控制器。它的设计灵感来自于纯函数式编程范式和transputers架构。


```
XIU

XIU是用纯Rust开发的一款简单和安全的流媒体服务器，目前支持的流媒体协议包括RTMP[cluster]/RTSP/WebRTC[Whip/Whep]/HLS/HTTPFLV。

支持多平台（Linux/Mac/Windows）



支持RTMP

支持发布和订阅H264/AAC 直播流；

支持秒开（Gop cache）；

支持转换到HLS/HTTP-FLV协议；

支持部署集群；

支持RTSP

支持通过TCP（Interleaved）和UDP发布或订阅H.265/H.264/AAC流；

支持转换到RTMP/HLS/HTTP-FLV协议；


支持WebRTC（Whip/Whep）

支持使用Whip发布rtc流；

支持使用Whep订阅rtc流；

支持转换到RTMP/HLS/HTTP-FLV协议；


支持订阅HLS/HTTPFLV直播流


支持命令行或者配置文件配置服务


支持HTTP API/notify

支持查询流信息；

支持流事件通知；

支持token鉴权


支持把直播流录制成HLS协议(m3u8+ts)文件
```

Servo - 并行浏览器引擎, Servo是一个用Rust语言编写的web浏览器引擎原型，支持WebGL和WebGPU，适用于桌面、移动和嵌入式应用程序。它目前在64位macOS、64位Linux、64位Windows和Android上开发。


Motūrus OS, Motūrus项目是为云构建的一个简单、快速、安全的操作系统(Motūrus OS)。更具体地说，Motūrus OS(有时称为Motor OS)是一种针对基于虚拟机(如web服务、“无服务器”、边缘缓存等)的新操作系统。Motūrus OS是一个基于微内核的操作系统，用Rust构建，专门针对虚拟化工作负载。它目前支持基于x64 kvm的虚拟机。

# Ockam
Ockam是一个开源软件工具包，专注于简化可信互联设备和应用程序的构建。它提供了一组编程库和命令行工具，用于协调端到端加密、相互身份验证、密钥管理、凭证管理和授权策略。Ockam旨在设备和云服务之间安全地交换信息，强调身份验证和通信安全。最初是用C语言开发的，Ockam开发团队决定用Rust完全重写Ockam。

Ockam的核心架构涉及一系列复杂的加密和基于消息的协议。在涉及加密消息协议的场景中，如果处理不当，很容易出现安全漏洞。Rust在内存安全、性能和并发性等方面具有固有的优势，很好地符合Ockam等应用程序的需求，这些应用程序要求高性能，并涉及复杂的加密操作。

# RisingWave
RisingWave是一个分布式SQL流数据库，提供最简单、最经济的方式来处理和管理流数据，并具有最大的可靠性。

# sudo-rs

用Rust重新编写的sudo和su，是一个面向内存安全的实现。sudo-rs是用Rust编写的，最低要求的Rust版本是1.70。如果你的Linux发行版没有打包该版本(或更高版本)，可以通过rustup安装最新版本。

sudo-rs支持的功能比sudo少，这其中有些是有意为之。在大多数情况下，如果你尝试不支持的东西(例如使用未实现的配置标志或命令行选项)，将得到一个明确的错误。

# Google：RRG

RRG是GRR(远程实时取证框架)的Rust重写。

Google评估重写GRR(代理服务)的客户端部分的可行性，是否不需要当前版本必须携带的所有历史包袱。例如，它不实现自己的通信层，而是利用Fleetspeak来实现。它还试图评估有多少与Python代码库相关的现有问题，是否可以通过使用具有强大类型系统和强大安全保证的Rust语言来解决。

RRG只是更大系统的一个组件，所以要使用它做任何有用的事情，还需要设置Fleetspeak和GRR。

# coreutils

utils coretils是GNU coretils在Rust中的跨平台重新实现。虽然所有程序都已实现，但可能缺少某些选项或有不同的行为。

uutils旨在在尽可能多的平台上工作，以便能够在Linux，Mac，Windows和其他平台上使用相同的utils，这确保了脚本可以轻松地在平台之间传输。

coretils二进制文件可以为bash、elvish、fish、powershell和zsh shell生成命令补全，它将结果打印到stdout。

uutils遵循Rust的发布渠道，并针对稳定版、beta版和夜间版进行了测试。当前最低支持的Rust版本(MSRV)是1.70.0。