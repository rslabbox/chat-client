<template>
  <div class="placeholder-block">
    <div class="placeholder-container">
      <div class="placeholder-icon">
        <el-icon :size="32">
          <component :is="placeholderIcon" />
        </el-icon>
      </div>
      
      <div class="placeholder-content">
        <div class="placeholder-title">{{ title }}</div>
        <div v-if="description" class="placeholder-description">
          {{ description }}
        </div>
      </div>
      
      <div v-if="showActions" class="placeholder-actions">
        <el-button size="small" @click="handleRetry">
          重试
        </el-button>
        <el-button size="small" type="info" @click="handleSkip">
          跳过
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { 
  Loading, 
  Warning, 
  InfoFilled, 
  QuestionFilled 
} from '@element-plus/icons-vue'

interface Props {
  block: any
  type?: string
  title?: string
  description?: string
  showActions?: boolean
}

interface Emits {
  (e: 'retry'): void
  (e: 'skip'): void
}

const props = withDefaults(defineProps<Props>(), {
  type: 'loading',
  showActions: false
})

const emit = defineEmits<Emits>()

// 占位符类型
const placeholderType = computed(() => {
  return props.type || props.block?.type || 'loading'
})

// 标题
const title = computed(() => {
  if (props.title) return props.title
  if (props.block?.title) return props.block.title
  
  const titleMap: Record<string, string> = {
    'loading': '加载中...',
    'error': '加载失败',
    'empty': '暂无内容',
    'processing': '处理中...',
    'waiting': '等待中...'
  }
  
  return titleMap[placeholderType.value] || '占位内容'
})

// 描述
const description = computed(() => {
  if (props.description) return props.description
  if (props.block?.description) return props.block.description
  
  const descriptionMap: Record<string, string> = {
    'loading': '正在加载内容，请稍候...',
    'error': '内容加载失败，请重试',
    'empty': '没有找到相关内容',
    'processing': '正在处理您的请求...',
    'waiting': '请等待前面的任务完成...'
  }
  
  return descriptionMap[placeholderType.value] || ''
})

// 占位符图标
const placeholderIcon = computed(() => {
  const iconMap: Record<string, any> = {
    'loading': Loading,
    'processing': Loading,
    'waiting': Loading,
    'error': Warning,
    'empty': InfoFilled,
    'default': QuestionFilled
  }
  
  return iconMap[placeholderType.value] || iconMap.default
})

// 处理重试
const handleRetry = () => {
  emit('retry')
}

// 处理跳过
const handleSkip = () => {
  emit('skip')
}
</script>

<style scoped>
.placeholder-block {
  margin: 12px 0;
}

.placeholder-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px 16px;
  background-color: var(--el-fill-color-lighter);
  border: 1px dashed var(--el-border-color);
  border-radius: 8px;
  text-align: center;
}

.placeholder-icon {
  margin-bottom: 16px;
  color: var(--el-text-color-secondary);
  opacity: 0.6;
}

.placeholder-content {
  margin-bottom: 16px;
}

.placeholder-title {
  font-size: 16px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  margin-bottom: 8px;
}

.placeholder-description {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  line-height: 1.5;
}

.placeholder-actions {
  display: flex;
  gap: 8px;
}

/* 不同类型的样式 */
.placeholder-container[data-type="loading"] .placeholder-icon,
.placeholder-container[data-type="processing"] .placeholder-icon,
.placeholder-container[data-type="waiting"] .placeholder-icon {
  animation: spin 1s linear infinite;
}

.placeholder-container[data-type="error"] {
  border-color: var(--el-color-danger-light-5);
  background-color: var(--el-color-danger-light-9);
}

.placeholder-container[data-type="error"] .placeholder-icon {
  color: var(--el-color-danger);
}

.placeholder-container[data-type="empty"] {
  border-color: var(--el-color-info-light-5);
  background-color: var(--el-color-info-light-9);
}

.placeholder-container[data-type="empty"] .placeholder-icon {
  color: var(--el-color-info);
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
