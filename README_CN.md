# cargo-make

[English](README.md) | 中文文档

[![crates.io](https://img.shields.io/crates/v/cargo-make.svg)](https://crates.io/crates/cargo-make)
[![CI](https://github.com/sagiegurari/cargo-make/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/cargo-make/actions)
[![codecov](https://codecov.io/gh/sagiegurari/cargo-make/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/cargo-make)
[![license](https://img.shields.io/crates/l/cargo-make.svg)](https://github.com/sagiegurari/cargo-make/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/cargo-make?label=cargo%20installs)](https://crates.io/crates/cargo-make)
[![GitHub All Releases](https://img.shields.io/github/downloads/sagiegurari/cargo-make/total?label=binary%20downloads)](https://github.com/sagiegurari/cargo-make/releases)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> [Rust](https://www.rust-lang.org/) 任务执行器（Task Runner）与构建工具。

* [总览 (Overview)](#overview)
* [安装 (Installation)](#installation)
    * [Arch Linux](#installation-arch-linux)
    * [二进制发布版 (Binary Release)](#installation-binary-release)
* [使用说明 (Usage)](#usage)
    * [简单示例 (Simple Example)](#usage-simple)
    * [任务、依赖与别名 (Tasks, Dependencies, and Aliases)](#usage-task-dependencies-alias)
    * [命令、脚本与子任务 (Commands, Scripts, and Sub Tasks)](#usage-task-command-script-task)
        * [子任务 (Sub Task)](#usage-task-command-script-task-examplesubtask)
        * [命令 (Command)](#usage-task-command-script-task-examplecommand)
        * [脚本 (Script)](#usage-task-command-script-task-examplescript)
        * [Duckscript](#usage-task-command-script-task-exampleduckscript)
        * [Rust 代码 (Rust Code)](#usage-task-command-script-task-examplerust)
        * [跨平台 Shell (Cross Platform Shell)](#usage-task-command-script-task-exampleshell2batch)
        * [其他编程语言 (Other Programming Languages)](#usage-task-command-script-task-examplegeneric)
        * [Shebang 支持 (Shebang Support)](#usage-task-command-script-task-exampleshebang)
    * [默认任务与扩展 (Default Tasks and Extending)](#usage-default-tasks)
        * [扩展外部 Makefile (Extending External Makefiles)](#usage-workspace-extending-external-makefile)
        * [自动扩展工作区 Makefile (Automatically Extend Workspace Makefile)](#usage-workspace-extend)
        * [加载脚本 (Load Scripts)](#usage-load-scripts)
        * [预定义 Makefile (Predefined Makefiles)](#usage-predefined-makefiles)
        * [默认任务 (The Default Task)](#usage-default-task)
    * [扩展任务 (Extending Tasks)](#usage-extending-tasks)
        * [任务覆盖 (Task Override)](#usage-task-override)
        * [平台覆盖 (Platform Override)](#usage-platform-override)
        * [extend 属性 (Extend Attribute)](#usage-task-extend-attribute)
    * [环境变量 (Environment Variables)](#usage-env)
        * [声明方式 (Declaration)](#env-declaration)
        * [全局配置 (Global Configuration)](#usage-env-config)
        * [任务级变量 (Task)](#usage-env-task)
        * [命令行参数 (Command Line)](#usage-env-cli)
        * [Env 文件 (Env File)](#usage-env-file)
        * [环境初始化脚本 (Env Setup Scripts)](#usage-env-setup-scripts)
        * [加载顺序 (Loading Order)](#usage-env-vars-loading-order)
        * [关于顺序的注意事项 (Note about Ordering)](#env-note-about-ordering)
        * [全局环境变量 (Global)](#usage-env-global)
    * [设置工作目录 (Setting Up Working Directory)](#usage-setting-up-working-directory)
    * [忽略错误 (Ignoring Errors)](#usage-ignoring-errors)
    * [条件执行 (Conditions)](#usage-conditions)
        * [条件判定准则 (Criteria)](#usage-conditions-structure)
        * [条件脚本 (Scripts)](#usage-conditions-script)
        * [And / Or / Group Or 组合](#usage-conditions-and-or)
        * [结合条件与子任务 (Combining Conditions and Sub Tasks)](#usage-conditions-and-subtasks)
        * [仅当源码变更时运行任务 (Running Tasks Only If Sources Changed)](#usage-running-tasks-only-if-sources-changed)
    * [安装依赖 (Installing Dependencies)](#usage-installing-dependencies)
        * [Cargo 插件 (Cargo Plugins)](#usage-installing-cargo-plugins)
        * [Crates](#usage-installing-crates)
        * [Rustup 组件 (Rustup Components)](#usage-installing-rustup-components)
        * [系统原生依赖 (Native Dependencies)](#usage-installing-native-dependencies)
        * [指定版本 (Defining Version)](#usage-installing-version)
        * [全局版本锁定 (Global Lock Of Versions)](#usage-installing-locked)
        * [备用 Cargo 安装命令 (Alternate Cargo Install Commands)](#usage-installing-alternate-cargo-install-commands)
        * [安装优先级 (Installation Priorities)](#usage-installing-dependencies-priorities)
        * [多依赖安装 (Multiple Installations)](#usage-installing-dependencies-multiple)
    * [工作区支持 (Workspace Support)](#usage-workspace-support)
        * [禁用工作区支持 (Disabling Workspace Support)](#usage-workspace-disabling-workspace-support)
        * [复合流程 (Composite Flow)](#usage-workspace-composite-flow)
        * [工作区配置环境 (Profiles)](#usage-workspace-profiles)
        * [跳过/包含特定成员 (Skipping/Including Specific Members)](#usage-workspace-support-skip-include-members)
        * [工作区仿真 (Workspace Emulation)](#usage-workspace-emulation)
    * [工具链支持 (Toolchain)](#usage-toolchain)
    * [Init 与 End 初始化与收尾任务 (Init and End tasks)](#usage-init-end-tasks)
    * [捕获错误 (Catching Errors)](#usage-catching-errors)
    * [Cargo 别名任务 (Cargo Alias Tasks)](#usage-cargo-alias-tasks)
    * [配置环境 (Profiles)](#usage-profiles)
        * [环境变量覆盖 (Environment Variables)](#usage-profiles-env)
        * [条件判断 (Conditions)](#usage-profiles-conditions)
        * [内置 Profile (Built In Profiles)](#usage-profiles-built-in)
    * [私有任务 (Private Tasks)](#usage-private-tasks)
    * [已废弃任务 (Deprecated Tasks)](#usage-deprecated-tasks)
    * [文件监听自动执行 (Watch)](#usage-watch)
        * [运行多个阻塞式监听 (Running Multiple Blocking Watches)](#usage-watch-running-multiple-blocking-watches)
    * [内置函数 (Functions)](#usage-functions)
        * [Split](#usage-functions-split)
        * [GetAt](#usage-functions-getat)
        * [Remove Empty](#usage-functions-remove-empty)
        * [Trim](#usage-functions-trim)
        * [Decode](#usage-functions-decode)
    * [持续集成 (Continuous Integration)](#usage-ci)
        * [Github Actions](#usage-ci-github-actions)
        * [Travis](#usage-ci-travis)
        * [AppVeyor](#usage-ci-appveyor)
        * [GitLab](#usage-ci-gitlab)
        * [CircleCI](#usage-ci-circleci)
        * [Azure Pipelines](#usage-ci-azure-pipelines)
        * [drone.io](#usage-ci-drone-io)
        * [Cirrus CI](#usage-ci-cirrus)
    * [预定义工作流 (Predefined Flows)](#usage-predefined-flows)
        * [代码覆盖率 (Coverage)](#usage-predefined-flows-coverage)
        * [完整任务列表 (Full List)](https://github.com/sagiegurari/cargo-make/blob/master/docs/cargo_make_task_list.md)
        * [禁用预定义任务/流程 (Disabling Predefined Tasks/Flows)](#usage-predefined-flows-disable)
        * [修改预定义任务/流程 (Modifying Predefined Tasks/Flows)](#usage-predefined-flows-modify)
    * [最低版本要求 (Minimal Version)](#usage-min-version)
    * [性能调优 (Performance Tuning)](#usage-performance-tuning)
    * [命令组 / 子命令 (Command Groups / Subcommands)](#usage-command-groups)
    * [变更对比 (Diff Changes)](#usage-diff-changes)
    * [实验性特性 (Unstable Features)](#usage-unstable-features)
    * [命令行选项 (CLI Options)](#usage-cli)
    * [插件系统 (Plugins)](#usage-plugins)
        * [定义插件 (Defining Plugins)](#usage-plugins-defining-plugins)
        * [插件 SDK (Plugin SDK)](#usage-plugins-plugin-sdk)
        * [插件示例 - Docker 集成 (Plugin Example - Docker Integration)](#usage-plugins-plugin-example-dockerize)
        * [插件示例 - 并行运行工作区成员 (Plugin Example - Run workspace members in parallel)](#usage-plugins-plugin-example-parallel-workspace-members)
        * [插件示例 - 从 Rust 脚本加载环境变量 (Plugin Example - load Env From Rust Script)](#usage-plugins-plugin-example-rustenv)
        * [插件示例 - 添加更简洁的 Windows Powershell 支持 (Plugin Example - Adding Simpler Windows Powershell Support)](#usage-plugins-plugin-example-powershell)
    * [Shell 自动补全 (Shell Completion)](#usage-shell-completion)
        * [Bash](#usage-shell-completion-bash)
        * [zsh](#usage-shell-completion-zsh)
        * [Zsh 任务补全 (Zsh Task Completion)](usage-task-completion-zsh)
        * [Fig / Amazon CodeWhisperer 命令行补全](#usage-shell-completion-fig)
    * [全局配置 (Global Configuration)](#cargo-make-global-config)
* [Makefile 结构定义 (Makefile Definition)](#descriptor-definition)
* [任务命名约定 (Task Naming Conventions)](#task-name-conventions)
* [相关文章 (Articles)](#articles)
    * [入门与基础 (Introduction and Basics)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-1-of-5-introduction-and-basics-b19ced7e7057)
    * [扩展任务、平台覆盖与别名 (Extending Tasks, Platform Overrides, and Aliases)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-2-of-5-extending-tasks-platform-overrides-1527386dcf87)
    * [环境变量、条件判断、子任务与混合 (Environment Variables, Conditions, Sub Tasks, and Mixing)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-3-of-5-environment-variables-conditions-3c740a837a01)
    * [工作区支持、Init/End 任务与 Makefile (Workspace Support, Init/End Tasks, and Makefiles)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-4-of-5-workspace-support-init-end-tasks-c3e738699421)
    * [预定义任务、CI 支持与约定 (Predefined Tasks, CI Support, and Conventions)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-5-final-predefined-tasks-ci-support-and-4594812e57da)
* [项目徽章 (Badge)](#badge)
* [开发路线图 (Roadmap)](#roadmap)
* [编辑器支持 (Editor Support)](#editor-support)
    * [vim](#editor-support-vim)
    * [vs-code](#editor-support-vs-code)
* [贡献指南 (Contributing)](.github/CONTRIBUTING.md)
* [发布历史 (Release History)](https://github.com/sagiegurari/cargo-make/blob/master/CHANGELOG.md)
* [开源许可证 (License)](#license)

<a name="overview"></a>
## 总览 (Overview)
cargo-make 任务执行器能够定义和配置一系列任务，并将它们作为工作流（Flow）来执行。<br>
一个任务可以是一条命令、一段脚本、一段 Rust 代码，或者要执行的其他子任务。<br>
任务之间可以设置依赖关系，这些依赖任务将在该任务本身执行之前运行。<br>
通过简洁的 TOML 配置文件，你便能定义跨平台构建脚本，只需运行一条命令，即可执行编译、测试、生成文档、运行性能基准测试、执行安全审计等多种流程。

<a name="installation"></a>
## 安装 (Installation)
若要安装 cargo-make，只需运行以下命令：

```sh
cargo install --force cargo-make
```

该命令会将 cargo-make 安装到你的 `~/.cargo/bin` 目录下。<br>
请确保已将 `~/.cargo/bin` 目录添加到系统的 `PATH` 环境变量中。<br>
<br>
安装后你将获得两个可执行文件：*`cargo-make`* 和 *`makers`*<br>

* **cargo-make** - 这是一个 Cargo 插件，通过 **cargo make ...** 形式调用。
* **makers** - 一个独立的可执行文件，提供与 cargo-make 完全相同的功能和命令行参数，但它是直接独立运行的，无需作为 cargo 插件。

完整 CLI 选项说明请参考 [命令行选项 (Cli Options)](#usage-cli) 章节。

若要以最小特性集安装（例如不包含 TLS 支持），请运行以下命令：

```sh
cargo install --no-default-features --force cargo-make
```

<a name="installation-arch-linux"></a>
### Arch Linux

```sh
sudo pacman -S cargo-make
```

<a name="installation-binary-release"></a>
### 二进制发布版 (Binary Release)
在 [GitHub Releases 页面](https://github.com/sagiegurari/cargo-make/releases) 可以直接下载预编译的二进制包。<br>
每个版本均提供以下平台的二进制文件：

* x86_64-unknown-linux-gnu
* x86_64-unknown-linux-musl
* x86_64-apple-darwin
* x86_64-pc-windows-msvc
* aarch64-apple-darwin

<a name="usage"></a>
## 使用说明 (Usage)
使用 cargo-make 时，所有任务均通过 TOML 文件进行定义和配置。<br>
以下是帮助你快速上手的简单说明。

<a name="usage-simple"></a>
### 简单示例 (Simple Example)
为了运行一系列任务，首先必须在 TOML 文件中定义它们。<br>
例如，如果我们想编写一个具备以下执行流程的构建脚本：

* 格式化代码
* 清理旧的 target 目录
* 执行构建
* 执行测试

默认情况下，如果当前目录下存在 `Makefile.toml`，cargo-make 将读取该文件。

我们将创建如下内容的 `Makefile.toml` 文件：

```toml
[tasks.format]
install_crate = "rustfmt"
command = "cargo"
args = ["fmt", "--", "--emit=files"]

[tasks.clean]
command = "cargo"
args = ["clean"]

[tasks.build]
command = "cargo"
args = ["build"]
dependencies = ["clean"]

[tasks.test]
command = "cargo"
args = ["test"]
dependencies = ["clean"]

[tasks.my-flow]
dependencies = [
    "format",
    "build",
    "test"
]
```

我们可以通过以下命令执行该工作流：

```sh
cargo make my-flow
```

输出内容大致如下：

```console
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: my-flow
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: format
[cargo-make] INFO - Execute Command: "cargo" "fmt" "--" "--emit=files"
[cargo-make] INFO - Running Task: clean
[cargo-make] INFO - Execute Command: "cargo" "clean"
[cargo-make] INFO - Running Task: build
[cargo-make] INFO - Execute Command: "cargo" "build"
   Compiling bitflags v0.9.1
   Compiling unicode-width v0.1.4
   Compiling quote v0.3.15
   Compiling unicode-segmentation v1.1.0
   Compiling strsim v0.6.0
   Compiling libc v0.2.24
   Compiling serde v1.0.8
   Compiling vec_map v0.8.0
   Compiling ansi_term v0.9.0
   Compiling unicode-xid v0.0.4
   Compiling synom v0.11.3
   Compiling rand v0.3.15
   Compiling term_size v0.3.0
   Compiling atty v0.2.2
   Compiling syn v0.11.11
   Compiling textwrap v0.6.0
   Compiling clap v2.25.0
   Compiling serde_derive_internals v0.15.1
   Compiling toml v0.4.2
   Compiling serde_derive v1.0.8
   Compiling cargo-make v0.1.2 (file:///home/ubuntu/workspace)
    Finished dev [unoptimized + debuginfo] target(s) in 79.75 secs
[cargo-make] INFO - Running Task: test
[cargo-make] INFO - Execute Command: "cargo" "test"
   Compiling cargo-make v0.1.2 (file:///home/ubuntu/workspace)
    Finished dev [unoptimized + debuginfo] target(s) in 5.1 secs
     Running target/debug/deps/cargo_make-d5f8d30d73043ede

running 10 tests
test log::tests::create_info ... ok
test log::tests::get_level_error ... ok
test log::tests::create_verbose ... ok
test log::tests::get_level_info ... ok
test log::tests::get_level_other ... ok
test log::tests::get_level_verbose ... ok
test installer::tests::is_crate_installed_false ... ok
test installer::tests::is_crate_installed_true ... ok
test command::tests::validate_exit_code_error ... ok
test log::tests::create_error ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

[cargo-make] INFO - Running Task: my-flow
[cargo-make] INFO - Build done in 72 seconds.
```

现在我们创建了一个可以在任何平台上运行的构建脚本。

任务可以保存在任何 TOML 文件中。调用 cargo-make 时传入 `--makefile other-filename.toml` 即可使用指定的 `other-filename.toml` 文件开始处理。

**cargo-make 既可以通过 `cargo make` 命令作为 cargo 插件调用，也可以通过 `makers` 命令作为独立可执行文件调用。**<br>
<br>
**重要提示：如果你是在 Cargo 工作区（workspace）中运行此示例，需要在文件顶部添加以下配置：**<br>

```toml
[env]
CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = true
```
**有关工作区支持的更多详情，请参阅本文档中的相关章节。**

<a name="usage-task-dependencies-alias"></a>
### 任务、依赖与别名 (Tasks, Dependencies, and Aliases)
在很多情况下，某些任务会依赖于其他任务。<br>
例如，你希望在执行构建之前先格式化代码，在运行测试之前先执行构建。<br>
这样的工作流可以定义如下：

```toml
[tasks.format]
install_crate = "rustfmt"
command = "cargo"
args = ["fmt", "--", "--emit=files"]

[tasks.build]
command = "cargo"
args = ["build"]
dependencies = ["format"]

[tasks.test]
command = "cargo"
args = ["test"]
dependencies = ["build"]
```

当你运行：

```sh
cargo make --makefile ./my_build.toml test
```

它会尝试运行 `test`，发现它有依赖项，而这些依赖项又有自己的依赖项。<br>
因此，它会根据任务及其依赖关系为这些任务生成一个执行计划。<br>
在我们的例子中，它将按顺序调用：format -> build -> test。<br>

同一个任务绝不会被重复执行两次。例如，假设我们有：

```toml
[tasks.A]
dependencies = ["B", "C"]

[tasks.B]
dependencies = ["D"]

[tasks.C]
dependencies = ["D"]

[tasks.D]
script = "echo hello"
```

在这个例子中，A 依赖于 B 和 C，而 B 和 C 都依赖于 D。<br>
然而，任务 D 不会被调用两次。<br>
执行输出将如下所示：

```console
[cargo-make] INFO - Task: A
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: D
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/CNuU47tIix.sh"
hello
[cargo-make] INFO - Running Task: B
[cargo-make] INFO - Running Task: C
[cargo-make] INFO - Running Task: A
```

正如你所见，'hello' 只由任务 D 打印了一次，因为它只被调用了一次。<br>
但如果我们确实想让 D 执行两次呢？<br>
最简单的办法是复制一份任务 D，让 B 依赖 D，让 C 依赖 D2（D 的副本）。<br>
但复制可能导致 bug 和庞大臃肿的 makefile，因此我们提供了别名（Alias）机制。<br>
别名任务拥有自己的名称，并指向另一个任务。<br>
别名任务本身的所有其他定义都会被忽略。<br>
现在，如果我们希望 D 执行两次，可以这样写：

```toml
[tasks.A]
dependencies = ["B", "C"]

[tasks.B]
dependencies = ["D"]

[tasks.C]
dependencies = ["D2"]

[tasks.D]
script = "echo hello"

[tasks.D2]
alias="D"
```

现在 C 依赖于 D2，而 D2 是 D 的别名。<br>
该 makefile 的执行输出如下所示：

```console
[cargo-make] INFO - Task: A
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: D
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/HP0UD7pgoX.sh"
hello
[cargo-make] INFO - Running Task: B
[cargo-make] INFO - Running Task: D2
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/TuuZJkqCE2.sh"
hello
[cargo-make] INFO - Running Task: C
[cargo-make] INFO - Running Task: A
```

现在可以看到 'hello' 被打印了两次。

任务还可以依赖其他文件中的任务。
为此，请使用对象格式指定依赖，并提供目标文件路径。
cargo-make 会像处理命令行传入的路径一样处理该路径：
如果提供了文件名，它将搜索该文件；
否则它将在该路径下搜索默认的 `Makefile.toml`。

```toml
[tasks.install]
command = "mv"
args = ["src/B/out", "src/C/static"]
dependencies = [
  { name = "compile", path = "src/B" },
  { name = "clean", path = "src/C/tasks.toml" },
]
```

*提示：`run_task` 属性会告知任务在全新的执行计划中调用另一个任务。这也会导致依赖项被多次执行。*

你还可以定义特定于平台的别名，例如：

```toml
[tasks.my_task]
linux_alias = "linux_my_task"
windows_alias = "windows_my_task"
mac_alias = "mac_my_task"

[tasks.linux_my_task]

[tasks.mac_my_task]

[tasks.windows_my_task]
```

如果找到了与当前平台匹配的平台别名，它将优先于非平台别名定义。<br>
例如：

```toml
[tasks.my_task]
linux_alias = "run"
alias = "do_nothing"

[tasks.run]
script = "echo hello"

[tasks.do_nothing]
```

如果在 Windows 或 Mac 上运行任务 **my_task**，它将调用 **do_nothing** 任务。<br>
但是如果在 Linux 平台上执行，它将调用 **run** 任务。

*补充说明：除非依赖项本身被定义为子依赖，否则 cargo-make 会尽量按照定义的顺序调用任务依赖项。*

<a name="usage-task-command-script-task"></a>
### 命令、脚本与子任务 (Commands, Scripts, and Sub Tasks)
任务实际执行的操作可以通过 3 种方式来定义。<br>
以下分别进行说明：

* **run_task** - 调用在此属性中指定的另一个任务。与在当前任务之前调用的 dependencies（依赖项）不同，**run_task** 中定义的任务是在当前任务执行之后调用的。
* **command** - command 属性定义要调用的可执行文件。你可以使用 **args** 属性定义作为命令一部分传递的命令行参数。
* **script** - 执行脚本。你可以使用 **script_runner** 属性更改用于调用该脚本的可执行文件。如果未定义，将使用平台默认运行器（Windows 为 `cmd`，其他平台为 `sh`）。

只能使用其中一种定义。<br>
如果定义了多个属性（例如同时定义了 command 和 script），任务在调用时将会失败。

script 属性也可以存放非操作系统级别的脚本，例如待编译并执行的 Rust 代码。<br>
为了使用非操作系统脚本运行器，必须定义带有 **@** 前缀的特殊 script_runner。<br>
目前支持以下特殊的 runner：

* **@duckscript** - 执行指定的 duckscript 代码。参见 [示例](#usage-task-command-script-task-exampleduckscript)
* **@rust** - 编译并执行指定的 Rust 代码。参见 [示例](#usage-task-command-script-task-examplerust)
* **@shell** - 在 Windows 平台上，会尝试将 shell 命令转换为 Windows batch 批处理命令（仅支持基础脚本）并执行；在其他平台上，脚本将原样执行。参见 [示例](#usage-task-command-script-task-exampleshell2batch)

下面是每种操作类型的基本示例。

<a name="usage-task-command-script-task-examplesubtask"></a>
#### 子任务 (Sub Task)
在这个示例中，如果我们执行 **flow** 任务，它将调用在 **run_task** 属性中定义的 **echo** 任务。

```toml
[tasks.echo]
script = "echo hello world"

[tasks.flow]
run_task = "echo"
```

下面更复杂的示例展示了定义多个任务名称以及为每个任务附加可选条件的能力。<br>
满足条件的**第一个**任务（或者完全没有定义任何条件时）将被调用。<br>
如果所有任务条件均不满足，则不会调用任何子任务。<br>
关于条件的更多信息请参阅 [条件执行章节](#usage-conditions)。

```toml
[tasks.test1]
command = "echo"
args = ["running test1"]

[tasks.test2]
command = "echo"
args = ["running test2"]

[tasks.test3]
command = "echo"
args = ["running test3"]

[tasks.test-default]
command = "echo"
args = ["running test-default"]

[tasks.test-routing]
run_task = [
    { name = "test1", condition = { platforms = ["windows", "linux"], channels = ["beta", "stable"] } },
    { name = "test2", condition = { platforms = ["mac"], rust_version = { min = "1.20.0", max = "1.30.0" } } },
    { name = "test3", condition_script = [ "somecommand" ] },
    { name = "test-default" }
]
```

还可以使用 **fork** 属性在分叉的子进程中运行子任务。<br>
这可以防止子任务中所做的任何环境变量更改影响父进程中的后续流程。<br>
在分叉子进程中调用子任务的示例：

```toml
[tasks.echo]
command = "echo"
args = ["hello world"]

[tasks.fork-example]
run_task = { name = "echo", fork = true }
```

**name** 属性可以包含单个任务名称，也可以包含任务列表。<br>
如果是列表，任务将按顺序依次调用。<br>
例如，下面的 **simple-multi** 和 **routing-multi** 展示了通过 **run_task** 定义多任务调用的不同方式：

```toml
[tasks.echo1]
command = "echo"
args = ["1"]

[tasks.echo2]
command = "echo"
args = ["2"]

[tasks.simple-multi]
run_task = { name = ["echo1", "echo2"] }

[tasks.routing-multi]
run_task = [
    { name = ["echo1", "echo2"] },
]
```

你还可以设置一个 **cleanup** 清理任务，在子任务结束后运行（即使子任务失败也会执行）。<br>
这仅在与 **fork=true** 属性结合使用时受支持。<br>
例如：<br>

```toml
[tasks.echo1]
command = "echo"
args = ["1"]

[tasks.echo2]
command = "echo"
args = ["2"]

[tasks.fail]
script =  "exit 1"

[tasks.cleanup]
command = "echo"
args = ["cleanup"]

[tasks.cleanup-example]
run_task = { name = ["echo1", "echo2", "fail"], fork = true, cleanup_task = "cleanup" }
```

若要并行运行多个任务，请在 `run_task` 对象中添加 **parallel = true**。<br>
例如：

```toml
[tasks.echo1]
command = "echo"
args = ["1"]

[tasks.echo2]
command = "echo"
args = ["2"]

[tasks.parallel-multi]
run_task = { name = ["echo1", "echo2"], parallel = true }
```

这允许并行运行独立的任务，从而提高流程的整体性能。<br>
请注意，如果使用了以下功能，并行调用任务可能会导致问题：

* 通过 **cwd** 属性设置任务的当前工作目录将导致所有并行任务都受到影响。
* 避免使用 **`CARGO_MAKE_CURRENT_TASK_`** 类型的环境变量，因为它们可能持有错误的值。

此外，在某些情况下，子进程可能会变成僵尸进程。<br>
你可以设置手动清理任务来解决此问题。

<a name="usage-task-command-script-task-examplecommand"></a>
#### 命令 (Command)
运行命令时，你还可以定义命令行参数，如下例所示，通过将插件名称作为命令行参数来调用 cargo 命令：

```toml
[tasks.build-with-verbose]
command = "cargo"
args = ["build", "--verbose", "--all-features"]
```

可以在命令和参数中提供环境变量，在运行时替换为实际值，例如：

```toml
[env]
SIMPLE = "SIMPLE VALUE"
ECHO_CMD = "echo"

[tasks.expand]
command = "${ECHO_CMD}"
args = [
    "VALUE: ${SIMPLE}"
]
```

cargo-make CLI 还支持所有任务均可访问的附加位置参数。<br>
以下示例打印这些附加参数：

```toml
[tasks.varargs]
command = "echo"
args = [
    "args are:", "${@}"
]
```

*对于原生系统脚本，请使用该原生脚本的语法。*<br>
*例如 Shell 中可以使用 `${0}`，Windows 批处理中可以使用 `%*`*

使用附加参数调用 cargo-make 将产生如下结果：

```console
> cargo make varargs arg1 arg2 arg3

[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: varargs
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: varargs
[cargo-make] INFO - Execute Command: "echo" "args are:" "arg1" "arg2" "arg3"
args are: arg1 arg2 arg3
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

在不带任何附加参数的情况下调用 cargo-make 将产生如下结果：

```console
> cargo make varargs

[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: varargs
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: varargs
[cargo-make] INFO - Execute Command: "echo" "args are:"
args are:
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

这也可以用于模板化展开，例如：

```toml
[tasks.varargs]
command = "echo"
args = [
    "args are:", "-o=${@}"
]
```

输出将为：

```console
> cargo make varargs arg1 arg2 arg3

[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: varargs
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: varargs
[cargo-make] INFO - Execute Command: "echo" "args are:" "arg1" "arg2" "arg3"
args are: -o=arg1 -o=arg2 -o=arg3
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

命令行参数还可以包含 [内置函数 (built-in functions)](#usage-functions)（见下文）。

<a name="usage-task-command-script-task-examplescript"></a>
#### 脚本 (Script)
下面是一个打印 hello world 的简单脚本：

```toml
[tasks.hello-world]
script = [
    "echo start...",
    "echo \"Hello World From Script\"",
    "echo end..."
]
```

你可以使用多行 TOML 字符串使脚本更具可读性，如下所示：

```toml
[tasks.hello-world]
script = '''
echo start...
echo "Hello World From Script"
echo end...
'''
```

cargo-make CLI 还支持所有任务均可访问的附加参数。<br>
以下示例打印附加参数：

```toml
[tasks.cli-args]
script = "echo args are: ${@}"
```

使用附加参数调用 cargo-make 将产生如下结果：

```console
> cargo make cli-args arg1 arg2 arg3

[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: cli-args
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: cli-args
+ cd /projects/rust/cargo-make/examples
+ echo args are: arg1 arg2 arg3
args are: arg1 arg2 arg3
[cargo-make] INFO - Running Task: end
```

在不带任何附加参数的情况下调用 cargo-make 将产生如下结果：

```console
> cargo make cli-args

[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: cli-args
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: cli-args
+ cd /projects/rust/cargo-make/examples
+ echo args are:
args are:
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

还可以使用 **file** 属性指向现有脚本文件（而不是将脚本内容保存在 makefile 内部），如下所示：

```toml
[tasks.hello-world-from-script-file]
script = { file = "script.sh" }
```

脚本文件路径始终相对于当前工作目录，除非使用 **absolute_path** 属性指定为绝对路径。例如：

```toml
[tasks.hello-world-from-script-file-absolute-path]
script = { file = "${CARGO_MAKE_WORKING_DIRECTORY}/script.sh", absolute_path = true }
```

文件路径支持环境变量替换。<br><br>
**建议优先使用命令（commands）而不是脚本（scripts），因为命令支持更多特性，例如 [自动安装依赖](#usage-installing-dependencies)、[参数内置函数](#usage-functions) 等等...**

为了在多个任务之间共享通用的脚本内容，你可以使用 script 的 pre/main/post 结构，如下所示：

```toml
[tasks.base-script]
script.pre = "echo start"
script.main = "echo old"
script.post = "echo end"

[tasks.extended-script]
extend = "base-script"
script.main = "echo new"
```

运行 extended-script 任务将打印：

```console
start
new
end
```

<a name="usage-task-command-script-task-exampleduckscript"></a>
#### Duckscript
[Duckscript](https://sagiegurari.github.io/duckscript/) 是一种非常简单的类 Shell 脚本语言，提供跨平台的 Shell 脚本能力。<br>
[Duckscript](https://sagiegurari.github.io/duckscript/) 内嵌在 cargo-make 内部，因此与其他脚本解决方案或系统命令不同，duckscript 可以直接从脚本内部修改 cargo-make 的环境变量。<br>
此外，你还可以在 duckscript 脚本内直接运行 cargo-make 任务。<br>
这实现了与 cargo-make 的高度双向集成。

```toml
[tasks.duckscript-example]
script_runner = "@duckscript"
script = '''
task_name = get_env CARGO_MAKE_CURRENT_TASK_NAME
echo 当前运行的 cargo make 任务是: ${task_name}

# 由于 cargo-make 会将所有环境变量自动作为 duckscript 变量加载
# 因此你可以直接访问它们
echo 当前运行的 cargo make 任务是: ${CARGO_MAKE_CURRENT_TASK_NAME}

cd .. # 这会改变 cargo-make 的当前工作目录（脚本执行完成后 cargo-make 会恢复原始目录）
pwd
set_env CARGO_MAKE_CURRENT_TASK_NAME tricking_cargo_make
'''
```

下一个示例展示了如何从 duckscript 中调用 cargo-make 任务：

```toml
[tasks.run-task-from-duckscript]
script_runner = "@duckscript"
script = '''
echo 首次调用 echo1 任务:
cm_run_task echo1
echo 第二次调用 echo1 任务:
cm_run_task echo1

echo 运行任务: echo2:
cm_run_task echo2
'''

[tasks.echo1]
command = "echo"
args = ["1"]

[tasks.echo2]
command = "echo"
args = ["2"]
```

与操作系统原生脚本一样，@duckscript 运行器也支持访问 cargo-make 命令行参数。<br>
此外，所有环境变量都会预先加载为 duckscript 变量，可直接从脚本中读取（无需调用 **get_env** 命令！）。

<a name="usage-task-command-script-task-examplerust"></a>
#### Rust 代码 (Rust Code)
在此示例中，当调用 **rust** 任务时，**script** 中的内容将被编译并执行。
你可以在代码内部看到如何以 `Cargo.toml` 格式定义依赖项。

```toml
[tasks.rust]
script_runner = "@rust"
script = '''
//! ```cargo
//! [dependencies]
//! envmnt = "*"
//! ```
fn main() {
    let value = envmnt::get_or("PATH", "NO PATH VAR DEFINED");
    println!("Path Value: {}", &value);
}
'''
```

与操作系统脚本一样，@rust 运行器也支持访问 cargo-make CLI 命令行参数。<br>
目前有几种不同的 Rust 脚本运行器可用：

* [rust-script](https://crates.io/crates/rust-script)
* [cargo-script](https://crates.io/crates/cargo-script)
* [cargo-play](https://crates.io/crates/cargo-play)

默认情况下使用 rust-script，但可以通过环境变量 **`CARGO_MAKE_RUST_SCRIPT_PROVIDER`**（填入对应的 crate 名称）进行更改。<br>
这使得可以在具体任务的 **env** 块中为每个任务单独定义不同的运行器。<br>
例如：

```toml
[tasks.rust-script]
env = { "CARGO_MAKE_RUST_SCRIPT_PROVIDER" = "rust-script" }
script_runner = "@rust"
script = '''
fn main() {
    println!("test");
}
'''

[tasks.cargo-script]
env = { "CARGO_MAKE_RUST_SCRIPT_PROVIDER" = "cargo-script" }
script_runner = "@rust"
script = '''
fn main() {
    println!("test");
}
'''

[tasks.cargo-play]
env = { "CARGO_MAKE_RUST_SCRIPT_PROVIDER" = "cargo-play" }
script_runner = "@rust"
script = '''
fn main() {
    println!("test");
}
'''
```

请注意，每个运行器定义 Rust 脚本所用依赖项的语法各有不同。<br>
详情请参阅对应 crate 的官方文档。

<a name="usage-task-command-script-task-exampleshell2batch"></a>
#### 跨平台 Shell (Cross Platform Shell)
在此示例中，当调用 **shell** 任务时，**script** 内容在 Windows 平台上运行时将自动转换为 Windows 批处理命令并执行。

```toml
[tasks.shell]
script_runner = "@shell"
script = '''
rm ./myfile.txt
'''
```

与操作系统脚本一样，@shell 运行器也支持访问 cargo-make CLI 命令行参数。<br>
<br>
完整功能支持请参阅 [shell2batch](https://github.com/sagiegurari/shell2batch) 项目。

<a name="usage-task-command-script-task-examplegeneric"></a>
#### 其他编程语言 (Other Programming Languages)
cargo-make 还可以运行使用各种脚本语言编写的脚本，例如 Python、Perl、Ruby、JavaScript 等等...<br>
只要运行器符合 **命令 文件** 的调用形式（例如 **`python ./program.py`**），均受支持。

以下是几个示例：

```toml
[tasks.python]
script_runner = "python"
script_extension = "py"
script = '''
print("Hello, World!")
'''

[tasks.perl]
script_runner = "perl"
script_extension = "pl"
script = '''
print "Hello, World!\n";
'''

[tasks.javascript]
script_runner = "node"
script_extension = "js"
script = '''
console.log('Hello, World!');
'''

[tasks.php]
script_runner = "php"
script_extension = "php"
script = '''
<?php
echo "Hello, World!\n";
'''

[tasks.powershell]
script_runner = "powershell"
script_extension = "ps1"
script = '''
Write-Host "Hello, World!"
'''
```

如果需要在脚本文件之前向脚本运行器传递参数，可以使用 **script_runner_args** 属性。<br>
例如：

```toml
[tasks.php-with-args]
script_runner = "php"
script_runner_args = ["-f"]
script_extension = "php"
script = '''
<?php
echo "Hello, World!\n";
'''
```

*注意：使用 script_runner_args 时必须同时定义 script_extension。*

<a name="usage-task-command-script-task-exampleshebang"></a>
#### Shebang 支持 (Shebang Support)
除了通过 **script_runner** 属性定义自定义运行器外，还可以在脚本的第一行使用 Shebang（`#!`）来指定。

如果在 Windows 上使用，请确保所用的解释器支持将 **#** 字符作为注释符（例如 `cmd.exe` 就不支持！），否则会导致执行错误。

使用 bash 的任务示例：

```toml
[tasks.shebang-sh]
script = '''
#!/usr/bin/env bash
echo hello
'''
```

输出：

```console
> cargo make --cwd ./examples --makefile ./shebang.toml shebang-sh
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: ./shebang.toml
[cargo-make] INFO - Task: shebang-sh
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: shebang-sh
[cargo-make] INFO - Execute Command: "/usr/bin/env" "bash" "/tmp/cargo-make/cJf6XEXrL9.sh"
hello
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

使用 Python 的任务示例：

```toml
[tasks.shebang-python]
script = '''
#!/usr/bin/env python3
print("Hello, World!")
'''
```

输出：

```console
> cargo make --cwd ./examples --makefile ./shebang.toml shebang-python
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: ./shebang.toml
[cargo-make] INFO - Task: shebang-python
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: shebang-python
[cargo-make] INFO - Execute Command: "/usr/bin/env" "python3" "/tmp/cargo-make/Wy3QMJiQaS.sh"
Hello, World!
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

Shebang 的另一个巧妙用法是直接指定特殊运行器（如 @duckscript）：

```toml
[tasks.duckscript-shebang-example]
script = '''
#!@duckscript
echo Running duckscript without runner attribute.
'''
```

不过该脚本语言必须支持以 **#** 开头的注释。

<a name="usage-default-tasks"></a>
### 默认任务与扩展 (Default Tasks and Extending)
实际上并没有必要定义前面示例中展示的一些基础 **build**、**test** 等任务。<br>
cargo-make 自带了一个内置的 toml 文件，作为每次执行的基础底座。<br>
在运行 cargo-make 时提供的**可选**外部 toml 文件，只需要扩展、添加或覆盖定义在 [默认 makefile](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/) 中的任务即可。<br>

以默认 toml 中定义的内置 **build** 任务为例：

```toml
[tasks.build]
description = "Runs the rust compiler."
category = "Build"
command = "cargo"
args = ["build", "--all-features"]
```

例如，如果你想为其增加详细输出（verbose）并移除 **--all-features** 标志，只需按如下方式修改 args 并添加 --verbose 即可：

```toml
[tasks.build]
args = ["build", "--verbose"]
```

如果你想禁用某个已有任务（同时也会禁用其依赖项），可以这样配置：

```toml
[tasks.build]
disabled = true
```

无需重新定义任务的已有属性，只需编写需要添加或覆盖的内容。<br>
默认 toml 文件自带了许多已经内置好的步骤和流程，因此值得先去查阅了解。<br>

如果你确实希望在扩展任务中删除原任务的所有属性，可以使用 **clear** 属性，如下所示：

```toml
[tasks.sometask]
clear = true
command = "echo"
args = [
    "extended task"
]
```

你还可以使用 **extend** 属性从外部 makefile 进一步扩展其他外部文件，例如：

```toml
extend = "my_common_makefile.toml"
```

**extend** 属性中的文件路径始终相对于当前所在 toml 文件的位置，而不是进程的工作目录。

当你在工作区中拥有一个包含所有通用自定义任务的 `Makefile.toml`，且每个子项目中都有一个简单的 `Makefile.toml` 指向工作区 makefile 时，extend 属性非常有用。

<a name="usage-workspace-extending-external-makefile"></a>
#### 扩展外部 Makefile (Extending External Makefiles)
为了让 makefile 能够通过 extend 属性扩展其他外部文件，例如：

```toml
extend = "my_common_makefile.toml"
```

extend 属性中的文件路径始终相对于当前所在 toml 文件的位置，而不是进程的工作目录。<br>
extend 属性所指向的 makefile 必须存在，否则构建将会失败。

为了定义可选的扩展 makefile，除了路径外，还需要传入 optional 标志，如下所示：

```toml
extend = { path = "does_not_exist_makefile.toml", optional = true }
```

你还可以定义一个要扩展的 makefile 列表。<br>
它们将按照你定义的顺序依次加载。<br>
例如：

```toml
extend = [
  { path = "alias.toml" },
  { path = "optional_makefile.toml", optional = true },
  { path = "cwd.toml" },
]
```

你还可以通过添加 relative 关键字，将相对于当前 makefile 所在位置的相对路径基准更改为 git 根目录、crate 根目录或工作区根目录：

```toml
extend = { path = "./examples/python.toml", relative = "crate" }
```

其中 relative 可以取以下值：

* git - 指向最近（向上查找）的 .git 文件夹位置。
* crate - 指向 crate 根目录（基于最内层第一个 Cargo.toml 文件）。
* workspace - 指向工作区根目录（基于最顶层第二个 Cargo.toml 文件）。

任何其他值均默认为当前 makefile 所在位置。<br>
需要特别说明的是，所有路径都是基于当前正在解析的 makefile 而言的。

<a name="usage-workspace-extend"></a>
#### 自动扩展工作区 Makefile (Automatically Extend Workspace Makefile)
在对工作区所属模块运行 cargo make 时，你可以让子成员 crate 的 makefile（即使其不存在）自动继承扩展工作区级别的 makefile。

工作区级 makefile 的 **env** 部分必须包含以下环境变量（你也可以通过 CLI 命令行设置）：

```toml
[env]
CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = true
```

这使你能够为整个工作区维护一份单独的 makefile，同时在每个成员 crate 中都能访问这些自定义任务。
<br>
**注意：这仅适用于在工作区根目录下触发的工作区构建。<br>
直接在成员 crate 目录下启动的流程，必须使用 extend 关键字手动扩展工作区级别的 makefile。**

<a name="usage-load-scripts"></a>
#### 加载脚本 (Load Scripts)
在更复杂的场景中，你可能希望多个无关的项目共享一些通用的自定义任务。例如，你可能希望将构建状态通知给公司内部服务器。<br>
与其在每个项目中重复定义这些任务，不如创建一个包含这些定义的单一 toml 文件，让所有项目都扩展该文件。<br>
但是，“extend” 功能只能在本地文件系统中查找文件。因此，为了从远程服务器拉取公共 toml（使用 `http` 或 `git clone` 等...），可以使用加载脚本（Load Scripts）。

加载脚本在 config 配置块中使用 **load_script** 属性定义，并且在解析评估 extend 属性**之前**被调用。<br>
这允许你先从远程服务器拉取 toml 文件，并将其放置在 extend 属性定义的位置。

以下是通过 HTTP 从远程服务器下载公共 toml 的加载脚本示例：

```toml
[config]
load_script = "wget -O /home/myuser/common.toml companyserver.com/common.toml"
```

以下是从 git 仓库拉取公共 toml 文件的示例：

```toml
[config]
load_script = "git clone git@mygitserver:user/project.git /home/myuser/common"
```

你可以运行任何所需的命令或命令集。因此，你可以构建更加复杂的逻辑来决定如何以及从何处拉取公共 toml 文件，以及将其保存到何处。<br>
如果需要，还可以使用 **linux_load_script**、**windows_load_script** 和 **mac_load_script** 属性按平台分别覆盖 load_script。

<a name="usage-predefined-makefiles"></a>
#### 预定义 Makefile (Predefined Makefiles)

虽然 cargo-make 自带了许多内置任务（定义在 [默认 makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/) 中），但它们并不总是适用于每个项目。<br>
[cargo-make-tasks](https://github.com/sagiegurari/cargo-make-tasks/) 仓库维护了一组可供加载的额外 makefile，用于替换 cargo-make 的内置任务。<br>
例如 cmake.toml 为使用 cmake 的项目提供了 cmake 相关的任务。

更多信息和使用示例请参阅 [cargo-make-tasks](https://github.com/sagiegurari/cargo-make-tasks/) 仓库。

<a name="usage-default-task"></a>
#### 默认任务 (The Default Task)

如果在调用 cargo make 命令时没有指定任务名称，将执行默认任务。<br>
默认任务实际上是另一个任务的别名，定义如下：

```toml
[tasks.default]
alias = "dev-test-flow"
```

可以通过多种方式重新定义默认任务，例如：

* 别名指向自定义 makefile 中的另一个任务：

```toml
[tasks.default]
alias = "my-custom-task"
```

* 清除别名并直接定义任务行为：

```toml
[tasks.default]
clear = true # 清除原别名
command = "echo"
args = ["custom!!!"]
```

<a name="usage-extending-tasks"></a>
### 扩展任务 (Extending Tasks)

在同一个 makefile 或从扩展的 makefile 中，有多种扩展任务的方式：

* [任务覆盖 (Task Override)](#usage-task-override)
* [平台覆盖 (Platform Override)](#usage-platform-override)
* [extend 属性 (Extend Attribute)](#usage-task-extend-attribute)

<a name="usage-task-override"></a>
#### 任务覆盖 (Task Override)
cargo-make 附带了许多预定义的任务和工作流，无需在项目中重新定义即可直接使用。<br>
但在某些情况下，你可能希望对其稍作调整以满足需求，而无需完全重写整个任务。<br>
以 cargo-make 内部预定义的 **build** 任务为例：

```toml
[tasks.build]
description = "Runs the rust compiler."
category = "Build"
command = "cargo"
args = ["build", "--all-features"]
```

如果你不想使用 **--all-features** 模式，只需在外部 `Makefile.toml` 中修改该任务的 args 属性：

```toml
[tasks.build]
args = ["build"]
```

当 cargo-make 启动时，它会加载外部 `Makefile.toml` 和内部 makefile 定义并将它们合并。<br>
由于外部文件会覆盖内部定义，因此仅重新定义的 **build** 任务 args 属性会覆盖内部定义的 args 属性，最终实际生效的结果为：

```toml
[tasks.build]
description = "Runs the rust compiler."
category = "Build"
command = "cargo"
args = ["build"]
```

相同的过程也可用于覆盖通过 [扩展外部 Makefile](#usage-workspace-extending-external-makefile) 章节中使用 extend 关键字加载的其他 makefile 中的任务。

<a name="usage-platform-override"></a>
#### 平台覆盖 (Platform Override)
如果你想针对特定平台覆盖某个任务（或任务中的特定属性），可以在该任务下定义带有平台名称（当前支持 Linux、Windows 和 macOS）的覆盖子任务。<br>
例如：

```toml
[tasks.hello-world]
script = '''
echo "Hello World From Unknown"
'''

[tasks.hello-world.linux]
script = '''
echo "Hello World From Linux"
'''
```

如果在 Linux 上使用任务 'hello-world' 运行 cargo make，它将重定向到 hello-world.linux，而在其他平台上则会执行原始的 hello-world。<br>
在 Linux 上的输出将是：

```console
[cargo-make] INFO - Task: hello-world
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: hello-world
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/kOUJfw8Vfc.sh"
Hello World From Linux
[cargo-make] INFO - Build done in 0 seconds.
```

而在其他平台上将输出：

```console
[cargo-make] INFO - Task: hello-world
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: hello-world
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/2gYnulOJLP.sh"
Hello World From Unknown
[cargo-make] INFO - Build done in 0 seconds.
```

在平台覆盖任务中，你可以定义任何属性来覆盖父任务的属性，未定义的属性将沿用父任务的值且不会被修改。<br>
如果需要删除父任务中的属性（例如父任务定义了 command，但你希望在覆盖任务中定义 script），则必须使用 clear 属性清空覆盖任务中的父任务定义：

```toml
[tasks.hello-world.linux]
clear = true
script = '''
echo "Hello World From Linux"
'''
```

但这意味着你必须在覆盖任务中重新定义希望从父任务保留的所有属性。<br>
**重要提示：alias（别名）的检查先于覆盖任务，因此如果父任务带有别名，它将重定向到该别名任务，而不是执行覆盖任务。**<br>
**若要按平台重定向别名，请使用 linux_alias、windows_alias、mac_alias 属性。**<br>
**此外，别名不能在平台覆盖任务中定义，只能在父任务中定义。**

<a name="usage-task-extend-attribute"></a>
#### extend 属性 (Extend Attribute)
到目前为止介绍的覆盖功能支持从不同 makefile 或不同平台覆盖同名任务。<br>
然而，**extend** 关键字也可以在任务级别使用，允许你按名称继承并覆盖任何任务。<br>
看下面这个例子：

```toml
[tasks.1]
category = "1"
description = "1"
command = "echo"
args = ["1"]

[tasks.2]
extend = "1"
category = "2"
args = ["2"]

[tasks.3]
extend = "2"
args = ["3"]
```

当任务 **3** 被加载时，它加载任务 **2**，而任务 **2** 又加载任务 **1**。<br>
最终任务 **3** 的定义为：

```toml
[tasks.3]
extend = "2"
category = "2"
description = "1"
command = "echo"
args = ["3"]
```

运行任务 **3** 的输出将是：

```console
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: task_extend.toml
[cargo-make] INFO - Task: 3
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: 3
[cargo-make] INFO - Execute Command: "echo" "3"
3
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

<a name="usage-env"></a>
### 环境变量 (Environment Variables)
`cargo-make` 支持以多种方式定义环境变量，这些变量随后可在整个任务执行生命周期中被访问。

由于环境变量在 `cargo-make` 中扮演着极其重要的角色，因此它提供了多种声明式手段，在不同的粒度级别注入环境变量。

* [声明方式 (Declaration)](#env-declaration)
* [全局配置 (Global Configuration)](#usage-env-config)
* [任务级变量 (Task)](#usage-env-task)
* [命令行参数 (Command Line)](#usage-env-cli)
* [Env 文件 (Env File)](#usage-env-file)
* [环境初始化脚本 (Env Setup Scripts)](#usage-env-setup-scripts)
* [加载顺序 (Loading Order)](#usage-env-vars-loading-order)
* [关于顺序的注意事项 (Note about Ordering)](#env-note-about-ordering)
* [全局环境变量 (Global)](#usage-env-global)

<a name="env-declaration"></a>
#### 声明方式 (Declaration)

有多种声明环境变量的方式，每种方式都适用于特定的场景。

##### 简单键值 (Simple)

最常见的是定义简单的 `KEY=Value` 键值对，类似于 [dotenv](https://www.npmjs.com/package/dotenv) 和 [bash 脚本](https://www.gnu.org/software/bash/)。值可以使用其他变量，使用 `${variable}` 语法在运行时进行插值求值。

```toml
STRING = "value"
RUST_BACKTRACE = 1
BOOL_VALUE = true
COMPOSITE = "${BOOL_VALUE} ${RUST_BACKTRACE}"
```

##### 列表 (List)

`cargo-make` 还支持列表，在运行时会使用 `;` 拼接成一个字符串。

```toml
LIST_VALUE = [ "VALUE1", "VALUE2", "VALUE3" ]
```

##### 脚本求值 (Script)

`cargo-make` 支持使用简单的脚本。该脚本的输出将决定该环境变量的值。

脚本对象有两个附加参数：`multi_line` 和 `depends_on`。如果 `multi_line` 设置为 `true`，提供的脚本将被视为多行脚本执行。`depends_on` 是该脚本所依赖的环境变量列表，在重新排序时会被纳入考量；如果未设置，`cargo-make` 将在重新排序时尝试推断所使用的变量。

> **注意：** 这使用操作系统默认的命令运行器（Windows 上为 `cmd`，UNIX 系统上为 `sh`），**不**支持 `duckscript`、`rust` 等其他运行器。

```toml
EVALUATED_VAR = { script = ["echo SOME VALUE"] }
```

##### 解码映射表 (Decode Map)

`cargo-make` 支持映射表匹配，将 `source` 与一组可能的 `mapping` 字典进行匹配，字典中的每个键与求值后的 `source` 进行比较。如果键与 `source` 相同，该键对应的值即为环境变量的值。如果没有匹配到任何键，则使用提供的 `default_value`。如果未提供默认值，则默认为空字符串。

```toml
LIBRARY_EXTENSION = { source = "${CARGO_MAKE_RUST_TARGET_OS}", default_value = "unknown", mapping = {"linux" = "so", "macos" = "dylib", "windows" = "dll", "openbsd" = "so" } }
```

##### 路径匹配 (Path)

`cargo-make` 支持使用 glob 通配符语法查找给定目录下的所有文件和目录。在执行过程中，文件列表将使用 `;` 拼接。

```toml
PATH_GLOB = { glob = "./src/**/mod.rs", include_files = true, include_dirs = false, ignore_type = "git" }
```

##### 条件变量 (Conditional)

`cargo-make` 支持条件变量，如果 `condition` 评估为 true，则将其设置为指定的 `value`。欲了解有关条件的更多信息，请参阅 [条件执行章节](#usage-conditions)。

##### 取消设置 (Unset)

可以取消设置环境变量：

```toml
VARIABLE = { unset = true }
```

<a name="usage-env-config"></a>
#### 全局配置 (Global Configuration)

可以使用顶层的 `[env]` 键全局设置环境变量，并能够提供多个 profile 配置环境，在执行 `cargo make` 时可以通过 `--profile <name>` 进行选择。

在全局 `[env]` 块[以及默认 `Makefile.toml`](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/stable.toml) 中设置的环境变量将在运行任何任务之前被设置。

##### 示例

```toml
[env]
RUST_BACKTRACE = 1
EVALUATED_VAR = { script = ["echo SOME VALUE"] }
TEST1 = "value1"
TEST2 = "value2"
BOOL_VALUE = true
DEV = false
PROD = false
COMPOSITE = "${TEST1} ${TEST2}"
MULTI_LINE_SCRIPT = { script = ["echo 1\necho 2"], multi_line = true }
CONDITIONAL_SCRIPT = { script = ["echo conditional_script"], condition = { env_not_set = ["CONDITIONAL_SCRIPT"] } }
LIBRARY_EXTENSION = { source = "${CARGO_MAKE_RUST_TARGET_OS}", default_value = "unknown", mapping = {"linux" = "so", "macos" = "dylib", "windows" = "dll", "openbsd" = "so" } }
TO_UNSET = { unset = true }
PREFER_EXISTING = { value = "new", condition = { env_not_set = ["PREFER_EXISTING"] } }
OVERWRITE_EXISTING = { value = "new", condition = { env_set = ["OVERWRITE_EXISTING"] } }
ENV_FROM_LIST = ["ARG1", "${SIMPLE}", "simple value: ${SIMPLE} script value: ${SCRIPT}"]
PATH_GLOB = { glob = "./src/**/mod.rs", include_files = true, include_dirs = false, ignore_type = "git" }

# 基于 profile 的环境覆盖
[env.development]
DEV = true

[env.production]
PROD = true
```

<a name="usage-env-task"></a>
#### 任务级变量 (Task)

可以在任务的作用域内设置环境变量，并在执行该任务时与全局环境合并。这意味着环境变量的求值发生在所有依赖项运行之后，但在该任务本身运行**之前**。

> **注意：** 任务变量与全局变量**不会**进行重新排序。任务只是直接覆盖先前声明的变量。

> **注意：** 执行后**不会**清理变量，这意味着执行任务之后的任务将继承前一个任务设置的变量。

`cargo-make` 在单独的任务级别支持与全局配置中概述的完全相同的能力。

```toml
[tasks.test-flow]
env = { "SOME_ENV_VAR" = "value" }
run_task = "actual-task"

[tasks.actual-task]
condition = { env_set = [ "SOME_ENV_VAR" ] }
script = '''
echo var: ${SOME_ENV_VAR}
'''
```

<a name="usage-env-cli"></a>
#### 命令行参数 (Command Line)
可以使用 `--env` / `-e` 参数在命令行中定义环境变量，如下所示：

```console
cargo make --env ENV1=VALUE1 --env ENV2=VALUE2 -e ENV3=VALUE3
```

<a name="usage-env-file"></a>
#### Env 文件 (Env File)
还可以将 env 文件路径作为 CLI 参数传入，如下所示：

```console
cargo make --env-file=./env/production.env
```

这允许使用相同的 `Makefile.toml`，但加载来自不同 env 文件的不同环境变量集。

env 文件是简单的 `key=value` 格式，类似于 [dotenv](https://www.npmjs.com/package/dotenv)，但只支持使用 `${}` 语法进行变量插值。

```properties
# 这是一段注释...
ENV1_TEST=TEST1
ENV2_TEST=TEST2
ENV3_TEST=VALUE OF ENV2 IS: ${ENV2_TEST}
```

环境文件的路径也可以在 `Makefile.toml` 的 `env_files` 键中全局定义，它们将按照定义的顺序加载。所有相对路径均相对于包含定义它们的 `Makefile.toml` 的目录。

> **注意：** `env_files` 也可以在任务级别使用。请注意，此时相对路径将相对于**当前工作目录**。

```toml
env_files = [
    "./env1.env",
    "./env2.env"
]
```

若要仅在变量尚未定义时加载环境变量，请使用 `defaults_only` 属性：

```toml
env_files = [
    { path = "./load_only_undefined.env", defaults_only = true },
    { path = "./load_all.env" }
]
```

使用 `profile` 属性可以指定仅在特定 profile 处于激活状态时才加载环境变量。

> 了解有关 profiles 的更多信息，请查阅 [配置环境章节](#usage-profiles)。

```toml
env_files = [
    { path = "./profile.env", profile = "development" },
    { path = "./env.env" }
]
```

<a name="usage-env-setup-scripts"></a>
#### 环境初始化脚本 (Env Setup Scripts)

环境设置脚本在环境变量文件和 env 块之后被调用。它们通过全局的 **env_scripts** 属性进行定义。这些脚本可以在启动工作流之前执行所需的任何前置逻辑。

如果是通过内嵌运行时调用的 `duckscript` 脚本，可以直接修改 `cargo-make` 运行时的环境变量。

例如：

```toml
env_scripts = [
'''
#!@duckscript
echo 第一个环境脚本...

composite_env_value = get_env COMPOSITE
echo COMPOSITE = ${composite_env_value}

set_env COMPOSITE_2 ${composite_env_value}
''',
'''
#!@duckscript
echo 第二个环境脚本...

composite_env_value = get_env COMPOSITE_2
echo COMPOSITE_2 = ${composite_env_value}
'''
]

[env]
SIMPLE = "SIMPLE VALUE"
SCRIPT = { script = ["echo SCRIPT VALUE"] }
COMPOSITE = "simple value: ${SIMPLE} script value: ${SCRIPT}"
```

在这个示例中，由于 **env** 块在 env scripts 之前执行，因此 `duckscript` 能够访问 `COMPOSITE` 环境变量。<br>
这些脚本利用该值创建了一个新的环境变量 **`COMPOSITE_2`**，并在第二个脚本中将其打印出来。

<a name="usage-env-vars-loading-order"></a>
#### 加载顺序 (Loading Order)

`cargo-make` 将按照以下顺序加载环境变量：

* 加载命令行中提供的环境文件（--env-file）
* 设置内部环境变量（参见 [全局环境变量](#usage-env-global) 章节）。**不包括任务级变量。**
* 加载 **env_files** 属性中定义的全局环境文件。
* 加载命令行中提供的全局环境变量（-e / --env）。
* 加载 **env** 块中定义的全局环境变量，以及基于 profile/附加 profiles 的相关子 env 块。
* 加载 **env.[当前 profile]** 块中定义的全局环境变量。
* 加载 **env_scripts** 属性中定义的全局环境初始化脚本。
* **按任务执行时 (Per Task)：**
  * 设置**当前任务**的内部环境变量（参见 [全局环境变量](#usage-env-global) 章节）。
  * 加载任务中 **env_files** 属性定义的环境文件（相对路径处理方式与全局 env_files 不同）。
  * 加载任务中 **env** 块定义的环境变量（行为与全局 env 块相同）。

在每个步骤中，变量都可以重新排序以确保满足所有变量依赖。在每次任务运行之前都会进行环境变量插值展开。

<a name="env-note-about-ordering"></a>
#### 关于顺序的注意事项 (Note about Ordering)

在 `cargo-make` 中，环境变量的顺序在定义与求值之间不一定相同。`cargo-make` 会分析具体的值，并根据它们所引用的变量对环境变量进行重新排序。

这种行为带来了许多好处，例如可以自由引用其他变量，或者在不同作用域中重新定义它们。

```toml
[env]
VAR1="${VAR2}"
VAR2=2
```

简单朴素的实现会导致 `VAR1=""`, `VAR2=2`，这种行为往往非常出乎意料，尤其是在扩展已有的环境变量声明时。`cargo-make` 则不同，它采用了类似于 [`terraform`](https://www.terraform.io) 等工具的方法，能够识别出 `VAR1` 依赖于 `VAR2`，从而输出 `VAR1=2`, `VAR2=2`。

```toml
[env]
VAR1="${VAR2}"

[env.prod]
VAR2=2

[env.devel]
VAR2=3
```

这是一个进阶示例，如果采用朴素实现将无法正常工作，因为不同的 profile 会与环境合并（基本上是追加方式）。在 `cargo-make` 中则不会出现这种情况，它能够识别依赖项并正确解析所有值。

###### 朴素实现结果对比：

```
--release=test
    VAR1=""
--release=prod
    VAR1=""
    VAR2=2
--release=devel
    VAR1=""
    VAR2=3
```

###### `cargo-make` 实现结果：

```
--release=test
    VAR1=""
--release=prod
    VAR1="2"
    VAR2=2
--release=devel
    VAR1="3"
    VAR2=3
```

<a name="usage-env-global"></a>
#### 全局环境变量 (Global)
除了手动设置环境变量外，cargo-make 还会自动注入一些内置环境变量，这些变量在运行任务脚本、命令、条件判断等场景下非常实用。

* **`CARGO_MAKE`** - 设置为 "true"，用于帮助子进程识别当前是在 `cargo make` 环境下运行。
* **`CARGO_MAKE_TASK`** - 保存正在执行的主任务名称。
* **`CARGO_MAKE_TASK_ARGS`** - 传递给 cargo-make 任务名称之后的参数列表，以 ';' 字符分隔。
* **`CARGO_MAKE_CURRENT_TASK_NAME`** - 保存当前正在执行的任务名称。
* **`CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE`** - 保存**最初**定义当前执行任务的 makefile 完整路径（对内置核心任务不可用）。
* **`CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY`** - 保存包含**最初**定义当前执行任务的 makefile 所在目录的完整路径（对内置核心任务不可用）。
* **`CARGO_MAKE_COMMAND`** - 用于调用 cargo-make 的命令（例如：*cargo make* 和 *makers*）。
* **`CARGO_MAKE_WORKING_DIRECTORY`** - 当前工作目录（可以通过设置 `--cwd` 命令行选项来指定）。
* **`CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY`** - 工作区的原始工作目录。使工作区成员可以访问工作区级别的 `CARGO_MAKE_WORKING_DIRECTORY`。
* **`CARGO_MAKE_PROFILE`** - 当前小写的 profile 名称（不应在全局/任务 env 块中手动修改）。
* **`CARGO_MAKE_ADDITIONAL_PROFILES`** - 附加 profile 名称（小写），以 `;` 字符分隔（不应在全局/任务 env 块中手动修改）。
* **`CARGO_MAKE_PROJECT_NAME`** - 对于独立 crate，与 `CARGO_MAKE_CRATE_NAME` 相同；对于工作区，默认为工作目录的基础名称。
* **`CARGO_MAKE_PROJECT_VERSION`** - 对于独立 crate，与 `CARGO_MAKE_CRATE_VERSION` 相同；对于工作区，为主要 crate 的版本（主要 crate 由 config 部分中的可选 **main_project_member** 属性定义）。
* **`CARGO_MAKE_CARGO_HOME`** - 如 [cargo 官方文档](https://doc.rust-lang.org/cargo/guide/cargo-home.html) 中所述的 `CARGO_HOME` 路径。
* **`CARGO_MAKE_CARGO_PROFILE`** - 从 **`CARGO_MAKE_PROFILE`** 映射得到的 [cargo profile](https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections) 名称（未映射的值将默认为 `CARGO_MAKE_PROFILE` 的值）。
* **`CARGO_MAKE_RUST_VERSION`** - Rust 版本号（例如 1.20.0）。
* **`CARGO_MAKE_RUST_CHANNEL`** - Rust 发行通道（stable、beta、nightly）。
* **`CARGO_MAKE_RUST_TARGET_ARCH`** - x86、x86_64、arm 等 ...（参见 rust cfg 特性）。
* **`CARGO_MAKE_RUST_TARGET_ENV`** - gnu、msvc 等 ...（参见 rust cfg 特性）。
* **`CARGO_MAKE_RUST_TARGET_OS`** - Windows、macOS、iOS、Linux、Android 等 ...（参见 rust cfg 特性）。
* **`CARGO_MAKE_RUST_TARGET_POINTER_WIDTH`** - 指针宽度：32、64。
* **`CARGO_MAKE_RUST_TARGET_VENDOR`** - apple、pc、unknown 等。
* **`CARGO_MAKE_RUST_TARGET_TRIPLE`** - x86_64-unknown-linux-gnu、x86_64-apple-darwin、x86_64-pc-windows-msvc 等 ...
* **`CARGO_MAKE_CRATE_TARGET_DIRECTORY`** - 获取 cargo 存放构建输出的目标目录，遵循 `${CARGO_TARGET_DIR}`、`.cargo/config.toml` 和 `${CARGO_HOME}/config.toml`，但不考虑 `--target-dir` 命令行标志。
* **`CARGO_MAKE_CRATE_CUSTOM_TRIPLE_TARGET_DIRECTORY`** - 类似于 `CARGO_MAKE_CRATE_TARGET_DIRECTORY`，但遵循 `.cargo/config.toml` 中的 `build.target`。
* **`CARGO_MAKE_CRATE_HAS_DEPENDENCIES`** - 根据 `Cargo.toml` 中是否定义了依赖项返回 `true`/`false`（如果未找到 `Cargo.toml` 则定义为 *false*）。
* **`CARGO_MAKE_CRATE_IS_WORKSPACE`** - 根据当前是否为工作区 crate 返回 `true`/`false`（即使未找到 `Cargo.toml` 也会被定义）。
* **`CARGO_MAKE_CRATE_WORKSPACE_MEMBERS`** - 保存成员路径列表（如果未找到 `Cargo.toml` 则定义为空值）。
* **`CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER`** - 保存当前正在构建的工作区成员名称（仅在流程作为工作区级流程启动时有效）。
* **`CARGO_MAKE_CRATE_LOCK_FILE_EXISTS`** - 当前工作目录中是否存在 `Cargo.lock` 文件时返回 `true`/`false`（在工作区项目中，每个成员拥有不同的工作目录）。
* **`CARGO_MAKE_CRATE_TARGET_TRIPLE`** - 获取默认构建所使用的目标三元组，遵循 `.cargo/config.toml` 和 `${CARGO_HOME}/config.toml`。
* **`CARGO_MAKE_WORKSPACE_PACKAGE_NAME`** - 保存当前工作目录中 `Cargo.toml` 文件定义的工作区根包名。
* **`CARGO_MAKE_WORKSPACE_PACKAGE_VERSION`** - 保存当前工作目录中 `Cargo.toml` 文件定义的工作区根包版本。
* **`CARGO_MAKE_WORKSPACE_PACKAGE_DESCRIPTION`** - 保存当前工作目录中 `Cargo.toml` 文件定义的工作区根包描述。
* **`CARGO_MAKE_WORKSPACE_PACKAGE_LICENSE`** - 保存当前工作目录中 `Cargo.toml` 文件定义的工作区根包许可证。
* **`CARGO_MAKE_WORKSPACE_PACKAGE_DOCUMENTATION`** - 保存当前工作目录中 `Cargo.toml` 文件定义的工作区根包文档链接。
* **`CARGO_MAKE_WORKSPACE_PACKAGE_HOMEPAGE`** - 保存当前工作目录中 `Cargo.toml` 文件定义的工作区根包主页链接。
* **`CARGO_MAKE_WORKSPACE_PACKAGE_REPOSITORY`** - 保存当前工作目录中 `Cargo.toml` 文件定义的工作区根包仓库链接。
* **`CARGO_MAKE_CI`** - 任务是否在持续集成系统（如 Travis CI、GitHub Actions 等）中运行（`true`/`false`）。
* **`CARGO_MAKE_PR`** - 任务是否作为 Pull Request 构建在 CI 系统中运行（`true`/`false`，未知时设为 false）。
* **`CARGO_MAKE_CI_BRANCH_NAME`** - CI 分支名称（如果可用）。
* **`CARGO_MAKE_CI_VENDOR`** - CI 供应商名称（如果可用）。
* **`CARGO_MAKE_DUCKSCRIPT_VERSION`** - 内嵌的 `duckscript` 运行时版本。
* **`CARGO_MAKE_DUCKSCRIPT_SDK_VERSION`** - 内嵌的 `duckscript` SDK 版本。

如果 `Cargo.toml` 文件存在并且定义了相关值，cargo-make 还会设置以下环境变量：

* **`CARGO_MAKE_CRATE_NAME`** - 当前工作目录中 `Cargo.toml` 文件定义的 crate 名称。
* **`CARGO_MAKE_CRATE_FS_NAME`** - 与 `CARGO_MAKE_CRATE_NAME` 相同，但部分字符被替换（例如 '-' 替换为 '_'）。
* **`CARGO_MAKE_CRATE_VERSION`** - 当前工作目录中 `Cargo.toml` 文件的 crate 版本。
* **`CARGO_MAKE_CRATE_DESCRIPTION`** - 当前工作目录中 `Cargo.toml` 文件的 crate 描述。
* **`CARGO_MAKE_CRATE_LICENSE`** - 当前工作目录中 `Cargo.toml` 文件的 crate 许可证。
* **`CARGO_MAKE_CRATE_DOCUMENTATION`** - 当前工作目录中 `Cargo.toml` 文件的文档链接。
* **`CARGO_MAKE_CRATE_HOMEPAGE`** - 当前工作目录中 `Cargo.toml` 文件的项目主页链接。
* **`CARGO_MAKE_CRATE_REPOSITORY`** - 当前工作目录中 `Cargo.toml` 文件的源码仓库链接。

如果项目是 Git 仓库的一部分，cargo-make 还会自动设置以下环境变量：

* **`CARGO_MAKE_GIT_BRANCH`** - 当前分支名称。
* **`CARGO_MAKE_GIT_USER_NAME`** - 从 git config 中的 `user.name` 获取的用户名。
* **`CARGO_MAKE_GIT_USER_EMAIL`** - 从 git config 中的 `user.email` 获取的用户邮箱。
* **`CARGO_MAKE_GIT_HEAD_LAST_COMMIT_HASH`** - 最新的 HEAD commit 完整哈希值。
* **`CARGO_MAKE_GIT_HEAD_LAST_COMMIT_HASH_PREFIX`** - 最新的 HEAD commit 短哈希前缀。

<a name="usage-setting-up-working-directory"></a>
### 设置工作目录 (Setting Up Working Directory)
若要修改特定任务（而非整个运行过程）的当前工作目录，请使用 **cwd** 属性。<br>
例如：

```toml
[tasks.move-dir]
cwd = "./mysubdir/"
```

<a name="usage-ignoring-errors"></a>
### 忽略错误 (Ignoring Errors)
在某些情况下，你可能希望运行某些可选任务作为大型工作流的一部分，但又不希望这些可选任务出错时打断整个构建流程。<br>
对于这些任务，可以添加 **ignore_errors = true** 属性：

```toml
[tasks.unstable_task]
ignore_errors = true
```

<a name="usage-conditions"></a>
### 条件执行 (Conditions)
条件机制允许你在运行时评估是否执行某个特定任务。<br>
这些条件会在任务运行其依赖安装或具体命令之前进行评估；如果不满足条件，该任务将不会被调用。<br>
但是，任务本身的依赖项不受父任务条件结果的影响（即依赖项仍会先行执行）。

条件有两种类型：

* [判定准则 (Criteria)](#usage-conditions-structure)
* [条件脚本 (Scripts)](#usage-conditions-script)

任务执行器会评估所有已定义的条件，一个任务定义中可以同时包含这两种类型。

<a name="usage-conditions-structure"></a>
#### 判定准则 (Criteria)
condition 属性可以定义多个验证参数。<br>
所有定义的参数都必须满足，整个条件才为 true，任务才会执行。

以下是一个条件定义示例，检查我们当前运行在 Windows 或 Linux 上（但不是 macOS），且处于 beta 或 nightly 通道（但不是 stable）：

```toml
[tasks.test-condition]
condition = { platforms = ["windows", "linux"], channels = ["beta", "nightly"] }
script = '''
echo "condition was met"
'''
```

支持以下条件类型：

* **profile** - 详情参见 [配置环境 (profiles)](#usage-profiles)
* **os** - 操作系统名称列表（如 cfg!(target_os) 所定义的 Windows、macOS、iOS、Linux、Android 等）
* **platforms** - 平台名称列表（windows、linux、mac）
* **channels** - Rust 发行通道列表（stable、beta、nightly）
* **env_set** - 必须定义的环境变量列表
* **env_not_set** - 必须未定义的环境变量列表
* **env_true** - 必须定义且不能为以下任何值（不区分大小写）的环境变量列表：false、no、0 或空字符串
* **env_false** - 必须定义且必须为以下任何值（不区分大小写）的环境变量列表：false、no、0 或空字符串
* **env** - 必须定义且必须等于指定值的环境变量映射表
* **env_not** - 必须不等于指定值的环境变量映射表
* **env_contains** - 必须定义且必须包含（不区分大小写）指定值的环境变量映射表
* **rust_version** - 可选定义 min（最低）、max（最高）和/或特定 Rust 版本号
* **files_exist** - 检查其必须存在的文件绝对路径列表。支持环境变量替换，因此可以定义相对路径，如 **`${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml`**
* **files_not_exist** - 检查其必须不存在的文件绝对路径列表。支持环境变量替换，因此可以定义相对路径，如 **`${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml`**
* **files_modified** - 列出输入（input）和输出（output）的 glob 通配符。如果任何输入文件比所有输出文件更新，则满足条件。支持环境变量替换，因此可以定义相对路径，如 **`${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml`**

几个示例：

```toml
[tasks.test-condition]
condition = {
    profiles = ["development", "production"],
    platforms = ["windows", "linux"],
    channels = ["beta", "nightly"],
    env_set = [ "CARGO_MAKE_KCOV_VERSION" ],
    env_not_set = [ "CARGO_MAKE_SKIP_CODECOV" ],
    env = { "CARGO_MAKE_CI" = true, "CARGO_MAKE_RUN_CODECOV" = true },
    rust_version = { min = "1.20.0", max = "1.30.0" },
    files_exist = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml"],
    files_not_exist = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml"],
    files_modified = { input = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml", "./src/**/*.rs"], output = ["./target/**/myapp*"] }
}
```

若要设置自定义失败提示信息，请在 condition 对象中使用 **fail_message**，例如：

```toml
[tasks.test-condition-with-message]
condition = { platforms = ["windows"], fail_message = "Condition Failed." }
command = "echo"
args = ["condition was met"]
```

仅当日志级别为 verbose，或在 config 中将 reduce_output 标志设置为 false 时，才会打印失败提示信息：

```toml
[config]
reduce_output = false
```

<a name="usage-conditions-script"></a>
#### 条件脚本 (Scripts)
这些脚本在任务运行其依赖安装或执行命令之前被调用；如果条件脚本的退出状态码非零，则不会调用该任务。

下面是一个条件脚本示例，该脚本始终返回非零值，因此该命令永远不会被执行：

```toml
[tasks.never]
condition_script = """
exit 1
"""
command = "cargo"
args = ["build"]
```

条件脚本可用于确保仅在满足特定条件时才调用任务，例如是否已安装某个特定的第三方软件。

若要设置自定义失败信息，请在 condition 对象内使用 **fail_message**，例如：

```toml
[tasks.test-condition-script-with-message]
condition = { fail_message = "Condition Script Failed." }
condition_script = [
    "exit 1"
]
command = "echo"
args = ["condition was met"]
```

<a name="usage-conditions-and-or"></a>
#### And / Or / Group Or 组合

默认情况下，所有条件组以及每个组内的所有条件都会被评估，并采用 'AND'（逻辑与）规则确保所有条件均满足。<br>
不过，也支持其他条件组合模式：

* Or - 遍历所有组和组内的所有条件，只要有一个条件满足即可。
* GroupOr - 遍历每个组内的所有条件，只要组内有一个条件满足该组即算通过，但所有条件组都必须通过。

只需在 condition 对象中添加带有对应值的 condition_type 即可。<br>
例如：

```toml
[tasks.test-or-condition]
condition = { condition_type = "Or", env_true = [
  "TRUE_ENV",
  "FALSE_ENV",
], env_false = [
  "TRUE_ENV",
  "FALSE_ENV",
] }
script = '''
echo "condition was met"
'''
```

<a name="usage-conditions-and-subtasks"></a>
#### 结合条件与子任务 (Combining Conditions and Sub Tasks)

将条件与 run_task 结合，可以定义条件化的子工作流。<br>
例如，假设你有一个覆盖率流程，仅应在 Linux 上的 CI 构建中触发，并且仅当 `CARGO_MAKE_RUN_CODECOV` 环境变量被定义为 "true" 时运行：

```toml
[tasks.ci-coverage-flow]
description = "Runs the coverage flow and uploads the results to codecov."
condition = { platforms = ["linux"], env = { "CARGO_MAKE_CI" = true, "CARGO_MAKE_RUN_CODECOV" = true } }
run_task = "codecov-flow"

[tasks.codecov-flow]
description = "Runs the full coverage flow and uploads the results to codecov."
windows_alias = "empty"
dependencies = [
    "coverage-flow",
    "codecov"
]
```

第一个任务 **ci-coverage-flow** 定义了条件，检查我们处于 Linux 环境、在 CI 构建中运行且 `CARGO_MAKE_RUN_CODECOV` 环境变量设置为 "true"。<br>
仅当所有条件都满足时，它才会运行 **codecov-flow** 任务。<br>
我们不能直接在 **codecov-flow** 任务上定义该条件，因为那样它会在检查条件之前就先行调用该任务的依赖项。

<a name="usage-running-tasks-only-if-sources-changed"></a>
#### 仅当源码变更时运行任务 (Running Tasks Only If Sources Changed)

**files_modified** 条件允许根据文件修改时间戳跳过任务。<br>
如果输入文件中没有任何一个文件比输出中的文件更新，该条件将导致任务被跳过。<br>
输入和输出定义为要检查的文件的 **glob 通配符**数组（非正则表达式）。<br>
在下面的示例中，如果 target 构建出的二进制文件比 Cargo.toml 或 src 目录中的任何 Rust 源文件更新，则不会运行 cargo build 命令：

```toml
[tasks.compile-if-modified]
condition = { files_modified = { input = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml", "./src/**/*.rs"], output = ["./target/**/myapp*"] } }
command = "cargo"
args = ["build"]
```

<a name="usage-installing-dependencies"></a>
### 安装依赖 (Installing Dependencies)

某些任务可能需要第三方 crate、rustup 组件或其他原生系统工具。<br>
cargo-make 提供了多种在运行任务之前安装配置这些依赖项的方式。

* [Cargo 插件 (Cargo Plugins)](#usage-installing-cargo-plugins)
* [Crates](#usage-installing-crates)
* [Rustup 组件 (Rustup Components)](#usage-installing-rustup-components)
* [系统原生依赖 (Native Dependencies)](#usage-installing-native-dependencies)
* [指定版本 (Defining Version)](#usage-installing-version)
* [全局版本锁定 (Global Lock Of Versions)](#usage-installing-locked)
* [安装优先级 (Installation Priorities)](#usage-installing-dependencies-priorities)
* [多依赖安装 (Multiple Installations)](#usage-installing-dependencies-multiple)

<a name="usage-installing-cargo-plugins"></a>
#### Cargo 插件 (Cargo Plugins)

当任务使用 **command** 属性调用 cargo 插件时，例如：

```toml
[tasks.audit]
command = "cargo"
args = ["audit"]
```

cargo-make 会首先检查该命令是否可用。<br>
只有在该命令不可用时，它才会尝试运行 **cargo install cargo-<第一个参数>** 来安装它。<br>
如果 cargo 插件的 crate 名称不同，可以通过 **install_crate** 属性手动指定。<br>
可以使用 **install_crate_args** 属性指定附加安装参数（例如版本号）。

若要禁用自动 crate 安装，可以将 **install_crate** 属性设置为 false，例如：

```toml
[tasks.test]
command = "cargo"
args = ["test"]
install_crate = false
```

<a name="usage-installing-crates"></a>
#### Crates

如果提供了相关的安装信息，cargo-make 可以验证是否已安装第三方 crate。<br>
它会首先检查 crate 是否已安装，只有在不可用时才会尝试安装。<br>
如果提供了组件名称，第三方 crate 的安装将首先通过 rustup 完成。<br>
如果 rustup 失败或未提供组件名称，它将回退使用 cargo install 命令。<br>
例如：

```toml
[tasks.rustfmt]
install_crate = { crate_name = "rustfmt-nightly", rustup_component_name = "rustfmt-preview", binary = "rustfmt", test_arg = "--help" }
command = "rustfmt"
```

在此示例中，cargo 会首先测试 **rustfmt --help** 命令是否可以正常执行；仅当失败时，它会首先尝试通过 rustup 安装组件 **rustfmt-preview**；若依然失败，则尝试针对 crate **rustfmt-nightly** 运行 cargo install。

如果需要传递多个参数，`test_arg` 可以包含一个参数数组。例如：

```toml
[tasks.doc-upload]
install_crate = { crate_name = "cargo-travis", binary = "cargo", test_arg = ["doc-upload", "--help"] }
command = "cargo"
args = ["doc-upload"]
```

在此示例中，cargo-make 将通过运行命令 `cargo doc-upload --help` 测试 cargo-travis 是否存在，并仅在命令执行失败时才安装该 crate。

<a name="usage-installing-rustup-components"></a>
#### Rustup 组件 (Rustup Components)

未作为 crate 发布的 rustup 组件，或者纯源码组件（没有可执行二进制文件），也可以通过 cargo-make 进行安装。<br>
以下示例演示如何安装带有二进制文件的 rustup 组件：

```toml
[tasks.install-rls]
install_crate = { rustup_component_name = "rls-preview", binary = "rls", test_arg = "--help" }
```

在此示例中，cargo-make 将首先检查 **rls** 二进制文件是否可用，只有在执行失败时才会使用 rustup 安装 **rls** 组件。<br>
<br>
某些 rustup 组件是纯源代码，因此在这些情况下，cargo-make 无法验证它们是否已安装，每次都会尝试安装它们。<br>
示例：

```toml
[tasks.install-rust-src]
install_crate = { rustup_component_name = "rust-src" }
```

<a name="usage-installing-native-dependencies"></a>
#### 系统原生依赖 (Native Dependencies)

也可以安装系统原生依赖，但这需要 Makefile 编写者编写脚本来检查依赖是否存在，如果不存在则正确安装。<br>
这可以通过在任务的 **install_script** 属性中编写安装脚本来实现。<br>
可以使用平台覆盖为 Linux/macOS/Windows 平台指定不同的安装脚本。<br>
例如：

```toml
[tasks.coverage-kcov]
windows_alias = "empty"
install_script = '''
KCOV_INSTALLATION_DIRECTORY=""
KCOV_BINARY_DIRECTORY=""
if [ -n "CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY" ]; then
    mkdir -p ${CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY}
    cd ${CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY}
    KCOV_INSTALLATION_DIRECTORY="$(pwd)/"
    cd -
    echo "Kcov Installation Directory: ${KCOV_INSTALLATION_DIRECTORY}"
    KCOV_BINARY_DIRECTORY="${KCOV_INSTALLATION_DIRECTORY}/build/src/"
    echo "Kcov Binary Directory: ${KCOV_BINARY_DIRECTORY}"
fi

# 获取帮助信息以提取所有支持的命令行参数
KCOV_HELP_INFO=`${KCOV_BINARY_DIRECTORY}kcov --help` || true

# 检查是否支持所需参数，否则进行安装
if [[ $KCOV_HELP_INFO != *"--include-pattern"* ]] || [[ $KCOV_HELP_INFO != *"--exclude-line"* ]] || [[ $KCOV_HELP_INFO != *"--exclude-region"* ]]; then
    # 检查是否在受支持的平台上
    if [ "$(grep -Ei 'debian|buntu|mint' /etc/*release)" ]; then
        echo "Installing/Upgrading kcov..."
        sudo apt-get update || true
        sudo apt-get install -y libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc binutils-dev

        mkdir -p ${CARGO_MAKE_KCOV_DOWNLOAD_DIRECTORY}
        cd ${CARGO_MAKE_KCOV_DOWNLOAD_DIRECTORY}
        KCOV_DOWNLOAD_DIRECTORY=$(pwd)

        wget https://github.com/SimonKagstrom/kcov/archive/v${CARGO_MAKE_KCOV_VERSION}.zip
        unzip v${CARGO_MAKE_KCOV_VERSION}.zip
        cd kcov-${CARGO_MAKE_KCOV_VERSION}
        mkdir -p build
        cd ./build
        cmake ..
        make

        # 如果是自定义安装目录，则将 kcov 保留在本地
        if [ -n "CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY" ]; then
            cd ${KCOV_DOWNLOAD_DIRECTORY}/kcov-${CARGO_MAKE_KCOV_VERSION}
            mv ./* ${KCOV_INSTALLATION_DIRECTORY}
        else
            sudo make install
            cd ../..
            rm -rf kcov-${CARGO_MAKE_KCOV_VERSION}
        fi
    fi
fi
'''
```

该任务检查是否已安装 kcov；如果未安装，它将安装 kcov 以及它所需的任何其他系统依赖。

<a name="usage-installing-version"></a>
#### 指定版本 (Defining Version)

可以定义所依赖 crate 的最低版本，例如：

```toml
[tasks.simple-example]
install_crate = { min_version = "0.0.1" }
command = "cargo"
args = ["make", "--version"]

[tasks.complex-example]
install_crate = { crate_name = "cargo-make", binary = "cargo", test_arg = ["make", "--version"], min_version = "0.0.1" }
command = "cargo"
args = ["make", "--version"]
```

这确保我们使用的 crate 版本支持构建所需的特性。<br>
目前定义 **min_version** 时有几个限制：

* 在任务中指定 **toolchain** 或在 install_crate 结构中指定 **rustup_component_name** 时，cargo-make 将忽略 min_version 值。
* 如果 cargo-make 由于任何错误无法检测当前安装的版本，cargo-make 将假定该版本有效并打印一条警告。

如果希望确保使用特定版本，可以定义 **version** 属性替代，例如：

```toml
[tasks.complex-example]
install_crate = { crate_name = "cargo-make", binary = "cargo", test_arg = ["make", "--version"], version = "0.0.1" }
command = "cargo"
args = ["make", "--version"]
```

<a name="usage-installing-locked"></a>
#### 全局版本锁定 (Global Lock Of Versions)

如果定义了 [min_version](#usage-installing-version)，
可以通过定义环境变量 **`CARGO_MAKE_CRATE_INSTALLATION_LOCKED=true`**，让 crate 安装命令自动添加 **--locked** 标志。
如果定义的是 version 而不是 min_version，这将自动设为 true。

<a name="usage-installing-alternate-cargo-install-commands"></a>
#### 备用 Cargo 安装命令 (Alternate Cargo Install Commands)

你可以指定不同的 cargo install 命令，以便 crate 安装使用某些自定义的 cargo 安装器插件。
例如，如果你希望使用诸如 **local-install** 之类的插件代替 **install**，只需添加带有相关值的 **install_command** 属性即可。<br>
例如：

```toml
[tasks.alt-command-example1]
install_crate = { install_command = "custom-install" }
command = "cargo"
args = ["somecrate"]

[tasks.alt-command-example2]
install_crate = { crate_name = "somecrate", install_command = "custom-install" }
```

默认情况下会添加 **--force** 标志。若要移除该标志，请在 install_crate 定义中添加 force = false，如下所示：

```toml
[tasks.alt-command-example2]
install_crate = { crate_name = "somecrate", install_command = "custom-install", force = false }
```

<a name="usage-installing-dependencies-priorities"></a>
### 安装优先级 (Installation Priorities)

每个任务只会调用一种类型的安装流程。<br>
以下按优先级排序定义了安装类型，cargo-make 据此决定调用哪种安装流程：

* **install_crate** - 允许安装 crate 和 rustup 组件。
* **install_script** - 自定义脚本，可用于安装或运行任务命令所需的任何内容。
* **automatic cargo plugin** - 如果命令是 **cargo**，cargo-make 将自动检查要安装哪个 cargo 插件（如果需要）。

如果定义了多种安装类型（例如同时定义了 install_crate 和 install_script），将仅根据上述优先级列表调用其中一种安装类型。

<a name="usage-installing-dependencies-multiple"></a>
### 多依赖安装 (Multiple Installations)

在某些情况下，任务需要安装多个项目才能正常运行。<br>
例如，你可能在同一个任务中需要 rustup 组件 **rls**、**rust-src** 以及 cargo 插件 **cargo-xbuild**。<br>
为了实现这一点，你可以将任务拆分为调用任务和安装任务，并将安装任务设置为依赖项。<br>
以下示例定义了一个由两个类似任务组成的工作流，它们具有相同的依赖项：cargo-xbuild crate、rls rustup 二进制组件和 rust-src rustup 纯源码组件。<br>
你可以将这两个 rustup 依赖项作为纯安装任务，将其设置为 xbuild 任务的依赖项。<br>
由于依赖项仅调用一次，这也确保了这些 rustup 组件不会被重复安装两次。

```toml
[tasks.install-rls]
# 仅在需要时安装 rls-preview
install_crate = { rustup_component_name = "rls-preview", binary = "rls", test_arg = "--help" }

[tasks.install-rust-src]
# 始终通过 rustup component add 安装 rust-src
install_crate = { rustup_component_name = "rust-src" }

[tasks.xbuild1]
# 运行 cargo xbuild，如果未安装 xbuild，将自动为你安装
command = "cargo"
args = [ "xbuild", "some arg" ]
dependencies = [ "install-rls", "install-rust-src" ]

[tasks.xbuild2]
# 运行 cargo xbuild，如果未安装 xbuild，将自动为你安装
command = "cargo"
args = [ "xbuild", "another arg" ]
dependencies = [ "install-rls", "install-rust-src" ]

[tasks.myflow]
dependencies = [ "xbuild1", "xbuild2" ]
```

<a name="usage-workspace-support"></a>
### 工作区支持 (Workspace Support)
如果 cargo-make 检测到当前工作目录是工作区根目录（即包含定义工作区及其成员的 `Cargo.toml` 的目录），它不会在该目录中直接调用所请求的任务。<br>
相反，它将在运行时生成一个任务定义，依次进入每个成员目录并在该成员上调用所请求的任务。<br>
例如，如果我们有以下目录结构：

```console
workspace
├── Cargo.toml
├── member1
│   └── Cargo.toml
└── member2
    └── Cargo.toml
```

当我们运行 **cargo make mytask** 时，它将进入每个工作区成员目录并在该目录下执行：**cargo make mytask**，
其中 mytask 是最初在工作区级别请求的任务。<br>
成员的执行顺序由工作区 `Cargo.toml` 中的 member 属性定义。

该流程被称为**工作区 (workspace) 工作流**，因为它识别工作区并为每个工作区成员处理请求，而定义工作区结构的根目录本身将被跳过。

我们可以利用这种能力在所有工作区成员 crate 上运行相同的功能，例如如果我们要格式化所有 crate，可以在工作区目录中运行：**cargo make format**。<br>

成员 crate 的 makefile 也可以自动继承扩展工作区目录的 makefile。<br>
更多信息请参阅 [相关章节](#usage-workspace-extend)。

<a name="usage-workspace-disabling-workspace-support"></a>
#### 禁用工作区支持 (Disabling Workspace Support)
如果你希望在工作区根目录下运行任务而不是在各个成员上运行（例如生成工作区级别的 README 文件），请在运行 cargo make 时使用 **`--no-workspace`** 命令行标志。<br>
例如：

```sh
cargo make --no-workspace mytask
```

这使 cargo-make 忽略该目录是工作区根目录的事实，仅作为包含 makefile 的普通目录运行普通流程。

在工作区级别调用任务（而不是针对每个成员调用）的另一种方法是，在工作区的 `Makefile.toml` 中将该任务的 **workspace** 属性设置为 false，如下所示：

```toml
[tasks.ignore-members]
workspace = false
```

为 cargo-make 命令行中请求的任务设置 **workspace = false** 等同于使用 **--no-workspace** 标志调用它。<br>
此标志仅对在 cargo-make 命令行中请求的任务进行检查，对于作为流程一部分执行的所有其他后续任务完全忽略。<br>
默认情况下，所有任务的 workspace 标志都设置为 true，但可以在 config 部分中进行全局配置：

```toml
[config]
default_to_workspace = false
```

在这种情况下，除非任务显式定义了 **workspace = true**，否则工作区级支持将**始终**被禁用。

<a name="usage-workspace-composite-flow"></a>
#### 复合流程 (Composite Flow)

你可以定义一个复合流程，既在工作区根目录下运行任务，又在成员目录下运行任务。<br>
以下是支持运行此类流程的工作区级 `Makefile.toml` 示例：

```toml
[tasks.composite]
dependencies = ["member_flow", "workspace_flow"]

[tasks.member_flow]
# 通过 fork，cargo make 启动并默认检测到这是一个工作区，并为每个成员运行 member_task
run_task = { name = "member_task", fork = true }

[tasks.workspace_flow]
# 运行某些工作区级别的命令或流程
```

你可以按如下方式启动该复合流程：

```sh
cargo make --no-workspace composite
```

<a name="usage-workspace-profiles"></a>
#### 工作区配置环境 (Profiles)

你可以通过将 **`CARGO_MAKE_USE_WORKSPACE_PROFILE`** 设置为 false，来防止 profile 传递给工作区成员：

```toml
[env]
CARGO_MAKE_USE_WORKSPACE_PROFILE = false
```

有关 profile 的更多信息请参阅 [配置环境章节](#usage-profiles)。

<a name="usage-workspace-support-skip-include-members"></a>
#### 跳过/包含特定成员 (Skipping/Including Specific Members)

在大多数情况下，你希望在所有成员上运行特定的流程，但在极少数情况下，你可能希望跳过特定成员。

通过将 **`CARGO_MAKE_WORKSPACE_SKIP_MEMBERS`** 环境变量设置为要跳过的成员名称（以数组形式），你可以定义不参与该流程的成员。

在下面的示例中，我们将跳过 member3 和 member4（应定义在工作区级别的 `Makefile.toml` 中）：

```toml
[env]
CARGO_MAKE_WORKSPACE_SKIP_MEMBERS = ["member3", "member4"]
```

你也可以定义 glob 通配符路径，例如：

```toml
[env]
CARGO_MAKE_WORKSPACE_SKIP_MEMBERS = "tools/*"
```

然而在某些情况下，你可能只想在满足特定条件时才跳过特定成员。<br>
例如，你只想在 Rust nightly 编译器上才构建某个成员模块。<br>
这是 member3 和 member4 条件跳过的简单示例（应定义在工作区级别的 `Makefile.toml` 中）：

```toml
[tasks.workspace-task]
condition = { channels = ["beta", "stable"] }
env = { "CARGO_MAKE_WORKSPACE_SKIP_MEMBERS" = ["member3", "member4"] }
run_task = { name = "member-task", fork = true }
```

你必须将其作为复合流程调用：

```sh
cargo make workspace-task --no-workspace
```

此外，你还可以指定相反的逻辑，即通过 **`CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS`** 环境变量指定要包含哪些成员。<br>
它遵循与 **`CARGO_MAKE_WORKSPACE_SKIP_MEMBERS`** 环境变量相同的规则。<br>
如果两者都定义了，则包含的成员将是非排除成员的子集，即两个过滤器同时生效。

<a name="usage-workspace-emulation"></a>
#### 工作区仿真 (Workspace Emulation)
工作区仿真允许你为项目创建类似工作区的结构，而无需实际定义 Rust 工作区。<br>
这意味着你可以拥有一个没有 `Cargo.toml` 的顶级项目目录，并拥有多个子 crate。<br>
这使得在根项目文件夹下可以对所有**成员** crate 运行 cargo make，而无需真正的 Cargo 工作区（真正的 Cargo 工作区会带来某些副作用，例如共享的 target 文件夹和依赖项）。

为了设置工作区仿真，你需要在工作区级别的 `Makefile.toml` 中定义以下内容：

```toml
[env]
# 这告知 cargo-make 该目录作为工作区根目录运行
CARGO_MAKE_WORKSPACE_EMULATION = true

# crate 成员列表。因为我们没有 Cargo.toml，所以需要在此处显式指定。
CARGO_MAKE_CRATE_WORKSPACE_MEMBERS = [
    "member1",
    "member2"
]
```

<a name="usage-toolchain"></a>
### 工具链支持 (Toolchain)
cargo-make 支持在任务定义中设置 **toolchain** 属性，以指定在调用命令和安装 Rust 依赖时所使用的工具链。<br>
以下示例展示如何分别打印当前安装的 stable 和 nightly 版本的 rustc：

```toml
[tasks.rustc-version-stable]
toolchain = "stable"
command = "rustc"
args = [ "--version" ]

[tasks.rustc-version-nightly]
toolchain = "nightly"
command = "rustc"
args = [ "--version" ]

[tasks.rustc-version-flow]
dependencies = [
    "rustc-version-stable",
    "rustc-version-nightly"
]
```

上述 **rustc-version-flow** 的示例输出为：

```console
[cargo-make] INFO - Task: rustc-version-flow
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: rustc-version-stable
[cargo-make] INFO - Execute Command: "rustup" "run" "stable" "rustc" "--version"
rustc 1.30.1 (1433507eb 2018-11-07)
[cargo-make] INFO - Running Task: rustc-version-nightly
[cargo-make] INFO - Execute Command: "rustup" "run" "nightly" "rustc" "--version"
rustc 1.32.0-nightly (451987d86 2018-11-01)
[cargo-make] INFO - Running Task: rustc-version-flow
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 2 seconds.
```

在与脚本（相对于系统命令）一起使用时，将为请求的工具链自动注入 **`CARGO`** 环境变量。<br>
以下示例展示如何分别打印 stable 和 nightly 的 CARGO 二进制文件路径：

```toml
[tasks.echo-cargo-stable]
toolchain = "stable"
script = '''
echo ${CARGO}
'''

[tasks.echo-cargo-nightly]
toolchain = "nightly"
script = '''
echo ${CARGO}
'''

[tasks.echo-cargo-all]
dependencies = ["echo-cargo-stable", "echo-cargo-nightly"]
```

上述 **echo-cargo-all** 的示例输出：

```console
[cargo-make] INFO - Task: echo-cargo-all
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: legacy-migration
[cargo-make] INFO - Running Task: echo-cargo-stable
/home/someuser/.rustup/toolchains/stable-armv7-unknown-linux-gnueabihf/bin/cargo
[cargo-make] INFO - Running Task: echo-cargo-nightly
/home/someuser/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/cargo
[cargo-make] INFO - Build Done in 4.44 seconds.
```

还可以为指定的 channel 断言 rustc 的最低版本要求。这有助于声明所需的编译器特性，并提醒开发者升级编译器。

```toml
[tasks.requires-stable-edition-2021]
toolchain = { channel = "stable", min_version = "1.56" }
command = "rustc"
args = ["--version"]
```

当工具链未安装，或者现有版本低于指定的 **min_version** 时，该任务将会失败。

<a name="usage-init-end-tasks"></a>
### Init 与 End 初始化与收尾任务 (Init and End tasks)
由 cargo-make 执行的每个任务或流程都会额外附带 2 个特殊任务。<br>
一个是在所有流程启动时调用的 init 任务，另一个是在所有流程结束时调用的 end 任务。<br>
init 和 end 任务的名称在 toml 文件的 config 部分中定义，以下是默认设置：

```toml
[config]
init_task = "init"
end_task = "end"

[tasks.init]

[tasks.end]
```

默认情况下，init 和 end 任务为空，可以通过外部 toml 文件进行修改，或者只需在外部 toml 文件中更改 init 和 end 任务的名称以指向不同的任务。<br>
无论你运行什么流程，这些任务都允许执行通用的全局前置/后置操作。

需要特别说明的是，init 和 end 任务的调用与其他普通任务不同：

* 别名（Aliases）和依赖项（dependencies）将被忽略。
* 如果在执行的流程中也显式定义了相同的任务，这些任务将被调用多次。

因此，不建议在自己的常规工作流中直接复用 init/end 任务名。

<a name="usage-catching-errors"></a>
### 捕获错误 (Catching Errors)
默认情况下，任何未设置 **ignore_errors = true** 的任务发生错误，都会导致整个工作流失败。<br>
然而在某些场景下，你可能希望在失败的流程结束之前执行某些清理工作。<br>
cargo-make 允许你定义一个 **on_error** 错误处理任务，该任务仅在流程失败时才会被调用。<br>
若要定义此特殊任务，必须在 Makefile 的 **config** 部分中添加 **on_error_task** 属性并指向你的任务，例如：

```toml
[config]
on_error_task = "catch"

[tasks.catch]
script = '''
echo "在 catch 中执行清理工作"
'''
```

<a name="usage-cargo-alias-tasks"></a>
### Cargo 别名任务 (Cargo Alias Tasks)

[Cargo 别名命令](https://doc.rust-lang.org/cargo/reference/config.html#alias) 可以作为 cargo-make 任务自动加载。<br>
若要自动加载它们，必须在 `Makefile.toml` 的 config 部分中配置以下内容：

```toml
[config]
load_cargo_aliases = true
```

config.toml 中定义的每个别名都将作为与该别名同名的任务被加载。<br>
如果已存在同名任务，则该别名将被忽略。<br>
任务定义将直接调用 cargo 以及别名对应的值，因此不会触发自动 cargo 插件安装。

<a name="usage-profiles"></a>
### 配置环境 (Profiles)

Profile 是用于定义自定义执行行为的实用工具。<br>
若要指定执行 profile，请使用 **`--profile`** 或 **`-p`** 命令行参数并提供 profile 名称。<br>
Profile 名称会自动转换为小写下划线形式并去除前后空白。<br>
如果未提供 profile 名称，profile 将默认为 **development**。

设置 Profile 的示例：

```sh
cargo make --profile production mytask
```

Profile 提供了多种能力：

* [环境变量覆盖](#usage-profiles-env)
* [基于 Profile 的条件判断](#usage-profiles-conditions)，例如：
```toml
condition = { profiles = ["development", "production"] }
```
* [内置环境变量](#usage-env-global) **`CARGO_MAKE_PROFILE`**：保存当前 profile 名称，可供条件、脚本和命令使用。

还可以使用 **additional_profiles** 同时激活多个 profile（支持度有限）。<br>
默认 profile（如果未通过命令行提供）为 `"development"`。<br>
但这可以通过设置环境变量 `CARGO_MAKE_DEFAULT_PROFILE` 进行覆盖。

```toml
[config]
additional_profiles = ["second_profile", "another_profile"]
```

附加 profile 可用于定义额外的环境变量块，它们将被保存在新的环境变量 **`CARGO_MAKE_ADDITIONAL_PROFILES`** 中。

<a name="usage-profiles-env"></a>
#### 环境变量覆盖 (Environment Variables)

Profile 允许你定义新的环境变量子集，这些变量仅在当前 profile 与配置的 env profile 匹配时才会在运行时生效。

```toml
[env]
RUST_BACKTRACE = "1"
EVALUATED_VAR = { script = ["echo SOME VALUE"] }
TEST1 = "value1"
TEST2 = "value2"
COMPOSITE = "${TEST1} ${TEST2}"

# 基于 profile 的环境覆盖
[env.development]
DEV = true

[env.production]
PROD = true
```

例如，给定以下带有 2 个基于 profile 的 env 映射表的 makefile：

```toml
[env]
COMMON = "COMMON"
PROFILE_NAME = "${CARGO_MAKE_PROFILE}"

[env.development]
IS_DEV = true
IS_PROD = false

[env.production]
IS_DEV = false
IS_PROD = true

[tasks.echo]
script = [
'''
echo COMMON: ${COMMON}
echo PROFILE_NAME: ${PROFILE_NAME}
echo IS_DEV: ${IS_DEV}
echo IS_PROD: ${IS_PROD}
'''
]
```

我们使用 **production** profile 运行 **echo** 任务：

```sh
cargo make --cwd ./examples --makefile profile.toml --profile production echo
```

输出：

```console
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: profile.toml
[cargo-make] INFO - Task: echo
[cargo-make] INFO - Profile: production
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: echo
+ cd /media/devhdd/projects/rust/cargo-make/examples
+ echo COMMON: COMMON
COMMON: COMMON
+ echo PROFILE_NAME: production
PROFILE_NAME: production
+ echo IS_DEV: FALSE
IS_DEV: FALSE
+ echo IS_PROD: TRUE
IS_PROD: TRUE
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

还可以使用 **profile** 属性基于 profile 过滤环境文件，如下所示：

```toml
env_files = [
    { path = "./development.env", profile = "development" },
    { path = "./production.env", profile = "production" },
    { path = "./env.env" }
]
```

在 config 部分中定义的附加 profiles 也会触发加载额外的 env 块/文件，例如：

```toml
env_files = [
    { path = "./second.env", profile = "second_profile" },
    { path = "./another.env", profile = "another_profile" }
]

[config]
additional_profiles = ["second_profile", "another_profile"]

[env.second_profile]
IS_SECOND_AVAILABLE = true

[env.another_profile]
IS_OTHER_AVAILABLE = true
```

这对于通过环境变量块来启用/禁用特定任务非常方便。

<a name="usage-profiles-conditions"></a>
#### 条件判断 (Conditions)

[条件执行](#usage-conditions) 允许你触发或跳过任务。<br>
条件内置了对 profile 的支持，因此你可以根据 profile 名称触发或跳过任务。

示例：

```toml
[tasks.echo-development]
condition = { profiles = [ "development" ] }
command = "echo"
args = [ "running in development profile" ]

[tasks.echo-production]
condition = { profiles = [ "production" ] }
command = "echo"
args = [ "running in production profile" ]
```

<a name="usage-profiles-built-in"></a>
#### 内置 Profile (Built In Profiles)

cargo-make 提供了几个内置 profile，可用于快速启用额外的条件任务：

* **ci-coverage-tasks** - 启用所有代码覆盖率任务，并设置 Rust 编译以消除死代码。
* **none-thread-safe-tests** - 将 Rust 测试运行器设置为单线程模式。
* **multi-phase-tests** - 允许将测试拆分为多个阶段（线程安全、多线程、自定义）。
* **ci-static-code-analysis-tasks** - 启用所有静态代码分析任务（如格式检查和 clippy）作为 CI 流程的一部分（请参阅下方关于向后兼容性的特别说明）。
* **ci-all-build-tasks** - 启用所有额外编译任务（例如 bench 和 example 示例代码）作为 CI 流程的一部分（请参阅下方关于向后兼容性的特别说明）。
* **all-default-tasks** - 启用在运行默认任务时调用的额外任务（如 toml 格式化）。

*其中某些 profile 将来可能会发生更改以启用更多任务，这可能会破坏你的构建，因此在定义上它们绝不是向后兼容的。*<br>
*请谨慎使用。*

<a name="usage-private-tasks"></a>
### 私有任务 (Private Tasks)

私有任务是指只能由其他任务调用、而不能直接通过 CLI 命令行运行的任务。

若要将任务定义为私有任务，请添加值为 true 的 **private** 属性，如下所示：

```toml
[tasks.internal-task]
private = true
```

<a name="usage-deprecated-tasks"></a>
### 已废弃任务 (Deprecated Tasks)

可以将任务标记为已废弃（deprecated），以警告用户不应再使用该任务，而应改用更新或不同的任务。<br>
一旦调用，将显示包含废弃信息的警告消息。<br>
你可以通过将 **deprecated** 设置为 true 或提供相应的提示消息来声明任务已废弃。<br>
例如：

```toml
[tasks.legacy]
deprecated = "Please use task OTHER instead"

[tasks.legacy-extended]
extend = "legacy"
deprecated = false

[tasks.legacy2]
deprecated = true
```

例如调用 **legacy** 任务时，输出为：

```console
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: deprecated.toml
[cargo-make] INFO - Task: legacy
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: legacy
[cargo-make] WARN - Task: legacy is deprecated - Please use task OTHER instead
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

在列出所有任务时，已废弃的任务也会展示相应提示：

```console
No Category
----------
default - Empty Task
empty - Empty Task
legacy - No Description. (deprecated - Please use task OTHER instead)
legacy-extended - No Description.
legacy2 - No Description. (deprecated)
```

<a name="usage-watch"></a>
### 文件监听自动执行 (Watch)
在 cargo-make 中监听项目中的文件变更并自动触发任务非常简单。<br>
只需为任务添加 **watch** 属性并将其设置为 true，一旦任务被触发，每当项目中的文件发生更改时它都会自动运行。<br>
必须终止该进程才能停止监听。

示例：

```toml
[tasks.watch-example]
command = "echo"
args = [ "Triggered by watch" ]
watch = true
```

以下是调用该任务的示例输出：

```console
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: ./examples/watch.toml
[cargo-make] INFO - Task: watch-example
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: watch-example
[cargo-make] INFO - Running Task: watch-example-watch
[cargo-make] INFO - Execute Command: "cargo" "watch" "-q" "-x" "make --disable-check-for-updates --no-on-error --loglevel=info --makefile=/projects/rust/cargo-make/examples/watch.toml watch-example"
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: /projects/rust/cargo-make/examples/watch.toml
[cargo-make] INFO - Task: watch-example
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: watch-example
[cargo-make] INFO - Execute Command: "echo" "Triggered by watch"
Triggered by watch
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
^C
```

你还可以通过向 **watch** 属性提供配置对象来微调监听行为（底层基于 **cargo-watch**）：

```toml
[tasks.watch-args-example]
command = "echo"
args = [ "Triggered by watch" ]
watch = { postpone = true, no_git_ignore = true, ignore_pattern = "examples/files/*", watch = ["./docs/"] }
```

所有可用选项的描述请参见 [API 文档](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.WatchOptions.html)。

<a name="usage-watch-running-multiple-blocking-watches"></a>
#### 运行多个阻塞式监听 (Running Multiple Blocking Watches)

在需要运行多个阻塞式监听的场景中（例如同时运行编译 + HTTP 服务器），需要将所有这些 watch 作为并行分叉的子任务运行。<br>
为了实现这一点，你需要同时使用 fork = true 和 parallel = true 属性。<br>
例如：

```toml
[tasks.multiple-watches]
run_task = { name = ["build", "http-server", "something-else"], fork = true, parallel = true }
```

<a name="usage-functions"></a>
### 内置函数 (Functions)

cargo-make 附带了内置函数，有助于扩展环境变量无法满足的能力。<br>
函数并不是在 makefile 的任何地方都受支持，目前仅在命令参数数组结构中受支持。<br>
定义函数调用采用以下格式：**@@FUNCTION_NAME(ARG1,ARG2,ARG3,...)**<br>
例如：

```toml
[tasks.split-example]
command = "echo"
args = ["@@split(ENV_VAR,|)"]
```

目前支持的函数：

* [Split](#usage-functions-split)
* [GetAt](#usage-functions-getat)
* [Remove Empty](#usage-functions-remove-empty)
* [Trim](#usage-functions-trim)
* [Decode](#usage-functions-decode)

<a name="usage-functions-split"></a>
#### Split

split 函数接受两个参数：

* 环境变量名称
* 分隔字符

并返回一个子字符串数组。<br>
这允许将一个环境变量拆分为多个命令行参数，例如：

```toml
[env]
MULTIPLE_VALUES="1 2 3 4"

[tasks.split]
command = "echo"
args = ["@@split(MULTIPLE_VALUES, )"]

[tasks.no-split]
command = "echo"
args = ["${MULTIPLE_VALUES}"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml split
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: split
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: split
[cargo-make] INFO - Execute Command: "echo" "1" "2" "3" "4"
1 2 3 4
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.

> cargo make --cwd ./examples --makefile functions.toml no-split
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: no-split
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: no-split
[cargo-make] INFO - Execute Command: "echo" "1 2 3 4"
1 2 3 4
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

split 函数还支持可选的第三个 *mode* 属性。<br>
如果 mode 为 *remove-empty*，则输出中不会包含空值。

<a name="usage-functions-getat"></a>
#### GetAt

getat 函数接受三个参数：

* 环境变量名称
* 分隔字符
* 要返回的项的索引位置（0-based）

并根据指定索引返回仅包含单个值的数组。<br>
这使得可以拆分环境变量并仅提取所需的参数，例如：

```toml
[env]
MULTIPLE_VALUES="1 2 3 4"

[tasks.getat]
command = "echo"
args = ["@@getat(MULTIPLE_VALUES,|,3)"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml getat
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: getat
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: getat
[cargo-make] INFO - Execute Command: "echo" "4"
4
[cargo-make] INFO - Build Done in 0 seconds.
```

<a name="usage-functions-remove-empty"></a>
#### Remove Empty

remove empty 函数接受单个参数：

* 环境变量名称

如果环境变量未定义或为空，它将完全移除该命令行参数；否则返回实际的环境变量值。

```toml
[tasks.remove-empty]
command = "echo"
args = ["1", "@@remove-empty(DOES_NOT_EXIST)", "2"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml remove-empty
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: remove-empty
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: remove-empty
[cargo-make] INFO - Execute Command: "echo" "1" "2"
1 2
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

<a name="usage-functions-trim"></a>
#### Trim

trim 函数接受以下参数：

* 环境变量名称
* 可选的修剪类型：start / end（如果未提供，将同时修剪两端空白）

如果环境变量未定义，或者修剪后为空，它将完全移除该命令行参数；否则返回修剪后的实际环境变量值。

```toml
[env]
TRIM_VALUE="   123    "

[tasks.trim]
command = "echo"
args = ["@@trim(TRIM_VALUE)"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml remove-empty
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: trim
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: trim
[cargo-make] INFO - Execute Command: "echo" "123"
123
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

以下是使用 start/end 属性的示例：

```toml
[env]
TRIM_VALUE="   123    "

[tasks.trim-start]
command = "echo"
args = ["@@trim(TRIM_VALUE,start)"]

[tasks.trim-end]
command = "echo"
args = ["@@trim(TRIM_VALUE,end)"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml trim-start
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: trim-start
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: trim-start
[cargo-make] INFO - Execute Command: "echo" "123    "
123
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.

> cargo make --cwd ./examples --makefile functions.toml trim-end
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: trim-end
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: trim-end
[cargo-make] INFO - Execute Command: "echo" "   123"
   123
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

<a name="usage-functions-decode"></a>
#### Decode

decode 函数接受以下参数：

* 环境变量名称
* 可选的一组映射值列表（源值/目标值成对出现）
* 可选的默认值

如果最终输出为空，它将完全移除该命令行参数。

例如：

```toml
[tasks.decode]
command = "echo"
args = ["Env:", "${CARGO_MAKE_PROFILE}", "Decoded:", "@@decode(CARGO_MAKE_PROFILE,development,dev,ci,test)"]
```

我们检查 `CARGO_MAKE_PROFILE` 环境变量的值并在映射中查找。<br>
如果值为 **development**，它将被映射为 **dev**，而 **ci** 将被映射为 **test**。<br>
如果未找到匹配的映射，则返回原始值。<br>
找到映射的运行示例：

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=development decode
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: decode
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: decode
[cargo-make] INFO - Execute Command: "echo" "Env:" "development" "Decoded:" "dev"
Env: development Decoded: dev
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

未找到映射的运行示例：

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=unmapped decode
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: decode
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: decode
[cargo-make] INFO - Execute Command: "echo" "Env:" "unmapped" "Decoded:" "unmapped"
Env: unmapped Decoded: unmapped
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

另一个示例：

```toml
[tasks.decode-with-default]
command = "echo"
args = ["Env:", "${DECODE_ENV_VAR}", "Decoded:", "@@decode(DECODE_ENV_VAR,development,dev,ci,test,unknown)"]
```

与前面的示例相同，不同之处在于如果未找到任何映射，则返回默认值（最后一个参数）。<br>
运行示例：

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=unmapped decode-with-default
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: decode-with-default
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: decode-with-default
[cargo-make] INFO - Execute Command: "echo" "Env:" "unmapped" "Decoded:" "unknown"
Env: unmapped Decoded: unknown
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

映射的目标值还可以包含环境变量表达式，例如：

```toml
[tasks.decode-with-eval]
command = "echo"
args = ["Env:", "${DECODE_ENV_VAR}", "Decoded:", "@@decode(DECODE_ENV_VAR,test,The current profile is: ${CARGO_MAKE_PROFILE})"]
```

运行示例：

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=test decode-with-eval
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: decode-with-eval
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: decode-with-eval
[cargo-make] INFO - Execute Command: "echo" "Env:" "test" "Decoded:" "The current profile is: development"
Env: test Decoded: The current profile is: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

<a name="usage-ci"></a>
### 持续集成 (Continuous Integration)
cargo-make 附带了用于 CI 持续集成构建的预定义工作流，可由内部系统或在线服务（如 GitHub Actions、Travis CI、AppVeyor 等）执行。<br>
建议在安装 cargo-make 时附带 debug 标志以加快安装速度。

<a name="usage-ci-github-actions"></a>
#### Github Actions
在你的 workflow yml 文件中添加以下内容：

```yaml
- name: Install cargo-make
  uses: actions-rs/cargo@v1
  with:
    command: install
    args: --debug cargo-make
- name: Run CI
  uses: actions-rs/cargo@v1
  with:
    command: make
    args: ci-flow
```

这将使用包含所有最新特性的最新版本 cargo-make。

你可以在 [ci.yml](https://github.com/sagiegurari/ci_info/blob/master/.github/workflows/ci.yml) 查看完整的 YAML 文件。

如果你想运行代码覆盖率并将其上传到 codecov，还可以定义以下环境变量：

```toml
CARGO_MAKE_RUN_CODECOV=true
```

在处理工作区（workspaces）时，为了对每个成员运行 ci-flow 并汇总所有覆盖率数据，请使用以下命令：

```yaml
- name: Install cargo-make
  uses: actions-rs/cargo@v1
  with:
    command: install
    args: --debug cargo-make
- name: Run CI
  uses: actions-rs/cargo@v1
  with:
    command: make
    args: --no-workspace workspace-ci-flow
```

为了在构建期间加速 cargo-make 安装，你可以使用 [rust-cargo-make](https://github.com/marketplace/actions/rust-cargo-make) GitHub Action 直接下载预编译好的二进制文件。

<a name="usage-ci-travis"></a>
#### Travis
在 `.travis.yml` 文件中添加以下内容：

```yaml
script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
```

这将使用包含所有最新特性的最新版本 cargo-make。<br>
当开启 `cargo` 缓存时：

```yaml
cache: cargo
script:
  - which cargo-make || cargo install cargo-make
  - cargo make ci-flow
```

*注意：使用缓存时，为了更新 cargo-make，你需要手动清除 Travis 缓存。*

如果你想运行代码覆盖率并上传到 codecov，还可以定义以下环境变量：

```yaml
env:
  global:
    - CARGO_MAKE_RUN_CODECOV="true"
```

*注意：如果你使用 kcov 覆盖率工具，可以通过将 `CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY` 环境变量设置为 Travis 缓存的路径，从而缓存 kcov 安装包。*

在处理工作区时，为了对每个成员运行 ci-flow 并打包所有覆盖率数据，请使用以下命令：

```yaml
script:
  - cargo install --debug cargo-make
  - cargo make --no-workspace workspace-ci-flow
```

<a name="usage-ci-appveyor"></a>
#### AppVeyor
在 `appveyor.yml` 文件中添加以下内容：

```yaml
build: false

test_script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
```

在处理工作区时，为了对每个成员运行 ci-flow 并打包所有覆盖率数据，请使用以下命令：

```yaml
build: false

test_script:
  - cargo install --debug cargo-make
  - cargo make --no-workspace workspace-ci-flow
```

<a name="usage-ci-gitlab"></a>
#### GitLab CI
在 `gitlab-ci.yml` 文件中添加以下内容：

```yaml
test:cargo:
  script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
```

在处理工作区时，请使用以下命令：

```yaml
build: false

test:cargo:
  script:
  - cargo install --debug cargo-make
  - cargo make --no-workspace workspace-ci-flow
```

若要将覆盖率信息上传到 codecov，你需要进入 GitLab 仓库的设置，[添加保密变量 (Secret Variable)](https://docs.gitlab.com/ce/ci/variables/README.html#secret-variables)，存入该仓库的 codecov token。<br>
然后可以在 `gitlab-ci.yml` 中添加以下内容以启用覆盖率支持：

```yaml
variables:
  CARGO_MAKE_RUN_CODECOV: "true"
```

<a name="usage-ci-circleci"></a>
#### CircleCI
在 `.circleci/config.yml` 文件中添加以下内容：

```yaml
- run:
    name: install cargo-make
    command: cargo install --debug cargo-make
- run:
    name: ci flow
    command: cargo make ci-flow
```

开启 `cargo` 缓存时：

```yaml
  - restore_cache:
      key: project-cache
  # ....
  - run:
      name: install cargo-make
      command: which cargo-make || cargo install cargo-make
  - run:
      name: ci flow
      command: cargo make ci-flow
  # ....
  - save_cache:
      key: project-cache
      paths:
        - "~/.cargo"
```

*注意：使用缓存时，为了更新 cargo-make，你需要手动清除 CircleCI 缓存。*

*注意：如果你使用 kcov 覆盖率，可以通过将 `CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY` 环境变量设置为被 CircleCI 缓存的位置，以缓存 kcov 安装。*

在处理工作区时，请使用以下配置：

```yaml
- run:
    name: install cargo-make
    command: cargo install --debug cargo-make
- run:
    name: ci flow
    command: cargo make --no-workspace workspace-ci-flow
```

<a name="usage-ci-azure-pipelines"></a>
#### Azure Pipelines
在 `azure-pipelines.yml` 文件中添加以下内容：

```yaml
- script: cargo install --debug cargo-make
  displayName: install cargo-make
- script: cargo make ci-flow
  displayName: ci flow
```

处理工作区时：

```yaml
- script: cargo install --debug cargo-make
  displayName: install cargo-make
- script: cargo make --no-workspace workspace-ci-flow
  displayName: ci flow
```

<a name="usage-ci-drone-io"></a>
#### drone.io
这是使用 Docker 运行器运行 ci-flow 任务的最小 `.drone.yml` 示例：

```yaml
pipeline:
  ci-flow:
    image: rust:1.38-slim
    commands:
    - cargo install --debug cargo-make
    - cargo make ci-flow
```

<a name="usage-ci-cirrus"></a>
#### Cirrus CI
这是运行 ci-flow 任务的最小 `.cirrus.yml` 示例：

```yaml
container:
  image: rust:latest

task:
  name: ci-flow
  install_script: cargo install --debug cargo-make
  flow_script: cargo make ci-flow
```

<a name="usage-predefined-flows"></a>
### 预定义工作流 (Predefined Flows)
[默认 makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/) 附带了许多预定义任务和工作流。<br>
以下是一些无需外部 `Makefile.toml` 定义即可直接使用的主要工作流：

* **default** - 无需添加任务名称即可执行，只需运行 `cargo make`。该任务是 dev-test-flow 的别名。
* **dev-test-flow** - 同样也是默认工作流，因此无需编写任务名称即可调用（只需运行 **cargo make**）。<br>该任务运行代码格式化、cargo build 和 cargo test，很可能是你在开发和测试 Rust 项目时最常运行的一组任务。
* **watch-flow** - 监听任何文件更改，检测到变更时自动触发测试流程。
* **ci-flow** - 应用于 CI 构建（如 Travis/AppVeyor），以 verbose 详细级别运行构建和测试。
* **workspace-ci-flow** - 应用于工作区项目的 CI 构建。
* **publish-flow** - 清理旧的 target 目录并发布项目。
* **build-flow** - 运行包含构建、测试、安全审计、依赖最新性验证以及文档生成的完整生命周期流程。<br>此流程可用于确保你的项目经过充分测试且保持最新。
* **coverage-flow** - 从所有单元测试和集成测试生成覆盖率报告（Windows 上不支持）。默认情况下 cargo-make 使用 kcov 生成代码覆盖率；但也定义了其他备用实现。
* **codecov-flow** - 运行 coverage-flow 并将覆盖率结果上传至 codecov（Windows 上不支持）。

<a name="usage-predefined-flows-coverage"></a>
#### 代码覆盖率 (Coverage)
cargo-make 内置支持多种覆盖率任务。<br>
无需修改工作流，通过在 **`CARGO_MAKE_COVERAGE_PROVIDER`** 环境变量中设置覆盖率提供程序名称即可完成切换：

```toml
[env]
# 可以定义为 kcov、tarpaulin 等...
CARGO_MAKE_COVERAGE_PROVIDER = "kcov"
```

如果你有自定义的覆盖率任务，可以通过更改主 coverage 任务别名将其插入到覆盖率流程中，例如：

```toml
[tasks.coverage]
alias = "coverage-some-custom-provider"
```

你可以运行：

```sh
cargo make --list-all-steps | grep "coverage-"
```

查看当前支持的所有提供程序。示例输出：

```console
ci-coverage-flow: No Description.
coverage-tarpaulin: Runs coverage using tarpaulin rust crate (linux only)
coverage-flow: Runs the full coverage flow.
coverage-kcov: Installs (if missing) and runs coverage using kcov (not supported on windows)
```

所有内置的覆盖率提供程序均由其各自的作者提供维护支持，而非 cargo-make。

根据上述说明，若要为普通项目生成覆盖率报告，请运行以下命令：

```sh
cargo make coverage
```

若要在工作区项目中运行覆盖率并在工作区级别打包所有成员覆盖率报告，请运行以下命令：

```sh
cargo make --no-workspace workspace-coverage
```

如果你使用的是 **kcov**，可以在 Makefile.toml 中声明以下环境变量来自定义覆盖率任务：

指定要忽略的代码行或区域：

```toml
[env]
CARGO_MAKE_KCOV_EXCLUDE_LINE = "unreachable,kcov-ignore"             # 自定义匹配模式
CARGO_MAKE_KCOV_EXCLUDE_REGION = "kcov-ignore-start:kcov-ignore-end" # 自定义标记边界
```

默认情况下，收集覆盖率所执行的二进制文件由正则表达式进行过滤。如果不匹配你系统上生成的二进制文件名称，可以按如下方式覆盖：

```toml
[env]
# 例如：cargo make 的过滤正则为 cargo_make-[a-z0-9]*$
CARGO_MAKE_TEST_COVERAGE_BINARY_FILTER = "${CARGO_MAKE_CRATE_FS_NAME}-[a-z0-9]*$"
```

*有关 grcov 支持，请查阅以下仓库的说明：*<br>
*https://github.com/kazuk/cargo-make-coverage-grcov*

<a name="usage-predefined-flows-full"></a>
#### 完整任务列表 (Full List)

请参阅 [所有预定义任务的完整列表](https://github.com/sagiegurari/cargo-make/blob/master/docs/cargo_make_task_list.md)（通过 **`cargo make --list-all-steps`** 生成）。

<a name="usage-predefined-flows-disable"></a>
#### 禁用预定义任务/流程 (Disabling Predefined Tasks/Flows)

为了阻止加载内部核心任务和流程，只需在外部 Makefile.toml 中添加以下配置属性：

```toml
[config]
skip_core_tasks = true
```

*即使跳过核心任务，也会加载少数几个空任务，以确保 cargo-make 拥有已定义的默认任务。*

<a name="usage-predefined-flows-modify"></a>
#### 修改预定义任务/流程 (Modifying Predefined Tasks/Flows)

可以修改内部核心任务。<br>
所有修改均在 **config.modify_core_tasks** 节中定义。

```toml
[config.modify_core_tasks]
# 如果为 true，则将所有核心任务设置为私有（默认为 false）
private = true

# 如果设置为某个值，则所有核心任务都会修改为：<命名空间>::<名称>，例如 default::build
namespace = "default"
```

<a name="usage-min-version"></a>
### 最低版本要求 (Minimal Version)
如果你使用的 cargo-make 特性仅在特定版本及以上提供，可以确保构建在较旧的 cargo-make 版本调用时立即失败。<br>
若要指定最低版本，请在 config 部分中使用 **min_version**，如下所示：

```toml
[config]
min_version = "0.37.24"
```

<a name="usage-performance-tuning"></a>
### 性能调优 (Performance Tuning)
可以禁用 cargo-make 的某些功能以缩短启动初始化时间。<br>
以下是当前所有支持的开关：

```toml
[config]
# 跳过加载所有核心任务，从而节省 toml 解析和任务创建开销
skip_core_tasks = true
# 跳过加载 Git 相关的环境变量
skip_git_env_info = true
# 跳过加载 Rust 相关的环境变量
skip_rust_env_info = true
# 跳过加载当前 crate 相关的环境变量
skip_crate_env_info = true
```

在 Rust 工作区中运行时，可以在成员 makefile 中禁用某些特性。<br>
例如，如果所有成员与整个项目位于同一个 git 仓库中，可以在成员 makefile 中添加 **skip_git_env_info**，它们仍然能够沿用从父进程继承的环境变量。

对于在未修改输入文件时可以跳过的任务，请参阅 [仅当源码变更时运行任务](#usage-running-tasks-only-if-sources-changed) 章节。

<a name="usage-command-groups"></a>
### 命令组 / 子命令 (Command Groups / Subcommands)

你可以通过创建一个顶层任务来调用其他内部任务，从而将任务作为*命令组*暴露出来。

有两种实现途径：

#### 本地任务 (Local tasks)

使用私有任务（私有并非强制）更适合简单的场景以及将一个命令重定向到另一个命令。

此方法存在一些限制：
- `--list-all-steps` 不会列出私有任务
- 任务需要遵循特定的命名模式

例如，如果你希望拥有 server start/stop 和 client start/stop 命令并按如下方式执行：

```sh
cargo make server start
cargo make server stop
cargo make client start
cargo make client stop
```

你可以定义两个顶层任务（server 和 client）来调用内部任务。<br>
示例实现：

```toml
[tasks.server]
private = false
extend = "subcommand"
env = { "SUBCOMMAND_PREFIX" = "server" }

[tasks.client]
private = false
extend = "subcommand"
env = { "SUBCOMMAND_PREFIX" = "client" }

[tasks.subcommand]
private = true
script = '''
#!@duckscript

cm_run_task ${SUBCOMMAND_PREFIX}_${1}
'''

[tasks.server_start]
private = true
command = "echo"
args = ["starting server..."]

[tasks.server_stop]
private = true
command = "echo"
args = ["stopping server..."]

[tasks.client_start]
private = true
command = "echo"
args = ["starting client..."]

[tasks.client_stop]
private = true
command = "echo"
args = ["stopping client..."]
```

#### 外部子命令文件 (External subcommand file)

另一种方法是为子命令使用不同的配置文件。

这允许对子命令使用 `--list-all-steps`，也可以在子命令文件中设置 `[config]` 选项。

对于包含子目录项目的工程，可以为每个子项目创建一个 `Makefile.toml`，并从主文件夹将其作为子命令进行调用。

在 `foo/` 文件夹中使用配置文件的 `foo` 子命令示例实现：

```toml
[tasks.foo]
description = "Foo subcommands"
category = "Subcommands"
cwd = "foo/"
command = "makers"
args = ["${@}"]
```

在同一文件夹中使用 `Makefile.foo.toml` 配置文件的 `foo` 子命令示例实现：

```toml
[tasks.foo]
description = "Foo subcommands"
category = "Subcommands"
command = "makers"
args = ["--makefile", "Makefile.foo.toml", "${@}"]
```

<a name="usage-diff-changes"></a>
### 变更对比 (Diff Changes)
使用 **`--diff-steps`** CLI 命令行标志，你可以对比当前自定义覆盖项与预置内部 makefile 工作流之间的差异。

使用示例：

```console
cargo make --diff-steps --makefile ./examples/override_core.toml post-build
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - Build File: ./examples/override_core.toml
[cargo-make] INFO - Task: post-build
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Printing diff...
[cargo-make] INFO - Execute Command: "git" "diff" "--no-index" "/tmp/cargo-make/Lz7lFgjj0x.toml" "/tmp/cargo-make/uBpOa9THwD.toml"
diff --git a/tmp/cargo-make/Lz7lFgjj0x.toml b/tmp/cargo-make/uBpOa9THwD.toml
index 5152290..ba0ef1d 100644
--- a/tmp/cargo-make/Lz7lFgjj0x.toml
+++ b/tmp/cargo-make/uBpOa9THwD.toml
@@ -42,7 +42,9 @@
         name: "post-build",
         config: Task {
             clear: None,
-            description: None,
+            description: Some(
+                "Override description"
+            ),
             category: Some(
                 "Build"
             ),
[cargo-make] INFO - Done
```

*需要安装并可用 Git，因为它用于对比数据结构并通过标准 git 着色方案输出到控制台。*

<a name="usage-unstable-features"></a>
### 实验性特性 (Unstable Features)
cargo-make 的某些功能虽然运行良好，但尚未设置为默认行为。<br>
因此它们通过 **unstable_features** 属性进行门禁控制。<br>
若要启用此类特性，你需要指定其名称。<br>
例如：

```toml
[config]
unstable_features = ["CTRL_C_HANDLING"]
```

以下是当前存在的实验性特性列表：

* **CTRL_C_HANDLING** - 添加 <kbd>ctrl-c</kbd> 信号处理器，将停止当前任务调用的任何正在运行的命令并退出 cargo-make。

<a name="usage-cli"></a>
### 命令行选项 (CLI Options)
以下是运行 cargo-make 时的所有可用命令行选项：

```console
用法 (USAGE):
    [makers | cargo make | cargo-make make] [OPTIONS] [--] [<TASK_CMD>...]

参数 (ARGS):
    <TASK_CMD>    要执行的任务，可包含传递给任务本身的参数。

选项 (OPTIONS):
    --help, -h                           打印帮助信息
    --version, -V                        打印版本信息
    --makefile <FILE>                    包含任务定义的可选 toml 文件
    --task, -t <TASK>                    要执行的任务名称（如果任务名是最后一个参数则可省略该标志）[默认: default]
    --profile, -p <PROFILE>              Profile 配置环境名称（将转换为小写）[默认: development]
    --cwd <DIRECTORY>                    设置当前工作目录。如果指定，将从此目录开始搜索 makefile。
    --no-workspace                       禁用工作区支持（在工作区根目录下触发任务，而非在各成员上触发）
    --no-on-error                        禁用错误处理流程（即使在 config 部分中已定义）
    --allow-private                      允许直接调用私有任务
    --skip-init-end-tasks                跳过 init 和 end 任务
    --skip-tasks <SKIP_TASK_PATTERNS>    跳过所有匹配给定正则表达式的任务（例如: pre.*|post.*）
    --env-file <FILE>                    从提供的文件加载环境变量
    --env, -e <ENV>                      设置环境变量
    --loglevel, -l <LOG LEVEL>           日志级别 (verbose, info, error, off) [默认: info]
    --verbose, -v                        设置日志级别为 verbose（--loglevel verbose 的简写）
    --quiet                              设置日志级别为 error（--loglevel error 的简写）
    --silent                             设置日志级别为 off（--loglevel off 的简写）
    --no-color                           禁用终端彩色输出
    --time-summary                       在流程结束时打印任务级别耗时统计摘要
    --experimental                       允许访问不受支持的实验性预定义任务
    --disable-check-for-updates          在启动时禁用版本更新检查
    --output-format <OUTPUT FORMAT>      打印/列出步骤的格式（某些操作不支持所有格式）(default, short-description, markdown, markdown-single-page, markdown-sub-section, autocomplete)
    --output-file <OUTPUT_FILE>          列出步骤时的输出文件名
    --hide-uninteresting                 隐藏次要任务（如 pre/post 钩子）
    --print-steps                        仅按调用顺序打印构建步骤而不实际执行它们
    --list-all-steps                     列出所有已知任务步骤
    --list-category-steps <CATEGORY>     列出指定分类的任务步骤
    --diff-steps                         对比自定义工作流与预置工作流之间的差异（需要 git）
```

<a name="usage-plugins"></a>
### 插件系统 (Plugins)

插件使用户能够完全接管任务的执行控制权。<br>
cargo-make 仍会根据任务及其依赖关系创建执行计划，但会将各个任务的具体执行委托给插件代码处理。<br>
<br>
插件本质上是一个单独的 duckscript 代码块，可以访问任务和流程的元数据，并能调用 cargo-make 的专用命令或常规 duckscript 命令。<br>
例如，如果任务定义了要执行的命令和参数，而插件只需要调用它们，可以实现一个简单的插件如下：

```sh
args_string = array_join ${task.args} " " # 简易示例，不支持包含空格的参数
exec --fail-on-error ${task.command} %{args_string}
```

一旦为任务定义了插件，任务的执行控制权就转移给了插件本身。<br>
所有的脚本、命令、条件、env 等配置都会被忽略，应由插件代码自行处理。<br>
所有特定于任务的环境变量（所有 CARGO_MAKE_CURRENT_TASK_ 变量）将不会在全局定义，而是作为 json 字符串在 task env 块中提供。

<a name="usage-plugins-defining-plugins"></a>
### 定义插件 (Defining Plugins)

插件在 `plugins.impl` 前缀下定义，例如：

```toml
[plugins.impl.command-runner]
script = '''
echo task: ${task.name}

args_string = array_join ${task.args} " " # 简易示例，不支持包含空格的参数
exec --fail-on-error ${task.command} %{args_string}
'''
```

你可以根据需要定义任意数量的插件。<br>
还可以为它们提供别名，将新名称映射到已有插件。<br>
例如：

```toml
[plugins.aliases]
original = "new"
this = "that"
```

要让任务将执行控制权移交给插件，只需在 **plugin** 属性中填入插件名称即可。<br>
例如：

```toml
[tasks.my-task]
plugin = "my-plugin"
# 根据需要定义其他属性...
```

**你可以创建可复用的插件，并使用内置的 [加载脚本 (load scripts)](#usage-load-scripts) 功能进行远程加载。**

<a name="usage-plugins-plugin-sdk"></a>
### 插件 SDK (Plugin SDK)

插件 SDK 包含以下内容：

* [通用 Duckscript SDK](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md)
* 元数据变量
    * flow.task.name - 保存触发此任务的流程任务（非当前任务）名称
    * flow.cli.args - 包含在命令行上提供给 cargo-make 的所有任务参数的数组
    * plugin.impl.name - 当前插件名称（别名修改后）
    * task.as_json - JSON 字符串格式的整个任务配置（可使用 json_parse 转换为 duckscript 变量）
    * task.has_condition - 如果任务有任何条件定义（包括空的）则为 true
    * task.has_env - 如果任务有任何 env 定义（包括空的）则为 true
    * task.has_install_instructions - 如果任务有安装定义则为 true
    * task.has_command - 如果任务有 command 定义则为 true
    * task.has_script - 如果任务有 script 定义则为 true
    * task.has_run_task - 如果任务有 run_task 定义则为 true
    * task.has_dependencies - 如果任务有依赖项则为 true
    * task.has_toolchain_specifier - 如果任务有 toolchain 定义则为 true
    * task.name - 任务名称
    * task.description - 任务描述
    * task.category - 任务分类
    * task.disabled - 根据 disabled 属性返回 true/false
    * task.private - 根据 private 属性返回 true/false
    * task.deprecated - 根据 deprecated 属性返回 true/false
    * task.workspace - 根据 workspace 属性返回 true/false
    * task.plugin.name - 任务中定义的插件名称（别名前）
    * task.watch - 根据 watch 属性返回 true/false
    * task.ignore_errors - 根据 ignore_errors 属性返回 true/false
    * task.cwd - 任务的当前工作目录值
    * task.command - 命令
    * task.args - 所有命令参数的数组
    * task.script_runner - 脚本运行器值
    * task.script_runner_args - 所有脚本运行器参数的数组
    * task.script_extension - 脚本文件扩展名值
* cargo-make 任务脚本专用命令
    * ```cm_run_task [--async] taskname``` - 运行任务及其依赖项。支持异步执行（通过 --async 标志）。必须传入要调用的任务名称。
* cargo-make 插件专用命令
    * ```cm_plugin_run_task``` - 运行调用该插件的当前任务（不包括依赖项），包含条件处理、env、cwd 以及 cargo-make 的所有内置逻辑。
    * ```cm_plugin_run_custom_task``` - 接受任务 JSON 字符串并运行该任务定义（不包括依赖项），包含条件处理、env、cwd 以及 cargo-make 的所有内置逻辑。
    * ```cm_plugin_check_task_condition``` - 如果当前任务条件满足则返回 true/false。
    * ```cm_plugin_force_plugin_set``` - 后续所有即将被调用的任务无论其配置如何，都会调用当前插件。
    * ```cm_plugin_force_plugin_clear``` - 撤销 cm_plugin_force_plugin_set 的更改，任务恢复原有行为。

<a name="usage-plugins-plugin-example-dockerize"></a>
### 插件示例 - Docker 集成 (Plugin Example - Docker Integration)

下面是一个在 Docker 容器中运行任务（以及从该点开始的后续流程）的简易示例：

```toml
[plugins.impl.dockerize]
script = '''
plugin_force_set = get_env PLUGIN_FORCE_SET
plugin_force_set = eq "${plugin_force_set}" 1

if not ${plugin_force_set}
    cm_plugin_force_plugin_set
    set_env PLUGIN_FORCE_SET 1

    dockerfile = set ""
    fn add_docker
        dockerfile = set "${dockerfile}${1}\n"
    end

    taskjson = json_parse ${task.as_json}
    makefile = basename ${taskjson.env.CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE}

    add_docker "FROM debian:stable"
    add_docker "RUN mkdir /workdir/"
    add_docker "RUN mkdir /workdir/project/"
    add_docker "RUN apt-get update"
    add_docker "RUN apt-get install -y curl build-essential libssl-dev pkg-config"
    add_docker "ENV PATH=\"$PATH:$HOME/.cargo/bin\""
    add_docker "RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
    add_docker "RUN $HOME/.cargo/bin/cargo install cargo-make"
    add_docker "RUN $HOME/.cargo/bin/cargo make --version"
    add_docker "RUN echo \"cd ./workdir/project/ && ls -lsa && $HOME/.cargo/bin/cargo make --makefile ${makefile} --profile ${CARGO_MAKE_PROFILE} ${CARGO_MAKE_TASK}\" > ./run.sh"
    add_docker "RUN chmod 777 ./run.sh"
    add_docker "ADD . /workdir/project/"
    add_docker "CMD [\"sh\", \"./run.sh\"]"

    writefile ./Dockerfile ${dockerfile}
    exec --fail-on-error docker build --tag cmimg:build ./

    exec --fail-on-error docker run cmimg:build
end
'''

[tasks.default]
alias = "docker_flow"

[tasks.docker_flow]
dependencies = ["part1", "part2", "part3"]

[tasks.base-task]
command = "echo"
args = ["task", "${CARGO_MAKE_CURRENT_TASK_NAME}"]

[tasks.part1]
plugin = "dockerize"
extend = "base-task"

[tasks.part2]
extend = "base-task"

[tasks.part3]
extend = "base-task"
```

运行：

```sh
cargo make docker_flow
```

将创建一个新的 Docker 容器并在其中运行 part 1-3。<br>
**该示例可以正常工作，但尚未支持传递 CLI 命令行参数等高级特性。**

<a name="usage-plugins-plugin-example-parallel-workspace-members"></a>
### 插件示例 - 并行运行工作区成员 (Plugin Example - Run workspace members in parallel)

以下示例演示如何在工作区级 makefile 中定义一个任务，使其能够并行调用各个成员：

```toml
[plugins.impl.parallel-members]
script = '''
plugin_used = get_env PLUGIN_USED
plugin_used = eq "${plugin_used}" 1

if not ${plugin_used}
    set_env PLUGIN_USED 1
    members = split ${CARGO_MAKE_CRATE_WORKSPACE_MEMBERS} ,

    workspace_dir = pwd
    for member in ${members}
        cd ./${member}
        spawn cargo make --disable-check-for-updates --allow-private --no-on-error ${flow.task.name} %{args}
        cd ${workspace_dir}
    end

    release ${members}
else
    task_definition = json_parse --collection ${task.as_json}
    map_remove ${task_definition} workspace
    task_json = json_encode --collection ${task_definition}
    cm_plugin_run_custom_task ${task_json}
end
'''

[tasks.sometask]
# 若要使该任务串行而非并行运行，请删除以下两行
plugin = "parallel-members"
workspace = false
```

<a name="usage-plugins-plugin-example-rustenv"></a>
### 插件示例 - 从 Rust 脚本加载环境变量 (Plugin Example - Load Env From Rust Script)

以下示例展示如何允许从 cargo-make 调用的 Rust 脚本更新主 cargo-make 进程的环境变量。<br>
它假定任务包含 script 行且该脚本是 Rust 脚本。它将执行该脚本（为简便起见忽略任何 rust script provider 配置），并将每个输出行作为环境变量键/值对加载。

```toml
[plugins.impl.rust-env]
script = '''
# 确保任务定义了 script
assert ${task.has_script}

taskjson = json_parse ${task.as_json}
script = set ${taskjson.script}
writefile ./target/_tempplugin/main.rs ${script}

out = exec --fail-on-error rust-script ./target/_tempplugin/main.rs

output = trim ${out.stdout}
lines = split ${output} \n
for line in ${lines}
    parts = split ${line} =
    key = array_get ${parts} 0
    value = array_get ${parts} 1
    set_env ${key} ${value}
end
'''

[tasks.default]
alias = "test"

[tasks.test]
dependencies = ["dorust"]
command = "echo"
args = ["${ENV_FROM_RUST1}", "${ENV_FROM_RUST2}"]

[tasks.dorust]
private = true
plugin = "rust-env"
script = '''
fn main() {
    println!("ENV_FROM_RUST1=hello");
    println!("ENV_FROM_RUST2=world");
}
'''
```

<a name="usage-plugins-plugin-example-powershell"></a>
### 插件示例 - 添加更简洁的 Windows Powershell 支持 (Plugin Example - Adding Simpler Windows Powershell Support)

在下面的示例中，我们添加了简易的 PowerShell 命令支持。<br>
此插件将获取现有任务，将其命令设置为 powershell 并前置 **`-C`** 参数。<br>
该示例还演示了如何在运行时创建新任务并调用它们：

```toml
[plugins.impl.powershell]
script = '''
# 添加更简洁的 powershell 集成

# 确保处于 Windows 平台
windows = is_windows
assert ${windows}

# 确保任务包含参数
args_empty = array_is_empty ${task.args}
assert_false ${args_empty}

task_definition = json_parse --collection ${task.as_json}

# 将 powershell 参数前置到任务参数中
powershell_args = array -C
all_args = array_concat ${powershell_args} ${task.args}
args = map_get ${task_definition} args
release ${args}
map_put ${task_definition} args ${all_args}

# 设置 powershell 命令
map_put ${task_definition} command pwsh.exe

powershell_task_json = json_encode --collection ${task_definition}

echo Custom Task:\n${powershell_task_json}
cm_plugin_run_custom_task ${powershell_task_json}
'''

[tasks.default]
alias = "test"

[tasks.test]
plugin = "powershell"
args = ["echo hello from windows powershell"]
```

<a name="usage-shell-completion"></a>
### Shell 自动补全 (Shell Completion)

cargo-make 提供了 Shell 自动补全支持。为了提供当前目录中可用的准确任务名称，它会运行 `--list-all-steps` 命令，这可能需要稍作等待。

<a name="usage-shell-completion-bash"></a>
#### Bash
在 Shell 会话启动时 source 引入 `extra/shell` 文件夹中的 `makers-completion.bash` 文件。<br>
这将启用 **makers** 可执行文件的自动补全。

<a name="usage-shell-completion-zsh"></a>
#### zsh
zsh 支持 Bash 自动补全。因此，可以通过运行以下脚本来使用现有的 Bash 自动补全：

```sh
autoload -U +X compinit && compinit
autoload -U +X bashcompinit && bashcompinit

# 请确保根据你的文件系统位置更新路径
source ./extra/shell/makers-completion.bash
```

这将启用 **makers** 可执行文件的自动补全。

<a name="usage-task-completion-zsh"></a>
#### Zsh 任务补全 (Zsh Task Completion)
通过执行 `cargo make --completion zsh`，将创建启用任务自动补全所需的必要组件。需要重启 Shell 或执行 `source ~/.zshrc`。<br>
之后就可以在输入制表符时列出 Makefile.toml 中定义的任务：`cargo make <tab>`

<a name="usage-shell-completion-fig"></a>
#### Fig / Amazon CodeWhisperer 命令行补全

Fig 自 [此 PR](https://github.com/withfig/autocomplete/pull/2180) 起已原生支持 cargo-make，无需特殊配置，只需下载最新版本的 [Fig](https://fig.io/) 或 [Amazon CodeWhisperer for command line](https://aws.amazon.com/blogs/devops/introducing-amazon-codewhisperer-for-command-line/) 即可。

通过运行以下命令再次确认是否已全局安装 `cargo-make`：

```bash
cargo --list
```

如果你能在列表中看到 `make`，Fig 就能正常工作并自动从 `./Makefile.toml` 或使用 `--makefile <path>` 指定的任何目录加载补全。

<a name="cargo-make-global-config"></a>
### 全局配置 (Global Configuration)
某些默认 CLI 选项和 cargo-make 行为可以通过位于 cargo-make 目录下的可选全局配置文件 `config.toml` 进行配置。

cargo-make 目录位置可以通过 `CARGO_MAKE_HOME` 环境变量的值进行定义。<br>
如果未定义 `CARGO_MAKE_HOME`，cargo-make 的默认位置为：

| 操作系统 (OS) | 配置路径 (Location)               |
| ------------- | --------------------------------- |
| Linux         | $XDG_CONFIG_HOME 或 $HOME/.config |
| Windows       | RoamingAppData                    |
| Mac           | $HOME/Library/Preferences         |

如果出于任何原因上述路径在指定平台上无效，将回退使用 `$HOME/.cargo-make`。

以下示例 `config.toml` 展示了所有可用选项及其默认值：

```toml
# 如果 CLI 未通过 `--loglevel` 参数指定，则使用的默认日志级别
log_level = "info"

# 是否禁用终端彩色输出的默认配置
disable_color = false

# 如果在调用 cargo-make 时未提供任务名称，则使用的默认任务名称
default_task_name = "default"

# cargo-make 在调用期间会检查更新。
# 此配置定义在 cargo-make 调用尝试检查更新之前必须流逝的最短时间。
# 如果最短时间未到达，cargo-make 将不会检查更新（等同于 --disable-check-for-updates）
# 有效值为：always, daily, weekly, monthly
# 如果提供了任何其他值，将被视为 weekly。
update_check_minimum_interval = "weekly"

# 如果设置为 true，且命令行参数中未提供 cwd，并且当前 cwd 不是项目根目录（Cargo.toml 不存在），
# cargo make 将尝试通过向上搜索父目录来查找项目根目录，直到找到包含 Cargo.toml 的目录。
# cargo make 将把 cwd 设置为该目录，并使用在该位置找到的任何 Makefile.toml。
search_project_root = false
```

<a name="descriptor-definition"></a>
## Makefile 结构定义 (Makefile Definition)

[Config 配置节](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.ConfigSection.html)

[Task 任务结构](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.Task.html)

[Platform Override 平台覆盖](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.PlatformOverrideTask.html)

[Condition 条件结构](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.TaskCondition.html)

更多信息请参阅 API 文档的 [类型 (types)](https://sagiegurari.github.io/cargo-make/api/cli/types/index.html) 章节。

<a name="task-name-conventions"></a>
## 任务命名约定 (Task Naming Conventions)
本节介绍默认任务名称背后的设计逻辑。<br>
虽然默认名称逻辑可以作为任何项目 Makefile.toml 中定义新任务时的命名规范，但并非强制要求。

[默认 makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/) 文件包含几种类型的任务：

* 单命令或单脚本任务（例如 **cargo build**）
* 在单命令任务之前或之后调用的任务（钩子，hooks）
* 使用依赖项定义工作流的任务
* 仅用于安装某些依赖项的任务

单命令任务通常根据其命令命名，例如运行 cargo build 的任务命名为 build：

```toml
[tasks.build]
command = "cargo"
args = ["build"]
```

这有助于直观理解该任务的作用。

在这些任务之前/之后调用的任务采用与原任务相同的名称，但带有 pre/post 前缀。<br>
例如，对于 build 任务，默认 toml 还定义了 pre-build 和 post-build 任务：

```toml
[tasks.pre-build]

[tasks.post-build]
```

在 [默认 makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/) 中，所有 pre/post 任务均为空，它们作为占位符供外部 `Makefile.toml` 覆盖，以便在运行特定任务之前/之后轻松定义自定义行为。

工作流任务带有 flow 后缀。例如 **ci-flow**：

```toml
[tasks.ci-flow]
# CI 任务将以 verbose 详细输出运行 cargo build 和 cargo test
dependencies = [
    "pre-build",
    "build-verbose",
    "post-build",
    "pre-test",
    "test-verbose",
    "post-test"
]
```

这可以防止工作流任务名称与单命令任务名称发生冲突，并使用户能够快速明白该任务是一个工作流定义。

仅安装某些依赖项而不调用任何命令的任务以 **install-** 前缀开头，例如：

```toml
[tasks.install-rust-src]
install_crate = { rustup_component_name = "rust-src" }
```

<a name="error-codes"></a>
## 错误代码 (Error Codes)

<a name="e001"></a>
### E001: Environment Variables Cycle Detected (检测到环境变量循环引用)

检测到不同环境变量之间存在循环依赖；
这可能在合并环境（在每个加载步骤）期间发生。
由于重新排序以确保不存在循环引用，因此会抛出此错误。

你可以通过检查你的 env 配置来解决此问题，查看是否在某个环节发生了循环引用。
错误消息中会列出可能导致循环依赖的环境变量候选者。

最好的解决方法是通过创建新的环境变量或多次使用静态值来打破循环。
循环通常是由频繁变动的配置、遗忘的未使用变量或设计缺陷导致的；
即使没有循环检测或不进行重新排序，这也可能会在执行过程中引发隐蔽的问题，因为 `cargo-make` 否则只能将这些实例设置为空值。
通过这种方式，你可以在它演变成难以调试的隐蔽隐患之前自行排查并修复它。

> **注意：** 已知脚本有时可能会引发假阳性错误。
> 在这种情况下，请使用 `depends_on` 属性显式告知 `cargo-make` 哪些环境变量应被视为依赖项，而不是尝试从脚本中自动推断。

<a name="articles"></a>
## 相关文章 (Articles)
以下是解释 cargo-make 大多数特性的系列文章：

* [入门与基础 (Introduction and Basics)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-1-of-5-introduction-and-basics-b19ced7e7057)
* [扩展任务、平台覆盖与别名 (Extending Tasks, Platform Overrides and Aliases)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-2-of-5-extending-tasks-platform-overrides-1527386dcf87)
* [环境变量、条件判断、子任务与混合 (Environment Variables, Conditions, Sub Tasks and Mixing)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-3-of-5-environment-variables-conditions-3c740a837a01)
* [工作区支持、Init/End 任务与 Makefile (Workspace Support, Init/End Tasks and Makefiles)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-4-of-5-workspace-support-init-end-tasks-c3e738699421)
* [预定义任务、CI 支持与约定 (Predefined Tasks, CI Support and Conventions)](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-5-final-predefined-tasks-ci-support-and-4594812e57da)

这些文章发表较早，缺少发表后添加的一些新特性，例如：

* [Rust 任务](#usage-task-command-script-task-examplerust)
* [跨平台 Shell 转换](#usage-task-command-script-task-exampleshell2batch)
* [完整预定义工作流列表](#usage-predefined-flows)
* [全局配置](#cargo-make-global-config)
* [捕获错误](#usage-catching-errors)
* [Env 文件](#usage-env-file)
* [私有任务](#usage-private-tasks)
* [其他编程语言](#usage-task-command-script-task-examplegeneric)
* [Rust 版本条件](#usage-conditions-structure)
* [工具链支持](#usage-toolchain)
* [文件监听自动执行](#usage-watch)
* [配置环境 (Profiles)](#usage-profiles)
* [内置函数](#usage-functions)
* [最低版本要求](#usage-min-version)
* [已废弃任务](#usage-deprecated-tasks)

以及更多...

<a name="badge"></a>
## 项目徽章 (Badge)
如果你在项目中使用了 cargo-make，并希望在项目 README 或网站中展示，可以嵌入 "Built with cargo-make" 徽章：

[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

代码示例：

### Markdown


```md
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
```

### HTML

```html
<a href="https://sagiegurari.github.io/cargo-make">
  <img src="https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg" alt="Built with cargo-make">
</a>
```

<a name="roadmap"></a>
## 开发路线图 (Roadmap)
尽管已经拥有极为丰富的功能，cargo-make 仍在持续大力开发中。<br>
你可以在 [GitHub 项目 Issues 列表](https://github.com/sagiegurari/cargo-make/issues) 中查看未来的开发规划项。

<a name="editor-support"></a>
## 编辑器支持 (Editor Support)

<a name="editor-support-vim"></a>
### Vim

* [vim-cargo-make](https://github.com/nastevens/vim-cargo-make)
* [vim-duckscript](https://github.com/nastevens/vim-duckscript)

<a name="editor-support-vs-code"></a>
### VSCode

出于调试目的，在 [docs/vscode-example](./docs/vscode-example/) 目录下提供了一些示例 .vscode 配置文件。

你可能还需要：

  * 本地安装 LLVM（用于 LLDB 调试器），并在系统 PATH 中可访问
  * VSCode 扩展 - CodeLLDB
  * VSCode 扩展 - "rust-analyser"（非旧版 "rust" 扩展）
  * VSCode 扩展 - "Task Explorer"
  * VSCode 扩展 - "crates"

<a name="contributing"></a>
## 贡献指南 (Contributing)
请参阅 [贡献指南](.github/CONTRIBUTING.md)

<a name="history"></a>
## 发布历史 (Release History)

请参阅 [Changelog 更新日志](https://github.com/sagiegurari/cargo-make/blob/master/CHANGELOG.md)

<a name="license"></a>
## 开源许可证 (License)
由 Sagie Gur-Ari 开发，并在 Apache 2 开源许可证下授权分发。
