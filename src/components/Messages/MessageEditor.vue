<template>
  <div class="message-editor">
    <div class="editor-header">
      <h4>编辑消息</h4>
      <div class="editor-actions">
        <el-button size="small" @click="handleCancel">
          取消
        </el-button>
        <el-button size="small" type="primary" @click="handleSave">
          保存
        </el-button>
        <el-button 
          v-if="message.role === 'user'" 
          size="small" 
          type="success" 
          @click="handleResend"
        >
          保存并重新发送
        </el-button>
      </div>
    </div>

    <div class="editor-content">
      <!-- 文本编辑器 -->
      <el-input
        v-model="editedContent"
        type="textarea"
        :rows="Math.max(3, Math.min(15, contentLines))"
        placeholder="请输入消息内容..."
        resize="vertical"
        class="content-editor"
      />

      <!-- 预览区域 -->
      <div v-if="showPreview" class="preview-area">
        <div class="preview-header">
          <span>预览</span>
          <el-button 
            size="small" 
            text 
            @click="showPreview = false"
          >
            隐藏预览
          </el-button>
        </div>
        <div class="preview-content">
          <MarkdownRenderer :content="editedContent" />
        </div>
      </div>
    </div>

    <div class="editor-footer">
      <div class="editor-options">
        <el-checkbox v-model="showPreview">
          显示预览
        </el-checkbox>
        <el-checkbox v-model="enableMarkdown">
          启用 Markdown
        </el-checkbox>
      </div>
      
      <div class="editor-stats">
        <span class="char-count">
          {{ editedContent.length }} 字符
        </span>
        <span class="line-count">
          {{ contentLines }} 行
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import MarkdownRenderer from './Markdown/MarkdownRenderer.vue'

interface Props {
  message: any
}

interface Emits {
  (e: 'save', content: string): void
  (e: 'cancel'): void
  (e: 'resend', content: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const editedContent = ref('')
const showPreview = ref(false)
const enableMarkdown = ref(true)

// 计算内容行数
const contentLines = computed(() => {
  return editedContent.value.split('\n').length
})

// 初始化编辑内容
onMounted(() => {
  editedContent.value = props.message.content || ''
})

// 处理保存
const handleSave = () => {
  emit('save', editedContent.value)
}

// 处理取消
const handleCancel = () => {
  emit('cancel')
}

// 处理重新发送
const handleResend = () => {
  emit('resend', editedContent.value)
}
</script>

<style scoped>
.message-editor {
  border: 1px solid var(--el-border-color);
  border-radius: 8px;
  background-color: var(--el-bg-color);
  overflow: hidden;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background-color: var(--el-fill-color-lighter);
  border-bottom: 1px solid var(--el-border-color-light);
}

.editor-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.editor-actions {
  display: flex;
  gap: 8px;
}

.editor-content {
  padding: 16px;
}

.content-editor {
  margin-bottom: 16px;
}

.preview-area {
  border: 1px solid var(--el-border-color-light);
  border-radius: 6px;
  overflow: hidden;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: var(--el-fill-color-lighter);
  border-bottom: 1px solid var(--el-border-color-light);
  font-size: 12px;
  font-weight: 500;
  color: var(--el-text-color-regular);
}

.preview-content {
  padding: 12px;
  min-height: 60px;
  background-color: var(--el-bg-color);
}

.editor-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background-color: var(--el-fill-color-lighter);
  border-top: 1px solid var(--el-border-color-light);
}

.editor-options {
  display: flex;
  gap: 16px;
}

.editor-stats {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.char-count,
.line-count {
  opacity: 0.8;
}

/* 文本域样式调整 */
:deep(.el-textarea__inner) {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 14px;
  line-height: 1.5;
}
</style>
