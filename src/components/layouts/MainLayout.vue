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
  NBadge,
  NDivider,
  NTooltip,
  NDropdown,
  NSwitch,
  NInput,
  NGrid,
  NGridItem,
  NCard,
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
  SparklesOutline,
  GridOutline,
  HeartOutline,
  TrophyOutline,
} from '@vicons/ionicons5'

const router = useRouter()
const route = useRoute()

// 状态管理
const collapsed = ref(false)
const isDark = ref(false)
const searchQuery = ref('')
const isSearchFocused = ref(false)

// 导航配置 - 采用更现代的分类方式
const navigationItems = [
  {
    title: '探索',
    items: [
      {
        name: '/',
        label: '发现',
        icon: SparklesOutline,
        tooltip: '探索热门修改器',
        color: '#7c3aed',
      },
      {
        name: '/games',
        label: '游戏库',
        icon: GameControllerOutline,
        tooltip: '按游戏分类浏览',
        color: '#0891b2',
      },
    ],
  },
  {
    title: '管理',
    items: [
      {
        name: '/downloads',
        label: '我的收藏',
        icon: HeartOutline,
        tooltip: '管理已下载内容',
        color: '#dc2626',
      },
      {
        name: '/achievements',
        label: '成就',
        icon: TrophyOutline,
        tooltip: '查看使用成就',
        color: '#ea580c',
      },
    ],
  },
  {
    title: '系统',
    items: [
      {
        name: '/about',
        label: '关于',
        icon: InformationCircleOutline,
        tooltip: '应用信息',
        color: '#6b7280',
      },
    ],
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
    label: '新版本可用',
    key: 'update',
    icon: () => h(NIcon, null, { default: () => h(DownloadOutline) }),
  },
  {
    label: '欢迎使用',
    key: 'welcome',
    icon: () => h(NIcon, null, { default: () => h(HomeOutline) }),
  },
]

// 当前激活的页面
const currentPage = computed(() => route.path)

// 处理导航
const handleNavigation = (name: string) => {
  router.push(name)
}

// 处理搜索
const handleSearch = () => {
  if (searchQuery.value.trim()) {
    // 实现搜索逻辑
    console.log('搜索:', searchQuery.value)
  }
}

// 切换主题
const toggleTheme = () => {
  isDark.value = !isDark.value
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
  if (isDark.value) {
    document.documentElement.classList.add('dark-theme')
  } else {
    document.documentElement.classList.remove('dark-theme')
  }
}

// 获取页面标题
const pageTitle = computed(() => {
  for (const section of navigationItems) {
    const item = section.items.find(item => item.name === route.path)
    if (item) return item.label
  }
  return 'Game Mod Master'
})

// 处理用户菜单
const handleUserMenuSelect = (key: string) => {
  if (key === 'settings') {
    router.push('/settings')
  } else if (key === 'profile') {
    router.push('/profile')
  } else if (key === 'logout') {
    console.log('退出登录')
  }
}

// 组件挂载时初始化
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
    <!-- 现代化侧边栏 -->
    <NLayoutSider
      :collapsed="collapsed"
      :collapsed-width="80"
      :width="280"
      show-trigger
      collapse-mode="width"
      @collapse="collapsed = true"
      @expand="collapsed = false"
      :native-scrollbar="false"
      class="modern-sider"
      bordered
    >
      <!-- 品牌区域 -->
      <div class="brand-section">
        <router-link to="/" class="brand-link">
          <div class="brand-logo">
            <div class="logo-icon">
              <NIcon size="32">
                <GridOutline />
              </NIcon>
            </div>
            <transition name="fade">
              <div v-show="!collapsed" class="brand-text">
                <h1 class="brand-title">Game Mod</h1>
                <p class="brand-subtitle">Master</p>
              </div>
            </transition>
          </div>
        </router-link>
      </div>

      <!-- 现代化导航 -->
      <div class="navigation-container">
        <div v-for="section in navigationItems" :key="section.title" class="nav-section">
          <transition name="fade">
            <h3 v-show="!collapsed" class="section-title">{{ section.title }}</h3>
          </transition>

          <div class="nav-items">
            <div v-for="item in section.items" :key="item.name" class="nav-item-wrapper">
              <NTooltip v-if="collapsed" placement="right" trigger="hover">
                <template #trigger>
                  <NButton
                    :type="currentPage === item.name ? 'primary' : 'default'"
                    circle
                    size="large"
                    class="modern-nav-button"
                    @click="handleNavigation(item.name)"
                    :style="{
                      background: currentPage === item.name ? `${item.color}15` : 'transparent',
                      borderColor: currentPage === item.name ? item.color : 'transparent'
                    }"
                  >
                    <NIcon :style="{ color: currentPage === item.name ? item.color : 'inherit' }">
                      <component :is="item.icon" />
                    </NIcon>
                  </NButton>
                </template>
                {{ item.tooltip }}
              </NTooltip>

              <NButton
                v-else
                :type="currentPage === item.name ? 'primary' : 'default'"
                class="modern-nav-button expanded"
                @click="handleNavigation(item.name)"
                :style="{
                  background: currentPage === item.name ? `${item.color}15` : 'transparent',
                  borderColor: currentPage === item.name ? item.color : 'transparent',
                  justifyContent: 'flex-start'
                }"
              >
                <template #icon>
                  <NIcon :style="{ color: currentPage === item.name ? item.color : 'inherit' }">
                    <component :is="item.icon" />
                  </NIcon>
                </template>
                <span class="nav-label">{{ item.label }}</span>
              </NButton>
            </div>
          </div>
        </div>
      </div>

      <!-- 底部控制区 -->
      <div class="bottom-controls">
        <NDivider v-if="!collapsed" />

        <!-- 主题切换 -->
        <div class="theme-control">
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

          <div v-else class="theme-switcher">
            <NIcon size="18"><SunnyOutline /></NIcon>
            <NSwitch v-model:value="isDark" @update:value="toggleTheme" />
            <NIcon size="18"><MoonOutline /></NIcon>
          </div>
        </div>
      </div>
    </NLayoutSider>

    <!-- 主内容区域 -->
    <NLayout class="content-wrapper">
      <!-- 现代化顶部栏 -->
      <NLayoutHeader class="modern-header" bordered>
        <div class="header-content">
          <!-- 左侧：折叠按钮和搜索 -->
          <div class="header-left">
            <NButton quaternary circle size="large" class="collapse-btn" @click="collapsed = !collapsed">
              <NIcon size="20">
                <MenuOutline />
              </NIcon>
            </NButton>

            <!-- 现代化搜索框 -->
            <div class="modern-search" :class="{ focused: isSearchFocused }">
              <NInput
                v-model:value="searchQuery"
                placeholder="搜索修改器、游戏..."
                size="large"
                class="search-input"
                @focus="isSearchFocused = true"
                @blur="isSearchFocused = false"
                @keydown.enter="handleSearch"
              >
                <template #prefix>
                  <NIcon class="search-icon">
                    <SearchOutline />
                  </NIcon>
                </template>
              </NInput>
            </div>
          </div>

          <!-- 中间：页面标题 -->
          <div class="header-center">
            <h1 class="page-title">{{ pageTitle }}</h1>
          </div>

          <!-- 右侧：操作按钮 -->
          <div class="header-right">
            <!-- 通知 -->
            <NDropdown :options="notificationMenuOptions" placement="bottom-end" trigger="click">
              <NBadge :value="2" :max="99" :show-zero="false" dot>
                <NButton quaternary circle size="large" class="action-btn">
                  <NIcon size="20">
                    <NotificationsOutline />
                  </NIcon>
                </NButton>
              </NBadge>
            </NDropdown>

            <!-- 用户菜单 -->
            <NDropdown
              :options="userMenuOptions"
              placement="bottom-end"
              trigger="click"
              @select="handleUserMenuSelect"
            >
              <NButton text class="user-profile">
                <NAvatar
                  round
                  size="small"
                  src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
                  class="user-avatar"
                />
                <transition name="fade">
                  <span v-show="!collapsed" class="user-name">用户</span>
                </transition>
              </NButton>
            </NDropdown>
          </div>
        </div>
      </NLayoutHeader>

      <!-- 内容区域 -->
      <NLayoutContent class="modern-content">
        <router-view v-slot="{ Component }">
          <transition name="page-transition" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </NLayoutContent>
    </NLayout>
  </NLayout>
</template>

<style scoped>
/* 全局布局 */
.main-layout {
  height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  position: relative;
  overflow: hidden;
}

/* 背景装饰元素 */
.main-layout::before {
  content: '';
  position: absolute;
  top: -50%;
  right: -50%;
  width: 200%;
  height: 200%;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 1px, transparent 1px);
  background-size: 50px 50px;
  animation: backgroundMove 20s linear infinite;
  z-index: 0;
}

@keyframes backgroundMove {
  0% { transform: translate(0, 0) rotate(0deg); }
  100% { transform: translate(-50px, -50px) rotate(360deg); }
}

/* 现代化侧边栏 */
.modern-sider {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-right: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  z-index: 100;
  position: relative;
}

/* 品牌区域 */
.brand-section {
  padding: 2rem 1.5rem;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  background: linear-gradient(135deg, rgba(124, 58, 237, 0.1) 0%, rgba(8, 145, 178, 0.1) 100%);
}

.brand-link {
  text-decoration: none;
  display: block;
}

.brand-logo {
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: transform 0.3s ease;
}

.brand-logo:hover {
  transform: translateY(-2px);
}

.logo-icon {
  width: 48px;
  height: 48px;
  border-radius: 16px;
  background: linear-gradient(135deg, #7c3aed 0%, #0891b2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 8px 16px rgba(124, 58, 237, 0.3);
}

.brand-text {
  flex: 1;
}

.brand-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 800;
  background: linear-gradient(135deg, #7c3aed 0%, #0891b2 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  line-height: 1.2;
}

.brand-subtitle {
  margin: 0;
  font-size: 0.875rem;
  color: #6b7280;
  font-weight: 500;
}

/* 导航区域 */
.navigation-container {
  flex: 1;
  padding: 1.5rem 1rem;
  overflow-y: auto;
}

.nav-section {
  margin-bottom: 2rem;
}

.section-title {
  font-size: 0.75rem;
  font-weight: 700;
  color: #9ca3af;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin: 0 0 1rem 0.75rem;
}

.nav-items {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.nav-item-wrapper {
  position: relative;
}

.modern-nav-button {
  height: 48px !important;
  border-radius: 16px !important;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1) !important;
  border: 2px solid transparent !important;
  font-weight: 600 !important;
  position: relative;
  overflow: hidden;
}

.modern-nav-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.4), transparent);
  transition: left 0.5s ease;
}

.modern-nav-button:hover::before {
  left: 100%;
}

.modern-nav-button:hover {
  transform: translateY(-2px) scale(1.02);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.modern-nav-button.expanded {
  width: 100%;
  padding: 0 1.25rem !important;
}

.nav-label {
  font-size: 0.95rem;
  font-weight: 600;
}

/* 底部控制区 */
.bottom-controls {
  padding: 1.5rem 1rem;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
  background: rgba(248, 250, 252, 0.5);
}

.theme-control {
  display: flex;
  justify-content: center;
}

.theme-switcher {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

/* 主内容区域 */
.content-wrapper {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

/* 现代化顶部栏 */
.modern-header {
  height: 80px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  z-index: 90;
  position: relative;
}

.header-content {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 2rem;
  max-width: 1600px;
  margin: 0 auto;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  flex: 1;
}

.collapse-btn {
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
}

.collapse-btn:hover {
  background: rgba(255, 255, 255, 1);
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

/* 现代化搜索框 */
.modern-search {
  position: relative;
  max-width: 400px;
  flex: 1;
  transition: all 0.3s ease;
}

.modern-search.focused {
  transform: scale(1.02);
}

.search-input {
  background: rgba(255, 255, 255, 0.9);
  border: 2px solid rgba(0, 0, 0, 0.1);
  border-radius: 16px !important;
  transition: all 0.3s ease;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
}

.search-input:focus-within {
  border-color: #7c3aed;
  box-shadow: 0 0 0 4px rgba(124, 58, 237, 0.1), 0 8px 25px rgba(124, 58, 237, 0.15);
  transform: translateY(-2px);
}

.search-icon {
  color: #9ca3af;
  transition: color 0.3s ease;
}

.search-input:focus-within .search-icon {
  color: #7c3aed;
}

.header-center {
  flex: 0 0 auto;
  padding: 0 2rem;
}

.page-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 700;
  background: linear-gradient(135deg, #7c3aed 0%, #0891b2 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  text-align: center;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
  justify-content: flex-end;
}

.action-btn {
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  position: relative;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 1);
  transform: translateY(-2px) rotate(5deg);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.user-profile {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 1rem;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 50px;
  transition: all 0.3s ease;
  text-decoration: none;
}

.user-profile:hover {
  background: rgba(255, 255, 255, 1);
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.user-avatar {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border: 2px solid white;
}

.user-name {
  font-weight: 600;
  color: #374151;
  font-size: 0.9rem;
}

/* 现代化内容区域 */
.modern-content {
  flex: 1;
  padding: 2rem;
  overflow-y: auto;
  background: rgba(248, 250, 252, 0.5);
  position: relative;
}

.modern-content::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba(124, 58, 237, 0.2), transparent);
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}

.page-transition-enter-active,
.page-transition-leave-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-transition-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
}

.page-transition-leave-to {
  opacity: 0;
  transform: translateY(-20px) scale(1.05);
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .header-content {
    padding: 0 1.5rem;
  }

  .modern-search {
    max-width: 300px;
  }
}

@media (max-width: 768px) {
  .modern-sider {
    position: fixed;
    height: 100vh;
    z-index: 1000;
  }

  .header-center {
    display: none;
  }

  .modern-search {
    display: none;
  }

  .user-name {
    display: none;
  }

  .header-content {
    padding: 0 1rem;
  }

  .modern-content {
    padding: 1rem;
  }

  .page-title {
    font-size: 1.25rem;
  }
}

/* 暗色主题 */
:deep(.dark-theme) .main-layout {
  background: linear-gradient(135deg, #1e1b4b 0%, #1e293b 100%);
}

:deep(.dark-theme) .modern-sider {
  background: rgba(30, 41, 59, 0.95);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .brand-section {
  background: linear-gradient(135deg, rgba(124, 58, 237, 0.2) 0%, rgba(8, 145, 178, 0.2) 100%);
}

:deep(.dark-theme) .modern-header {
  background: rgba(30, 41, 59, 0.95);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .content-wrapper {
  background: rgba(15, 23, 42, 0.9);
}

:deep(.dark-theme) .modern-content {
  background: rgba(30, 41, 59, 0.5);
}

:deep(.dark-theme) .search-input {
  background: rgba(30, 41, 59, 0.9);
  border-color: rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .collapse-btn,
:deep(.dark-theme) .action-btn,
:deep(.dark-theme) .user-profile {
  background: rgba(30, 41, 59, 0.8);
  border-color: rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .page-title {
  background: linear-gradient(135deg, #a78bfa 0%, #67e8f9 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

:deep(.dark-theme) .user-name {
  color: #e2e8f0;
}

:deep(.dark-theme) .theme-switcher {
  background: rgba(30, 41, 59, 0.8);
}
</style>
