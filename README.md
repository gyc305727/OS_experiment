# 操作系统实验仓库索引

本仓库按“实验资料、实验代码、过程记录、报告成品”分区整理，便于后续检索、复盘和分支提交。

## 目录结构

```text
.
├── AGENTS.md                  # Codex 协作规则
├── rust-toolchain             # Rust 工具链锁定
├── os/                        # 当前实验代码项目
├── docs/                      # 实验要求与模板
├── records/                   # 实验过程记录与 REPORT_SYNC_BLOCK
└── reports/                   # 已整理的实验报告成品
```

## 实验索引

| 实验 | 资料 | 代码 | 过程记录 | 报告 |
| --- | --- | --- | --- | --- |
| 实验0 环境配置 | `docs/exp0-env/` | 无独立代码项目 | 暂无 | `reports/exp0-env/` |
| 实验1 独立的可执行程序 | `docs/exp1-standalone/` | `os/` | `records/exp1-standalone/` | `reports/exp1-standalone/` |
| 实验2 裸机环境和最小化内核 | `docs/exp2-bare-metal-kernel/` | 待建立 | 待建立 | 待整理 |

## 分支建议

- `master`：仓库总入口和目录索引。
- `exp0-env-setup`：实验0环境配置资料与报告。
- `exp1-standalone-executable`：实验1独立可执行程序代码、资料、记录和报告。

提交或推送前先用 `git status --short` 检查当前改动范围，避免把未完成实验混入已完成实验分支。
