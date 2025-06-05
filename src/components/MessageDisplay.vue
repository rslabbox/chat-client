<template>
  <div class="message-display">
    <div class="message-header">
      <h3>消息记录</h3>
      <el-button type="primary" size="small" @click="handleNewChat" :icon="Plus" plain>
        新的聊天
      </el-button>
    </div>

    <div ref="messageContainer" class="message-container">
      <div v-for="message in currentMessages" :key="message.id"
        :class="['message-item', message.type, getMessageTypeClass(message)]">
        <div class="message-content">
          <!-- 流式消息特殊处理 -->
          <div v-if="message.isStreaming" class="streaming-message">
            <div class="message-text">
              <span class="streaming-content">{{ message.content }}</span>
              <span class="streaming-cursor">|</span>
            </div>
            <div class="streaming-status">
              <el-icon class="streaming-icon">
                <Loading />
              </el-icon>
              <span class="streaming-text">正在接收...</span>
            </div>
          </div>

          <!-- 普通消息 -->
          <div v-else class="normal-message">
            <div class="message-text">{{ message.content }}</div>

            <!-- 消息类型标识 -->
            <!-- <div v-if="message.messageType && message.messageType !== 'normal'" class="message-type-badge">
              <el-icon>
                <SuccessFilled v-if="message.messageType === 'success'" />
                <WarningFilled v-else-if="message.messageType === 'warning'" />
                <CircleCloseFilled v-else-if="message.messageType === 'error'" />
                <InfoFilled v-else-if="message.messageType === 'info'" />
              </el-icon>
              <span>{{ getMessageTypeText(message.messageType) }}</span>
            </div> -->
          </div>

          <div class="message-time">
            {{ formatTime(message.timestamp) }}
          </div>
        </div>
      </div>

      <div v-if="currentMessages.length === 0" class="empty-messages">
        <el-empty description="暂无消息" :image-size="100" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, watch } from 'vue'
import { storeToRefs } from 'pinia'
import {
  Plus,
  Loading,
  SuccessFilled,
  WarningFilled,
  CircleCloseFilled,
  InfoFilled
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useMessageStore } from '@/stores/messages'

const messageStore = useMessageStore()
const { currentMessages } = storeToRefs(messageStore)

const messageContainer = ref<HTMLElement>()

const formatTime = (date: Date) => {
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 获取消息类型的CSS类名
const getMessageTypeClass = (message: any) => {
  const classes = []

  if (message.messageType && message.messageType !== 'normal') {
    classes.push(`message-type-${message.messageType}`)
  }

  if (message.isStreaming) {
    classes.push('streaming')
  }

  return classes.join(' ')
}

// 获取消息类型的文本
const getMessageTypeText = (messageType: string) => {
  const typeMap: Record<string, string> = {
    success: '成功',
    warning: '警告',
    error: '错误',
    info: '信息'
  }
  return typeMap[messageType] || messageType
}

const handleNewChat = () => {
  const newSession = messageStore.createNewSession()
  if (newSession) {
    ElMessage.success('已创建新的聊天')
  } else {
    ElMessage.error('创建聊天失败')
  }
}

const scrollToBottom = () => {
  nextTick(() => {
    if (messageContainer.value) {
      messageContainer.value.scrollTop = messageContainer.value.scrollHeight
    }
  })
}

// 监听消息变化，自动滚动到底部
watch(currentMessages, () => {
  scrollToBottom()
}, { deep: true, immediate: true })
</script>

<style scoped>
.message-display {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px;
}

.message-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
  border-bottom: 1px solid #e4e7ed;
  padding-bottom: 10px;
}

.message-header h3 {
  margin: 0;
  color: #303133;
  font-size: 16px;
  font-weight: 600;
}

.message-container {
  flex: 1;
  overflow-y: auto;
  padding: 10px 0;
}

.message-item {
  margin-bottom: 15px;
  display: flex;
}

.message-item.sent {
  justify-content: flex-end;
}

.message-item.received {
  justify-content: flex-start;
}

.message-content {
  max-width: 70%;
  padding: 10px 15px;
  border-radius: 10px;
  position: relative;
}

.sent .message-content {
  background-color: #409eff;
  color: white;
}

.received .message-content {
  background-color: #f0f0f0;
  color: #303133;
}

.message-text {
  word-wrap: break-word;
  line-height: 1.4;
}

.message-time {
  font-size: 12px;
  margin-top: 5px;
  opacity: 0.7;
}

.empty-messages {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 滚动条样式 */
.message-container::-webkit-scrollbar {
  width: 6px;
}

.message-container::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.message-container::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.message-container::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
/* 流式消息样式 */
.streaming-message {
  position: relative;
}

.streaming-content {
  display: inline;
}

.streaming-cursor {
  display: inline-block;
  animation: blink 1s infinite;
  color: #409eff;
  font-weight: bold;
}

@keyframes blink {

  0%,
  50% {
    opacity: 1;
  }

  51%,
  100% {
    opacity: 0;
  }
}

.streaming-status {
  display: flex;
  align-items: center;
  margin-top: 8px;
  font-size: 12px;
  color: #909399;
}

.streaming-icon {
  margin-right: 4px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

.streaming-text {
  font-style: italic;
}

/* 消息类型样式 */
.message-type-badge {
  display: flex;
  align-items: center;
  margin-top: 6px;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 500;
}

.message-type-badge .el-icon {
  margin-right: 3px;
  font-size: 12px;
}

/* 不同消息类型的颜色 */
.message-type-success .message-content {
  border-left: 3px solid #67c23a;
}

.message-type-success .message-type-badge {
  background-color: #f0f9ff;
  color: #67c23a;
}

.message-type-warning .message-content {
  border-left: 3px solid #e6a23c;
}

.message-type-warning .message-type-badge {
  background-color: #fdf6ec;
  color: #e6a23c;
}

.message-type-error .message-content {
  border-left: 3px solid #f56c6c;
}

.message-type-error .message-type-badge {
  background-color: #fef0f0;
  color: #f56c6c;
}

.message-type-info .message-content {
  border-left: 3px solid #409eff;
}

.message-type-info .message-type-badge {
  background-color: #ecf5ff;
  color: #409eff;
}

/* 流式消息特殊样式 */
.streaming .message-content {
  border-left: 3px solid #409eff;
  background: linear-gradient(90deg, #f0f0f0 0%, #f8f9fa 100%);
  position: relative;
  overflow: hidden;
}

.streaming .message-content::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(64, 158, 255, 0.1), transparent);
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  0% {
    left: -100%;
  }

  100% {
    left: 100%;
  }
}

/* 发送消息的流式样式调整 */
.sent.streaming .message-content {
  background: linear-gradient(90deg, #409eff 0%, #66b1ff 100%);
  border-left: 3px solid #ffffff;
}

.sent.streaming .message-content::before {
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
}
</style>