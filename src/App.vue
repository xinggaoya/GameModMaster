<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { darkTheme, NConfigProvider, NIcon, NScrollbar, NBadge, NTooltip } from 'naive-ui'
import {
  HomeOutline,
  DownloadOutline,
  SettingsOutline,
  GameControllerOutline,
  CloseOutline,
  RemoveOutline,
  SquareOutline,
} from '@vicons/ionicons5'
import themeOverrides from '@/assets/naive-ui-theme-overrides.json'
import UpdateDialog from './components/update/UpdateDialog.vue'
import { useTrainerStore } from './stores/trainer'
import {
  checkForUpdates,
  initUpdateListener,
  getAppVersion,
} from './services/updaterService'
import { getCurrentWindow } from '@tauri-apps/api/window'

const router = useRouter()
const route = useRoute()
const store = useTrainerStore()
const isDark = ref(false)
const appWindow = getCurrentWindow()

// 根据isDark的值返回主题
const theme = computed(() => (isDark.value ? darkTheme : null))

// 导航项
const navItems = [
  { path: '/', label: '探索', icon: HomeOutline },
  { path: '/downloads', label: '收藏', icon: DownloadOutline, badge: () => store.downloadedTrainers.length },
  { path: '/settings', label: '设置', icon: SettingsOutline },
]

// 当前激活的路由
const currentPath = computed(() => route.path)

// 导航到指定路由
const navigateTo = (path: string) => {
  router.push(path)
}

// 窗口控制
const minimizeWindow = () => appWindow.minimize()
const maximizeWindow = () => appWindow.toggleMaximize()
const closeWindow = () => appWindow.close()

// 更新对话框
const showUpdateDialog = ref(false)
const currentVersion = ref('')

onMounted(async () => {
  // 加载主题设置
  const savedTheme = localStorage.getItem('theme')
  if (savedTheme) {
    isDark.value = savedTheme === 'dark'
  }

  // 初始化 store
  await store.initialize()

  // 初始化更新检查
  try {
    await initUpdateListener()
    currentVersion.value = await getAppVersion()

    setTimeout(async () => {
      const result = await checkForUpdates()
      if (result?.has_update) {
        showUpdateDialog.value = true
      }
    }, 5000)
  } catch (error) {
    console.error('自动检查更新失败:', error)
  }
})
</script>

<template>
  <n-config-provider :theme="theme" :theme-overrides="themeOverrides">
    <n-global-style />
    <n-message-provider>
      <n-modal-provider>
        <n-notification-provider>
          <n-loading-bar-provider>
            <n-dialog-provider>
              <div class="app-container" :class="{ dark: isDark }">
                <!-- 自定义标题栏 -->
                <header class="app-titlebar" data-tauri-drag-region>
                  <div class="titlebar-left" data-tauri-drag-region>
                    <NIcon size="20" class="app-logo">
                      <GameControllerOutline />
                    </NIcon>
                    <span class="app-title">GameMod Master</span>
                  </div>
                  <div class="titlebar-controls">
                    <button class="control-btn" @click="minimizeWindow">
                      <NIcon size="14"><RemoveOutline /></NIcon>
                    </button>
                    <button class="control-btn" @click="maximizeWindow">
                      <NIcon size="12"><SquareOutline /></NIcon>
                    </button>
                    <button class="control-btn close-btn" @click="closeWindow">
                      <NIcon size="14"><CloseOutline /></NIcon>
                    </button>
                  </div>
                </header>

                <!-- 主体区域 -->
                <div class="app-body">
                  <!-- 侧边导航栏 -->
                  <aside class="app-sidebar">
                    <nav class="sidebar-nav">
                      <NTooltip
                        v-for="item in navItems"
                        :key="item.path"
                        placement="right"
                        :show-arrow="false"
                      >
                        <template #trigger>
                          <button
                            class="nav-item"
                            :class="{ active: currentPath === item.path || (item.path !== '/' && currentPath.startsWith(item.path)) }"
                            @click="navigateTo(item.path)"
                          >
                            <NBadge
                              v-if="item.badge && item.badge() > 0"
                              :value="item.badge()"
                              :max="99"
                              type="info"
                            >
                              <NIcon size="22"><component :is="item.icon" /></NIcon>
                            </NBadge>
                            <NIcon v-else size="22"><component :is="item.icon" /></NIcon>
                          </button>
                        </template>
                        {{ item.label }}
                      </NTooltip>
                    </nav>
                  </aside>

                  <!-- 主内容区域 -->
                  <main class="app-main">
                    <n-scrollbar class="main-scrollbar">
                      <div class="main-content">
                        <router-view v-slot="{ Component }">
                          <transition name="fade" mode="out-in">
                            <component :is="Component" />
                          </transition>
                        </router-view>
                      </div>
                    </n-scrollbar>
                  </main>
                </div>
              </div>
            </n-dialog-provider>
          </n-loading-bar-provider>
        </n-notification-provider>
      </n-modal-provider>
    </n-message-provider>
  </n-config-provider>

  <!-- 更新对话框 -->
  <UpdateDialog v-model:show="showUpdateDialog" :current-version="currentVersion" />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
</style>

<style scoped>
.app-container {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
  overflow: hidden;
}

.app-container.dark {
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
}

/* 标题栏 */
.app-titlebar {
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  user-select: none;
  -webkit-user-select: none;
  flex-shrink: 0;
}

.dark .app-titlebar {
  background: rgba(15, 23, 42, 0.9);
  border-bottom-color: rgba(255, 255, 255, 0.06);
}

.titlebar-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.app-logo {
  color: #7c3aed;
}

.app-title {
  font-size: 13px;
  font-weight: 600;
  color: #1f2937;
  letter-spacing: -0.02em;
}

.dark .app-title {
  color: #e2e8f0;
}

.titlebar-controls {
  display: flex;
  gap: 4px;
}

.control-btn {
  width: 32px;
  height: 26px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: 6px;
  color: #64748b;
  cursor: pointer;
  transition: all 0.15s ease;
}

.control-btn:hover {
  background: rgba(0, 0, 0, 0.08);
  color: #1f2937;
}

.dark .control-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #e2e8f0;
}

.close-btn:hover {
  background: #ef4444 !important;
  color: white !important;
}

/* 主体区域 */
.app-body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* 侧边栏 */
.app-sidebar {
  width: 64px;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-right: 1px solid rgba(0, 0, 0, 0.06);
  display: flex;
  flex-direction: column;
  padding: 16px 0;
  flex-shrink: 0;
}

.dark .app-sidebar {
  background: rgba(15, 23, 42, 0.6);
  border-right-color: rgba(255, 255, 255, 0.06);
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.nav-item {
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: 14px;
  color: #64748b;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
}

.nav-item:hover {
  background: rgba(124, 58, 237, 0.1);
  color: #7c3aed;
}

.nav-item.active {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.3);
}

.dark .nav-item {
  color: #94a3b8;
}

.dark .nav-item:hover {
  color: #a78bfa;
}

/* 主内容区域 */
.app-main {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.main-scrollbar {
  height: 100%;
}

.main-content {
  padding: 24px;
  min-height: 100%;
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
