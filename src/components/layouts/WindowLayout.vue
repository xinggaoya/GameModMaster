<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Window } from '@tauri-apps/api/window'
import {
  Moon,
  Sunny,
  RemoveOutline,
  ExpandOutline,
  ContractOutline,
  CloseOutline,
} from '@vicons/ionicons5'
import { NButton, NIcon, NAvatar } from 'naive-ui'
import avatar from '@/assets/logo.png'
import CheckUpdateButton from '../update/CheckUpdateButton.vue'

const props = defineProps({
  isDark: {
    type: Boolean,
    required: true,
  },
})

const emit = defineEmits(['update:isDark'])

const isMaximized = ref(false)
const appWindow = Window.getCurrent()

// 窗口控制函数
const minimize = async () => {
  await appWindow.minimize()
}

const toggleMaximize = async () => {
  isMaximized.value = !isMaximized.value
  await appWindow.toggleMaximize()
}

const close = async () => {
  await appWindow.close()
}

const toggleTheme = () => {
  emit('update:isDark', !props.isDark)
}

// 监听窗口状态变化
appWindow.onResized(async () => {
  isMaximized.value = await appWindow.isMaximized()
})

// 生产环境禁用右键
onMounted(() => {
  if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', (e) => {
      e.preventDefault()
    })
  }
})
</script>

<template>
  <div class="window-container" :class="{ dark: isDark }">
    <div class="window-header">
      <div class="header-left" data-tauri-drag-region>
        <div class="app-icon">
          <n-avatar size="small" :src="avatar" />
        </div>
        <div class="app-title" data-tauri-drag-region>GameMod Master</div>
      </div>
      <div class="header-right">
        <CheckUpdateButton class="update-button" />
        <n-button text circle @click="toggleTheme" class="action-btn">
          <template #icon>
            <n-icon :color="isDark ? '#fff' : '#333'">
              <component :is="isDark ? Moon : Sunny" />
            </n-icon>
          </template>
        </n-button>
        <div class="window-controls">
          <button class="window-control" @click="minimize">
            <n-icon size="20">
              <component :is="RemoveOutline" />
            </n-icon>
          </button>
          <button class="window-control" @click="toggleMaximize">
            <n-icon size="20">
              <component :is="isMaximized ? ContractOutline : ExpandOutline" />
            </n-icon>
          </button>
          <button class="window-control close" @click="close">
            <n-icon size="20">
              <component :is="CloseOutline" />
            </n-icon>
          </button>
        </div>
      </div>
    </div>
    <div class="window-content">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.window-container {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  background-color: #f5f5f5;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  overflow: hidden;
}

.window-container.dark {
  background-color: #242424;
  border-color: #333;
}

.window-header {
  height: 48px;
  padding: 0 8px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid #e0e0e0;
}

.dark .window-header {
  background-color: rgba(36, 36, 36, 0.8);
  border-bottom-color: #333;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  -webkit-app-region: drag;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.app-icon {
  display: flex;
  align-items: center;
  padding: 0 4px;
}

.app-title {
  font-size: 16px;
  font-weight: bold;
  color: #333;
  margin-right: 24px;
  width: 160px;
}

.dark .app-title {
  color: #fff;
}

.window-controls {
  display: flex;
  gap: 4px;
  -webkit-app-region: no-drag;
}

.window-control {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #666;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.dark .window-control {
  color: #999;
}

.window-control:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #333;
}

.dark .window-control:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.window-control.close:hover {
  background: #e81123;
  color: #fff;
}

.action-btn {
  margin-right: 8px;
}

.update-button {
  margin-right: 8px;
}

.window-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
