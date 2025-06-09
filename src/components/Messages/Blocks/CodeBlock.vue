<template>
  <div class="code-block">
    <!-- 代码块头部 -->
    <div class="code-header">
      <div class="code-info">
        <span v-if="language" class="language-tag">{{ language }}</span>
        <span v-if="filename" class="filename">{{ filename }}</span>
      </div>

      <div class="code-actions">
        <el-tooltip content="复制代码" placement="top">
          <el-button size="small" type="primary" text @click="copyCode" class="copy-button">
            <el-icon>
              <DocumentCopy />
            </el-icon>
            复制
          </el-button>
        </el-tooltip>

        <el-tooltip content="下载代码" placement="top">
          <el-button size="small" text @click="downloadCode">
            <el-icon>
              <Download />
            </el-icon>
          </el-button>
        </el-tooltip>
      </div>
    </div>

    <!-- 代码内容 -->
    <div class="code-content">
      <pre :class="['code-pre', `language-${language}`]" ref="codeRef"><code 
        :class="['code-element', `language-${language}`]"
        v-html="highlightedCode"
      ></code></pre>

      <!-- 行号 -->
      <div v-if="showLineNumbers" class="line-numbers">
        <span v-for="lineNumber in totalLines" :key="lineNumber" class="line-number">
          {{ lineNumber }}
        </span>
      </div>
    </div>

    <!-- 代码统计 -->
    <div v-if="showStats" class="code-stats">
      <span class="lines-count">{{ totalLines }} 行</span>
      <span class="chars-count">{{ codeContent.length }} 字符</span>
      <span v-if="language" class="language-info">{{ getLanguageName(language) }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { DocumentCopy, Download } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'

interface Props {
  block: any
  language?: string
  filename?: string
  showLineNumbers?: boolean
  showStats?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  language: 'text',
  showLineNumbers: true,
  showStats: true
})

const codeRef = ref<HTMLElement>()

// 获取代码内容
const codeContent = computed(() => {
  return props.block?.content || props.block?.code || ''
})

// 获取语言
const language = computed(() => {
  return props.language || props.block?.language || 'text'
})

// 获取文件名
const filename = computed(() => {
  return props.filename || props.block?.filename
})

// 计算总行数
const totalLines = computed(() => {
  return codeContent.value.split('\n').length
})

// 使用 highlight.js 进行代码高亮
const highlightedCode = computed(() => {
  const code = codeContent.value
  const lang = language.value

  if (!code) return ''

  try {
    // 如果指定了语言且 highlight.js 支持该语言
    if (lang && hljs.getLanguage(lang)) {
      const result = hljs.highlight(code, { language: lang })
      return result.value
    } else {
      // 自动检测语言
      const result = hljs.highlightAuto(code)
      return result.value
    }
  } catch (error) {
    console.error('代码高亮失败:', error)
    // 如果高亮失败，返回转义后的原始代码
    return code
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;')
  }
})



// 复制代码
const copyCode = () => {
  navigator.clipboard.writeText(codeContent.value).then(() => {
    ElMessage.success('代码已复制到剪贴板')
  }).catch(() => {
    ElMessage.error('复制失败')
  })
}

// 下载代码
const downloadCode = () => {
  const blob = new Blob([codeContent.value], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')

  link.href = url
  link.download = filename.value || `code.${getFileExtension(language.value)}`
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  URL.revokeObjectURL(url)
}

// 获取文件扩展名
const getFileExtension = (lang: string) => {
  const extensions: Record<string, string> = {
    'javascript': 'js',
    'typescript': 'ts',
    'python': 'py',
    'java': 'java',
    'cpp': 'cpp',
    'c': 'c',
    'html': 'html',
    'css': 'css',
    'json': 'json',
    'xml': 'xml',
    'yaml': 'yml',
    'markdown': 'md'
  }
  return extensions[lang] || 'txt'
}

// 获取语言显示名称
const getLanguageName = (lang: string) => {
  const names: Record<string, string> = {
    'javascript': 'JavaScript',
    'typescript': 'TypeScript',
    'python': 'Python',
    'java': 'Java',
    'cpp': 'C++',
    'c': 'C',
    'html': 'HTML',
    'css': 'CSS',
    'json': 'JSON',
    'xml': 'XML',
    'yaml': 'YAML',
    'markdown': 'Markdown'
  }
  return names[lang] || lang.toUpperCase()
}
</script>

<style scoped>
.code-block {
  margin: 12px 0;
  border: 1px solid var(--el-border-color-light);
  border-radius: 8px;
  overflow: hidden;
  background-color: var(--el-fill-color-lighter);
}

.code-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: var(--el-fill-color-light);
  border-bottom: 1px solid var(--el-border-color-light);
}

.code-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.language-tag {
  background-color: var(--el-color-primary-light-9);
  color: var(--el-color-primary);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.filename {
  font-size: 12px;
  color: var(--el-text-color-regular);
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
}

.code-actions {
  display: flex;
  gap: 4px;
}

.copy-button {
  font-weight: 500;
}

.copy-button:hover {
  background-color: var(--el-color-primary-light-9);
}

.code-content {
  position: relative;
  overflow-x: auto;
}

.code-pre {
  margin: 0;
  padding: 16px;
  background-color: var(--el-bg-color);
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 14px;
  line-height: 1.5;
  overflow-x: auto;
}

.code-element {
  display: block;
  white-space: pre;
  word-wrap: normal;
}

.line-numbers {
  position: absolute;
  top: 0;
  left: 0;
  padding: 16px 8px;
  background-color: var(--el-fill-color-light);
  border-right: 1px solid var(--el-border-color-light);
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 14px;
  line-height: 1.5;
  color: var(--el-text-color-secondary);
  user-select: none;
  text-align: right;
  min-width: 40px;
}

.line-number {
  display: block;
  opacity: 0.6;
}

.code-stats {
  display: flex;
  gap: 12px;
  padding: 8px 12px;
  background-color: var(--el-fill-color-light);
  border-top: 1px solid var(--el-border-color-light);
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

/* highlight.js 会提供自己的语法高亮样式 */

/* 有行号时调整代码内容位置 */
.code-content:has(.line-numbers) .code-pre {
  padding-left: 60px;
}
</style>
