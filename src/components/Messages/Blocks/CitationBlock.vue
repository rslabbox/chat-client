<template>
  <div class="citation-block">
    <div class="citation-header">
      <el-icon class="citation-icon">
        <Link />
      </el-icon>
      <span class="citation-title">引用来源</span>
      <el-tag size="small" type="info">{{ citations.length }} 个来源</el-tag>
    </div>

    <div class="citation-list">
      <div v-for="(citation, index) in citations" :key="index" class="citation-item">
        <div class="citation-number">{{ index + 1 }}</div>

        <div class="citation-content">
          <div class="citation-meta">
            <a v-if="citation.url" :href="citation.url" target="_blank" rel="noopener noreferrer" class="citation-link">
              {{ citation.title || citation.hostname || citation.url }}
            </a>
            <span v-else class="citation-title-text">
              {{ citation.title || '未知来源' }}
            </span>

            <span v-if="citation.hostname" class="citation-hostname">
              {{ citation.hostname }}
            </span>
          </div>

          <div v-if="citation.content" class="citation-excerpt">
            {{ citation.content }}
          </div>

          <div class="citation-actions">
            <el-button v-if="citation.url" size="small" text @click="openLink(citation.url)">
              <el-icon>
                <DocumentCopy />
              </el-icon>
              访问
            </el-button>
            <el-button size="small" text @click="copyCitation(citation)">
              <el-icon>
                <DocumentCopy />
              </el-icon>
              复制
            </el-button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Link, DocumentCopy } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

interface Citation {
  title?: string
  url?: string
  hostname?: string
  content?: string
  number?: number
}

interface Props {
  block: any
  citations?: Citation[]
  source?: string
}

const props = defineProps<Props>()

// 引用列表
const citations = computed(() => {
  return props.citations || props.block?.citations || []
})

// 打开链接
const openLink = (url: string) => {
  window.open(url, '_blank', 'noopener,noreferrer')
}

// 复制引用
const copyCitation = (citation: Citation) => {
  let text = ''

  if (citation.title) {
    text += citation.title
  }

  if (citation.url) {
    text += text ? `\n${citation.url}` : citation.url
  }

  if (citation.content) {
    text += text ? `\n\n${citation.content}` : citation.content
  }

  navigator.clipboard.writeText(text).then(() => {
    ElMessage.success('引用内容已复制到剪贴板')
  }).catch(() => {
    ElMessage.error('复制失败')
  })
}
</script>

<style scoped>
.citation-block {
  margin: 12px 0;
  border: 1px solid var(--el-border-color-light);
  border-radius: 8px;
  overflow: hidden;
  background-color: var(--el-fill-color-lighter);
}

.citation-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background-color: var(--el-fill-color-light);
  border-bottom: 1px solid var(--el-border-color-light);
}

.citation-icon {
  color: var(--el-color-primary);
}

.citation-title {
  flex: 1;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.citation-list {
  padding: 16px;
}

.citation-item {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
  padding: 12px;
  background-color: var(--el-bg-color);
  border: 1px solid var(--el-border-color-light);
  border-radius: 6px;
}

.citation-item:last-child {
  margin-bottom: 0;
}

.citation-number {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--el-color-primary);
  color: white;
  border-radius: 50%;
  font-size: 12px;
  font-weight: 600;
}

.citation-content {
  flex: 1;
  min-width: 0;
}

.citation-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  flex-wrap: wrap;
}

.citation-link {
  color: var(--el-color-primary);
  text-decoration: none;
  font-weight: 500;
  word-break: break-all;
}

.citation-link:hover {
  text-decoration: underline;
}

.citation-title-text {
  font-weight: 500;
  color: var(--el-text-color-primary);
  word-break: break-all;
}

.citation-hostname {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  background-color: var(--el-fill-color-light);
  padding: 2px 6px;
  border-radius: 4px;
}

.citation-excerpt {
  font-size: 14px;
  line-height: 1.5;
  color: var(--el-text-color-regular);
  margin-bottom: 8px;
  word-break: break-word;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.citation-actions {
  display: flex;
  gap: 8px;
}
</style>
