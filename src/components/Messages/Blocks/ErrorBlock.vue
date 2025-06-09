<template>
  <div class="error-block">
    <el-alert
      :title="errorTitle"
      :type="alertType"
      :description="errorDescription"
      show-icon
      :closable="false"
    >
      <template #default>
        <div class="error-content">
          <div class="error-message">
            {{ errorMessage }}
          </div>
          
          <!-- 错误详情（可展开） -->
          <el-collapse v-if="hasDetails" class="error-details">
            <el-collapse-item name="details" title="查看详情">
              <div class="error-stack">
                <pre>{{ errorDetails }}</pre>
              </div>
            </el-collapse-item>
          </el-collapse>

          <!-- 错误操作 -->
          <div v-if="showActions" class="error-actions">
            <el-button size="small" @click="handleRetry">
              重试
            </el-button>
            <el-button size="small" type="info" @click="handleReport">
              报告问题
            </el-button>
          </div>
        </div>
      </template>
    </el-alert>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ElMessage } from 'element-plus'

interface Props {
  block: any
  errorType?: string
  errorMessage?: string
  showActions?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  errorType: 'general',
  showActions: true
})

// 错误标题
const errorTitle = computed(() => {
  const titles: Record<string, string> = {
    'network': '网络错误',
    'timeout': '请求超时',
    'auth': '认证失败',
    'rate_limit': '请求频率限制',
    'server': '服务器错误',
    'validation': '参数验证失败',
    'general': '发生错误'
  }
  return titles[props.errorType] || '发生错误'
})

// 错误描述
const errorDescription = computed(() => {
  const descriptions: Record<string, string> = {
    'network': '无法连接到服务器，请检查网络连接',
    'timeout': '请求处理时间过长，请稍后重试',
    'auth': '身份验证失败，请检查API密钥',
    'rate_limit': '请求过于频繁，请稍后再试',
    'server': '服务器内部错误，请联系管理员',
    'validation': '请求参数不正确',
    'general': '操作失败，请重试'
  }
  return descriptions[props.errorType] || '操作失败，请重试'
})

// 错误消息
const errorMessage = computed(() => {
  return props.errorMessage || props.block?.errorMessage || props.block?.content || ''
})

// 错误详情
const errorDetails = computed(() => {
  return props.block?.stack || props.block?.details || props.block?.trace || ''
})

// 是否有详情
const hasDetails = computed(() => {
  return !!errorDetails.value
})

// 警告类型
const alertType = computed(() => {
  const typeMap: Record<string, string> = {
    'network': 'warning',
    'timeout': 'warning',
    'auth': 'error',
    'rate_limit': 'warning',
    'server': 'error',
    'validation': 'warning',
    'general': 'error'
  }
  return typeMap[props.errorType] || 'error'
})

// 处理重试
const handleRetry = () => {
  // 发出重试事件
  ElMessage.info('重试功能待实现')
}

// 处理报告问题
const handleReport = () => {
  // 复制错误信息到剪贴板
  const errorInfo = {
    type: props.errorType,
    message: errorMessage.value,
    details: errorDetails.value,
    timestamp: new Date().toISOString()
  }
  
  const errorText = JSON.stringify(errorInfo, null, 2)
  
  navigator.clipboard.writeText(errorText).then(() => {
    ElMessage.success('错误信息已复制到剪贴板，请联系技术支持')
  }).catch(() => {
    ElMessage.error('复制失败')
  })
}
</script>

<style scoped>
.error-block {
  margin: 12px 0;
}

.error-content {
  margin-top: 8px;
}

.error-message {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 14px;
  line-height: 1.5;
  background-color: var(--el-fill-color-darker);
  padding: 12px;
  border-radius: 6px;
  margin-bottom: 12px;
  word-break: break-all;
  white-space: pre-wrap;
}

.error-details {
  margin: 12px 0;
}

.error-stack {
  background-color: var(--el-fill-color-darker);
  padding: 12px;
  border-radius: 6px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 12px;
  line-height: 1.4;
  overflow-x: auto;
  max-height: 300px;
  overflow-y: auto;
}

.error-stack pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
}

.error-actions {
  display: flex;
  gap: 8px;
  margin-top: 12px;
}

/* 折叠面板样式调整 */
:deep(.el-collapse) {
  border: none;
  background-color: transparent;
}

:deep(.el-collapse-item__header) {
  background-color: transparent;
  border: none;
  padding: 8px 0;
  font-size: 14px;
}

:deep(.el-collapse-item__content) {
  padding: 8px 0 0 0;
}

/* 警告框样式调整 */
:deep(.el-alert__content) {
  width: 100%;
}

:deep(.el-alert__description) {
  margin-bottom: 0;
}
</style>
