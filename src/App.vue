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
import { checkForUpdates, initUpdateListener, getAppVersion } from './services/updaterService'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useI18n } from 'vue-i18n'
import { setLocale, type Locale } from './i18n'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()
const store = useTrainerStore()
type ThemeSetting = 'light' | 'dark' | 'system'
const isDark = ref(false)
const appWindow = getCurrentWindow()
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

const syncDomTheme = (resolvedTheme: 'light' | 'dark') => {
  const root = document.documentElement
  const body = document.body
  const isDarkMode = resolvedTheme === 'dark'
  root.classList.toggle('dark-theme', isDarkMode)
  root.classList.toggle('light-theme', !isDarkMode)
  root.classList.toggle('dark', isDarkMode)
  body.classList.toggle('dark', isDarkMode)
  body.classList.toggle('dark-theme', isDarkMode)
  root.dataset.theme = resolvedTheme
}

const resolveTheme = (themeSetting: ThemeSetting) => {
  if (themeSetting === 'system') {
    return mediaQuery.matches ? 'dark' : 'light'
  }
  return themeSetting === 'dark' ? 'dark' : 'light'
}

const theme = computed(() => (isDark.value ? darkTheme : null))

const navItems = computed(() => [
  { path: '/', label: t('nav.explore'), icon: HomeOutline },
  {
    path: '/downloads',
    label: t('nav.downloads'),
    icon: DownloadOutline,
    badge: () => store.downloadedTrainers.length,
  },
  { path: '/settings', label: t('nav.settings'), icon: SettingsOutline },
])

const currentPath = computed(() => route.path)

const navigateTo = (path: string) => {
  router.push(path)
}

const minimizeWindow = () => appWindow.minimize()
const maximizeWindow = () => appWindow.toggleMaximize()
const closeWindow = () => appWindow.close()

const showUpdateDialog = ref(false)
const currentVersion = ref('')

const applyTheme = (themeSetting: ThemeSetting) => {
  localStorage.setItem('theme', themeSetting)
  const resolvedTheme = resolveTheme(themeSetting)
  isDark.value = resolvedTheme === 'dark'
  syncDomTheme(resolvedTheme)
}

mediaQuery.addEventListener('change', (e) => {
  const savedTheme = (localStorage.getItem('theme') as ThemeSetting | null) || 'system'
  if (savedTheme === 'system') {
    isDark.value = e.matches
    syncDomTheme(e.matches ? 'dark' : 'light')
  }
})

onMounted(async () => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const settings = await invoke<{ theme?: string; language?: string }>('get_settings')

    if (settings?.theme) {
      applyTheme((settings.theme as ThemeSetting) || 'system')
    } else {
      const savedTheme = (localStorage.getItem('theme') as ThemeSetting | null) || 'system'
      applyTheme(savedTheme)
    }

    if (settings?.language) {
      setLocale(settings.language as Locale)
    }
  } catch {
    const savedTheme = (localStorage.getItem('theme') as ThemeSetting | null) || 'system'
    applyTheme(savedTheme)
  }

  window.addEventListener('theme-change', ((event: CustomEvent<ThemeSetting>) => {
    applyTheme(event.detail)
  }) as EventListener)

  await store.initialize()

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
    console.error('自动检查更新失败', error)
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
                <header class="app-titlebar" data-tauri-drag-region>
                  <div class="titlebar-left" data-tauri-drag-region>
                    <NIcon size="20" class="app-logo">
                      <GameControllerOutline />
                    </NIcon>
                    <span class="app-title">{{ t('common.appName') }}</span>
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

                <div class="app-body">
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

.app-body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

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

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
