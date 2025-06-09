<template>
  <div ref="messageContainerRef" :class="['message-container', getMessageClasses()]" :style="messageStyle">
    <!-- 消息头部 -->
    <MessageHeader :message="message" :index="index" :total="total" />

    <!-- 消息内容容器 -->
    <div class="message-content-container">
      <!-- 消息内容 -->
      <MessageErrorBoundary>
        <MessageContent :message="message" />
      </MessageErrorBoundary>

      <!-- 消息底部信息 -->
      <div class="message-footer">
        <MessageTokens :message="message" />
        <div class="message-time">
          {{ formatTime(message.timestamp) }}
        </div>
      </div>

      <!-- 消息操作按钮 -->
      <MessageMenubar v-if="showMenubar" :message="message" :class="['menubar', { show: showMenubar }]"
        @delete-message="handleDeleteMessage" @copy-message="handleCopyMessage" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, useTemplateRef } from 'vue'
import MessageContent from './MessageContent.vue'
import MessageErrorBoundary from './MessageErrorBoundary.vue'
import MessageHeader from './MessageHeader.vue'
import MessageMenubar from './MessageMenubar.vue'
import MessageTokens from './MessageTokens.vue'

interface Props {
  message: any
  index?: number
  total?: number
  hideMenuBar?: boolean
  style?: Record<string, any>
  isGrouped?: boolean
  isStreaming?: boolean
}

interface Emits {
  (e: 'delete-message', messageId: string): void
  (e: 'copy-message', content: string): void
}

const props = withDefaults(defineProps<Props>(), {
  index: 0,
  total: 0,
  hideMenuBar: false,
  style: () => ({}),
  isGrouped: false,
  isStreaming: false
})

const emit = defineEmits<Emits>()

const messageContainerRef = useTemplateRef<HTMLElement>('messageContainerRef')

// 计算属性
const showMenubar = computed(() =>
  !props.hideMenuBar &&
  !props.isStreaming &&
  !props.message.status?.includes('ing')
)

const messageStyle = computed(() => ({
  ...props.style,
  // 可以根据消息类型添加不同的样式
}))

const getMessageClasses = () => {
  const classes = ['message-item']

  if (props.message.type) {
    classes.push(props.message.type)
  }

  if (props.message.role) {
    classes.push(`role-${props.message.role}`)
  }

  if (props.isStreaming) {
    classes.push('streaming')
  }

  return classes
}

// 时间格式化
const formatTime = (date: Date) => {
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 事件处理
const handleDeleteMessage = () => {
  emit('delete-message', props.message.id)
}

const handleCopyMessage = () => {
  emit('copy-message', props.message.content)
}




</script>

<style scoped>
.message-container {
  display: flex;
  flex-direction: column;
  position: relative;
  transition: background-color 0.3s ease;
  padding: 0 20px;
  margin: 0 20px 15px 20px;
  transform: translateZ(0);
  will-change: transform;
}

.message-container.message-highlight {
  background-color: var(--el-color-primary-light-9);
}

.message-content-container {
  max-width: calc(100% - 86px);
  display: flex;
  flex: 1;
  flex-direction: column;
  justify-content: space-between;
  margin-left: 36px;
  margin-top: 5px;
  overflow: visible;
}

.message-footer {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  padding: 2px 0;
  margin-top: 2px;
  border-top: 1px dotted var(--el-border-color);
  gap: 20px;
}

.message-time {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  opacity: 0.7;
}

.menubar {
  opacity: 0;
  transition: opacity 0.2s ease;
  transform: translateZ(0);
  will-change: opacity;
  margin-top: 8px;
}

.menubar.show {
  opacity: 1;
}

.message-container:hover .menubar {
  opacity: 1;
}

/* 不同角色的消息样式可以在这里添加 */
/* 流式消息样式可以在这里添加 */
</style>
