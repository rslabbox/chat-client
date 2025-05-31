# Tauri API 模块

这个文件夹包含了所有与 Tauri 后端交互的 API 调用函数。

## 文件结构

```
src/api/
├── index.ts          # 主入口文件，导出所有 API
├── types.ts          # TypeScript 类型定义
├── plugins.ts        # 插件相关 API
└── README.md         # 说明文档
```

## 使用方法

### 1. 导入 API 函数

```typescript
// 从主入口导入
import { scanPlugins, type PluginMetadata } from '@/api'

// 或者从具体模块导入
import { scanPlugins } from '@/api/plugins'
```

### 2. 在 Vue 组件中使用

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { scanPlugins, type PluginMetadata } from '@/api'

const plugins = ref<PluginMetadata[]>([])

onMounted(async () => {
  try {
    plugins.value = await scanPlugins()
  } catch (error) {
    console.error('Failed to load plugins:', error)
  }
})
</script>
```

### 3. 错误处理

所有 API 函数都包含了错误处理逻辑，但建议在调用时也添加 try-catch：

```typescript
try {
  const result = await scanPlugins()
  // 处理成功结果
} catch (error) {
  // 处理错误
  console.error('API call failed:', error)
}
```

## 注意事项

- 所有异步函数都返回 Promise
- 错误处理统一使用 try-catch
- 类型定义与 Rust 后端保持同步
- 使用路径别名 `@/api` 进行导入
