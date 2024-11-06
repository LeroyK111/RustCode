# Zephyr RTOS入门 —— 复杂但是强大

ephyr是一个针对资源受限设备优化的小型、可缩放、多体系架构实时操作系统（RTOS）。Zephyr由Linux基金会维护[1]，是一个以构建业界最佳的RTOS为目标的开源合作项目。近年来，Zephyr OS在嵌入式开发中的知名度逐渐增加，新的微控制器和开发板都支持Zephyr。本文将深入讨论Zephyr RTOS的技术细节。

  Zephyr有着可缩放性，这让它能够宽泛地适用于资源限制各不相同的多种设备。Zephyr模块化的结构让开发者能够只包括他们需要的组件，从而降低系统尺寸，达到系统的可缩放性。Zephyr在网站上声称可以在内存小到8KB，大到GB的系统上运行。


## **1. 宽泛的硬件支持**

Zephyr支持众多体系结构，包括Arm、x86、RISC-V、Xtensa、MIPS等，并借由Nios2和MicroBlaze软核支持FPGA。本文成文时，Zephyr支持超过600种开发板，包括Arduino UNO R4 Minima、GIGA R1 WiFi和Portenta H7、多种ESP32板、BBC micro:bit的所有版本、树莓派Pico（甚至是树莓派4B+）、nRF51和nRF52板、NXP MIMXRT1010-EVK和其家族，以及STM32 Nucleo和Discovery家族。以上我只列举了在Elektor上常看到的开发板，还有其他许多Zephyr支持的硬件。

除了包含处理器的开发板，Zephyr还支持许多附加板（背板），并包括了各类接口和超过150种传感器的驱动。

## **2. 多任务、网络和电源管理**

作为一款实时操作系统（RTOS），Zephyr提供抢占式多任务调度、任务间通信和实时时钟支持。OS还支持多种网络技术与协议，例如TCP/IP、蓝牙、IEEE 802.15.4（在Zigbee中使用）、MQTT、NFS与LoRaWAN。网络支持加上电源管理功能让Zephyr适合强调能源效率的物联网（IoT）应用和使用电池驱动的设备。

Zephyr中的一组软件库和中间件能够简化常见的任务，像是通信协议、文件系统和设备驱动。

Zephyr还针对安全认证设计，比如ISO 26262，这让它适用于安全关键应用。

## **3. 受Linux启发**

Zephyr并不是Linux，但是它借鉴了Linux的概念、技术和工具。例如Zephyr使用Kconfig配置操作系统，硬件的属性和配置则用设备树规范（Device Tree Specification——DTS）[2]描述。因此，Linux开发者能快速上手Zephyr编程。

## **4. 开源**

Zephyr使用较为宽松的Apache 2.0许可证，允许商业和非商业用途。Zephyr的用户社区提供支持和文档，你也可以加入社区。

## **5. 尝试Zephyr**

我一直都想要尝试Zephyr，但是我对它的第一印象不太好，所以就一直没有深入了解。我之前遇到的主要问题（除了顺利编译源代码以外）是它需要一个能对目标控制器进行编程的连接器，这对于创客不太友好。多亏了Arduino和其引导程序，我们不再需要特别的编程工具。

### 5.1  选择一个开发板

自从我上次尝试Zephyr以来，其发展日新月异。今天，Zephyr支持超过600种不同的微控制器开发板，你很可能已经拥有几块支持Zephyr的开发板了。阅读详细的支持列表，我发现我已经拥有十几块支持Zephyr的开发板了。

### 5.2  BBC micro:bit

我尝试了许多块开发板，最后决定在BBC micro:bit上进行实验（见图1，取决于型号，Zephyr使用目标名bbc_microbit或者bbc_microbit_v2）。micro:bit除了很常见以外，它对Zephyr的支持可能是最好的，所有的外设都可以使用，而且有样例代码。最好的是，编程和调试都不需要额外的工具。流行的ESP-WROOM-32（Zephyr目标名esp32_devkitc_wroom）也是个很好的选择，但是调试需要外部工具。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106101644.png)

图1：迷你的BBC micro:bit开发板非常适合尝试Zephyr RTOS。谁能想到了为了让儿童学习MakeCode（一门类似Scratch的图形语言）的小板子也适合帮助一位专业嵌入式软件开发者熟悉一款工业级RTOS呢？

Arduino GIGA R1 WiFi也是一个不错的备选方案，但是使用Zephyr的时候会覆写它的引导程序。即使之后你可以手动恢复，这也会添加不需要的副作用。

官方说法是Arduino UNO R4 Minima需要SWD（串行线调试）连接器。许多其他开发板也需要SWD，包括树莓派Pico。但是我发现使用dfu-util（见下文）可以绕过这一要求。和GIGA R1类似，Arduino引导程序会被覆盖。

### 5.3  使用仿真器

如果你没有合适的开发板，但是还是非常想要尝试Zephyr，Zephyr内建QEMU仿真器支持（只在Linux和macOS上），让你可以虚拟地运行和测试应用。Antmicro的Renode也能达到同样效果，但是我没有亲自尝试。

### 5.4  安装Zephyr

我在运行Windows 11的计算机上安装了Zephyr，我没有尝试Linux或macOS。在线可以找到详细的安装指南[3]，所有步骤都非常清晰，我不需要寻找其他参考资料。按照指南，我使用了虚拟的Python环境，这意味着需要把创建环境的命令记下来，因为每次开始开发时都需要用到。如果你使用Windows PowerShell，可以运行脚本activate.ps1；命令行中则是activate.bat批处理文件。Windows PowerShell处理编译器和链接器输出的表现会更好（图2）。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106101717.png)
图2：Zephyr的west构建工具是针对命令行终端设计的，Windows的cmd.exe虽然也可以运行，但它不是一个终端。Windows PowerShell是一个终端，更适合运行构建。
Zephyr包括两个部分，OS本身和包含一组MCU工具链的软件开发套件（SDK）。截至本文成文，有21套不同的工具链。
OS和SDK并不需要在同一个位置安装，在我的环境中两个总共需要12GB的存储空间，你可以删除不需要的工具链以节约空间。
安装完成后，让我们测试安装是否成功。通过下面的命令构建一个样例代码，上传到开发板上（将<my_board>替换为你的开发板的目标名，例如arduino_uno_r4_minima）：

```sh
west build -p always -b <my_board> samples/basic/blinky
```

如果你想直接使用上述命令，必须先更改工作路径。运行下面的命令（.venv表示你正在虚拟环境中）：
```sh
(.venv) <my_path_to_zephyr>\zephyrproject\zephyr
```
如果样例能成功构建，运行下面的命令将其上传到开发板上：
west flash

板上的“默认”LED会以0.5Hz的速率开始闪烁。

前面已经提到过，烧录可能需要额外的编程工具，像是J-Link适配器，或是另一种JTAG/SWD兼容的编程器（图3）。west程序也必须能够找到驱动程序（即在搜索路径中，对Windows而言是%PATH%），如果不能的话程序会告知你（但是消息经常又长又难以理解）。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106101830.png)
图3：在许多情况下（不是全部），你都需要一个JTAG或者SWD编程/调试工具来进行你的Zephyr实验。

在BBC micro:bit V2上，第一次烧录的时候我需要参考标准micro:bit编程步骤将HEX文件手工复制到板上，之后flash命令就可以顺利运行了。可执行的zephyr.hex文件在zephyrproject\zephyr\build\zephyr路径中。

Arduino板的默认flash命令（UNO R4 Minima和GIGA R1 WiFi）需要将dfu-util编程工具加入到搜索路径中（在启动虚拟环境之前）。这一工具在Arduino IDE中可以找到，但是具体路径取决于你的环境（默认为%HOMEPATH%\AppData\Local\Arduino15\packages\arduino\tools\dfu-util\<最新的Arduino版本>）。开发板还必须在DFU模式下，按下reset按键两次即可。当LED开始“呼吸”时你就可以烧录程序了。

### 5.5  闪灯（Blinky）样例的兼容性

你也许注意到了，我建议用Arduino UNO R4 Minima开发板运行闪灯样例，而不是BBC micro:bit，这是因为尽管板上有25个LED（不包括电源指示），它们和闪灯样例并不兼容。ESP-WROOM-32也有类似问题，但是R4 Minima没有。

GIGA R1也与样例兼容，板上的MCU有两个核（Cortex M4和M7），Zephyr允许你在构建命令中使用arduino_giga_r1_m4或者arduino_giga_r1_m7来选择其一。构建和烧录闪灯样例两次，一次M4，一次M7，你就可以发现两个核互为独立。GIGA有RGB LED，闪灯样例在不同核上会使用不同的颜色，M4为蓝色，M7为红色。还可以通过改变闪烁速率（samples\basic\blinky\src\main.c中的SLEEP_TIME_MS）进一步区分两次运行的行为。

### 5.6  Hello World

对于缺少用于闪烁的LED的开发板而言，下面是在串口输出字符串的hello_world样例：

```sh
west build -p always -b <my_board> samples/hello_world
west flash
```

这一样例在BBC micro:bit和ESP-WROOM-32上都可以运行。要看到输出字符串的话，需要在你的计算机上运行任一串口终端程序。数据率一般为115200（115200,n,8,1）。消息只会输出一次，如果错过了就需要对开发板进行重置（图4）。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106102342.png)
图4：hello_world样例很简短，如果打开串口终端太慢的话就会错过输出。

在R4 Minima和GIGA R1上，串口输出在1号引脚上，而不是使用Arduino IDE时的USB-C端口。这是因为USB端口是MCU的外设，而不在一个单独的芯片上。Zephyr是一个模块化和可扩展的OS，每一个模块和外设都需要被专门启用。在工程的配置文件中可以启用外设，我们在后面会讨论。

对于没有内建串口到USB转换器的开发板，你必须找到串口（MCU上有超过一个端口时，一般为0号端口）并通过外置的串口到USB转换器连接到你的计算机。

## **6. 进阶实验**

如果你已经能够在你的开发板上运行闪灯和hello_world样例，接下来你就可以开始创建你的第一个应用了。如果只有一个样例能运行，你应该调试另一个样例，因为接下来的内容很快会变得复杂起来。

虽然BBC micro:bit和闪灯样例不兼容，这并不会有什么严重的影响，我还是选择它作为实验板。Zephyr中还有几个针对它的样例（在samples\boards\bbc_microbit文件夹中），“显示”（display）样例比单一LED闪烁样例要更好。Zephyr只针对600多种支持的开发板的不到5%提供了样例。另外，许多样例是更加进阶或者少见的用例。

当你针对BBC micro:bit、ESP-WROOM-32或者其他不兼容的开发板构建闪灯样例时，编译会给出一个难以理解的错误。错误信息想要告诉你，led0是一个未知的对象，led0是默认的闪灯LED（类似Arduino中的LED_BUILTIN）。micro:bit有着可以连接LED和其他外设的扩展端口，让我们将其中一个定义为led0。

在此之前，让我们复制一份samples/basic/blinky文件夹作为备份。下面的步骤中我们会修改samples/basic/blinky中的内容。

### 6.1  设备树

为了定义led0，我们需要使用设备树。设备树由一个或者更多文本文件定义，其中列出了电路板上或者控制器中的外设和可用存储。在Zephyr中，这些文件的后缀名为.dts或.dtsi（i表示include——包含），一个文件可以包含另一个文件。处理器.dtsi文件在dts文件夹中，开发板.dts和.dtsi文件在boards文件夹中，两个文件夹内容都按照处理器架构分类。

你可以使用Visual Studio Code中的DeviceTree插件[4]查看DTS/DTSI文件，这一插件支持语法高亮和查找，增加文件的可读性（DTS文件的语法类似C语言）。图5所示的是nRF51822（BBC micro:bit V1的核心）的.dtsi文件的一部分，开发板的DTS文件包含这一文件。注意到uart0的状态是“disabled”（禁用），但是在开发板的DTS文件中将其覆盖为“okay”，即可以使用，gpio2和i2c0也是同样。

![](../../learning/src/objInfo/assets/Pasted%20image%2020241106102507.png)

图5：nrf51822.dtsi文件的一部分，从右侧的缩略图可以看到文件十分长。

### 6.2  设备树中的I²C总线

图6所示的是BBC micro:bit的.dts文件的一部分，其中展示了I²C总线的设备树。micro:bit的总线上有一个或两个传感器（根据不同的V1板种类有所不同），在树中分别为mma8653fc和lsm303agr（后者中有两个传感器，所以在树中出现两次）。前者状态为“okay”，后者则为“disabled”，这对我的初代micro:bit V1开发板而言是正确的。

![](../../learning/src/objInfo/assets/Pasted%20image%2020241106102527.png)
图6：设备树的这一部分代表了BBC micro:bit板的I²C总线，而不是处理器；出自bbc_microbit.dts文件。

片段中所示的传感器与FXOS8700和MMA8653FC兼容，在I²C总线上的地址为0x1d，两个被声明的中断信号（int）分别与GPIO引脚27和28相连。该传感器有一个代码样例供实验用：

```sh
west build -p always -b bbc_microbit samples/sensor/fxos8700

west flash
```

这个样例无法在BBC micro:bit V2上工作，因为其设备树上的传感器有所不同。

样例的输出如图7所示。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106102553.png)
图7：samples/sensor/fxos8700样例在BBC micro:bit上的输出。

### 6.3  设备树的覆盖  

回到前面提到过的led0，设备树中没有提及led0，所以我们需要进行添加。我们可以直接在设备树文件中添加，但这并不正确，开发板上并没有led0。正确的方法是在设备树上添加一个覆盖层，已有的设备树会根据覆盖文件的内容进行扩展（新的项目）或是覆写（项目已经在树中存在）。在这里我们需要添加新的项目。

覆盖文件必须放置在工程文件夹的boards子文件夹中。当这一子文件夹存在时，构建过程会在其中搜索名为<my_board>.overlay的文件。对我来说文件名是bbc_microbit.overlay，对于V2用户则是bbc_microbit_v2.overlay。图8所示的是文件内容。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106102815.png)
图8：让BBC micro:bit V1与闪灯样例兼容的设备树覆盖文件。

### 6.4  增加一个闪灯LED

Zephyr针对LED有一个特殊的设备树关键字leds。当创建一个节点（分支）时，你可以起任何名字，但是需要将其放置在leds下，这样才能覆盖已有的leds节点。这一新节点会被添加到设备树的根上，所以在节点前添加一个正斜杠“/”，在DT语言中表示根。下一行代表这一分支与Zephyr内建的gpio-leds驱动兼容（驱动的接口可以在zephyr\include\zephyr\drivers\led.h中找到）。

### 6.5  子节点

接下来是一组LED子节点，因为我只有一盏LED，所以只有一个子节点led_0，我将其标记为led0。标签（label）是可选的，但是标签让你可以在设备树的其他地方引用这一节点。另外，应用中（开发者）可以通过标签访问节点和节点的属性。

一个子结点必须指定设备的属性，这里的设备是LED，必需属性只有GPIO引脚，可选标签可以用于向应用提供文档或是用户可读信息，除此之外没有其他用途。

GPIO引脚我选择了1，即micro:bit扩展连接器上的2号大孔。如果你使用BBC micro:bit V2的话，在这里使用4（而不是1）。

### 6.6  创建一个别称

接下来这一步是闪灯样例所必需的：为我们的LED创建led0别称（alias）。你也许会觉得为子节点添加一个标签就足够了，但实际上并非如此。闪灯样例使用DT_ALIAS宏访问LED子节点，所以我们必须配合这个宏。这个宏需要使用别称，因此我们在设备树中添加了aliases块。如果闪灯样例使用的是DT_NODELABEL宏，别称就没有必要了，因为这个宏会直接获取led0子节点。标签和别称同为led0会有些令人疑惑，这是为了后面能够更好地解释。

### 6.7  Zephyr宏

尽管在C/C++编程中宏的名声并不好，Zephyr大量使用宏。有许多像DT_ALIAS和DT_NODELABEL这样的宏帮助应用和配置工具从设备树提取信息，Zephyr手册的“设备树API”（“Devicetree API”）章节有对它们的介绍。

一个有趣的发现：许多（也许是全部？）Zephyr宏都要求参数为小写字母和数字，其他符号都会被替代为下划线，这被称为“小写字母和下划线兼容”。举例来说，如果我将LED子节点标记为LED-0（而不是led0），那么传递给DT_NODELABEL的参数将会成为led_0，即DT_NODELABEL(led_0)，这是因为‘-’不是字母或者数字，而且字母必须都为小写。换言之，对于使用设备树宏的应用开发者而言，下划线就是一个通配符，led_0可以指led_0、led-0、Led_0、LED-0或者ledé0等等。建议你仔细阅读Zephyr的宏文档。

### 6.8  你不能犯错误

注意，如果你在设备树上犯了错误，编译器会显示致命错误，而并不会提供有用的信息。

### 6.9  纯净构建

当你修改设备树时，你很可能会经常需要重新构建你的应用。为了加快速度，去掉build命令中的“-p always”（p代表pristine——纯净），这样命令就不会从头构建所有文件。但是，如果你在逐个尝试不同的样例，你需要保留这个选项，否则你会看到构建文件夹不正确的错误。

flash命令也会运行最后执行的build命令，所以每次你做出更改之后可以直接运行flash命令。

### 6.10  使用设备驱动

闪灯样例通过调用gpio_pin_toggle_dt()函数改变LED的状态，该函数在GPIO驱动中。这一方法当然没有问题，Zephyr还另外有一组LED驱动。使用LED驱动可以让代码变得更可读，还可以提高程序的弹性和可移植性。这是因为LED驱动可以快速替换，应用代码不需改变。Zephyr的可缩放性和模块化在这里就体现出来了。

### 6.11  Kconfig的图形用户界面

将LED设备驱动整合到程序中需要几个步骤。首先，重新配置工程以包含驱动。工程的配置由Kconfig处理，即在Linux中使用的内核构建配置系统。有多种方式可以和Kconfig交互，其中一种是通过图形用户界面（GUI）。在Zephyr中通过以下命令打开图形界面：

```sh
west build -t guiconfig
```
GUI需要些时间才会打开，你会看到类似图9这样的界面，其中可以看到许多当前工程的信息。为了保证Kconfig能够配合你的工程，在运行GUI前先进行一次纯净构建（-p always选项）。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106113902.png)
图9：Kconfig工程配置工具的GUI，其中有许多选项。

### 6.12  众多的配置选项

先花一点时间熟悉Kconfig的配置树，点击加号展开分支，点击选项进行标记，在下方的面板中能看到说明。可以看到printf()的浮点数支持是一个C++语言支持的配置选项。与此类似，还可以在Build and Link Features（构建和链接功能）中找到编译器优化选项。

配置树中有许多选项，Device Driver（设备驱动）分支下的选项和我们相关。展开分支，向下滚动，LED驱动大概是在一半的位置：Light Emitting Diode (LED) Drivers（LED驱动），选中这一选项。选项下的子分支不需要改变，保持默认值即可（图10）。点击Save（保存）键，记下窗口底端显示的配置文件位置，之后你可以打开这一文件查看内容。最后关闭GUI。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106113937.png)
图10：选择Light Emitting Diode (LED) Drivers（LED驱动），子分支的值保持默认不变。
接下来在构建时就不要使用-p always选项了，否则会将你前面改变的配置复原。之后会介绍如何将选项永久保存。

### 6.13  使用LED设备驱动的闪灯样例

现在我们可以编写新的闪灯程序了，见图11。首先包含设备和LED驱动的头文件，然后在主函数中通过DEVICE_DT_GET_ANY宏从设备树取得LED设备的引用。注意到宏的参数“gpio_leds”是“小写字母和下划线兼容”的，对应设备树中leds节点compatible（兼容）属性的值“gpio-leds”（上文已经解释过）。如果你传入了错误的参数，程序无法找到节点，会输出“No device with compatible gpio-leds found”（无法找到具有gpio-leds兼容的设备）。如果设备的status属性被设为“disabled”，同样的消息也会出现。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106114011.png)
图11重新编写了 Blinky 程序以与 LED 设备驱动程序一起使用。请注意，该代码没有局限在某种开发板。该程序可以在任何设备驱动中列出了 gpio_leds（或 GPIO-LED） 设备树的开发板上运行。

Zephyr将兼容用作名词这件事需要花时间习惯，上面的错误信息不意味着没有兼容的设备，而是没有设备的“兼容”属性的值为“gpio-leds”（也就是“gpio_leds”，下划线替代了a到z和0到9以外的字符）。

第二个检查验证设备是否正确地初始化，如果是的话，我们将继续。

在无限while()循环中，代码通过驱动提供的led_on和led_off命令[6]点亮和熄灭LED，参数0代表DEVICE_DT_GET_ANY宏找到的第一个（也是唯一一个）设备，也就是led0。

### 6.14  检查返回值  

因为我们使用设备驱动，而不是在硬件寄存器层直接切换GPIO引脚状态，所以应当检查驱动函数调用的返回值，确保没有发生错误。驱动必须提供函数和回调，也有可选的功能。举例来说，LED驱动必须实现led_on和led_off，但是led_blink是可选的。在本实验中，led_blink未被实现，如果你调用的话，什么也不会发生。这一函数存在，但是其中没有内容，返回值会告诉你这一点。一般来讲，检查每个函数调用的返回值是良好的编程习惯。

  

通过下面的命令构建和上传程序（注意没有添加-p always选项）：

```sh
west build -b bbc_microbit samples/basic/blinky

west flash
```

### 6.15  配置工程

当LED以0.5HZ的频率开始闪烁，就说明我们的程序能够运行。这时我们需要永久保存当前的配置，打开闪灯文件夹prj.conf文件，添加下面一行（Kconfig配置文件使用Python语法，不像设备树使用C语言语法）：
```
CONFIG_LED=y
```
为了检查配置成功，进行一次纯净构建，并将可执行文件上传到开发板上。

### 6.16  调试

如果你的开发板允许调试（比如BBC micro:bit），或者你有合适的调试工具，运行下面的命令进行调试：

```sh
west debug
```
该命令会启动GDB服务器，并打开GDB命令行界面（图12）。你可以在线学习如何使用GDB，这一部分不属于本文的范畴之内。
![](../../learning/src/objInfo/assets/Pasted%20image%2020241106114152.png)图12：BBC micro:bit有原生gdb调试支持，不需要额外的工具。

## **7. Zephyr与Arduino的对比**

现在你上手了Zephyr，你可能会想：为什么要使用它？Arduino难道不是更简单？Arduino与Zephyr类似，支持许多处理器架构和数百款开发板。Arduino还有着数千种驱动和软件库，如果一款应用或者一个软件库使用Arduino核心API，它可以被快速移植到有着类似外设的兼容平台上。Arduino的软件也是开源的，所以Zephyr有什么优势？

Zephyr是作为一款工业级、健壮的RTOS设计的，支持任务调度、内存管理和设备驱动等功能。Zephyr能够适应不同的项目复杂度，从小型的IoT设备到复杂的嵌入式系统。Zephyr提供更大的弹性，但是需要开发者对嵌入式开发有着深入的理解。

Arduino提供一定的实时性，但它不是一款RTOS，而是一款强调简洁和易用的单线程应用框架。Arduino将许多底层细节都进行了抽象化，让它对初学者非常友好。对于更加复杂的应用，Arduino可以在RTOS上使用，例如Mbed OS。将Arduino核心API移植到Zephyr的工作正在进行中。