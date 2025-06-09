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
              <div v-if="settings.enableMarkdown" class="streaming-content markdown-content"
                v-html="renderMarkdown(message.content)"></div>
              <span v-else class="streaming-content">{{ message.content }}</span>
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
            <div v-if="settings.enableMarkdown" class="message-text markdown-content"
              v-html="renderMarkdown(message.content)"></div>
            <div v-else class="message-text">{{ message.content }}</div>
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
  Loading
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useMessageStore } from '@/stores/messages'
import { useSettingsStore } from '@/stores/settings'
// @ts-ignore
import MarkdownIt from 'markdown-it'

const messageStore = useMessageStore()
const { currentMessages } = storeToRefs(messageStore)

const settingsStore = useSettingsStore()
const { settings } = storeToRefs(settingsStore)

const messageContainer = ref<HTMLElement>()

const formatTime = (date: Date) => {
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 创建 markdown-it 实例
const md = new MarkdownIt({
  html: true,         // 启用 HTML 标签
  breaks: true,       // 支持换行符转换为 <br>
  linkify: true,      // 自动识别链接
  typographer: true,  // 启用智能引号和其他排版功能
  quotes: '""\'\'',     // 设置引号样式
  xhtmlOut: false,    // 使用 HTML 而不是 XHTML
  langPrefix: 'language-', // 代码块语言前缀
})

// 自定义链接渲染，添加安全属性
// @ts-ignore
const defaultRender = md.renderer.rules.link_open || function (tokens: any, idx: any, options: any, env: any, renderer: any) {
  return renderer.renderToken(tokens, idx, options)
}

// @ts-ignore
md.renderer.rules.link_open = function (tokens: any, idx: any, options: any, env: any, renderer: any) {
  // 添加 target="_blank" 和 rel="noopener noreferrer" 到外部链接
  const aIndex = tokens[idx].attrIndex('target')

  if (aIndex < 0) {
    tokens[idx].attrPush(['target', '_blank'])
  } else {
    tokens[idx].attrs[aIndex][1] = '_blank'
  }

  const relIndex = tokens[idx].attrIndex('rel')
  if (relIndex < 0) {
    tokens[idx].attrPush(['rel', 'noopener noreferrer'])
  } else {
    tokens[idx].attrs[relIndex][1] = 'noopener noreferrer'
  }

  return defaultRender(tokens, idx, options, env, renderer)
}

// Markdown 渲染函数
const renderMarkdown = (content: string): string => {
  if (!content) return ''

  try {
    // 使用 markdown-it 解析 Markdown
    const html = md.render(content)
    // 移除最外层的 <p> 标签（如果只有一个段落）
    const trimmedHtml = html.trim()
    if (trimmedHtml.startsWith('<p>') && trimmedHtml.endsWith('</p>') && trimmedHtml.indexOf('<p>', 1) === -1) {
      return trimmedHtml.slice(3, -4)
    }
    return html
  } catch (error) {
    console.error('Markdown 渲染失败:', error)
    // 如果渲染失败，返回原始文本
    return content.replace(/\n/g, '<br>')
  }
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

/* 非 Markdown 内容保留换行符和空格 */
.message-text:not(.markdown-content) {
  white-space: pre-wrap;
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

/* 流式消息的 Markdown 内容样式调整 */
.streaming-content.markdown-content {
  display: inline-block;
  vertical-align: top;
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
/* Markdown 内容样式 */
.markdown-content {
  line-height: 1.6;
  white-space: normal;
  /* 重置 white-space */
  word-break: break-word;
}

/* 确保 Markdown 内容中的第一个和最后一个元素没有多余的边距 */
.markdown-content>*:first-child {
  margin-top: 0;
}

.markdown-content>*:last-child {
  margin-bottom: 0;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5,
.markdown-content h6 {
  margin: 0.6em 0 0.3em 0;
  font-weight: 600;
  line-height: 1.3;
}

.markdown-content h1 {
  font-size: 1.4em;
  border-bottom: 2px solid #eee;
  padding-bottom: 0.2em;
}

.markdown-content h2 {
  font-size: 1.3em;
  border-bottom: 1px solid #eee;
  padding-bottom: 0.15em;
}

.markdown-content h3 {
  font-size: 1.15em;
}

.markdown-content h4 {
  font-size: 1.05em;
}

.markdown-content h5,
.markdown-content h6 {
  font-size: 1em;
}

.markdown-content p {
  margin: 0.3em 0;
}

.markdown-content ul,
.markdown-content ol {
  margin: 0.3em 0;
  padding-left: 1.2em;
}

.markdown-content li {
  margin: 0.1em 0;
}

.markdown-content blockquote {
  margin: 0.4em 0;
  padding: 0.4em 0.8em;
  border-left: 3px solid #ddd;
  background-color: rgba(0, 0, 0, 0.02);
  font-style: italic;
}

.markdown-content code {
  background-color: rgba(0, 0, 0, 0.1);
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 0.9em;
}

.markdown-content pre {
  background-color: rgba(0, 0, 0, 0.05);
  padding: 0.8em;
  border-radius: 4px;
  overflow-x: auto;
  margin: 0.5em 0;
  border: 1px solid rgba(0, 0, 0, 0.1);
}

.markdown-content pre code {
  background-color: transparent;
  padding: 0;
  border-radius: 0;
  font-size: 0.85em;
}

.markdown-content strong {
  font-weight: 600;
}

.markdown-content em {
  font-style: italic;
}

.markdown-content a {
  color: #409eff;
  text-decoration: none;
}

.markdown-content a:hover {
  text-decoration: underline;
}

.markdown-content table {
  border-collapse: collapse;
  margin: 0.8em 0;
  width: 100%;
}

.markdown-content th,
.markdown-content td {
  border: 1px solid #ddd;
  padding: 0.5em;
  text-align: left;
}

.markdown-content th {
  background-color: rgba(0, 0, 0, 0.05);
  font-weight: 600;
}

.markdown-content hr {
  border: none;
  border-top: 2px solid #eee;
  margin: 1em 0;
}

/* 发送消息中的 Markdown 样式调整 */
.sent .markdown-content {
  color: white;
}

.sent .markdown-content h1,
.sent .markdown-content h2 {
  border-bottom-color: rgba(255, 255, 255, 0.3);
}

.sent .markdown-content code {
  background-color: rgba(255, 255, 255, 0.2);
  color: white;
}

.sent .markdown-content pre {
  background-color: rgba(255, 255, 255, 0.1);
  border-color: rgba(255, 255, 255, 0.2);
}

.sent .markdown-content blockquote {
  border-left-color: rgba(255, 255, 255, 0.4);
  background-color: rgba(255, 255, 255, 0.1);
}

.sent .markdown-content a {
  color: #ffffff;
  text-decoration: underline;
}

.sent .markdown-content th,
.sent .markdown-content td {
  border-color: rgba(255, 255, 255, 0.3);
}

.sent .markdown-content th {
  background-color: rgba(255, 255, 255, 0.1);
}

.sent .markdown-content hr {
  border-top-color: rgba(255, 255, 255, 0.3);
}
</style>