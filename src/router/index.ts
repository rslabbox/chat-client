import { createRouter, createWebHistory } from 'vue-router'
import HomeView from "../views/HomeView.vue"
import PluginUITest from "../views/PluginUITest.vue"

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/test',
      name: 'test',
      component: PluginUITest,
    },
  ],
})

export default router
