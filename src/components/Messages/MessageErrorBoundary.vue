<template>
  <div class="message-error-boundary">
    <!-- 错误状态 -->
    <div v-if="hasError" class="error-container">
      <el-alert
        title="消息渲染错误"
        type="error"
        :description="errorMessage"
        show-icon
        :closable="false"
      />
      
      <div class="error-actions">
        <el-button size="small" @click="retry">
          重试
        </el-button>
        <el-button size="small" type="info" @click="showDetails = !showDetails">
          {{ showDetails ? '隐藏' : '显示' }}详情
        </el-button>
      </div>

      <!-- 错误详情 -->
      <el-collapse v-if="showDetails" class="error-details">
        <el-collapse-item name="error" title="错误详情">
          <pre class="error-stack">{{ errorStack }}</pre>
        </el-collapse-item>
      </el-collapse>

      <!-- 降级内容 -->
      <div class="fallback-content">
        <h4>原始内容：</h4>
        <div class="raw-content">
          {{ rawContent }}
        </div>
      </div>
    </div>

    <!-- 正常内容 -->
    <div v-else>
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onErrorCaptured, computed } from 'vue'

interface Props {
  message?: any
}

const props = defineProps<Props>()

const hasError = ref(false)
const error = ref<Error | null>(null)
const showDetails = ref(false)

// 错误信息
const errorMessage = computed(() => {
  if (!error.value) return ''
  return error.value.message || '未知错误'
})

// 错误堆栈
const errorStack = computed(() => {
  if (!error.value) return ''
  return error.value.stack || '无堆栈信息'
})

// 原始内容
const rawContent = computed(() => {
  if (!props.message) return '无消息内容'
  return JSON.stringify(props.message, null, 2)
})

// 捕获错误
onErrorCaptured((err: Error) => {
  console.error('MessageErrorBoundary 捕获到错误:', err)
  hasError.value = true
  error.value = err
  return false // 阻止错误继续传播
})

// 重试
const retry = () => {
  hasError.value = false
  error.value = null
  showDetails.value = false
}
</script>

<style scoped>
.message-error-boundary {
  width: 100%;
}

.error-container {
  padding: 16px;
  border: 1px solid var(--el-color-danger-light-5);
  border-radius: 8px;
  background-color: var(--el-color-danger-light-9);
}

.error-actions {
  margin-top: 12px;
  display: flex;
  gap: 8px;
}

.error-details {
  margin-top: 12px;
}

.error-stack {
  background-color: var(--el-fill-color-darker);
  padding: 12px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 12px;
  line-height: 1.4;
  overflow-x: auto;
  white-space: pre-wrap;
  word-break: break-all;
}

.fallback-content {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--el-border-color-light);
}

.fallback-content h4 {
  margin: 0 0 8px 0;
  font-size: 14px;
  color: var(--el-text-color-primary);
}

.raw-content {
  background-color: var(--el-fill-color-light);
  padding: 12px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 12px;
  line-height: 1.4;
  overflow-x: auto;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 200px;
  overflow-y: auto;
}

/* 折叠面板样式调整 */
:deep(.el-collapse) {
  border: none;
}

:deep(.el-collapse-item__header) {
  background-color: transparent;
  border: none;
  padding: 8px 0;
}

:deep(.el-collapse-item__content) {
  padding: 8px 0 0 0;
}
</style>
