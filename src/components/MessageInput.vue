<template>
  <div class="message-input">
    <div class="input-header">
      <h3>发送消息</h3>
    </div>
    
    <div class="input-content">
      <el-input
        v-model="inputText"
        type="textarea"
        :rows="6"
        placeholder="请输入消息内容..."
        resize="none"
        @keydown.ctrl.enter="handleSend"
      />
    </div>
    
    <div class="input-footer">
      <div class="input-tips">
        <el-text size="small" type="info">
          Ctrl + Enter 快速发送
        </el-text>
      </div>
      
      <div class="button-group">
        <el-button 
          @click="handleClear"
          :icon="Delete"
          plain
        >
          清空
        </el-button>
        
        <el-button 
          type="primary" 
          @click="handleSend"
          :disabled="!inputText.trim()"
          :icon="Promotion"
        >
          发送
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Delete, Promotion } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const emit = defineEmits<{
  send: [content: string]
  clear: []
}>()

const inputText = ref('')

const handleSend = () => {
  const content = inputText.value.trim()
  if (!content) {
    ElMessage.warning('请输入消息内容')
    return
  }
  
  emit('send', content)
  inputText.value = ''
  ElMessage.success('消息发送成功')
}

const handleClear = () => {
  inputText.value = ''
  emit('clear')
}
</script>

<style scoped>
.message-input {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px;
}

.input-header {
  margin-bottom: 15px;
  border-bottom: 1px solid #e4e7ed;
  padding-bottom: 10px;
}

.input-header h3 {
  margin: 0;
  color: #303133;
  font-size: 16px;
  font-weight: 600;
}

.input-content {
  flex: 1;
  margin-bottom: 15px;
}

.input-content :deep(.el-textarea__inner) {
  resize: none !important;
  height: 100% !important;
  min-height: 120px !important;
}

.input-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.input-tips {
  flex: 1;
}

.button-group {
  display: flex;
  gap: 10px;
}
</style>