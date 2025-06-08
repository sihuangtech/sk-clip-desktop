# Multisay 样式文件组织结构

本项目采用模块化的CSS架构，将样式文件按功能和组件进行分离，便于维护和扩展。

## 文件结构

```
src/styles/
├── README.md           # 本说明文件
├── main.css           # 主样式文件，导入所有模块
├── variables.css      # CSS变量和设计系统
├── base.css          # 基础重置和全局样式
├── layout.css        # 应用布局样式
├── navigation.css    # 导航组件样式
├── components.css    # 通用组件样式
└── video-upload.css  # 视频上传组件专用样式
```

## 样式模块说明

### 1. variables.css
- CSS自定义属性（变量）
- 颜色系统
- 间距系统
- 字体系统
- 布局尺寸
- 动画时长
- 深色模式支持

### 2. base.css
- CSS重置样式
- 基础HTML元素样式
- 滚动条样式
- 选择文本样式
- 焦点样式
- 链接、标题、段落等基础样式

### 3. layout.css
- 应用主要布局结构
- 头部、主体、状态栏样式
- 响应式布局调整
- 容器和包装器样式

### 4. navigation.css
- 侧边导航样式（桌面端/平板端）
- 底部导航样式（移动端）
- 导航项交互效果
- 导航动画和工具提示

### 5. components.css
- 通用组件样式
- 按钮、卡片、表单组件
- 进度指示器、消息提示
- 徽章、分隔符、工具提示
- 模态框样式

### 6. video-upload.css
- 视频上传组件专用样式
- 上传区域、语言选择
- 状态消息、进度条
- 视频预览、错误处理

## 设计系统

### 颜色系统
- **主色调**: `--primary-color` (#2563eb)
- **成功色**: `--success-color` (#10b981)
- **警告色**: `--warning-color` (#f59e0b)
- **错误色**: `--error-color` (#ef4444)

### 间距系统
- **xs**: 0.25rem (4px)
- **sm**: 0.5rem (8px)
- **md**: 1rem (16px)
- **lg**: 1.5rem (24px)
- **xl**: 2rem (32px)
- **2xl**: 3rem (48px)

### 响应式断点
- **平板端**: max-width: 1024px
- **移动端**: max-width: 768px
- **小屏移动端**: max-width: 480px

## 使用方式

### 在根目录styles.css中导入
```css
@import './src/styles/main.css';
```

### 添加新的组件样式
1. 在 `src/styles/` 目录下创建新的CSS文件
2. 在 `main.css` 中添加 `@import` 语句
3. 使用CSS变量保持设计一致性

### 自定义主题
修改 `variables.css` 中的CSS变量即可全局调整主题：

```css
:root {
  --primary-color: #your-color;
  --border-radius: 12px;
  /* 其他变量... */
}
```

## 最佳实践

1. **使用CSS变量**: 所有颜色、间距、字体大小都应使用预定义的CSS变量
2. **模块化**: 每个组件的样式应该独立成文件
3. **响应式优先**: 使用移动端优先的响应式设计
4. **语义化类名**: 使用有意义的类名，避免样式耦合
5. **性能优化**: 避免深层嵌套，合理使用CSS选择器

## 深色模式支持

项目内置深色模式支持，通过 `@media (prefers-color-scheme: dark)` 自动切换。如需手动控制，可以添加类名切换机制。

## 工具类

`main.css` 中包含了常用的工具类：
- 布局: `.flex`, `.flex-col`, `.items-center`
- 间距: `.m-1`, `.p-2`, `.m-4`
- 显示: `.hidden`, `.block`, `.inline`
- 动画: `.fade-in`, `.slide-up`, `.scale-in` 