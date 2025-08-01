<template>
  <div class="plugin-ui">
    <div v-if="!instanceId" class="no-plugin">
      <el-empty description="请选择一个插件实例" :image-size="60" />
    </div>

    <div v-else-if="loading" class="loading">
      <el-skeleton :rows="3" animated />
    </div>

    <div v-else-if="error" class="error">
      <el-alert :title="error" type="error" :closable="false" show-icon />
    </div>

    <div v-else-if="uiComponents.length === 0" class="no-ui">
      <el-empty description="该插件没有UI配置" :image-size="60" />
    </div>

    <div v-else class="ui-components">
      <div v-for="component in uiComponents" :key="component.id" class="ui-component">
        <!-- 标签组件 -->
        <div v-if="component.component.type === 'Label'" class="label-component">
          {{ component.component.text }}
        </div>

        <!-- 按钮组件 -->
        <el-button v-else-if="component.component.type === 'Button'"
          :type="component.component.enabled ? 'primary' : 'info'" :disabled="!component.component.enabled"
          @click="handleButtonClick(component.id)" style="width: 100%; margin-bottom: 10px;">
          {{ component.component.text }}
        </el-button>

        <!-- 文本编辑框组件 -->
        <el-input v-else-if="component.component.type === 'TextEdit'" v-model="textFieldValues[component.id]"
          :placeholder="component.component.hint" @keyup.enter="handleTextFieldSubmit(component.id)"
          @blur="handleTextFieldSubmit(component.id)" style="margin-bottom: 10px;">
        </el-input>

        <!-- 组合框/下拉选择框组件 -->
        <el-select v-else-if="component.component.type === 'ComboBox'" v-model="selectValues[component.id]"
          :placeholder="component.component.placeholder" @change="handleSelectChange(component.id, $event)"
          style="width: 100%; margin-bottom: 10px;">
          <el-option v-for="(option, index) in component.component.options" :key="index" :label="option"
            :value="index" />
        </el-select>

        <!-- 开关组件 -->
        <div v-else-if="component.component.type === 'Toggle'" class="toggle-container">
          <el-switch v-model="toggleValues[component.id]" @change="handleToggleChange(component.id, $event)">
          </el-switch>
        </div>

        <!-- 水平布局容器组件 -->
        <div v-else-if="component.component.type === 'Horizontal'" class="horizontal-container"
          style="display: flex; gap: 10px; margin-bottom: 10px; align-items: flex-start;">
          <div v-for="child in component.component.children" :key="child.id"
            :class="['horizontal-child', child.component.type === 'Label' ? 'horizontal-child-fixed' : 'horizontal-child-flexible']">
            <!-- 递归渲染子组件 -->
            <div class="ui-component">
              <!-- 子组件标签 -->
              <div v-if="child.component.type === 'Label'" class="label-component">
                {{ child.component.text }}
              </div>

              <!-- 子组件按钮 -->
              <el-button v-else-if="child.component.type === 'Button'"
                :type="child.component.enabled ? 'primary' : 'info'" :disabled="!child.component.enabled"
                @click="handleButtonClick(child.id)" style="width: 100%;">
                {{ child.component.text }}
              </el-button>

              <!-- 子组件文本编辑框 -->
              <el-input v-else-if="child.component.type === 'TextEdit'" v-model="textFieldValues[child.id]"
                :placeholder="child.component.hint" @keyup.enter="handleTextFieldSubmit(child.id)"
                @blur="handleTextFieldSubmit(child.id)">
              </el-input>

              <!-- 子组件组合框 -->
              <el-select v-else-if="child.component.type === 'ComboBox'" v-model="selectValues[child.id]"
                :placeholder="child.component.placeholder" @change="handleSelectChange(child.id, $event)"
                style="width: 100%;">
                <el-option v-for="(option, index) in child.component.options" :key="index" :label="option"
                  :value="index" />
              </el-select>

              <!-- 子组件开关 -->
              <div v-else-if="child.component.type === 'Toggle'" class="toggle-container">
                <el-switch v-model="toggleValues[child.id]" @change="handleToggleChange(child.id, $event)">
                </el-switch>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, reactive } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getPluginUi, handlePluginUiEvent, refreshPluginUi } from '@/api/plugin-ui'
import type { Component } from '@/api/types'
import { usePluginStore } from '@/stores/plugins'

// Props
interface Props {
  instanceId?: string
}

const props = defineProps<Props>()

// 响应式数据
const uiComponents = ref<Component[]>([])
const loading = ref(false)
const error = ref<string>('')
const textFieldValues = reactive<Record<string, string>>({})
const selectValues = reactive<Record<string, string | number>>({})
const toggleValues = reactive<Record<string, boolean>>({})

// 插件状态管理
const pluginStore = usePluginStore()

// 事件监听器
let unlistenPluginUiUpdate: UnlistenFn | null = null
let unlistenPluginUiRefresh: UnlistenFn | null = null

// 加载插件UI
const loadPluginUI = async (instanceId: string) => {
  if (!instanceId) {
    uiComponents.value = []
    return
  }

  loading.value = true
  error.value = ''

  try {
    // 首先检查插件实例状态
    const instanceState = pluginStore.getInstanceState(instanceId)
    if (instanceState) {
      // 如果实例状态存在，同步最新状态
      const pluginId = instanceState.pluginId
      const status = await pluginStore.syncInstanceState(instanceId, pluginId)

      if (!status) {
        error.value = '无法获取插件实例状态，请检查插件是否正常运行'
        return
      }

      if (!status.isMounted) {
        error.value = '插件实例未挂载，请先挂载插件实例'
        return
      }

      // 可选：检查连接状态
      if (!status.isConnected) {
        console.warn('插件实例未连接，某些功能可能不可用')
      }
    }

    const ui = await getPluginUi(instanceId)
    uiComponents.value = ui

    // 初始化文本框和下拉选择框的值
    const initializeComponentValues = (components: Component[]) => {
      components.forEach(component => {
        if (component.component.type === 'TextEdit') {
          textFieldValues[component.id] = component.component.value || ''
        } else if (component.component.type === 'ComboBox') {
          selectValues[component.id] = component.component.selected !== null ? component.component.selected : ''
        } else if (component.component.type === 'Toggle') {
          toggleValues[component.id] = component.component.value
        } else if (component.component.type === 'TextField') {
          textFieldValues[component.id] = component.component.value || ''
        } else if (component.component.type === 'Select') {
          selectValues[component.id] = component.component.selected || ''
        } else if (component.component.type === 'Horizontal' && component.component.children) {
          // 递归初始化子组件的值
          initializeComponentValues(component.component.children)
        }
      })
    }

    initializeComponentValues(ui)
  } catch (err) {
    error.value = `加载插件UI失败: ${err}`
    console.error('加载插件UI失败:', err)
  } finally {
    loading.value = false
  }
}

// 处理按钮点击
const handleButtonClick = async (componentId: string) => {
  if (!props.instanceId) return

  try {
    const success = await handlePluginUiEvent(props.instanceId, componentId, '')
    if (!success) {
      console.warn('按钮点击事件处理失败')
    }
  } catch (err) {
    console.error('处理按钮点击事件失败:', err)
  }
}

// 处理文本框提交
const handleTextFieldSubmit = async (componentId: string) => {
  if (!props.instanceId) return

  const value = textFieldValues[componentId] || ''

  try {
    const success = await handlePluginUiEvent(props.instanceId, componentId, value)
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

// 处理下拉选择框变化
const handleSelectChange = async (componentId: string, value: string | number) => {
  if (!props.instanceId) return

  try {
    const success = await handlePluginUiEvent(props.instanceId, componentId, String(value))
    if (success) {
      console.log('下拉选择框变化事件处理成功:', value)
    } else {
      console.warn('下拉选择框变化事件处理失败')
    }
  } catch (err) {
    console.error('处理下拉选择框变化事件失败:', err)
  }
}

// 处理开关变化
const handleToggleChange = async (componentId: string, value: boolean) => {
  if (!props.instanceId) return

  try {
    const success = await handlePluginUiEvent(props.instanceId, componentId, String(value))
    if (success) {
      console.log('开关变化事件处理成功:', value)
    } else {
      console.warn('开关变化事件处理失败')
    }
  } catch (err) {
    console.error('处理开关变化事件失败:', err)
  }
}

// 监听实例ID变化
watch(() => props.instanceId, async (newInstanceId, _) => {

  if (newInstanceId) {
    // 新增：延迟加载，确保实例状态已经同步
    try {
      await loadPluginUI(newInstanceId)
    } catch (err) {
      console.error('加载插件UI失败:', err)
      // 错误状态下清空UI组件
      uiComponents.value = []
      error.value = `加载插件UI失败: ${err}`
    }
  } else {
    // 现有逻辑：清空状态
    uiComponents.value = []
    Object.keys(textFieldValues).forEach(key => {
      delete textFieldValues[key]
    })
    Object.keys(selectValues).forEach(key => {
      delete selectValues[key]
    })
    Object.keys(toggleValues).forEach(key => {
      delete toggleValues[key]
    })
    error.value = ''
  }
}, { immediate: true })

// 组件挂载时设置事件监听
onMounted(async () => {
  // 监听插件UI更新事件
  unlistenPluginUiUpdate = await listen('plugin-ui-updated', (event) => {
    const payload = JSON.parse(event.payload as string)
    if (payload.instance === props.instanceId) {
      loadPluginUI(props.instanceId!)
    }
  })

  // 监听插件UI刷新事件
  unlistenPluginUiRefresh = await listen('plugin-ui-refreshed', (event) => {
    const payload = JSON.parse(event.payload as string)
    if (payload.instance === props.instanceId) {
      refreshPluginUi(props.instanceId!)
    }
  })
})

// 组件卸载时清理事件监听
onUnmounted(() => {
  if (unlistenPluginUiUpdate) {
    unlistenPluginUiUpdate()
  }

  if (unlistenPluginUiRefresh) {
    unlistenPluginUiRefresh()
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
  gap: 4px;
  /* 添加底部间距，确保最后一个组件完全可见 */
  padding-bottom: 20px;
}

.ui-component {
  width: 100%;
}

.label-component {
  padding: 2px 0;
  color: #303133;
  font-size: 14px;
  line-height: 1.4;
  word-wrap: break-word;
  font-weight: 500;
  display: flex;
  align-items: center;
  min-height: 20px;
}

.text-component {
  padding: 6px 12px;
  background-color: #f5f7fa;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  color: #606266;
  font-size: 14px;
  line-height: 1.4;
  word-wrap: break-word;
  min-height: 32px;
  box-sizing: border-box;
  display: flex;
  align-items: center;
}

.toggle-container {
  display: flex;
  align-items: center;
  margin-bottom: 4px;
}

.horizontal-container {
  display: flex;
  gap: 10px;
  align-items: flex-start;
}

.horizontal-child {
  min-width: 0;
}

/* 固定宽度的子组件（如Label） */
.horizontal-child-fixed {
  flex: 0 0 auto;
  white-space: nowrap;
}

/* 可伸缩的子组件（如TextEdit, ComboBox等） */
.horizontal-child-flexible {
  flex: 1;
  min-width: 0;
}
</style>
