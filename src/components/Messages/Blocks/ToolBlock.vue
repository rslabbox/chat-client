<template>
  <div class="tool-block">
    <div class="tool-container">
      <!-- 工具头部 -->
      <div class="tool-header">
        <div class="tool-info">
          <el-icon class="tool-icon">
            <Tools />
          </el-icon>
          <span class="tool-name">{{ toolName }}</span>
          <el-tag :type="statusType" size="small">
            {{ statusText }}
          </el-tag>
        </div>
        
        <div class="tool-actions">
          <el-button size="small" text @click="toggleExpanded">
            <el-icon>
              <component :is="expandIcon" />
            </el-icon>
          </el-button>
        </div>
      </div>

      <!-- 工具内容 -->
      <el-collapse-transition>
        <div v-show="expanded" class="tool-content">
          <!-- 工具输入 -->
          <div v-if="toolInput" class="tool-section">
            <h5 class="section-title">输入参数</h5>
            <div class="tool-input">
              <pre>{{ formatInput(toolInput) }}</pre>
            </div>
          </div>

          <!-- 工具输出 -->
          <div v-if="toolOutput" class="tool-section">
            <h5 class="section-title">执行结果</h5>
            <div class="tool-output">
              <MarkdownRenderer v-if="isMarkdownOutput" :content="toolOutput" />
              <pre v-else>{{ toolOutput }}</pre>
            </div>
          </div>

          <!-- 错误信息 -->
          <div v-if="toolError" class="tool-section">
            <h5 class="section-title error">错误信息</h5>
            <div class="tool-error">
              <pre>{{ toolError }}</pre>
            </div>
          </div>
        </div>
      </el-collapse-transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Tools, ArrowDown, ArrowUp } from '@element-plus/icons-vue'
import MarkdownRenderer from '../Markdown/MarkdownRenderer.vue'

interface Props {
  block: any
  toolName?: string
  toolInput?: any
  toolOutput?: string
  status?: string
}

const props = defineProps<Props>()

const expanded = ref(false)

// 工具名称
const toolName = computed(() => {
  return props.toolName || props.block?.toolName || props.block?.name || '工具调用'
})

// 工具输入
const toolInput = computed(() => {
  return props.toolInput || props.block?.toolInput || props.block?.input
})

// 工具输出
const toolOutput = computed(() => {
  return props.toolOutput || props.block?.toolOutput || props.block?.output || props.block?.result
})

// 工具错误
const toolError = computed(() => {
  return props.block?.error || props.block?.errorMessage
})

// 状态
const status = computed(() => {
  return props.status || props.block?.status || 'completed'
})

// 状态文本
const statusText = computed(() => {
  const statusMap: Record<string, string> = {
    'pending': '等待中',
    'running': '执行中',
    'completed': '已完成',
    'error': '执行失败',
    'timeout': '执行超时'
  }
  return statusMap[status.value] || status.value
})

// 状态类型
const statusType = computed(() => {
  const typeMap: Record<string, string> = {
    'pending': 'info',
    'running': 'warning',
    'completed': 'success',
    'error': 'danger',
    'timeout': 'warning'
  }
  return typeMap[status.value] || 'info'
})

// 展开图标
const expandIcon = computed(() => {
  return expanded.value ? ArrowUp : ArrowDown
})

// 是否为 Markdown 输出
const isMarkdownOutput = computed(() => {
  const output = toolOutput.value
  if (!output) return false
  
  // 简单判断是否包含 Markdown 语法
  return /[#*`\[\]()_~]/.test(output) || output.includes('\n')
})

// 切换展开状态
const toggleExpanded = () => {
  expanded.value = !expanded.value
}

// 格式化输入参数
const formatInput = (input: any) => {
  if (typeof input === 'string') {
    return input
  }
  return JSON.stringify(input, null, 2)
}
</script>

<style scoped>
.tool-block {
  margin: 12px 0;
}

.tool-container {
  border: 1px solid var(--el-border-color-light);
  border-radius: 8px;
  overflow: hidden;
  background-color: var(--el-fill-color-lighter);
}

.tool-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background-color: var(--el-fill-color-light);
  border-bottom: 1px solid var(--el-border-color-light);
  cursor: pointer;
}

.tool-header:hover {
  background-color: var(--el-fill-color);
}

.tool-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tool-icon {
  color: var(--el-color-primary);
}

.tool-name {
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.tool-actions {
  display: flex;
  gap: 4px;
}

.tool-content {
  padding: 16px;
  background-color: var(--el-bg-color);
}

.tool-section {
  margin-bottom: 16px;
}

.tool-section:last-child {
  margin-bottom: 0;
}

.section-title {
  margin: 0 0 8px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.section-title.error {
  color: var(--el-color-danger);
}

.tool-input,
.tool-output,
.tool-error {
  background-color: var(--el-fill-color-lighter);
  border: 1px solid var(--el-border-color-light);
  border-radius: 6px;
  padding: 12px;
  overflow-x: auto;
}

.tool-input pre,
.tool-output pre,
.tool-error pre {
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 13px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
}

.tool-error {
  background-color: var(--el-color-danger-light-9);
  border-color: var(--el-color-danger-light-5);
}

.tool-error pre {
  color: var(--el-color-danger);
}

/* 不同状态的样式 */
.tool-container[data-status="pending"] {
  border-left: 3px solid var(--el-color-info);
}

.tool-container[data-status="running"] {
  border-left: 3px solid var(--el-color-warning);
}

.tool-container[data-status="completed"] {
  border-left: 3px solid var(--el-color-success);
}

.tool-container[data-status="error"] {
  border-left: 3px solid var(--el-color-danger);
}

.tool-container[data-status="timeout"] {
  border-left: 3px solid var(--el-color-warning);
}
</style>
