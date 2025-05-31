<template>
  <div class="message-display">
    <div class="message-header">
      <h3>消息记录</h3>
      <el-button 
        type="danger" 
        size="small"
        @click="handleClear"
        :icon="Delete"
        plain
      >
        清空消息
      </el-button>
    </div>
    
    <div ref="messageContainer" class="message-container">
      <div
        v-for="message in currentMessages"
        :key="message.id"
        :class="['message-item', message.type]"
      >
        <div class="message-content">
          <div class="message-text">{{ message.content }}</div>
          <div class="message-time">
            {{ formatTime(message.timestamp) }}
          </div>
        </div>
      </div>

      <div v-if="currentMessages.length === 0" class="empty-messages">
        <el-empty
          description="暂无消息"
          :image-size="100"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { Delete } from '@element-plus/icons-vue'
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

const handleClear = () => {
  messageStore.clearPluginMessages()
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
</style>