<script setup lang="ts">
import { ref, computed, h, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { darkTheme, NConfigProvider, NTabs, NTab, NIcon, NScrollbar } from 'naive-ui'
import { Home, Download, InformationCircle } from '@vicons/ionicons5'
import type { Component } from 'vue'
import themeOverrides from '@/assets/naive-ui-theme-overrides.json'
import WindowLayout from '@/components/layouts/WindowLayout.vue'
import UpdateDialog from './components/update/UpdateDialog.vue'
import {
  checkForUpdates,
  hasUpdate,
  updateInfo,
  initUpdateListener,
  getAppVersion,
} from './services/updaterService'

const router = useRouter()
const route = useRoute()
const isDark = ref(false)

// 根据isDark的值返回主题
const theme = computed(() => (isDark.value ? darkTheme : null))

// 标签页配置
const tabs = [
  {
    name: '/',
    label: '首页',
    icon: Home,
  },
  {
    name: '/downloads',
    label: '下载管理',
    icon: Download,
  },
]

// 当前激活的标签页
const activeTab = computed(() => route.path)

// 处理标签页切换
const handleTabChange = (name: string) => {
  router.push(name)
}

// 用于更新对话框
const showUpdateDialog = ref(false)
const currentVersion = ref('')

onMounted(() => {
  const savedTheme = localStorage.getItem('theme')
  if (savedTheme) {
    isDark.value = savedTheme === 'dark'
  }

  // 初始化更新检查
  initUpdateCheck()
})

// 初始化更新检查的函数
const initUpdateCheck = async () => {
  try {
    // 初始化更新监听器
    await initUpdateListener()

    // 获取当前版本
    currentVersion.value = await getAppVersion()

    // 自动检查更新（延迟5秒，避免影响应用启动）
    setTimeout(async () => {
      console.log('自动检查更新...')
      const result = await checkForUpdates()
      if (result?.has_update) {
        showUpdateDialog.value = true
      }
    }, 5000)
  } catch (error) {
    console.error('自动检查更新失败:', error)
  }
}
</script>

<template>
  <n-config-provider :theme="theme" :theme-overrides="themeOverrides">
    <n-global-style />
    <n-message-provider>
      <n-modal-provider>
        <n-notification-provider>
          <n-loading-bar-provider>
            <n-dialog-provider>
              <window-layout v-model:isDark="isDark">
                <n-card class="app-main" content-style="padding: 0;">
                  <NTabs
                    v-model:value="activeTab"
                    type="line"
                    animated
                    class="nav-tabs"
                    @update:value="handleTabChange"
                  >
                    <NTab
                      v-for="tab in tabs"
                      :key="tab.name"
                      :name="tab.name"
                      :tab="
                        () =>
                          h('div', { class: 'tab-content' }, [
                            h(NIcon, null, { default: () => h(tab.icon) }),
                            h('span', { class: 'tab-label' }, tab.label),
                          ])
                      "
                    />
                  </NTabs>
                  <div style="height: calc(100vh - 87px)">
                    <n-scrollbar class="app-content" trigger="none">
                      <router-view v-slot="{ Component }">
                        <transition name="fade" mode="out-in">
                          <component :is="Component" />
                        </transition>
                      </router-view>
                    </n-scrollbar>
                  </div>
                </n-card>
              </window-layout>
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
/* 重置基础样式 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

/* 应用主内容 */
.app-main {
  height: 100%;
  display: flex;
  flex-direction: column;
}

/* 导航标签页 */
.nav-tabs {
  padding: 0 16px;
  border-bottom: 1px solid var(--divider-color);
  background-color: var(--card-color);
  flex-shrink: 0;
}

.tab-content {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 0;
  line-height: 1;
}

.tab-content :deep(.n-icon) {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: -2px;
}

.tab-label {
  font-size: 14px;
  line-height: 1;
}

/* 内容区域 */
.app-content {
  flex: 1;
  padding: 20px;
  background-color: var(--body-color);
}

.app-content :deep(.n-scrollbar-container) {
  padding-right: 16px;
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

<style scoped>
:deep(.n-config-provider),
:deep(.n-message-provider),
:deep(.n-notification-provider),
:deep(.n-loading-bar-provider),
:deep(.n-dialog-provider) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

:deep(.n-card) {
  height: 100%;
}

:deep(.n-card-content) {
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style>
