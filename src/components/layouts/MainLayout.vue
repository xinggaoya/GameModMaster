<script setup lang="ts">
import { ref, computed, h, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NButton,
  NSpace,
  NIcon,
  NAvatar,
  NTabs,
  NTab,
  NBadge,
  NDivider,
  NTooltip,
  NDropdown,
  NMenu,
  NSwitch,
} from 'naive-ui'
import {
  HomeOutline,
  DownloadOutline,
  InformationCircleOutline,
  MenuOutline,
  SunnyOutline,
  MoonOutline,
  GameControllerOutline,
  SettingsOutline,
  NotificationsOutline,
  LogOutOutline,
  PersonOutline,
  SearchOutline,
  ChevronForwardOutline,
  BriefcaseOutline,
} from '@vicons/ionicons5'

const router = useRouter()
const route = useRoute()

// 侧边栏折叠状态
const collapsed = ref(false)
const isDark = ref(false) // 默认关闭暗黑模式

// 标签页配置
const tabs = [
  {
    name: '/',
    label: '发现',
    icon: HomeOutline,
    tooltip: '浏览和发现游戏修改器',
  },
  {
    name: '/downloads',
    label: '我的收藏',
    icon: DownloadOutline,
    tooltip: '管理已下载的修改器',
  },
  {
    name: '/games',
    label: '游戏库',
    icon: GameControllerOutline,
    tooltip: '按游戏浏览修改器',
  },
  {
    name: '/about',
    label: '关于',
    icon: InformationCircleOutline,
    tooltip: '关于软件',
  },
]

// 用户菜单选项
const userMenuOptions = [
  {
    label: '个人资料',
    key: 'profile',
    icon: () => h(NIcon, null, { default: () => h(PersonOutline) }),
  },
  {
    label: '设置',
    key: 'settings',
    icon: () => h(NIcon, null, { default: () => h(SettingsOutline) }),
  },
  {
    type: 'divider',
    key: 'd1',
  },
  {
    label: '退出登录',
    key: 'logout',
    icon: () => h(NIcon, null, { default: () => h(LogOutOutline) }),
  },
]

// 通知菜单选项
const notificationMenuOptions = [
  {
    label: '新版本可用 (v1.2.0)',
    key: 'update',
    icon: () => h(NIcon, null, { default: () => h(DownloadOutline) }),
  },
  {
    label: '欢迎使用游戏修改器管理中心',
    key: 'welcome',
    icon: () => h(NIcon, null, { default: () => h(HomeOutline) }),
  },
]

// 当前激活的标签页
const activeTab = computed(() => route.path)

// 处理标签页切换
const handleTabChange = (name: string) => {
  router.push(name)
}

// 切换主题
const toggleTheme = () => {
  isDark.value = !isDark.value
  // 保存主题设置到本地存储
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
  // 更新文档根元素的类名
  if (isDark.value) {
    document.documentElement.classList.add('dark-theme')
  } else {
    document.documentElement.classList.remove('dark-theme')
  }
}

// 获取页面标题
const pageTitle = computed(() => {
  const currentTab = tabs.find((tab) => tab.name === route.path)
  return currentTab?.label || '游戏修改器中心'
})

// 处理用户菜单
const handleUserMenuSelect = (key: string) => {
  if (key === 'settings') {
    router.push('/settings')
  } else if (key === 'profile') {
    router.push('/profile')
  } else if (key === 'logout') {
    // 处理退出登录
    console.log('退出登录')
  }
}

// 假设有3个新通知
const notificationCount = ref(3)

// 在组件挂载时初始化主题
onMounted(() => {
  const savedTheme = localStorage.getItem('theme')
  if (savedTheme) {
    isDark.value = savedTheme === 'dark'
    if (isDark.value) {
      document.documentElement.classList.add('dark-theme')
    }
  }
})
</script>

<template>
  <NLayout class="main-layout" has-sider position="absolute">
    <!-- 侧边栏 -->
    <NLayoutSider
      :collapsed="collapsed"
      :collapsed-width="70"
      :width="240"
      show-trigger
      collapse-mode="width"
      @collapse="collapsed = true"
      @expand="collapsed = false"
      :native-scrollbar="false"
      class="main-sider animate-slide-in"
      bordered
    >
      <!-- Logo -->
      <div class="logo-container">
        <router-link to="/" class="logo-link">
          <img src="/src/assets/logo.png" alt="Logo" class="logo" v-if="!collapsed" />
          <img src="/src/assets/logo-small.png" alt="Logo" class="logo-small" v-else />
          <h1 class="logo-text game-title" v-if="!collapsed">GameMod</h1>
        </router-link>
      </div>

      <!-- 导航菜单 -->
      <div class="nav-container">
        <div v-for="tab in tabs" :key="tab.name" class="nav-item">
          <NTooltip v-if="collapsed" placement="right" trigger="hover">
            <template #trigger>
              <NButton
                :quaternary="activeTab !== tab.name"
                :secondary="activeTab === tab.name"
                circle
                size="large"
                class="nav-button"
                @click="handleTabChange(tab.name)"
              >
                <NIcon :class="{ active: activeTab === tab.name }">
                  <component :is="tab.icon" />
                </NIcon>
              </NButton>
            </template>
            {{ tab.tooltip }}
          </NTooltip>

          <NButton
            v-else
            :quaternary="activeTab !== tab.name"
            :secondary="activeTab === tab.name"
            class="nav-button full-width"
            @click="handleTabChange(tab.name)"
          >
            <template #icon>
              <NIcon :class="{ active: activeTab === tab.name }">
                <component :is="tab.icon" />
              </NIcon>
            </template>
            {{ tab.label }}
          </NButton>
        </div>
      </div>

      <!-- 底部操作区 -->
      <div class="sider-footer">
        <NDivider v-if="!collapsed" />
        <div class="footer-actions">
          <NTooltip v-if="collapsed" placement="right" trigger="hover">
            <template #trigger>
              <NButton circle quaternary size="large" @click="toggleTheme">
                <NIcon>
                  <MoonOutline v-if="isDark" />
                  <SunnyOutline v-else />
                </NIcon>
              </NButton>
            </template>
            {{ isDark ? '切换至亮色模式' : '切换至暗色模式' }}
          </NTooltip>

          <div v-else class="theme-switch">
            <NIcon><SunnyOutline /></NIcon>
            <NSwitch size="medium" v-model:value="isDark" @update:value="toggleTheme" />
            <NIcon><MoonOutline /></NIcon>
          </div>
        </div>
      </div>
    </NLayoutSider>

    <!-- 主内容区 -->
    <NLayout class="content-wrapper">
      <!-- 顶部栏 -->
      <NLayoutHeader bordered class="main-header animate-fade">
        <div class="header-left">
          <NButton quaternary circle class="collapse-button" @click="collapsed = !collapsed">
            <NIcon><MenuOutline /></NIcon>
          </NButton>

          <div class="search-container">
            <NButton quaternary circle>
              <NIcon><SearchOutline /></NIcon>
            </NButton>
            <span class="search-placeholder">搜索游戏或修改器...</span>
          </div>
        </div>

        <div class="page-title-container">
          <h2 class="page-title">{{ pageTitle }}</h2>
        </div>

        <div class="header-right">
          <!-- 通知菜单 -->
          <NDropdown :options="notificationMenuOptions" placement="bottom-end" trigger="click">
            <NBadge :value="3" :max="99" :show-zero="false" dot>
              <NButton quaternary circle>
                <NIcon><NotificationsOutline /></NIcon>
              </NButton>
            </NBadge>
          </NDropdown>

          <!-- 主题切换 - 在侧边栏折叠时显示 -->
          <NButton v-if="collapsed" quaternary circle @click="toggleTheme">
            <NIcon>
              <MoonOutline v-if="isDark" />
              <SunnyOutline v-else />
            </NIcon>
          </NButton>

          <!-- 用户菜单 -->
          <NDropdown
            :options="userMenuOptions"
            placement="bottom-end"
            trigger="click"
            @select="handleUserMenuSelect"
          >
            <NButton text class="user-button">
              <NAvatar
                round
                size="small"
                src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
                class="user-avatar"
              />
              <span class="username" v-if="!collapsed">用户名</span>
              <NIcon class="dropdown-icon"><ChevronForwardOutline /></NIcon>
            </NButton>
          </NDropdown>
        </div>
      </NLayoutHeader>

      <!-- 内容区 -->
      <NLayoutContent class="main-content animate-fade">
        <router-view v-slot="{ Component }">
          <transition name="fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </NLayoutContent>
    </NLayout>
  </NLayout>
</template>

<style scoped>
.main-layout {
  height: 100vh;
  background-color: var(--body-color);
}

.main-sider {
  background: var(--card-color);
  border-right: 1px solid var(--divider-color);
  box-shadow: var(--shadow-sm);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  z-index: 100;
}

.logo-container {
  height: 70px;
  padding: 0 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid var(--divider-color);
}

.logo-link {
  display: flex;
  align-items: center;
  gap: 12px;
  text-decoration: none;
  width: 100%;
  justify-content: center;
}

.logo {
  height: 32px;
  object-fit: contain;
}

.logo-small {
  height: 36px;
  width: 36px;
  object-fit: contain;
}

.logo-text {
  font-size: 1.2rem;
  margin: 0;
  font-weight: 700;
}

.nav-container {
  flex: 1;
  overflow-y: auto;
  padding: 24px 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-item {
  width: 100%;
}

.nav-button {
  font-size: 1rem;
  transition: all 0.2s ease;
  height: 46px;
}

.nav-button.full-width {
  width: 100%;
  justify-content: flex-start;
  padding: 0 12px;
  border-radius: var(--radius-lg);
}

.nav-button:hover {
  background-color: var(--hover-color);
  transform: translateY(-2px);
}

.nav-button :deep(.n-icon) {
  font-size: 1.2rem;
  margin-right: 12px;
  color: var(--text-tertiary);
}

.nav-button :deep(.n-icon.active) {
  color: var(--primary);
}

.sider-footer {
  padding: 16px;
  margin-top: auto;
}

.footer-actions {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 12px 0;
}

.theme-switch {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-tertiary);
}

.content-wrapper {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-header {
  height: 70px;
  padding: 0 24px;
  background: var(--card-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: var(--shadow-sm);
  z-index: 90;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.collapse-button:hover {
  background-color: var(--hover-color);
}

.search-container {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 12px;
  border-radius: var(--radius-full);
  background-color: var(--hover-color);
  height: 40px;
  transition: all 0.2s ease;
  cursor: pointer;
}

.search-container:hover {
  background-color: var(--gray-200);
}

.search-placeholder {
  color: var(--text-tertiary);
  font-size: 0.9rem;
}

.page-title-container {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
}

.page-title {
  margin: 0;
  font-size: 1.2rem;
  font-weight: 600;
  color: var(--text-primary);
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.user-button {
  display: flex;
  align-items: center;
  gap: 8px;
  height: 40px;
  padding: 0 8px;
  border-radius: var(--radius-full);
  transition: all 0.2s ease;
  background-color: var(--hover-color);
}

.user-button:hover {
  background-color: var(--gray-200);
  transform: translateY(-2px);
}

.user-avatar {
  box-shadow: var(--shadow-sm);
}

.username {
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.dropdown-icon {
  font-size: 0.8rem;
  color: var(--text-tertiary);
  transition: transform 0.2s ease;
}

.user-button:hover .dropdown-icon {
  transform: rotate(90deg);
}

.main-content {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
  background-color: var(--body-color);
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

/* 响应式设计 */
@media (max-width: 768px) {
  .main-sider {
    position: fixed;
    height: 100vh;
    z-index: 1000;
  }

  .page-title-container {
    position: static;
    transform: none;
  }

  .page-title {
    font-size: 1rem;
  }

  .main-content {
    padding: 16px;
  }

  .search-container {
    display: none;
  }

  .username {
    display: none;
  }
}
</style>
