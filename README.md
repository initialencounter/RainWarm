# RainWarm

![rainwarm](https://socialify.git.ci/initialencounter/RainWarm/image?description=1&font=Rokkitt&forks=1&issues=1&language=1&name=1&owner=1&pattern=Diagonal%20Stripes&pulls=1&stargazers=1&theme=Dark)

# 使用方法

## 安装（已安装则忽略）

前往 [Releases]('./releases') 下载最新版本的安装包，运行安装程序即可。

- 注意：安装包可能会被 Windows Defender 拦截，需要手动允许。

## 基础使用

1. 打开软件
2. 将要对比的文件拖入软件
3. 相同的文件会标注相同的颜色

## 快捷键

点击文件可以选中它，然后使用快捷键：

- `Ctrl + Shift + O` 打开文件所在目录
- `Ctrl + Shift + D` 移除文件
- `Ctrl + Shift + W` 使用 WPS 打开文件（需要设置wps路径）
- `Ctrl + Shift + B` 隐藏或显示 App
- `Ctrl + Shift + I` 打开开发者工具
- `Ctrl + R` 刷新页面


# ChangLog
<details>
<summary>点我查看更新日志</summary>

- v0.2.3
  - 
  - 升级 `tauri` 到 `v2.0.0-rc`

- v0.2.2
  - 
  - 桌面端使用 `el-table`
  - 引入快捷键

- v0.2.1
  - 
  - 修复
    - Windows10 窗口置顶
  - 优化
    - 文件显示

- v0.2.0
  -
  - 迁移到 v2
  - 重构
    - 桌面段使用 blake2 代替 md5
    - 多线程计算 blake2

- v0.1.0
  -
    - 透明背景

- v0.0.9
  - 
    - use element-plus
    - 去除边框

- v0.0.8
  - 
    - 使用 dialog API
    - 优化布局

- v0.0.7
  - 
    - 窗口置于桌面顶层

- v0.0.6
  - 
    - 新增移除文件按钮

- v0.0.5
  - 
    - 新增检查更新

- v0.0.4
  - 
    - 新增系统托盘(systemTray)

- v0.0.3
  - 
    - 使用 spark-md5 在前端计算 MD5

- v0.0.2
  - 
    - MD5 相同的文件会标注相同的颜色

- v0.0.1
  -
  - 能跑了

</details>