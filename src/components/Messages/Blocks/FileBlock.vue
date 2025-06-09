<template>
  <div class="file-block">
    <div class="file-container">
      <div class="file-icon">
        <el-icon :size="32">
          <component :is="fileIcon" />
        </el-icon>
      </div>
      
      <div class="file-info">
        <div class="file-name">{{ fileName }}</div>
        <div class="file-meta">
          <span v-if="fileSize" class="file-size">{{ formatFileSize(fileSize) }}</span>
          <span v-if="fileType" class="file-type">{{ fileType.toUpperCase() }}</span>
          <span v-if="uploadTime" class="upload-time">{{ formatTime(uploadTime) }}</span>
        </div>
      </div>
      
      <div class="file-actions">
        <el-button 
          v-if="downloadUrl" 
          size="small" 
          type="primary" 
          @click="downloadFile"
        >
          <el-icon><Download /></el-icon>
          下载
        </el-button>
        <el-button 
          v-if="previewable" 
          size="small" 
          @click="previewFile"
        >
          <el-icon><View /></el-icon>
          预览
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { 
  Document, 
  Picture, 
  VideoPlay, 
  Headset, 
  Files, 
  Download, 
  View 
} from '@element-plus/icons-vue'

interface Props {
  block: any
  filename?: string
  fileSize?: number
  fileType?: string
  downloadUrl?: string
}

const props = defineProps<Props>()

// 文件名
const fileName = computed(() => {
  return props.filename || props.block?.filename || props.block?.name || '未知文件'
})

// 文件大小
const fileSize = computed(() => {
  return props.fileSize || props.block?.fileSize || props.block?.size
})

// 文件类型
const fileType = computed(() => {
  return props.fileType || props.block?.fileType || props.block?.type || getFileTypeFromName(fileName.value)
})

// 下载链接
const downloadUrl = computed(() => {
  return props.downloadUrl || props.block?.downloadUrl || props.block?.url
})

// 上传时间
const uploadTime = computed(() => {
  return props.block?.uploadTime || props.block?.createdAt
})

// 文件图标
const fileIcon = computed(() => {
  const type = fileType.value.toLowerCase()
  
  if (['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'].includes(type)) {
    return Picture
  } else if (['mp4', 'avi', 'mov', 'wmv', 'flv', 'webm'].includes(type)) {
    return VideoPlay
  } else if (['mp3', 'wav', 'flac', 'aac', 'ogg'].includes(type)) {
    return Headset
  } else if (['zip', 'rar', '7z', 'tar', 'gz'].includes(type)) {
    return Files
  } else {
    return Document
  }
})

// 是否可预览
const previewable = computed(() => {
  const type = fileType.value.toLowerCase()
  return ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg', 'pdf', 'txt', 'md'].includes(type)
})

// 从文件名获取文件类型
const getFileTypeFromName = (name: string) => {
  const ext = name.split('.').pop()
  return ext || 'unknown'
}

// 格式化文件大小
const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 格式化时间
const formatTime = (date: Date | string) => {
  const d = new Date(date)
  return d.toLocaleString('zh-CN')
}

// 下载文件
const downloadFile = () => {
  if (!downloadUrl.value) return
  
  const link = document.createElement('a')
  link.href = downloadUrl.value
  link.download = fileName.value
  link.target = '_blank'
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
}

// 预览文件
const previewFile = () => {
  if (!downloadUrl.value) return
  
  // 在新窗口中打开文件
  window.open(downloadUrl.value, '_blank')
}
</script>

<style scoped>
.file-block {
  margin: 12px 0;
}

.file-container {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  border: 1px solid var(--el-border-color-light);
  border-radius: 8px;
  background-color: var(--el-fill-color-lighter);
  transition: all 0.3s ease;
}

.file-container:hover {
  border-color: var(--el-color-primary-light-5);
  background-color: var(--el-color-primary-light-9);
}

.file-icon {
  flex-shrink: 0;
  color: var(--el-color-primary);
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-weight: 600;
  font-size: 14px;
  color: var(--el-text-color-primary);
  margin-bottom: 4px;
  word-break: break-all;
}

.file-meta {
  display: flex;
  gap: 8px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
  flex-wrap: wrap;
}

.file-size {
  font-weight: 500;
}

.file-type {
  background-color: var(--el-color-info-light-9);
  color: var(--el-color-info);
  padding: 2px 6px;
  border-radius: 4px;
  font-weight: 500;
}

.upload-time {
  opacity: 0.8;
}

.file-actions {
  flex-shrink: 0;
  display: flex;
  gap: 8px;
}
</style>
