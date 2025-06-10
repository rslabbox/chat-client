# Tabs 组件模块

这个文件夹包含了聊天客户端的多标签页功能的所有相关组件和工具。

## 文件结构

```
src/components/Tabs/
├── index.ts                # 统一导出文件
├── TabManager.vue          # 标签页管理器主组件
├── TabBar.vue              # 标签页栏组件
├── TabItem.vue             # 单个标签页项组件
├── TabContent.vue          # 标签页内容容器组件
├── TabContextMenu.vue      # 标签页右键菜单组件
├── examples.ts             # 使用示例和最佳实践
└── README.md               # 本文档
```

## 组件说明

### TabManager.vue
- **作用**: 标签页管理器的主组件，负责整体的标签页管理逻辑
- **功能**: 
  - 管理标签页的创建、切换、关闭
  - 处理键盘快捷键
  - 显示空状态页面
  - 加载状态管理
- **使用**: 在 HomeView.vue 中直接使用

### TabBar.vue
- **作用**: 标签页栏，显示所有标签页和操作按钮
- **功能**:
  - 显示固定标签页和普通标签页
  - 新建标签页下拉菜单
  - 标签页管理下拉菜单
  - 右键菜单集成
- **特性**: 支持标签页滚动、响应式设计

### TabItem.vue
- **作用**: 单个标签页的显示组件
- **功能**:
  - 显示标签页图标、标题、状态
  - 支持拖拽排序
  - 中键点击关闭
  - 右键菜单触发
  - 加载和未保存状态指示
- **交互**: 点击切换、悬停效果、关闭按钮

### TabContent.vue
- **作用**: 标签页内容的容器组件
- **功能**:
  - 显示当前标签页的内容
  - 集成左右面板和消息区域
  - 支持面板大小调整
  - 可选的标签页头部信息
- **布局**: 三栏布局（左面板 + 消息区 + 右面板）

### TabContextMenu.vue
- **作用**: 标签页右键菜单组件
- **功能**:
  - 重命名标签页
  - 固定/取消固定
  - 复制标签页（开发中）
  - 关闭操作（单个、其他、右侧）
  - 显示会话信息
- **特性**: 传送门渲染、点击外部关闭、重命名对话框

## 使用方法

### 基本使用

```vue
<template>
  <div class="app">
    <!-- 直接使用标签页管理器 -->
    <TabManager />
  </div>
</template>

<script setup lang="ts">
import { TabManager } from '@/components/Tabs'
</script>
```

### 通过索引文件导入

```typescript
// 导入所有组件
import { 
  TabManager, 
  TabBar, 
  TabItem, 
  TabContent, 
  TabContextMenu 
} from '@/components/Tabs'

// 导入类型
import type { Tab, PluginSessionStats } from '@/components/Tabs'
```

### 单独导入组件

```typescript
import TabManager from '@/components/Tabs/TabManager.vue'
import TabBar from '@/components/Tabs/TabBar.vue'
// ... 其他组件
```

## 状态管理

标签页功能依赖以下 Store：

- **tabManager**: 核心标签页状态管理
- **pageManager**: 页面状态管理（扩展支持）
- **plugins**: 插件管理
- **history**: 历史消息管理
- **settings**: 设置管理

## 数据流

```
用户操作 → TabBar → TabManager → tabManagerStore → 
pageManagerStore → pluginStore/historyStore → TabContent
```

## 事件系统

### TabItem 事件
- `click`: 标签页点击
- `close`: 标签页关闭
- `context-menu`: 右键菜单
- `drag-start/drag-over/drop`: 拖拽事件

### TabContextMenu 事件
- `rename`: 重命名
- `pin`: 固定/取消固定
- `close`: 关闭
- `close-others`: 关闭其他
- `close-to-right`: 关闭右侧

## 样式设计

### 主题色彩
- 活跃标签页: `#409eff`
- 固定标签页: `#f0f9ff`
- 悬停效果: `#f5f7fa`
- 边框颜色: `#e4e7ed`

### 响应式断点
- 桌面端: 正常显示
- 平板端: 调整标签页宽度
- 移动端: 压缩标签页，隐藏部分功能

## 键盘快捷键

- `Ctrl/Cmd + T`: 新建标签页
- `Ctrl/Cmd + W`: 关闭当前标签页
- `Ctrl/Cmd + 1-9`: 切换到对应标签页
- `Ctrl/Cmd + Tab`: 切换到下一个标签页
- `Ctrl/Cmd + Shift + Tab`: 切换到上一个标签页
- `Ctrl/Cmd + Shift + T`: 恢复最近关闭的标签页（开发中）

## 性能优化

1. **虚拟化**: 大量标签页时的性能优化
2. **懒加载**: 标签页内容的按需加载
3. **缓存**: 使用 KeepAlive 缓存非活跃标签页
4. **防抖**: 拖拽和调整大小的防抖处理

## 开发指南

### 添加新功能

1. 在对应组件中添加新的方法或属性
2. 更新 tabManagerStore 中的状态管理
3. 添加相应的事件处理
4. 更新类型定义
5. 编写测试用例

### 调试技巧

```javascript
// 查看当前标签页状态
console.log(useTabManagerStore().tabs)

// 查看活跃标签页
console.log(useTabManagerStore().activeTab)

// 开启调试模式
localStorage.setItem('tabs-debug', 'true')
```

### 测试

参考 `examples.ts` 文件中的示例代码进行功能测试。

## 未来规划

- [ ] 标签页分组功能
- [ ] 标签页搜索和过滤
- [ ] 标签页导出/导入
- [ ] 多窗口支持
- [ ] 标签页同步（多设备）
- [ ] 协作标签页

## 注意事项

1. **内存管理**: 及时关闭不需要的标签页
2. **状态同步**: 确保标签页状态与插件实例状态一致
3. **错误处理**: 处理插件加载失败等异常情况
4. **用户体验**: 保持操作的一致性和直观性

## 贡献指南

1. 遵循现有的代码风格和命名规范
2. 添加适当的 TypeScript 类型注解
3. 编写清晰的注释和文档
4. 提交前进行充分测试
5. 更新相关的文档和示例
