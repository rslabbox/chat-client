<template>
  <div class="markdown-renderer" :class="{ 'streaming': isStreaming }">
    <!-- 流式内容 -->
    <div v-if="isStreaming" class="streaming-content">
      <div class="markdown-content" v-html="renderedContent"></div>
      <span class="streaming-cursor">|</span>
    </div>

    <!-- 普通内容 -->
    <div v-else class="markdown-content" v-html="renderedContent"></div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
// @ts-ignore
import MarkdownIt from 'markdown-it'
// @ts-ignore
import markdownItTexmath from 'markdown-it-texmath'
import hljs from 'highlight.js'
import 'katex/dist/katex.min.css'
import 'highlight.js/styles/github.css'

interface Props {
  content: string
  isStreaming?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isStreaming: false
})

// 创建 markdown-it 实例
const md = new MarkdownIt({
  html: true,         // 启用 HTML 标签
  breaks: true,       // 支持换行符转换为 <br>
  linkify: true,      // 自动识别链接
  typographer: true,  // 启用智能引号和其他排版功能
  quotes: '""\'\'',     // 设置引号样式
  xhtmlOut: false,    // 使用 HTML 而不是 XHTML
  langPrefix: 'language-', // 代码块语言前缀
  highlight: function (str: string, lang: string) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value
      } catch (__) { }
    }
    return '' // 使用额外的默认转义
  }
})

// 添加 KaTeX 数学公式支持（使用 texmath 插件）
md.use(markdownItTexmath, {
  engine: 'katex',
  delimiters: 'dollars',
  katexOptions: {
    throwOnError: false,
    errorColor: '#cc0000',
    strict: false,
    trust: true,
    fleqn: false,
    macros: {
      "\\RR": "\\mathbb{R}",
      "\\NN": "\\mathbb{N}",
      "\\ZZ": "\\mathbb{Z}",
      "\\QQ": "\\mathbb{Q}",
      "\\CC": "\\mathbb{C}"
    }
  }
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

// 复制代码到剪贴板
const copyCode = async (code: string) => {
  try {
    await navigator.clipboard.writeText(code)
    ElMessage.success('代码已复制到剪贴板')
  } catch (error) {
    console.error('复制失败:', error)
    ElMessage.error('复制失败')
  }
}

// 为代码块添加复制按钮
const addCopyButtons = (html: string) => {
  return html.replace(
    /<pre><code class="language-([^"]*)">([\s\S]*?)<\/code><\/pre>/g,
    (_match, lang, code) => {
      // 解码 HTML 实体
      const decodedCode = code
        .replace(/&lt;/g, '<')
        .replace(/&gt;/g, '>')
        .replace(/&amp;/g, '&')
        .replace(/&quot;/g, '"')
        .replace(/&#39;/g, "'")

      const copyButtonId = `copy-btn-${Math.random().toString(36).substr(2, 9)}`

      return `
        <div class="code-block-container">
          <div class="code-block-header">
            <span class="code-language">${lang || 'text'}</span>
            <button
              class="copy-button"
              id="${copyButtonId}"
              onclick="window.copyCodeToClipboard('${encodeURIComponent(decodedCode)}')"
              title="复制代码"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
              </svg>
            </button>
          </div>
          <pre><code class="language-${lang}">${code}</code></pre>
        </div>
      `
    }
  )
}

// 预处理数学公式，处理 KaTeX 不完全支持的环境
const preprocessMath = (content: string) => {
  // 首先处理 \[ \] 分隔符，将其转换为 $$ $$ 格式
  content = content.replace(
    /\\?\\\[\s*([\s\S]*?)\s*\\?\\\]/g,
    (_match, mathContent) => {
      return `$$${mathContent.trim()}$$`
    }
  )

  // 处理 align* 环境，转换为 aligned 环境
  content = content.replace(
    /\$\$\s*\\begin\{align\*\}([\s\S]*?)\\end\{align\*\}\s*\$\$/g,
    (_match, equations) => {
      // 移除多余的换行符和空格
      const cleanEquations = equations.trim()
      return `$$\\begin{aligned}${cleanEquations}\\end{aligned}$$`
    }
  )

  // 处理 align 环境（带编号），转换为 aligned 环境
  content = content.replace(
    /\$\$\s*\\begin\{align\}([\s\S]*?)\\end\{align\}\s*\$\$/g,
    (_match, equations) => {
      const cleanEquations = equations.trim()
      return `$$\\begin{aligned}${cleanEquations}\\end{aligned}$$`
    }
  )

  // 处理方括号包围的 align* 环境（已经被上面的 \[ \] 处理转换了）
  content = content.replace(
    /\$\$\s*\\begin\{align\*\}([\s\S]*?)\\end\{align\*\}\s*\$\$/g,
    (_match, equations) => {
      const cleanEquations = equations.trim()
      return `$$\\begin{aligned}${cleanEquations}\\end{aligned}$$`
    }
  )

  // 处理方括号包围的 align 环境（已经被上面的 \[ \] 处理转换了）
  content = content.replace(
    /\$\$\s*\\begin\{align\}([\s\S]*?)\\end\{align\}\s*\$\$/g,
    (_match, equations) => {
      const cleanEquations = equations.trim()
      return `$$\\begin{aligned}${cleanEquations}\\end{aligned}$$`
    }
  )

  return content
}

// 渲染 Markdown 内容
const renderedContent = computed(() => {
  if (!props.content) return ''

  try {
    // 预处理数学公式
    let processedContent = preprocessMath(props.content)

    // 使用 markdown-it 解析 Markdown
    let html = md.render(processedContent)

    // 为代码块添加复制按钮
    html = addCopyButtons(html)

    // 移除最外层的 <p> 标签（如果只有一个段落）
    const trimmedHtml = html.trim()
    if (trimmedHtml.startsWith('<p>') && trimmedHtml.endsWith('</p>') && trimmedHtml.indexOf('<p>', 1) === -1) {
      return trimmedHtml.slice(3, -4)
    }

    return html
  } catch (error) {
    console.error('Markdown 渲染失败:', error)
    // 如果渲染失败，返回原始文本
    return props.content.replace(/\n/g, '<br>')
  }
})

// 在组件挂载时设置全局复制函数
onMounted(() => {
  // 设置全局复制函数
  ; (window as any).copyCodeToClipboard = async (encodedCode: string) => {
    try {
      const code = decodeURIComponent(encodedCode)
      await copyCode(code)
    } catch (error) {
      console.error('复制代码失败:', error)
    }
  }
})
</script>

<style scoped>
.markdown-renderer {
  width: 100%;
  line-height: 1.6;
  overflow: visible;
}

.streaming-content {
  display: flex;
  align-items: flex-end;
}

.streaming-cursor {
  display: inline-block;
  animation: blink 1s infinite;
  color: var(--el-color-primary);
  font-weight: bold;
  margin-left: 2px;
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

/* Markdown 内容样式 */
.markdown-content {
  width: 100%;
  line-height: 1.6;
  white-space: normal;
  word-break: break-word;
  overflow: visible;
  padding: 0;
  margin: 0;
}

/* 确保 Markdown 内容中的第一个和最后一个元素没有多余的边距 */
.markdown-content :deep(> *:first-child) {
  margin-top: 0 !important;
}

.markdown-content :deep(> *:last-child) {
  margin-bottom: 0 !important;
}

.markdown-content :deep(h1),
.markdown-content :deep(h2),
.markdown-content :deep(h3),
.markdown-content :deep(h4),
.markdown-content :deep(h5),
.markdown-content :deep(h6) {
  margin: 0.6em 0 0.3em 0;
  font-weight: 600;
  line-height: 1.3;
}

.markdown-content :deep(h1) {
  font-size: 1.4em;
  border-bottom: 2px solid var(--el-border-color-lighter);
  padding-bottom: 0.2em;
}

.markdown-content :deep(h2) {
  font-size: 1.3em;
  border-bottom: 1px solid var(--el-border-color-lighter);
  padding-bottom: 0.15em;
}

.markdown-content :deep(h3) {
  font-size: 1.15em;
}

.markdown-content :deep(h4) {
  font-size: 1.05em;
}

.markdown-content :deep(h5),
.markdown-content :deep(h6) {
  font-size: 1em;
}

.markdown-content :deep(p) {
  margin: 0.3em 0;
}

.markdown-content :deep(p:first-child) {
  margin-top: 0;
}

.markdown-content :deep(ul),
.markdown-content :deep(ol) {
  margin: 0.3em 0;
  padding-left: 1.2em;
}

.markdown-content :deep(li) {
  margin: 0.1em 0;
}

.markdown-content :deep(blockquote) {
  margin: 0.4em 0;
  padding: 0.4em 0.8em;
  border-left: 3px solid var(--el-border-color);
  background-color: var(--el-fill-color-lighter);
  font-style: italic;
}

.markdown-content :deep(code) {
  background-color: var(--el-fill-color-light);
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  font-size: 0.9em;
}

/* 代码块容器样式 */
.markdown-content :deep(.code-block-container) {
  margin: 0.8em 0;
  border: 1px solid var(--el-border-color-light);
  border-radius: 6px;
  overflow: hidden;
  background-color: var(--el-fill-color-lighter);
}

.markdown-content :deep(.code-block-header) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5em 0.8em;
  background-color: var(--el-fill-color-light);
  border-bottom: 1px solid var(--el-border-color-light);
  font-size: 0.85em;
}

.markdown-content :deep(.code-language) {
  color: var(--el-text-color-regular);
  font-weight: 500;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
}

.markdown-content :deep(.copy-button) {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.25em 0.5em;
  background-color: transparent;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  color: var(--el-text-color-regular);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.8em;
}

.markdown-content :deep(.copy-button:hover) {
  background-color: var(--el-color-primary-light-9);
  border-color: var(--el-color-primary);
  color: var(--el-color-primary);
}

.markdown-content :deep(.copy-button:active) {
  transform: scale(0.95);
}

.markdown-content :deep(.copy-button svg) {
  width: 14px;
  height: 14px;
}

.markdown-content :deep(.code-block-container pre) {
  background-color: transparent;
  padding: 0.8em;
  border-radius: 0;
  overflow-x: auto;
  margin: 0;
  border: none;
}

.markdown-content :deep(.code-block-container pre code) {
  background-color: transparent;
  padding: 0;
  border-radius: 0;
  font-size: 0.85em;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
}

/* 普通代码块样式（没有语言标识的） */
.markdown-content :deep(pre) {
  background-color: var(--el-fill-color-lighter);
  padding: 0.8em;
  border-radius: 4px;
  overflow-x: auto;
  margin: 0.5em 0;
  border: 1px solid var(--el-border-color-light);
}

.markdown-content :deep(pre code) {
  background-color: transparent;
  padding: 0;
  border-radius: 0;
  font-size: 0.85em;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
}

.markdown-content :deep(strong) {
  font-weight: 600;
}

.markdown-content :deep(em) {
  font-style: italic;
}

.markdown-content :deep(a) {
  color: var(--el-color-primary);
  text-decoration: none;
}

.markdown-content :deep(a:hover) {
  text-decoration: underline;
}

.markdown-content :deep(table) {
  border-collapse: collapse;
  margin: 0.8em 0;
  width: 100%;
}

.markdown-content :deep(th),
.markdown-content :deep(td) {
  border: 1px solid var(--el-border-color);
  padding: 0.5em;
  text-align: left;
}

.markdown-content :deep(th) {
  background-color: var(--el-fill-color-lighter);
  font-weight: 600;
}

.markdown-content :deep(hr) {
  border: none;
  border-top: 2px solid var(--el-border-color-lighter);
  margin: 1em 0;
}

/* KaTeX 数学公式样式 */
.markdown-content :deep(.katex) {
  font-size: 1em;
}

.markdown-content :deep(.katex-display) {
  margin: 1em 0;
  text-align: center;
}

.markdown-content :deep(.katex-display > .katex) {
  display: inline-block;
  white-space: nowrap;
}

/* 确保数学公式在不同主题下正常显示 */
.markdown-content :deep(.katex .base) {
  color: var(--el-text-color-primary);
}
</style>
