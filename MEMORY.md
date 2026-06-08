# MEMORY.md

## 项目范围

本文件是当前操作系统实验仓库的项目级记忆，只服务于本仓库。不要把这些内容写入全局 Codex 记忆。

仓库路径：

`D:\GYC\2026.3-2026.7\操作系统\experiment`

## 稳定环境事实

- 用户主要在 Windows PowerShell 中操作。
- 本地实验目录通过 openEuler Docker 容器的 `/mnt` 访问。
- Rust 工具链固定为 `nightly-2022-10-19`。
- 编译目标为 `riscv64gc-unknown-none-elf`。
- 已配置 `cargo-binutils`、`llvm-tools-preview`、`rust-src`。
- QEMU 版本为 5.2.0，并包含 `riscv64-softmmu` 与 `riscv64-linux-user`。
- `cargo-binutils v0.4.0` 与当前 Rust 版本不兼容，实验0中实际采用 `cargo-binutils v0.3.6 --locked`。

## 用户当前偏好

- 不再维护单独的实验过程记录 Markdown。
- Git 历史、报告草稿、截图和用户贴回的真实输出作为证据链。
- 报告先生成完整骨架，除截图和真实输出外尽量写完整。
- 报告中必须提前放好截图占位符。
- 后续按阶段逐行执行命令，用户贴回输出后再判断。
- 通过后提醒截图整个命令行窗口。
- 出错后先恢复到上一阶段成功结束状态，再给新的当前阶段方案。
- 新方案验证成功后，更新报告中当前阶段命令和问题处理，不改已验证阶段。

## 当前实验重点

实验6：内存管理。

实验说明路径：

`docs/exp6-memory-management/实验6：内存管理.txt`

核心内容：

1. 内核动态内存分配。
2. 虚拟地址、物理地址、虚拟页号、物理页号。
3. 页表项和 PTE 标志位。
4. 物理帧管理与分配。
5. 多级页表管理。
6. 内核地址空间和应用地址空间。
7. 基于地址空间的分时多任务。
8. 跨地址空间的 `sys_write`。
9. 用户程序构建方式调整。
10. 新应用程序编写与测试。

## 报告策略

实验6报告应先放在 `reports/exp6-memory-management/` 下。报告可以使用 Markdown 草稿承载命令、截图占位符和待填输出，再按需要转换或整理为最终格式。

报告中的命令必须按阶段分组，每次只验证一组。不要把未运行的预期输出写成真实输出。

## 保留 Markdown 文件

项目根目录保留：

- `README.md`
- `AGENTS.md`
- `SKILL.md`
- `MEMORY.md`

实验要求、报告草稿和最终报告按需要保留。旧的过程记录 Markdown 和旧的独立流程 Markdown 不再使用。
