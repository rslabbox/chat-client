<template>
  <div v-if="showTokens" class="message-tokens">
    <div class="token-info">
      <!-- Token 数量 -->
      <span v-if="tokens" class="token-count">
        <el-icon><Coin /></el-icon>
        {{ tokens }} tokens
      </span>

      <!-- 成本信息 -->
      <span v-if="cost" class="cost-info">
        <el-icon><Money /></el-icon>
        ¥{{ cost.toFixed(4) }}
      </span>

      <!-- 处理时间 -->
      <span v-if="processingTime" class="processing-time">
        <el-icon><Timer /></el-icon>
        {{ processingTime }}ms
      </span>
    </div>

    <!-- 详细统计（可展开） -->
    <el-collapse v-if="hasDetailedStats" class="detailed-stats">
      <el-collapse-item name="stats" title="详细统计">
        <div class="stats-grid">
          <div v-if="inputTokens" class="stat-item">
            <span class="label">输入 Tokens:</span>
            <span class="value">{{ inputTokens }}</span>
          </div>
          <div v-if="outputTokens" class="stat-item">
            <span class="label">输出 Tokens:</span>
            <span class="value">{{ outputTokens }}</span>
          </div>
          <div v-if="inputCost" class="stat-item">
            <span class="label">输入成本:</span>
            <span class="value">¥{{ inputCost.toFixed(6) }}</span>
          </div>
          <div v-if="outputCost" class="stat-item">
            <span class="label">输出成本:</span>
            <span class="value">¥{{ outputCost.toFixed(6) }}</span>
          </div>
        </div>
      </el-collapse-item>
    </el-collapse>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Coin, Money, Timer } from '@element-plus/icons-vue'

interface Props {
  message: any
}

const props = defineProps<Props>()

// 计算是否显示 token 信息
const showTokens = computed(() => {
  return props.message.usage || props.message.tokens || props.message.cost
})

// 基础统计信息
const tokens = computed(() => {
  return props.message.usage?.total_tokens || props.message.tokens
})

const cost = computed(() => {
  return props.message.usage?.total_cost || props.message.cost
})

const processingTime = computed(() => {
  return props.message.processingTime || props.message.duration
})

// 详细统计信息
const inputTokens = computed(() => {
  return props.message.usage?.prompt_tokens || props.message.usage?.input_tokens
})

const outputTokens = computed(() => {
  return props.message.usage?.completion_tokens || props.message.usage?.output_tokens
})

const inputCost = computed(() => {
  return props.message.usage?.prompt_cost || props.message.usage?.input_cost
})

const outputCost = computed(() => {
  return props.message.usage?.completion_cost || props.message.usage?.output_cost
})

// 是否有详细统计信息
const hasDetailedStats = computed(() => {
  return inputTokens.value || outputTokens.value || inputCost.value || outputCost.value
})
</script>

<style scoped>
.message-tokens {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.token-info {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.token-count,
.cost-info,
.processing-time {
  display: flex;
  align-items: center;
  gap: 4px;
  opacity: 0.8;
}

.cost-info {
  color: var(--el-color-warning);
  font-weight: 500;
}

.processing-time {
  color: var(--el-color-info);
}

.detailed-stats {
  margin-top: 8px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 8px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 4px 8px;
  background-color: var(--el-fill-color-lighter);
  border-radius: 4px;
}

.label {
  font-weight: 500;
  color: var(--el-text-color-regular);
}

.value {
  font-weight: 600;
  color: var(--el-text-color-primary);
}

/* 折叠面板样式调整 */
:deep(.el-collapse) {
  border: none;
}

:deep(.el-collapse-item__header) {
  background-color: transparent;
  border: none;
  padding: 4px 0;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

:deep(.el-collapse-item__content) {
  padding: 8px 0 0 0;
}
</style>
