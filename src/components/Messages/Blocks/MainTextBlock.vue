<template>
  <div class="main-text-block">
    <!-- 提及的模型 -->
    <div v-if="mentions && mentions.length > 0" class="mentions">
      <span 
        v-for="model in mentions" 
        :key="model.id" 
        class="mention-tag"
      >
        @{{ model.name }}
      </span>
    </div>

    <!-- 文本内容 -->
    <div class="text-content">
      <!-- 用户消息且不渲染 Markdown -->
      <div 
        v-if="role === 'user' && !renderInputMessageAsMarkdown" 
        class="plain-text"
      >
        {{ content }}
      </div>
      
      <!-- Markdown 渲染 -->
      <MarkdownRenderer 
        v-else
        :content="processedContent"
        :isStreaming="isStreaming"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useSettingsStore } from '@/stores/settings'
import MarkdownRenderer from '../Markdown/MarkdownRenderer.vue'

interface Props {
  block?: any
  content?: string
  role?: string
  mentions?: any[]
  citationBlockId?: string
  isStreaming?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  role: 'assistant',
  mentions: () => [],
  isStreaming: false
})

const settingsStore = useSettingsStore()
const { settings } = storeToRefs(settingsStore)

// 是否渲染输入消息为 Markdown
const renderInputMessageAsMarkdown = computed(() => 
  settings.value.renderInputMessageAsMarkdown || false
)

// 获取内容
const content = computed(() => {
  return props.block?.content || props.content || ''
})

// 处理后的内容（包括引用处理等）
const processedContent = computed(() => {
  let text = content.value

  // TODO: 在这里添加引用处理逻辑
  // 类似 Cherry Studio 中的引用处理
  if (props.citationBlockId) {
    // 处理引用
  }

  // TODO: 移除工具使用标签
  const toolUseRegex = /(<tool_use>[\s\S]*?<\/tool_use>)/g
  text = text.replace(toolUseRegex, '')

  return text
})
</script>

<style scoped>
.main-text-block {
  width: 100%;
  color: black;
}

.mentions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-bottom: 8px;
}

.mention-tag {
  color: var(--el-color-primary);
  background-color: var(--el-color-primary-light-9);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.text-content {
  width: 100%;
  word-wrap: break-word;
  line-height: 1.6;
}

.plain-text {
  white-space: pre-wrap;
  word-break: break-word;
  font-family: inherit;
  font-size: inherit;
  line-height: inherit;
}
</style>
