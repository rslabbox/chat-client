import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { useSettingsStore } from './stores/settings'

const app = createApp(App)
app.use(ElementPlus)

const pinia = createPinia()
app.use(pinia)
app.use(router)

app.mount('#app')

// 初始化设置
const settingsStore = useSettingsStore()
settingsStore.initializeSettings()
