<template>
  <div class="app-container">

    <!-- 标签页管理器 -->
    <TabManager />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { TabManager } from '../components/Tabs'
import { cleanupEventListeners, setupEventListeners } from '@/api'

// 初始化
onMounted(() => {
  setupEventListeners();
})

onUnmounted(() => {
  cleanupEventListeners();
})
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  background-color: #f5f5f5;
  overflow: hidden;
  position: fixed;
  top: 0;
  left: 0;
}
</style>