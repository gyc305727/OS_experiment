# 操作系统实验仓库

本仓库用于整理操作系统课程实验。仓库按“实验资料、实验代码、过程记录、报告成品”分区管理，并用不同分支保存不同实验阶段，便于复盘、截图归档和远程提交。

## 目录结构

```text
.
├── AGENTS.md                  # Codex 协作规则
├── README.md                  # 仓库入口和实验索引
├── rust-toolchain             # Rust 工具链锁定
├── bootloader/                # 启动加载相关文件
├── os/                        # 内核代码项目
├── user/                      # 用户态程序项目
├── docs/                      # 老师给出的实验要求、模板和流程规则
├── records/                   # 实验过程记录、checkpoint 和 REPORT_SYNC_BLOCK
└── reports/                   # 最终实验报告和截图证据
```

## 目录职责

- `docs/`：保存实验要求原文、报告模板和通用工作流程。实验说明文件按实验编号归档。
- `records/`：保存实验过程记录。这里记录 checkpoint、执行命令、截图提醒和阶段同步素材，不是最终报告。
- `reports/`：保存最终报告文件和截图证据。报告内容以真实运行输出和截图为准。
- `os/`：保存内核侧代码。后续实验通常在此持续演进。
- `user/`：保存用户态库和用户程序。涉及系统调用、用户程序构建和测试应用时会修改此目录。
- `bootloader/`：保存启动相关文件，通常不作为实验主体修改。

## 实验索引

| 实验 | 资料 | 代码 | 过程记录 | 报告 |
| --- | --- | --- | --- | --- |
| 实验0 环境配置 | `docs/exp0-env/` | 无独立代码项目 | 暂无 | `reports/exp0-env/` |
| 实验1 独立的可执行程序 | `docs/exp1-standalone/` | `os/` | `records/exp1-standalone/` | `reports/exp1-standalone/` |
| 实验2 裸机环境和最小化内核 | `docs/exp2-bare-metal-kernel/` | `os/` | `records/exp2-bare-metal-kernel/` | `reports/exp2-bare-metal-kernel/` |
| 实验3 批处理与特权级 | `docs/exp3-batch-privilege/` | `os/`, `user/` | `records/exp3-batch-privilege/` | `reports/exp3-batch-privilege/` |
| 实验4 多道程序与协作式调度 | `docs/exp4-multiprogramming-scheduling/` | `os/`, `user/` | `records/exp4-multiprogramming-scheduling/` | `reports/exp4-multiprogramming-scheduling/` |
| 实验5 分时多任务与抢占式调度 | `docs/exp5-timesharing-preemptive-scheduling/` | `os/`, `user/` | `records/exp5-timesharing-preemptive-scheduling/` | `reports/exp5-timesharing-preemptive-scheduling/` |
| 实验6 内存管理 | `docs/exp6-memory-management/` | `os/`, `user/` | `records/exp6-memory-management/` | `reports/exp6-memory-management/` |

## 分支结构

| 分支 | 用途 |
| --- | --- |
| `master` | 仓库总入口，汇总当前全部实验资料、代码、过程记录和报告成品 |
| `exp0-env-setup` | 实验0环境配置资料与报告 |
| `exp1-standalone-executable` | 实验1独立可执行程序代码、资料、记录和报告 |
| `exp2-bare-metal-kernel` | 实验2裸机环境和最小化内核 |
| `exp3-batch-privilege` | 实验3批处理与特权级 |
| `exp4-multiprogramming-scheduling` | 实验4多道程序与协作式调度 |
| `exp5-timesharing-preemptive-scheduling` | 实验5分时多任务与抢占式调度 |
| `exp6-memory-management` | 实验6内存管理 |

每个实验分支保存该实验完成时的状态。`master` 保存最新汇总状态，不要求把后续实验内容反向同步到前面实验分支。

## 实验推进流程

1. 阅读对应 `docs/exp*/` 下的实验要求。
2. 在对应 `records/exp*/` 下建立或更新过程记录。
3. 按 checkpoint 小步执行命令、修改代码和截图。
4. 将最终报告和截图放入对应 `reports/exp*/` 目录。
5. 推送时只提交当前实验相关文件，避免把未完成实验或无关文件混入历史分支。

提交或推送前先用 `git status --short` 检查当前改动范围。
