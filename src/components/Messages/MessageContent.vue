<template>
  <div class="message-content">
    <!-- 消息提及的模型 -->
    <div v-if="message.mentions && message.mentions.length > 0" class="mentions">
      <span v-for="model in message.mentions" :key="model.id" class="mention-tag">
        @{{ model.name }}
      </span>
    </div>

    <!-- 渲染消息块 -->
    <MessageBlockRenderer v-for="(block, index) in messageBlocks" :key="`${message.id}-${index}`" :block="block"
      :message="message" :index="index" />

    <!-- 如果没有消息块，显示原始内容 -->
    <div v-if="messageBlocks.length === 0" class="fallback-content">
      <MainTextBlock :content="message.content" :role="message.role" :isStreaming="message.isStreaming" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import MessageBlockRenderer from './Blocks/index.vue'
import MainTextBlock from './Blocks/MainTextBlock.vue'

interface Props {
  message: any
}

const props = defineProps<Props>()

// 计算消息块
const messageBlocks = computed(() => {
  // 如果消息有 blocks 属性，使用它
  if (props.message.blocks && Array.isArray(props.message.blocks)) {
    return props.message.blocks
  }

  // 否则，根据消息内容创建默认的文本块
  if (props.message.content || props.message.isStreaming) {
    return [{
      type: 'text',
      content: props.message.content || '',
      id: `${props.message.id}-text-0`,
      isStreaming: props.message.isStreaming
    }]
  }

  return []
})
</script>

<style scoped>
.message-content {
  width: 100%;
  word-wrap: break-word;
  line-height: 1.6;
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

.fallback-content {
  /* 备用内容样式 */
}
</style>
