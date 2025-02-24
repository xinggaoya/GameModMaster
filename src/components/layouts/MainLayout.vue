<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NMenu,
  NButton,
  NSpace,
  NIcon,
  NDivider,
  NAvatar,
  type MenuOption,
} from 'naive-ui'
import {
  HomeOutline,
  DownloadOutline,
  InformationCircleOutline,
  MenuOutline,
  SunnyOutline,
  MoonOutline,
} from '@vicons/ionicons5'

const router = useRouter()
const route = useRoute()

// 侧边栏折叠状态
const collapsed = ref(false)
const isDark = ref(true)

// 菜单配置
const menuOptions: MenuOption[] = [
  {
    label: '首页',
    key: '/',
    icon: () => h(NIcon, null, { default: () => h(HomeOutline) }),
  },
  {
    label: '下载管理',
    key: '/downloads',
    icon: () => h(NIcon, null, { default: () => h(DownloadOutline) }),
  },
  {
    label: '关于',
    key: '/about',
    icon: () => h(NIcon, null, { default: () => h(InformationCircleOutline) }),
  },
]

// 当前激活的菜单项
const activeKey = computed(() => route.path)

// 处理菜单点击
const handleMenuClick = (key: string) => {
  router.push(key)
}

// 切换主题
const toggleTheme = () => {
  isDark.value = !isDark.value
  // 保存主题设置到本地存储
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
}
</script>

<template>
  <NLayout class="main-layout" has-sider>
    <!-- 侧边栏 -->
    <NLayoutSider
      bordered
      collapse-mode="width"
      :collapsed="collapsed"
      :collapsed-width="64"
      :width="240"
      show-trigger
      @collapse="collapsed = true"
      @expand="collapsed = false"
      class="main-sider"
    >
      <!-- Logo -->
      <div class="logo-container">
        <img src="/src/assets/logo.png" alt="Logo" class="logo" v-if="!collapsed" />
        <img src="/src/assets/logo.png" alt="Logo" class="logo-small" v-else />
      </div>

      <!-- 导航菜单 -->
      <NMenu
        :value="activeKey"
        :collapsed="collapsed"
        :collapsed-width="64"
        :collapsed-icon-size="22"
        :options="menuOptions"
        :render-label="collapsed ? undefined : undefined"
        @update:value="handleMenuClick"
      />
    </NLayoutSider>

    <!-- 主内容区 -->
    <NLayout class="main-content">
      <!-- 顶部栏 -->
      <NLayoutHeader bordered class="main-header">
        <div class="header-left">
          <NButton quaternary circle class="collapse-button" @click="collapsed = !collapsed">
            <NIcon><MenuOutline /></NIcon>
          </NButton>
          <h2 class="page-title">{{ route.meta.title || '游戏修改器管理器' }}</h2>
        </div>
        <div class="header-right">
          <NSpace>
            <!-- 主题切换 -->
            <NButton circle quaternary @click="toggleTheme">
              <template #icon>
                <NIcon>
                  <MoonOutline v-if="isDark" />
                  <SunnyOutline v-else />
                </NIcon>
              </template>
            </NButton>
            <!-- 用户头像 -->
            <NAvatar
              round
              size="small"
              src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
            />
          </NSpace>
        </div>
      </NLayoutHeader>

      <!-- 内容区 -->
      <NLayoutContent class="layout-content">
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
}

.main-sider {
  background: var(--card-color);
  border-right: 1px solid var(--divider-color);
}

.logo-container {
  height: 64px;
  padding: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid var(--divider-color);
}

.logo {
  height: 32px;
  object-fit: contain;
}

.logo-small {
  height: 32px;
  width: 32px;
  object-fit: contain;
}

.main-header {
  height: 64px;
  padding: 0 24px;
  background: var(--card-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.page-title {
  margin: 0;
  font-size: 1.2rem;
  font-weight: 500;
}

.layout-content {
  padding: 24px;
  background: var(--body-color);
  min-height: calc(100vh - 64px);
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

  .header-left {
    gap: 8px;
  }

  .page-title {
    font-size: 1rem;
  }

  .layout-content {
    padding: 16px;
  }
}
</style>
