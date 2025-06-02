<template>
  <div class="plugin-ui">
    <div v-if="!pluginId" class="no-plugin">
      <el-empty description="请选择一个插件" :image-size="60" />
    </div>
    
    <div v-else-if="loading" class="loading">
      <el-skeleton :rows="3" animated />
    </div>
    
    <div v-else-if="error" class="error">
      <el-alert
        :title="error"
        type="error"
        :closable="false"
        show-icon
      />
    </div>
    
    <div v-else-if="uiComponents.length === 0" class="no-ui">
      <el-empty description="该插件没有UI配置" :image-size="60" />
    </div>
    
    <div v-else class="ui-components">
      <div
        v-for="component in uiComponents"
        :key="component.id"
        class="ui-component"
      >
        <!-- 按钮组件 -->
        <el-button
          v-if="component.component.type === 'Button'"
          :type="component.component.enabled ? 'primary' : 'info'"
          :disabled="!component.component.enabled"
          :icon="getIcon(component.component.icon)"
          @click="handleButtonClick(component.id)"
          style="width: 100%; margin-bottom: 10px;"
        >
          {{ component.component.label }}
        </el-button>
        
        <!-- 文本输入框组件 -->
        <el-input
          v-else-if="component.component.type === 'TextField'"
          v-model="textFieldValues[component.id]"
          :placeholder="component.component.hint"
          @keyup.enter="handleTextFieldSubmit(component.id)"
          @blur="handleTextFieldSubmit(component.id)"
          style="margin-bottom: 10px;"
        >
        </el-input>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, reactive } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { Refresh, Search, Setting } from '@element-plus/icons-vue'
import { getPluginUi, handlePluginUiEvent } from '@/api/plugin-ui'
import type { Component } from '@/api/types'

// Props
interface Props {
  pluginId?: string
}

const props = defineProps<Props>()

// 响应式数据
const uiComponents = ref<Component[]>([])
const loading = ref(false)
const error = ref<string>('')
const textFieldValues = reactive<Record<string, string>>({})

// 事件监听器
let unlistenPluginUiUpdate: UnlistenFn | null = null

// 获取图标组件
const getIcon = (iconName?: string) => {
  const iconMap: Record<string, any> = {
    'refresh': Refresh,
    'search': Search,
    'setting': Setting,
  }
  return iconName ? iconMap[iconName] : undefined
}

// 加载插件UI
const loadPluginUI = async (pluginId: string) => {
  if (!pluginId) {
    uiComponents.value = []
    return
  }

  loading.value = true
  error.value = ''
  
  try {
    const ui = await getPluginUi(pluginId)
    console.log(ui)
    uiComponents.value = ui
    
    // 初始化文本框的值
    ui.forEach(component => {
      if (component.component.type === 'TextField') {
        textFieldValues[component.id] = component.component.value || ''
      }
    })
    
    console.log('加载插件UI成功:', ui)
  } catch (err) {
    error.value = `加载插件UI失败: ${err}`
    console.error('加载插件UI失败:', err)
  } finally {
    loading.value = false
  }
}

// 处理按钮点击
const handleButtonClick = async (componentId: string) => {
  if (!props.pluginId) return

  try {
    const success = await handlePluginUiEvent(props.pluginId, componentId, '')
    if (success) {
      console.log('按钮点击事件处理成功')
      // 不立即重新加载UI，等待插件发送更新事件
      // UI更新将通过 plugin-ui-updated 事件触发
    } else {
      console.warn('按钮点击事件处理失败')
    }
  } catch (err) {
    console.error('处理按钮点击事件失败:', err)
  }
}

// 处理文本框提交
const handleTextFieldSubmit = async (componentId: string) => {
  if (!props.pluginId) return
  
  const value = textFieldValues[componentId] || ''
  
  try {
    const success = await handlePluginUiEvent(props.pluginId, componentId, value)
    if (success) {
      console.log('文本框提交事件处理成功:', value)
      // 可以选择清空文本框或保持原值
      // textFieldValues[componentId] = ''
    } else {
      console.warn('文本框提交事件处理失败')
    }
  } catch (err) {
    console.error('处理文本框提交事件失败:', err)
  }
}

// 监听插件ID变化
watch(() => props.pluginId, (newPluginId) => {
  if (newPluginId) {
    loadPluginUI(newPluginId)
  } else {
    uiComponents.value = []
    Object.keys(textFieldValues).forEach(key => {
      delete textFieldValues[key]
    })
  }
}, { immediate: true })

// 组件挂载时设置事件监听
onMounted(async () => {
  // 监听插件UI更新事件
  unlistenPluginUiUpdate = await listen('plugin-ui-updated', (event) => {
    const payload = event.payload as { plugin: string }
    if (payload.plugin === props.pluginId) {
      console.log('收到插件UI更新事件，重新加载UI')
      loadPluginUI(props.pluginId!)
    }
  })
})

// 组件卸载时清理事件监听
onUnmounted(() => {
  if (unlistenPluginUiUpdate) {
    unlistenPluginUiUpdate()
  }
})
</script>

<style scoped>
.plugin-ui {
  width: 100%;
}

.no-plugin,
.loading,
.error,
.no-ui {
  padding: 20px 0;
}

.ui-components {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.ui-component {
  width: 100%;
}
</style>
