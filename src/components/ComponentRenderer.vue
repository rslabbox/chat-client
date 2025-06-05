<template>
  <!-- 按钮组件 -->
  <el-button v-if="component.component.type === 'Button'" :type="component.component.enabled ? 'primary' : 'info'"
    :disabled="!component.component.enabled" :icon="getIcon(component.component.icon)"
    @click="$emit('button-click', component.id)" style="width: 100%;">
    {{ component.component.label }}
  </el-button>

  <!-- 文本输入框组件 -->
  <el-input v-else-if="component.component.type === 'TextField'" v-model="props.textFieldValues[component.id]"
    :placeholder="component.component.hint" @keyup.enter="$emit('textfield-submit', component.id)"
    @blur="$emit('textfield-submit', component.id)">
  </el-input>

  <!-- 文本显示组件 -->
  <div v-else-if="component.component.type === 'Text'" class="text-component">
    {{ component.component.value }}
  </div>

  <!-- 下拉选择框组件 -->
  <el-select v-else-if="component.component.type === 'Select'" v-model="props.selectValues[component.id]"
    placeholder="请选择" @change="$emit('select-change', component.id, $event)" style="width: 100%;">
    <el-option v-for="option in component.component.options" :key="option" :label="option" :value="option" />
  </el-select>

  <!-- 下拉框组件 (ComboBox) -->
  <el-select v-else-if="component.component.type === 'ComboBox'" v-model="props.selectValues[component.id]"
    :placeholder="component.component.placeholder" @change="$emit('select-change', component.id, $event)"
    style="width: 100%;">
    <el-option v-for="(option, index) in component.component.options" :key="index" :label="option"
      :value="index.toString()" />
  </el-select>
</template>

<script setup lang="ts">
import { Refresh, Search, Setting } from '@element-plus/icons-vue'
import type { Component } from '@/api/types'

// Props
interface Props {
  component: Component
  pluginId?: string
  textFieldValues: Record<string, string>
  selectValues: Record<string, string>
}

const props = defineProps<Props>()

// Emits
defineEmits<{
  'button-click': [componentId: string]
  'textfield-submit': [componentId: string]
  'select-change': [componentId: string, value: string]
}>()

// 获取图标组件
const getIcon = (iconName?: string) => {
  const iconMap: Record<string, any> = {
    'refresh': Refresh,
    'search': Search,
    'setting': Setting,
  }
  return iconName ? iconMap[iconName] : undefined
}
</script>

<style scoped>
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
</style>
