<template>
  <div class="message-menubar">
    <div class="menu-buttons">
      <!-- 复制按钮 -->
      <el-tooltip content="复制消息" placement="top">
        <el-button size="small" text @click="handleCopy" :icon="DocumentCopy" />
      </el-tooltip>

      <!-- 更多操作下拉菜单 -->
      <el-dropdown @command="handleCommand" trigger="click">
        <el-button size="small" text :icon="MoreFilled" />
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="copy-raw">
              <el-icon>
                <DocumentCopy />
              </el-icon>
              复制原始内容
            </el-dropdown-item>
            <el-dropdown-item command="copy-markdown">
              <el-icon>
                <Document />
              </el-icon>
              复制为 Markdown
            </el-dropdown-item>
            <el-dropdown-item divided command="delete" class="danger">
              <el-icon>
                <Delete />
              </el-icon>
              删除消息
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>

    <!-- 消息统计信息 -->
    <div v-if="showStats" class="message-stats">
      <span v-if="message.tokens" class="token-count">
        {{ message.tokens }} tokens
      </span>
      <span v-if="message.cost" class="cost">
        ¥{{ message.cost.toFixed(4) }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">

import {
  DocumentCopy,
  MoreFilled,
  Document,
  Delete
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

interface Props {
  message: any
  showStats?: boolean
}

interface Emits {
  (e: 'delete-message'): void
  (e: 'copy-message'): void
}

const props = withDefaults(defineProps<Props>(), {
  showStats: true
})

const emit = defineEmits<Emits>()

// 处理复制
const handleCopy = () => {
  emit('copy-message')
}

// 处理下拉菜单命令
const handleCommand = (command: string) => {
  switch (command) {
    case 'copy-raw':
      copyToClipboard(props.message.content)
      break
    case 'copy-markdown':
      copyToClipboard(formatAsMarkdown(props.message))
      break
    case 'delete':
      emit('delete-message')
      break
  }
}

// 复制到剪贴板
const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text).then(() => {
    ElMessage.success('已复制到剪贴板')
  }).catch(() => {
    ElMessage.error('复制失败')
  })
}

// 格式化为 Markdown
const formatAsMarkdown = (message: any) => {
  const timestamp = new Date(message.timestamp).toLocaleString('zh-CN')
  const role = message.role === 'user' ? '用户' : 'AI助手'

  return `## ${role} (${timestamp})

${message.content}`
}
</script>

<style scoped>
.message-menubar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 12px;
  background-color: var(--el-fill-color-lighter);
  border-radius: 6px;
  border: 1px solid var(--el-border-color-light);
  margin-top: 8px;
}

.menu-buttons {
  display: flex;
  align-items: center;
  gap: 2px;
}

.message-stats {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.token-count {
  opacity: 0.8;
}

.cost {
  color: var(--el-color-warning);
  font-weight: 500;
}

/* 危险操作样式 */
:deep(.danger) {
  color: var(--el-color-danger);
}

:deep(.danger:hover) {
  background-color: var(--el-color-danger-light-9);
}
</style>
