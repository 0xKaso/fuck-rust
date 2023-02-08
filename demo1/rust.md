 快速入门  var path\_to\_root = "../"; var default\_theme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "light" : "light";  try { var theme = localStorage.getItem('mdbook-theme'); var sidebar = localStorage.getItem('mdbook-sidebar'); if (theme.startsWith('"') && theme.endsWith('"')) { localStorage.setItem('mdbook-theme', theme.slice(1, theme.length - 1)); } if (sidebar.startsWith('"') && sidebar.endsWith('"')) { localStorage.setItem('mdbook-sidebar', sidebar.slice(1, sidebar.length - 1)); } } catch (e) { }  var theme; try { theme = localStorage.getItem('mdbook-theme'); } catch(e) { } if (theme === null || theme === undefined) { theme = default\_theme; } var html = document.querySelector('html'); html.classList.remove('no-js') html.classList.remove('light') html.classList.add(theme); html.classList.add('js');  var html = document.querySelector('html'); var sidebar = 'hidden'; if (document.body.clientWidth \>= 1080) { try { sidebar = localStorage.getItem('mdbook-sidebar'); } catch(e) { } sidebar = sidebar || 'visible'; } html.classList.remove('sidebar-visible'); html.classList.add("sidebar-" + sidebar);

1. [**1.** Move 编程语言](../index.html)
2. [**2.** 序言](../introduction/foreword.html)
3. [**3.** 快速入门](../introduction/getting-started.html)
4. [**4.** 语法基础](../syntax-basics/index.html)
5. 1. [**4.1.** 基础概念](../syntax-basics/concept.html)
   2. [**4.2.** 基本类型](../syntax-basics/primitives.html)
   3. [**4.3.** 注释](../syntax-basics/comments.html)
   4. [**4.4.** 表达式和作用域](../syntax-basics/expression-and-scope.html)
   5. [**4.5.** 控制流](../syntax-basics/control-flow.html)
   6. [**4.6.** 模块和导入](../syntax-basics/module.html)
   7. [**4.7.** 常量](../syntax-basics/constants.html)
   8. [**4.8.** 函数](../syntax-basics/function.html)

6. [**5.** 进阶主题](../advanced-topics/index.html)
7. 1. [**5.1.** 结构体](../advanced-topics/struct.html)
   2. [**5.2.** Abilities](../advanced-topics/types-with-abilities.html)
   3. [**5.3.** 所有权和引用](../advanced-topics/ownership-and-references.html)
   4. [**5.4.** 泛型](../advanced-topics/understanding-generics.html)
   5. [**5.5.** 数组](../advanced-topics/managing-collections-with-vectors.html)

8. [**6.** 可编程的 Resource](../resources/index.html)
9. 1. [**6.1.** 发送者和签署者](../resources/signer-type.html)
   2. [**6.2.** 什么是 Resource](../resources/what-is-resource.html)
   3. [**6.3.** Resource 举例](../resources/resource-by-example/index.html)
   4. 1. [**6.3.1.** 创建和转移](../resources/resource-by-example/storing-new-resource.html)
      2. [**6.3.2.** 读取和修改](../resources/resource-by-example/access-resource-with-borrow.html)
      3. [**6.3.3.** 使用和销毁](../resources/resource-by-example/destroy-resource.html)
      4. [**6.3.4.** 下一步](../resources/resource-by-example/furher-steps.html)

10. [**7.** 实例](../tutorials/index.html)
11. 1. [**7.1.** ERC20 代币](../tutorials/erc20.html)

* Light (default)
* Rust
* Coal
* Navy
* Ayu

==========

[](../print.html)

 document.getElementById('sidebar-toggle').setAttribute('aria-expanded', sidebar === 'visible'); document.getElementById('sidebar').setAttribute('aria-hidden', sidebar !== 'visible'); Array.from(document.querySelectorAll('#sidebar a')).forEach(function(link) { link.setAttribute('tabIndex', sidebar === 'visible' ? 0 : -1); });

[快速入门](#快速入门)
==========

与任何编程语言一样，Move 应用程序也需要一组适当的工具来编译、运行和调试。由于 Move 语言是为区块链创建、并且仅在区块链中使用，因此在链下运行程序不是一件容易的事，因为每个应用都需要一个编辑环境、账户处理和编译-发布系统。

为了简化 Move 程序的开发，我在 Visual Studio Code 上开发了 [Move IDE](https://github.com/damirka/vscode-move-ide) 扩展。该扩展可以满足开发者对开发环境的基本需求。它的功能除了程序执行外还包括 Move 语法高亮显示，可以更好的帮助开发者在发布之前调试应用程序。开发者只需专注于 Move 语言本身，而不必为客户端（CLI）苦苦挣扎。

[安装 Move IDE](#安装-move-ide)
----------

需要安装下面的软件:

1. VSCode (1.43.0 或者更高版本) - 可以在 [这里](https://code.visualstudio.com/download) 获取; 当然如果你的机器上已经安装了 VSCode，可以直接进入下一步;
2. Move IDE - 安装 VSCode 后，请单击 [这里](https://marketplace.visualstudio.com/items?itemName=damirka.move-ide) 安装最新版本的 IDE。

### [环境设置](#环境设置) ###

Move IDE 提供了单一的方法来组织目录结构。只需要创建一个新目录，并在 VSCode 中打开它，就可以得到如下目录结构：

```
modules/   - directory for our modules
scripts/   - directory for transaction scripts
out/       - this directory will hold compiled sources

```

另外，还需要创建一个名为 `.mvconfig.json` 的文件，该文件将配置您的工作环境。下面这个配置指向了 `Libra` 网络:

```
{
    "network": "libra",
    "sender": "0x1"
}

```

或者使用 `dfinance` 作为目标网络:

```
{
    "network": "dfinance",
    "sender": "0x1"
}

```

>
>
> dfinance 使用 bech32 "wallet1 ..." 地址，Libra 使用16字节 “0x ...” 地址。对于本地运行或者测试，使用 Libra 地址就可以了。但是在测试网或生产环境中使用真实的区块链时，需要使用所选网络的正确地址。
>
>

[第一个 Move 应用](#第一个-move-应用)
----------

Move IDE 使开发者可以在测试环境中运行程序。让我们通过一个例子来了解其工作原理：实现 gimme\_five() 功能并在 VSCode 中运行它。

### [创建模块](#创建模块) ###

在项目的目录 modules/ 内创建一个新文件 hello\_world.move。

```
// modules/hello_world.move
address 0x1 {
module HelloWorld {
    public fun gimme_five(): u8 {
        5
    }
}
}

```

>
>
> 如果您想使用自己的地址（而非0x1），请确保更改此文件中的 0x1 以及下面文件中的地址。
>
>

### [写脚本](#写脚本) ###

然后在 scripts/ 目录中创建一个脚本 run\_hello.move，调用上面的模块：

```
// scripts/run_hello.move
script {
    use 0x1::HelloWorld;
    use 0x1::Debug;

    fun main() {
        let five = HelloWorld::gimme_five();

        Debug::print<u8>(&five);
    }
}

```

最后，在保持脚本打开的同时，执行以下步骤：

1.通过按`⌘+Shift+P`（在 Mac 上）或`Ctrl+Shift+P`（在Linux / Windows上）来切换 VSCode 的命令选项板
2.键入：`>Move: Run Script`并在看到正确的选项时按 Enter 或单击。
现在，你应该会看到执行结果，输出日志中有“5”信息。如果没有看到此窗口，请再次浏览上面部分，看看有没有漏掉什么。

目录结构应如下所示：

```
modules/
  hello_world.move
scripts/
  run_hello.move
out/
.mvconfig.json

```

>
>
> modules 目录下可以包含任意多的模块；所有这些模块都可以被你的脚本访问到，只要它们都被定义在 .mvconfig.json 所指定的地址下即可。
>
>

[](../introduction/foreword.html) [](../syntax-basics/index.html)

[](../introduction/foreword.html) [](../syntax-basics/index.html)

 window.playpen\_copyable = true;