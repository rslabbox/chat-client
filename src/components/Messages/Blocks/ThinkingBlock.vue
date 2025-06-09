<template>
  <div class="thinking-block">
    <el-collapse v-model="activeNames" class="thinking-collapse">
      <el-collapse-item :name="blockId" class="thinking-item">
        <template #title>
          <div class="thinking-header">
            <el-icon class="thinking-icon">
              <component :is="thinkingIcon" />
            </el-icon>
            <span class="thinking-title">{{ title }}</span>
            <el-tag v-if="showStatus" :type="statusType" size="small">
              {{ statusText }}
            </el-tag>
          </div>
        </template>

        <div class="thinking-content">
          <MarkdownRenderer :content="content" />
        </div>
      </el-collapse-item>
    </el-collapse>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Loading, Check, Warning } from '@element-plus/icons-vue'
import MarkdownRenderer from '../Markdown/MarkdownRenderer.vue'

interface Props {
  block: any
  title?: string
  isCollapsed?: boolean
  showStatus?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '思考过程',
  isCollapsed: true,
  showStatus: true
})

const blockId = computed(() => props.block?.id || 'thinking')
const activeNames = ref(props.isCollapsed ? [] : [blockId.value])

// 内容
const content = computed(() => {
  return props.block?.content || props.block?.thinking || ''
})

// 标题
const title = computed(() => {
  return props.title || props.block?.title || '思考过程'
})

// 状态
const status = computed(() => {
  return props.block?.status || 'completed'
})

// 状态文本
const statusText = computed(() => {
  const statusMap: Record<string, string> = {
    'thinking': '思考中...',
    'processing': '处理中...',
    'completed': '已完成',
    'error': '出错了'
  }
  return statusMap[status.value] || status.value
})

// 状态类型
const statusType = computed(() => {
  const typeMap: Record<string, string> = {
    'thinking': 'warning',
    'processing': 'info',
    'completed': 'success',
    'error': 'danger'
  }
  return typeMap[status.value] || 'info'
})

// 思考图标
const thinkingIcon = computed(() => {
  const iconMap: Record<string, any> = {
    'thinking': Loading,
    'processing': Loading,
    'completed': Check,
    'error': Warning
  }
  return iconMap[status.value] || Loading
})
</script>

<style scoped>
.thinking-block {
  margin: 12px 0;
}

.thinking-collapse {
  border: 1px solid var(--el-border-color-light);
  border-radius: 8px;
  overflow: hidden;
}

.thinking-item {
  background-color: var(--el-fill-color-lighter);
}

.thinking-header {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.thinking-icon {
  color: var(--el-color-primary);
  animation: var(--thinking-animation, none);
}

.thinking-title {
  flex: 1;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.thinking-content {
  padding: 16px;
  background-color: var(--el-bg-color);
  border-top: 1px solid var(--el-border-color-light);
}

/* 动画效果 */
.thinking-block:has([data-status="thinking"]) .thinking-icon,
.thinking-block:has([data-status="processing"]) .thinking-icon {
  --thinking-animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

/* 折叠面板样式调整 */
:deep(.el-collapse) {
  border: none;
}

:deep(.el-collapse-item__header) {
  background-color: var(--el-fill-color-lighter);
  border: none;
  padding: 12px 16px;
  font-size: 14px;
}

:deep(.el-collapse-item__content) {
  padding: 0;
}

:deep(.el-collapse-item__arrow) {
  color: var(--el-color-primary);
}

/* 不同状态的样式 */
.thinking-item[data-status="thinking"] {
  border-left: 3px solid var(--el-color-warning);
}

.thinking-item[data-status="processing"] {
  border-left: 3px solid var(--el-color-info);
}

.thinking-item[data-status="completed"] {
  border-left: 3px solid var(--el-color-success);
}

.thinking-item[data-status="error"] {
  border-left: 3px solid var(--el-color-danger);
}
</style>
