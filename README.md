# 操作系统实验仓库

本仓库用于整理操作系统课程实验。仓库按“实验资料、实验代码、报告成品和项目级协作规则”分区管理，并用不同分支保存不同实验阶段，便于复盘、截图归档和远程提交。

## 目录结构

```text
.
├── AGENTS.md                  # Codex 项目级协作规则
├── SKILL.md                   # 实验报告生成与逐阶段验证流程
├── MEMORY.md                  # 当前项目稳定事实和用户偏好
├── README.md                  # 仓库入口和实验索引
├── rust-toolchain             # Rust 工具链锁定
├── bootloader/                # 启动加载相关文件
├── os/                        # 内核代码项目
├── user/                      # 用户态程序项目
├── docs/                      # 老师给出的实验要求和模板
└── reports/                   # 最终实验报告和截图证据
```

## 目录职责

- `docs/`：保存实验要求原文和报告模板。实验说明文件按实验编号归档。
- `reports/`：保存实验报告草稿、最终报告和截图证据。报告内容以真实运行输出和截图为准。
- `AGENTS.md`、`SKILL.md`、`MEMORY.md`：保存当前项目的协作规则、报告生成流程和稳定事实。
- `os/`：保存内核侧代码。后续实验通常在此持续演进。
- `user/`：保存用户态库和用户程序。涉及系统调用、用户程序构建和测试应用时会修改此目录。
- `bootloader/`：保存启动相关文件，通常不作为实验主体修改。

## 实验索引

| 实验 | 资料 | 代码 | 报告 |
| --- | --- | --- | --- |
| 实验0 环境配置 | `docs/exp0-env/` | 无独立代码项目 | `reports/exp0-env/` |
| 实验1 独立的可执行程序 | `docs/exp1-standalone/` | `os/` | `reports/exp1-standalone/` |
| 实验2 裸机环境和最小化内核 | `docs/exp2-bare-metal-kernel/` | `os/` | `reports/exp2-bare-metal-kernel/` |
| 实验3 批处理与特权级 | `docs/exp3-batch-privilege/` | `os/`, `user/` | `reports/exp3-batch-privilege/` |
| 实验4 多道程序与协作式调度 | `docs/exp4-multiprogramming-scheduling/` | `os/`, `user/` | `reports/exp4-multiprogramming-scheduling/` |
| 实验5 分时多任务与抢占式调度 | `docs/exp5-timesharing-preemptive-scheduling/` | `os/`, `user/` | `reports/exp5-timesharing-preemptive-scheduling/` |
| 实验6 内存管理 | `docs/exp6-memory-management/` | `os/`, `user/` | `reports/exp6-memory-management/` |

## 分支结构

| 分支 | 用途 |
| --- | --- |
| `master` | 仓库总入口，汇总当前全部实验资料、代码、项目级规则和报告成品 |
| `exp0-env-setup` | 实验0环境配置资料与报告 |
| `exp1-standalone-executable` | 实验1独立可执行程序代码、资料和报告 |
| `exp2-bare-metal-kernel` | 实验2裸机环境和最小化内核 |
| `exp3-batch-privilege` | 实验3批处理与特权级 |
| `exp4-multiprogramming-scheduling` | 实验4多道程序与协作式调度 |
| `exp5-timesharing-preemptive-scheduling` | 实验5分时多任务与抢占式调度 |
| `exp6-memory-management` | 实验6内存管理 |

每个实验分支保存该实验完成时的状态。`master` 保存最新汇总状态，不要求把后续实验内容反向同步到前面实验分支。

## 实验推进流程

1. 阅读对应 `docs/exp*/` 下的实验要求。
2. 在对应 `reports/exp*/` 下生成带截图占位符的报告草稿。
3. 按报告阶段逐行执行命令、修改代码、贴回输出和截图。
4. 验证成功后将真实输出摘要、截图文件名和问题处理同步到报告。
5. 推送时只提交当前实验相关文件，避免把未完成实验或无关文件混入历史分支。

提交或推送前先用 `git status --short` 检查当前改动范围。
