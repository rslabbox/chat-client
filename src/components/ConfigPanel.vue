<template>
  <div class="config-panel">
    <div class="config-header">
      <h3>插件配置</h3>

      <div class="connection-status">
        <el-tag :type="isConnected ? 'success' : 'danger'" size="small">
          {{ isConnected ? '已连接' : '未连接' }}
        </el-tag>
      </div>
    </div>

    <div class="config-content">

      <el-select v-model="value" placeholder="Select" style="width: 100%; margin-bottom: 10px;">
        <el-option v-for="item in pluginLists" :key="item.id" :label="item.name" :value="item.id"
          :disabled="item.disabled" />
      </el-select>
      <!-- 这里可以添加其他配置项 -->
      <el-empty description="配置项待添加" :image-size="80" />
    </div>

    <div class="config-footer">
      <el-button v-if="!isConnected" type="primary" @click="handleConnect" :icon="Link"
        style="width: 100%; margin-bottom: 10px;">
        连接
      </el-button>

      <el-button v-else type="danger" @click="handleDisconnect" :icon="Close" style="width: 100%; margin-bottom: 10px;">
        断开连接
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Link, Close } from '@element-plus/icons-vue'
import { ref, onMounted } from 'vue'
import { scanPlugins, type PluginMetadata } from '@/api'

const value = ref('')
const pluginLists = ref<PluginMetadata[]>([])

// 加载插件列表
const loadPlugins = async () => {
  try {
    const plugins = await scanPlugins()
    pluginLists.value = plugins
  } catch (error) {
    console.error('Failed to load plugins:', error)
    // 如果调用失败，使用空数组
    pluginLists.value = []
  }
}

interface Props {
  isConnected: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  connect: []
  disconnect: []
}>()

const handleConnect = () => {
  emit('connect')
}

const handleDisconnect = () => {
  emit('disconnect')
}

// 组件挂载时加载插件列表
onMounted(() => {
  loadPlugins()
})
</script>

<style scoped>
.config-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px;
}

.config-header {
  margin-bottom: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.config-header h3 {
  margin: 0;
  color: #303133;
  font-size: 16px;
  font-weight: 600;
}

.config-content {
  flex: 1;
  display: block;
  align-items: center;
  justify-content: center;
  margin-top: auto;
}

.config-footer {
  margin-top: auto;
}


</style>