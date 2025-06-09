<template>
  <div class="message-display">
    <div class="message-header">
      <h3>消息记录</h3>
      <el-button type="primary" size="small" @click="handleNewChat" :icon="Plus" plain>
        新的聊天
      </el-button>
    </div>

    <div ref="messageContainer" class="message-container">
      <MessageItem v-for="(message, index) in currentMessages" :key="message.id" :message="message" :index="index"
        :total="currentMessages.length" :isStreaming="message.isStreaming" @delete-message="handleDeleteMessage"
        @copy-message="handleCopyMessage" />

      <div v-if="currentMessages.length === 0" class="empty-messages">
        <el-empty description="暂无消息" :image-size="100" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { Plus } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useMessageStore } from '@/stores/messages'
import MessageItem from './Messages/MessageItem.vue'

const messageStore = useMessageStore()
const { currentMessages } = storeToRefs(messageStore)

const messageContainer = ref<HTMLElement>()

// 消息操作处理函数
const handleDeleteMessage = (messageId: string) => {
  console.log('删除消息:', messageId)
// TODO: 实现消息删除功能
}

const handleCopyMessage = (content: string) => {
  navigator.clipboard.writeText(content).then(() => {
    ElMessage.success('消息已复制到剪贴板')
  }).catch(() => {
    ElMessage.error('复制失败')
  })
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
  max-width: 100%;
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