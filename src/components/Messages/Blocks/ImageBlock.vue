<template>
  <div class="image-block">
    <div class="image-container">
      <el-image
        :src="imageSrc"
        :alt="imageAlt"
        :style="imageStyle"
        fit="contain"
        :preview-src-list="[imageSrc]"
        :initial-index="0"
        loading="lazy"
        @error="handleImageError"
        @load="handleImageLoad"
      >
        <template #error>
          <div class="image-error">
            <el-icon><Picture /></el-icon>
            <span>图片加载失败</span>
          </div>
        </template>
        <template #placeholder>
          <div class="image-loading">
            <el-icon class="loading-icon"><Loading /></el-icon>
            <span>加载中...</span>
          </div>
        </template>
      </el-image>
    </div>

    <!-- 图片信息 -->
    <div v-if="showInfo" class="image-info">
      <div class="image-meta">
        <span v-if="imageAlt" class="image-alt">{{ imageAlt }}</span>
        <span v-if="imageSize" class="image-size">{{ imageSize }}</span>
        <span v-if="fileSize" class="file-size">{{ formatFileSize(fileSize) }}</span>
      </div>
      
      <div class="image-actions">
        <el-button size="small" text @click="downloadImage">
          <el-icon><Download /></el-icon>
          下载
        </el-button>
        <el-button size="small" text @click="copyImageUrl">
          <el-icon><Link /></el-icon>
          复制链接
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Picture, Loading, Download, Link } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

interface Props {
  block: any
  src?: string
  alt?: string
  width?: number | string
  height?: number | string
  showInfo?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showInfo: true
})

const imageLoaded = ref(false)
const imageError = ref(false)
const naturalWidth = ref(0)
const naturalHeight = ref(0)

// 计算图片源
const imageSrc = computed(() => {
  return props.src || props.block?.src || props.block?.url || props.block?.content
})

// 计算图片描述
const imageAlt = computed(() => {
  return props.alt || props.block?.alt || props.block?.title || '图片'
})

// 计算图片样式
const imageStyle = computed(() => {
  const style: Record<string, any> = {}
  
  if (props.width) {
    style.width = typeof props.width === 'number' ? `${props.width}px` : props.width
  }
  
  if (props.height) {
    style.height = typeof props.height === 'number' ? `${props.height}px` : props.height
  }
  
  // 默认最大宽度
  if (!style.width) {
    style.maxWidth = '100%'
  }
  
  return style
})

// 计算图片尺寸信息
const imageSize = computed(() => {
  if (imageLoaded.value && naturalWidth.value && naturalHeight.value) {
    return `${naturalWidth.value} × ${naturalHeight.value}`
  }
  return null
})

// 文件大小
const fileSize = computed(() => {
  return props.block?.fileSize || props.block?.size
})

// 处理图片加载成功
const handleImageLoad = (event: Event) => {
  const img = event.target as HTMLImageElement
  imageLoaded.value = true
  imageError.value = false
  naturalWidth.value = img.naturalWidth
  naturalHeight.value = img.naturalHeight
}

// 处理图片加载失败
const handleImageError = () => {
  imageLoaded.value = false
  imageError.value = true
}

// 下载图片
const downloadImage = () => {
  if (!imageSrc.value) return
  
  const link = document.createElement('a')
  link.href = imageSrc.value
  link.download = imageAlt.value || 'image'
  link.target = '_blank'
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
}

// 复制图片链接
const copyImageUrl = () => {
  if (!imageSrc.value) return
  
  navigator.clipboard.writeText(imageSrc.value).then(() => {
    ElMessage.success('图片链接已复制到剪贴板')
  }).catch(() => {
    ElMessage.error('复制失败')
  })
}

// 格式化文件大小
const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>

<style scoped>
.image-block {
  margin: 8px 0;
}

.image-container {
  display: inline-block;
  max-width: 100%;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.image-error,
.image-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  background-color: var(--el-fill-color-lighter);
  color: var(--el-text-color-secondary);
  min-height: 120px;
}

.image-error .el-icon,
.image-loading .el-icon {
  font-size: 32px;
  margin-bottom: 8px;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.image-info {
  margin-top: 8px;
  padding: 8px 12px;
  background-color: var(--el-fill-color-lighter);
  border-radius: 6px;
  font-size: 12px;
}

.image-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 8px;
  color: var(--el-text-color-secondary);
}

.image-alt {
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.image-size,
.file-size {
  opacity: 0.8;
}

.image-actions {
  display: flex;
  gap: 8px;
}

/* Element Plus 图片组件样式调整 */
:deep(.el-image) {
  display: block;
  max-width: 100%;
}

:deep(.el-image__inner) {
  max-width: 100%;
  height: auto;
}
</style>
