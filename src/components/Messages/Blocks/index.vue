<template>
  <component :is="blockComponent" v-bind="blockProps" :block="block" :message="message" :index="index" />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import MainTextBlock from './MainTextBlock.vue'
import ImageBlock from './ImageBlock.vue'
import CodeBlock from './CodeBlock.vue'
import ErrorBlock from './ErrorBlock.vue'
import FileBlock from './FileBlock.vue'
import ThinkingBlock from './ThinkingBlock.vue'
import ToolBlock from './ToolBlock.vue'
import CitationBlock from './CitationBlock.vue'
import PlaceholderBlock from './PlaceholderBlock.vue'

interface Props {
  block: any
  message: any
  index?: number
}

const props = withDefaults(defineProps<Props>(), {
  index: 0
})

// 块组件映射
const blockComponentMap = {
  'text': MainTextBlock,
  'main_text': MainTextBlock,
  'image': ImageBlock,
  'code': CodeBlock,
  'error': ErrorBlock,
  'file': FileBlock,
  'thinking': ThinkingBlock,
  'tool': ToolBlock,
  'tool_use': ToolBlock,
  'citation': CitationBlock,
  'placeholder': PlaceholderBlock,
  // 默认组件
  'default': MainTextBlock
}

// 计算要使用的组件
const blockComponent = computed(() => {
  const blockType = props.block.type || 'text'
  return blockComponentMap[blockType as keyof typeof blockComponentMap] || blockComponentMap.default
})

// 计算传递给块组件的属性
const blockProps = computed(() => {
  const baseProps = {
    block: props.block,
    message: props.message,
    index: props.index
  }

  // 根据不同的块类型添加特定属性
  switch (props.block.type) {
    case 'main_text':
    case 'text':
      return {
        ...baseProps,
        role: props.message.role,
        mentions: props.message.mentions,
        citationBlockId: props.block.citationBlockId,
        isStreaming: props.block.isStreaming || props.message.isStreaming
      }

    case 'image':
      return {
        ...baseProps,
        src: props.block.src || props.block.url,
        alt: props.block.alt || props.block.title,
        width: props.block.width,
        height: props.block.height
      }

    case 'code':
      return {
        ...baseProps,
        language: props.block.language || 'text',
        filename: props.block.filename,
        showLineNumbers: props.block.showLineNumbers !== false
      }

    case 'error':
      return {
        ...baseProps,
        errorType: props.block.errorType || 'general',
        errorMessage: props.block.errorMessage || props.block.content
      }

    case 'file':
      return {
        ...baseProps,
        filename: props.block.filename,
        fileSize: props.block.fileSize,
        fileType: props.block.fileType,
        downloadUrl: props.block.downloadUrl
      }

    case 'thinking':
      return {
        ...baseProps,
        isCollapsed: props.block.isCollapsed !== false,
        title: props.block.title || '思考过程'
      }

    case 'tool':
    case 'tool_use':
      return {
        ...baseProps,
        toolName: props.block.toolName,
        toolInput: props.block.toolInput,
        toolOutput: props.block.toolOutput,
        status: props.block.status
      }

    case 'citation':
      return {
        ...baseProps,
        citations: props.block.citations || [],
        source: props.block.source
      }

    default:
      return baseProps
  }
})
</script>

<style scoped>
/* 块渲染器的通用样式 */
</style>
