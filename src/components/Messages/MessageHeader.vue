<template>
  <div class="message-header">
    <!-- 头像 -->
    <div class="avatar">
      <el-avatar :size="32" :src="avatarSrc" :style="{ backgroundColor: avatarBgColor }">
        <el-icon v-if="!avatarSrc">
          <component :is="avatarIcon" />
        </el-icon>
      </el-avatar>
    </div>

    <!-- 消息信息 -->
    <div class="message-info">
      <!-- 发送者名称 -->
      <div class="sender-name">
        {{ senderName }}
      </div>

      <!-- 消息状态和时间 -->
      <div class="message-meta">
        <span v-if="message.status" class="status">
          {{ getStatusText(message.status) }}
        </span>
        <!-- <span class="timestamp">
          {{ formatTime(message.createdAt) }}
        </span> -->
        <span v-if="showIndex" class="index">
          #{{ index + 1 }}/{{ total }}
        </span>
      </div>
    </div>

    <!-- 模型信息 -->
    <!-- <div v-if="message.model" class="model-info">
      <el-tag size="small" type="info">
        {{ message.model.name || message.model.id }}
      </el-tag>
    </div> -->
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { User, Setting } from '@element-plus/icons-vue'
import { BaseMessage } from '@/stores/history'

interface Props {
  message: BaseMessage
  index?: number
  total?: number
  showIndex?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  index: 0,
  total: 0,
  showIndex: false
})

// 计算发送者名称
const senderName = computed(() => {
  switch (props.message.role) {
    case 'user':
      return '用户'
    case 'plugin':
      return '插件'
    case 'system':
      return '系统'
    default:
      return '未知'
  }
})

// 计算头像
const avatarSrc = computed(() => {
  // if (props.message.role === 'assistant' && props.message.assistant?.avatar) {
  //   return props.message.assistant.avatar
  // }
  return null
})

const avatarIcon = computed(() => {
  switch (props.message.role) {
    case 'user':
      return User
    case 'plugin':
      return Setting
    case 'system':
      return Setting
    default:
      return User
  }
})

const avatarBgColor = computed(() => {
  switch (props.message.role) {
    case 'user':
      return 'var(--el-color-primary)'
    case 'plugin':
      return 'var(--el-color-success)'
    case 'system':
      return 'var(--el-color-warning)'
    default:
      return 'var(--el-color-info)'
  }
})

// 获取状态文本
const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'sending': '发送中...',
    'sent': '已发送',
    'received': '已接收',
    'error': '发送失败',
    'processing': '处理中...',
    'completed': '已完成'
  }
  return statusMap[status] || status
}
</script>

<style scoped>
.message-header {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  margin-bottom: 8px;
}

.avatar {
  flex-shrink: 0;
}

.message-info {
  flex: 1;
  min-width: 0;
}

.sender-name {
  font-weight: 600;
  font-size: 14px;
  color: var(--el-text-color-primary);
  margin-bottom: 2px;
}

.message-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.status {
  color: var(--el-color-primary);
}

.timestamp {
  opacity: 0.8;
}

.index {
  opacity: 0.6;
}

.model-info {
  flex-shrink: 0;
}
</style>
