<script setup lang="ts">
import { h, ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { darkTheme, NConfigProvider } from 'naive-ui'
import { Moon, Sunny, Home, Download } from '@vicons/ionicons5'
import type { Component } from 'vue'
import themeOverrides from '@/assets/naive-ui-theme-overrides.json'

const router = useRouter()
const isDark = ref(true)
const activeKey = ref('home')

// 根据isDark的值返回主题
const theme = computed(() => (isDark.value ? darkTheme : null))

const menuOptions = [
  {
    label: '首页',
    key: 'home',
    icon: renderIcon(Home),
    onClick: () => router.push('/'),
  },
  {
    label: '下载管理',
    key: 'downloads',
    icon: renderIcon(Download),
    onClick: () => router.push('/downloads'),
  },
]

function renderIcon(icon: Component) {
  return () => h('div', { class: 'menu-icon' }, h(icon))
}

function toggleTheme() {
  isDark.value = !isDark.value
  // 保存主题设置到本地存储
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
}

// 在组件挂载时读取本地存储的主题设置
onMounted(() => {
  const savedTheme = localStorage.getItem('theme')
  if (savedTheme) {
    isDark.value = savedTheme === 'dark'
  }
})
</script>

<template>
  <n-config-provider :theme="theme" :theme-overrides="themeOverrides">
    <n-global-style />
    <n-message-provider>
      <n-notification-provider>
        <n-loading-bar-provider>
          <n-dialog-provider>
            <div class="app-container">
              <header class="app-header">
                <div class="logo">
                  <n-h2>游戏修改器管理器</n-h2>
                </div>
                <n-menu v-model:value="activeKey" mode="horizontal" :options="menuOptions" />
                <div class="actions">
                  <n-button circle @click="toggleTheme">
                    <template #icon>
                      <n-icon>
                        <Moon v-if="isDark" />
                        <Sunny v-else />
                      </n-icon>
                    </template>
                  </n-button>
                </div>
              </header>

              <main class="app-content">
                <router-view v-slot="{ Component }">
                  <transition name="fade" mode="out-in">
                    <component :is="Component" />
                  </transition>
                </router-view>
              </main>
            </div>
          </n-dialog-provider>
        </n-loading-bar-provider>
      </n-notification-provider>
    </n-message-provider>
  </n-config-provider>
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

/* 应用容器 */
.app-container {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  background-color: var(--body-color);
}

/* 头部样式 */
.app-header {
  height: 64px;
  padding: 0 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--divider-color);
  background-color: var(--card-color);
}

.logo {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.actions {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-shrink: 0;
}

/* 内容区域 */
.app-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  background-color: var(--body-color);
}

.menu-icon {
  display: flex;
  align-items: center;
  margin-right: 6px;
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
</style>
