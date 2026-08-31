# 📝 待办事项 - Windows 桌面应用

Windows 10 风格待办事项桌面应用，基于 Tauri + Vite 构建。

## ✨ 功能

- ✅ 添加/删除/标记完成任务
- 🏷️ 三级优先级（高/中/低）
- 🔍 按状态筛选（全部/待办/已完成）
- 📊 实时统计与进度条
- 💾 本地数据持久化
- 🎨 Windows 10 Fluent Design

## 🚀 快速开始

### 本地开发

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建安装包
npm run tauri build
```

### 前置要求

- Node.js 18+
- Rust
- Visual Studio Build Tools（Windows，勾选"使用 C++ 的桌面开发"）

## 📦 GitHub Actions 自动构建

每次推送到 `main` 分支，GitHub Actions 会自动构建 Windows 安装包。

构建完成后，在 **Actions** → 最新运行记录 → **Artifacts** 中下载：
- `todo-app-msi` — MSI 安装程序
- `todo-app-exe` — 直接运行的可执行文件

## 📁 项目结构

```
todo-app/
├── src/              # 前端源码
├── src-tauri/        # Tauri 后端（Rust）
├── .github/          # GitHub Actions
├── index.html
├── package.json
└── vite.config.js
```
