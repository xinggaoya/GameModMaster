<script setup lang="ts">
import { ref, computed, onMounted, h } from 'vue'
import { useRouter } from 'vue-router'
import {
  Search,
  SearchOutline,
  GameControllerOutline,
  DownloadOutline,
  PeopleOutline,
  RefreshOutline,
  SparklesOutline,
  TrendingUpOutline,
  StarOutline,
  TimeOutline,
} from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import {
  NCard,
  NInput,
  NButton,
  NIcon,
  NGrid,
  NGridItem,
  NSpin,
  NPagination,
  NSelect,
} from 'naive-ui'

// 导入自定义组件
import GameCard from '@/components/common/GameCard.vue'

const store = useTrainerStore()

// 状态管理
const searchQuery = ref('')
const sortBy = ref('latest')
const isSearchFocused = ref(false)

// 计算属性
const isLoading = computed(() => store.isLoading)
const totalPages = computed(() => store.totalPages)
const trainers = computed(() => store.trainers)
const errorMessage = computed(() => store.error || '')

// 排序选项
const sortOptions = [
  {
    label: '最新更新',
    value: 'latest',
    icon: TimeOutline,
    description: '按最后更新时间排序'
  },
  {
    label: '热门下载',
    value: 'downloads',
    icon: TrendingUpOutline,
    description: '按下载量排序'
  },
  {
    label: '评分最高',
    value: 'rating',
    icon: StarOutline,
    description: '按用户评分排序'
  },
  {
    label: '名称',
    value: 'name',
    icon: SearchOutline,
    description: '按名称字母排序'
  },
]

// 便当盒布局统计数据 - 优化为更紧凑的布局
const bentoStats = computed(() => [
  {
    title: '总修改器',
    value: formatNumber(store.trainers.length),
    subtitle: '可用修改器',
    icon: GameControllerOutline,
    color: '#7c3aed',
    gradient: 'from-purple-500 to-purple-600',
    size: 'large'
  },
  {
    title: '已下载',
    value: formatNumber(store.downloadedTrainers.length),
    subtitle: '我的收藏',
    icon: DownloadOutline,
    color: '#0891b2',
    gradient: 'from-cyan-500 to-cyan-600',
    size: 'medium'
  },
  {
    title: '已安装',
    value: formatNumber(store.installedTrainers.length),
    subtitle: '可直接启动',
    icon: SparklesOutline,
    color: '#059669',
    gradient: 'from-emerald-500 to-emerald-600',
    size: 'medium'
  },
  {
    title: '最近使用',
    value: getLastUsedGame(),
    subtitle: formatDate(getLastUsedDate()),
    icon: PeopleOutline,
    color: '#dc2626',
    gradient: 'from-red-500 to-red-600',
    size: 'small'
  }
])

// 获取最后使用的游戏名称
function getLastUsedGame() {
  const trainers = [...store.downloadedTrainers]
  if (trainers.length === 0) return '暂无'

  trainers.sort((a, b) => {
    const timeA = a.last_launch_time ? new Date(a.last_launch_time).getTime() : 0
    const timeB = b.last_launch_time ? new Date(b.last_launch_time).getTime() : 0
    return timeB - timeA
  })

  return trainers[0]?.name || '暂无'
}

// 获取最后使用日期
function getLastUsedDate() {
  const trainers = [...store.downloadedTrainers]
  if (trainers.length === 0) return null

  trainers.sort((a, b) => {
    const timeA = a.last_launch_time ? new Date(a.last_launch_time).getTime() : 0
    const timeB = b.last_launch_time ? new Date(b.last_launch_time).getTime() : 0
    return timeB - timeA
  })

  return trainers[0]?.last_launch_time || null
}

// 格式化日期
const formatDate = (dateString: string | undefined | null) => {
  if (!dateString) return '未知'
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
  })
}

// 格式化数字
const formatNumber = (num: number): string => {
  return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
}

// 处理搜索
const handleSearch = () => {
  if (searchQuery.value.trim()) {
    store.currentPage = 1
    store.searchTrainers(searchQuery.value)
  } else {
    store.fetchTrainers(1)
  }
}

// 处理分页
const handlePageChange = async (page: number) => {
  try {
    store.currentPage = page
    if (searchQuery.value.trim()) {
      await store.searchTrainers(searchQuery.value, page)
    } else {
      await store.fetchTrainers(page)
    }
    window.scrollTo({ top: 0, behavior: 'smooth' })
  } catch {
    // 错误处理由store处理
  }
}

// 处理刷新
const handleRefresh = async () => {
  try {
    if (searchQuery.value.trim()) {
      await store.searchTrainers(searchQuery.value, store.currentPage)
    } else {
      await store.fetchTrainers(store.currentPage)
    }
  } catch {
    // 错误处理由store处理
  }
}

// 组件挂载时初始化
onMounted(async () => {
  if (trainers.value.length === 0) {
    await store.fetchTrainers(1)
  }
})
</script>

<template>
  <div class="modern-home-view">
    <!-- 超紧凑统计区域 - 单行显示 -->
    <section class="ultra-compact-stats-section">
      <div class="stats-bar">
        <div class="stat-item stat-primary">
          <NIcon size="18" class="stat-icon">
            <GameControllerOutline />
          </NIcon>
          <div class="stat-info">
            <div class="stat-number">{{ bentoStats[0].value }}</div>
            <div class="stat-label">{{ bentoStats[0].title }}</div>
          </div>
        </div>

        <div class="stat-item stat-cyan">
          <NIcon size="18" class="stat-icon">
            <DownloadOutline />
          </NIcon>
          <div class="stat-info">
            <div class="stat-number">{{ bentoStats[1].value }}</div>
            <div class="stat-label">{{ bentoStats[1].title }}</div>
          </div>
        </div>

        <div class="stat-item stat-emerald">
          <NIcon size="18" class="stat-icon">
            <SparklesOutline />
          </NIcon>
          <div class="stat-info">
            <div class="stat-number">{{ bentoStats[2].value }}</div>
            <div class="stat-label">{{ bentoStats[2].title }}</div>
          </div>
        </div>

        <div class="stat-item stat-red">
          <NIcon size="16" class="stat-icon">
            <PeopleOutline />
          </NIcon>
          <div class="stat-info">
            <div class="stat-text">{{ bentoStats[3].value }}</div>
            <div class="stat-label">{{ bentoStats[3].title }}</div>
          </div>
        </div>
      </div>
    </section>

    <!-- 搜索和控制区域 -->
    <section class="search-control-section">
      <NCard class="search-card">
        <div class="search-content">
          <!-- 搜索框 -->
          <div class="modern-search-wrapper" :class="{ focused: isSearchFocused }">
            <NInput
              v-model:value="searchQuery"
              placeholder="搜索修改器、游戏..."
              size="large"
              class="modern-search-input"
              @focus="isSearchFocused = true"
              @blur="isSearchFocused = false"
              @keydown.enter="handleSearch"
              clearable
            >
              <template #prefix>
                <NIcon class="search-icon">
                  <Search />
                </NIcon>
              </template>
            </NInput>
            <NButton
              type="primary"
              size="large"
              class="search-button"
              @click="handleSearch"
              :loading="isLoading"
            >
              <template #icon>
                <NIcon><Search /></NIcon>
              </template>
              搜索
            </NButton>
          </div>

          <!-- 控制按钮 -->
          <div class="control-buttons">
            <NSelect
              v-model:value="sortBy"
              :options="sortOptions"
              :render-label="(option: any) => {
                return h('div', { class: 'sort-option' }, [
                  h(NIcon, { class: 'sort-option-icon' }, { default: () => h(option.icon) }),
                  h('div', { class: 'sort-option-content' }, [
                    h('div', { class: 'sort-option-label' }, option.label),
                    h('div', { class: 'sort-option-description' }, option.description)
                  ])
                ])
              }"
              size="large"
              class="sort-select"
              placeholder="排序方式"
            />

            <NButton
              size="large"
              class="refresh-button"
              @click="handleRefresh"
              :loading="isLoading"
            >
              <template #icon>
                <NIcon><RefreshOutline /></NIcon>
              </template>
              刷新
            </NButton>
          </div>
        </div>
      </NCard>
    </section>

    <!-- 结果标题 -->
    <section class="results-header" v-if="!isLoading && (trainers.length > 0 || searchQuery)">
      <div class="results-title">
        <h2 class="results-heading">
          <NIcon class="results-icon">
            <Search />
          </NIcon>
          {{ searchQuery ? `"${searchQuery}" 的搜索结果` : '探索修改器' }}
        </h2>
        <p class="results-subtitle">
          找到 {{ trainers.length }} 个修改器
        </p>
      </div>
    </section>

    <!-- 错误显示 -->
    <NCard v-if="errorMessage" class="error-card">
      <div class="error-content">
        <NIcon class="error-icon" size="48">
          <Search />
        </NIcon>
        <h3 class="error-title">搜索出错了</h3>
        <p class="error-message">{{ errorMessage }}</p>
        <NButton @click="handleRefresh" type="primary">
          <template #icon>
            <NIcon><RefreshOutline /></NIcon>
          </template>
          重试
        </NButton>
      </div>
    </NCard>

    <!-- 加载状态 -->
    <div v-else-if="isLoading" class="loading-container">
      <div class="loading-content">
        <NSpin size="large" />
        <p class="loading-text">正在加载修改器...</p>
      </div>
    </div>

    <!-- 空状态 -->
    <NCard v-else-if="trainers.length === 0" class="empty-card">
      <div class="empty-content">
        <div class="empty-icon">
          <NIcon size="64">
            <Search />
          </NIcon>
        </div>
        <h3 class="empty-title">
          {{ searchQuery ? `没有找到 "${searchQuery}" 相关的修改器` : '没有找到任何修改器' }}
        </h3>
        <p class="empty-subtitle">
          {{ searchQuery ? '尝试使用不同的关键词搜索' : '暂时没有可用的修改器' }}
        </p>
        <div class="empty-actions">
          <NButton @click="store.fetchTrainers(1)" type="primary" size="large">
            <template #icon>
              <NIcon><GameControllerOutline /></NIcon>
            </template>
            浏览所有修改器
          </NButton>
          <NButton v-if="searchQuery" @click="searchQuery = ''" size="large">
            清除搜索
          </NButton>
        </div>
      </div>
    </NCard>

    <!-- 修改器网格 -->
    <section v-else class="trainers-grid-section">
      <NGrid
        cols="1 s:2 m:3 l:4 xl:5"
        responsive="screen"
        :x-gap="20"
        :y-gap="24"
        class="trainers-grid"
      >
        <NGridItem
          v-for="trainer in trainers"
          :key="trainer.id"
          class="trainer-grid-item"
        >
          <GameCard :trainer="trainer" />
        </NGridItem>
      </NGrid>
    </section>

    <!-- 分页 -->
    <div class="pagination-container" v-if="totalPages > 1">
      <NPagination
        v-model:page="store.currentPage"
        :page-count="totalPages"
        :page-sizes="[12, 24, 36, 48]"
        show-size-picker
        show-quick-jumper
        @update:page="handlePageChange"
        class="modern-pagination"
      />
    </div>
  </div>
</template>

<style scoped>
/* 现代化首页布局 */
.modern-home-view {
  width: 100%;
  min-height: 100vh;
  background: transparent;
  position: relative;
}

/* 超紧凑统计区域 - 单行显示 */
.ultra-compact-stats-section {
  margin-bottom: 1.5rem;
}

.stats-bar {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.5rem;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 20px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  position: relative;
}

.stats-bar:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0.75rem;
  border-radius: 12px;
  transition: all 0.2s ease;
  flex: 1;
  min-width: 0;
}

.stat-item:hover {
  background: rgba(0, 0, 0, 0.05);
  transform: scale(1.02);
}

.stat-primary .stat-icon {
  color: #7c3aed;
}

.stat-cyan .stat-icon {
  color: #0891b2;
}

.stat-emerald .stat-icon {
  color: #059669;
}

.stat-red .stat-icon {
  color: #dc2626;
}

.stat-info {
  flex: 1;
  min-width: 0;
}

.stat-number {
  font-size: 1.25rem;
  font-weight: 800;
  color: #1f2937;
  line-height: 1.2;
  margin-bottom: 0.125rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.stat-text {
  font-size: 0.875rem;
  font-weight: 700;
  color: #1f2937;
  line-height: 1.2;
  margin-bottom: 0.125rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.stat-label {
  font-size: 0.688rem;
  color: #6b7280;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 紧凑统计区域 */
.compact-stats-section {
  margin-bottom: 1.5rem;
}

/* 紧凑卡片样式 */
.compact-card {
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  position: relative;
  height: 80px;
}

.compact-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
}

.compact-card-content {
  display: flex;
  align-items: center;
  gap: 1rem;
  height: 100%;
  padding: 1rem;
}

.compact-icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 12px;
  flex-shrink: 0;
}

.compact-card-primary .compact-icon-wrapper {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  color: white;
}

.compact-card-cyan .compact-icon-wrapper {
  background: linear-gradient(135deg, #0891b2 0%, #0e7490 100%);
  color: white;
}

.compact-card-emerald .compact-icon-wrapper {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  color: white;
}

.compact-card-red .compact-icon-wrapper {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  color: white;
}

.compact-icon {
  color: white;
}

.compact-content {
  flex: 1;
  min-width: 0;
}

.compact-value {
  font-size: 1.5rem;
  font-weight: 800;
  color: #1f2937;
  line-height: 1.2;
  margin-bottom: 0.125rem;
}

.compact-value-small {
  font-size: 1rem;
  font-weight: 700;
  color: #1f2937;
  line-height: 1.2;
  margin-bottom: 0.125rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.compact-title {
  font-size: 0.75rem;
  color: #6b7280;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* 便当盒统计区域 */
.bento-stats-section {
  margin-bottom: 2rem;
}

.bento-card {
  border-radius: 24px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  position: relative;
}

.bento-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.5), transparent);
}

.bento-card:hover {
  transform: translateY(-4px) scale(1.02);
  box-shadow:
    0 16px 48px rgba(0, 0, 0, 0.15),
    inset 0 1px 0 rgba(255, 255, 255, 0.3);
}

.bento-card-large {
  min-height: 180px;
  padding: 2rem;
}

.bento-card-medium {
  min-height: 140px;
  padding: 1.5rem;
}

.bento-card-small {
  min-height: 100px;
  padding: 1.25rem;
}

.bento-card-content {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  height: 100%;
  position: relative;
}

.stat-icon-wrapper {
  flex-shrink: 0;
  width: 64px;
  height: 64px;
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.9);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
}

.bento-card-medium .stat-icon-wrapper,
.bento-card-small .stat-icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: 16px;
}

.bento-card-small .stat-icon-wrapper {
  width: 40px;
  height: 40px;
  border-radius: 12px;
}

.stat-icon {
  transition: transform 0.3s ease;
}

.bento-card:hover .stat-icon {
  transform: scale(1.1) rotate(5deg);
}

.stat-content {
  flex: 1;
}

.stat-title {
  margin: 0 0 0.5rem 0;
  font-size: 0.875rem;
  font-weight: 600;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.stat-value {
  margin: 0 0 0.25rem 0;
  font-size: 2rem;
  font-weight: 800;
  line-height: 1;
  background: linear-gradient(135deg, #1f2937 0%, #374151 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.bento-card-medium .stat-value {
  font-size: 1.75rem;
}

.bento-card-small .stat-value {
  font-size: 1.5rem;
}

.stat-subtitle {
  margin: 0;
  font-size: 0.875rem;
  color: #9ca3af;
  font-weight: 500;
}

.stat-decoration {
  position: absolute;
  top: 1rem;
  right: 1rem;
  display: flex;
  gap: 0.5rem;
}

.decoration-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.1);
  animation: pulse 2s ease-in-out infinite;
}

.decoration-dot-1 {
  animation-delay: 0s;
}

.decoration-dot-2 {
  animation-delay: 0.5s;
}

@keyframes pulse {
  0%, 100% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.8; transform: scale(1.2); }
}

/* 统计卡片颜色主题 */
.stat-card-primary .stat-icon-wrapper {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  color: white;
}

.stat-card-primary .stat-value {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.stat-card-cyan .stat-icon-wrapper {
  background: linear-gradient(135deg, #0891b2 0%, #0e7490 100%);
  color: white;
}

.stat-card-cyan .stat-value {
  background: linear-gradient(135deg, #0891b2 0%, #0e7490 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.stat-card-emerald .stat-icon-wrapper {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  color: white;
}

.stat-card-emerald .stat-value {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.stat-card-red .stat-icon-wrapper {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  color: white;
}

.stat-card-red .stat-value {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

/* 搜索和控制区域 */
.search-control-section {
  margin-bottom: 2rem;
}

.search-card {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  border-radius: 24px;
  padding: 2rem;
}

.search-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.modern-search-wrapper {
  display: flex;
  gap: 1rem;
  align-items: stretch;
  transition: all 0.3s ease;
}

.modern-search-wrapper.focused {
  transform: scale(1.01);
}

.modern-search-input {
  flex: 1;
  background: rgba(255, 255, 255, 0.9);
  border: 2px solid rgba(0, 0, 0, 0.1);
  border-radius: 16px !important;
  transition: all 0.3s ease;
}

.modern-search-input:focus-within {
  border-color: #7c3aed;
  box-shadow: 0 0 0 4px rgba(124, 58, 237, 0.1);
  transform: translateY(-2px);
}

.search-icon {
  color: #9ca3af;
  transition: color 0.3s ease;
}

.modern-search-input:focus-within .search-icon {
  color: #7c3aed;
}

.search-button {
  border-radius: 16px;
  padding: 0 2rem;
  font-weight: 600;
  transition: all 0.3s ease;
}

.search-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(124, 58, 237, 0.3);
}

.control-buttons {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.sort-select {
  min-width: 200px;
}

/* 下拉选项样式 - 修复超出问题 */
:deep(.n-base-select-menu) {
  max-height: 320px;
  overflow-y: auto;
}

:deep(.n-base-select-option) {
  padding: 0.75rem 1rem;
  border-radius: 8px;
  margin: 0.25rem 0.5rem;
  transition: all 0.2s ease;
}

:deep(.n-base-select-option:hover) {
  background-color: rgba(124, 58, 237, 0.1);
}

.sort-option {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
}

.sort-option-icon {
  font-size: 1.125rem;
  color: #7c3aed;
  flex-shrink: 0;
}

.sort-option-content {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.sort-option-label {
  font-weight: 600;
  color: #1f2937;
  line-height: 1.3;
  margin-bottom: 0.125rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sort-option-description {
  font-size: 0.75rem;
  color: #6b7280;
  line-height: 1.3;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.refresh-button {
  border-radius: 16px;
  padding: 0 1.5rem;
  transition: all 0.3s ease;
}

.refresh-button:hover {
  transform: translateY(-2px);
}

/* 结果标题区域 */
.results-header {
  margin-bottom: 2rem;
  padding: 0 1rem;
}

.results-title {
  text-align: center;
}

.results-heading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  margin: 0 0 0.5rem 0;
  font-size: 2rem;
  font-weight: 800;
  background: linear-gradient(135deg, #1f2937 0%, #374151 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.results-icon {
  color: #7c3aed;
}

.results-subtitle {
  margin: 0;
  font-size: 1.125rem;
  color: #6b7280;
  font-weight: 500;
}

/* 错误卡片 */
.error-card {
  background: rgba(239, 68, 68, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(239, 68, 68, 0.2);
  box-shadow: 0 8px 32px rgba(239, 68, 68, 0.2);
  border-radius: 24px;
  margin-bottom: 2rem;
}

.error-content {
  text-align: center;
  padding: 3rem 2rem;
  color: white;
}

.error-icon {
  margin-bottom: 1.5rem;
  opacity: 0.8;
}

.error-title {
  margin: 0 0 1rem 0;
  font-size: 1.5rem;
  font-weight: 700;
}

.error-message {
  margin: 0 0 2rem 0;
  font-size: 1rem;
  opacity: 0.9;
}

/* 加载状态 */
.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}

.loading-content {
  text-align: center;
}

.loading-text {
  margin-top: 1rem;
  font-size: 1.125rem;
  color: #6b7280;
  font-weight: 500;
}

/* 空状态卡片 */
.empty-card {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  border-radius: 24px;
  margin-bottom: 2rem;
}

.empty-content {
  text-align: center;
  padding: 4rem 2rem;
}

.empty-icon {
  margin-bottom: 2rem;
  padding: 1.5rem;
  border-radius: 24px;
  background: rgba(124, 58, 237, 0.1);
  color: #7c3aed;
  display: inline-block;
}

.empty-title {
  margin: 0 0 1rem 0;
  font-size: 1.5rem;
  font-weight: 700;
  color: #1f2937;
}

.empty-subtitle {
  margin: 0 0 2rem 0;
  font-size: 1rem;
  color: #6b7280;
  line-height: 1.6;
}

.empty-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
}

/* 修改器网格区域 */
.trainers-grid-section {
  margin-bottom: 2rem;
}

.trainers-grid {
  animation: fadeInUp 0.6s ease-out;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.trainer-grid-item {
  animation: fadeInUp 0.6s ease-out backwards;
}

.trainer-grid-item:nth-child(1) { animation-delay: 0.1s; }
.trainer-grid-item:nth-child(2) { animation-delay: 0.2s; }
.trainer-grid-item:nth-child(3) { animation-delay: 0.3s; }
.trainer-grid-item:nth-child(4) { animation-delay: 0.4s; }
.trainer-grid-item:nth-child(5) { animation-delay: 0.5s; }

/* 分页容器 */
.pagination-container {
  display: flex;
  justify-content: center;
  padding: 2rem 0;
}

.modern-pagination {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  border-radius: 16px;
  padding: 1rem;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .search-content {
    flex-direction: column;
  }

  .modern-search-wrapper {
    flex-direction: column;
  }

  .control-buttons {
    flex-direction: column;
    width: 100%;
  }

  .sort-select {
    width: 100%;
  }
}

@media (max-width: 768px) {
  .ultra-compact-stats-section {
    margin-bottom: 1rem;
  }

  .stats-bar {
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
  }

  .stat-item {
    flex: 1 1 calc(50% - 0.25rem);
    min-width: calc(50% - 0.25rem);
    padding: 0.5rem;
    gap: 0.5rem;
  }

  .stat-number {
    font-size: 1.125rem;
  }

  .stat-text {
    font-size: 0.75rem;
  }

  .stat-label {
    font-size: 0.625rem;
  }

  .compact-stats-section {
    margin-bottom: 1rem;
  }

  .compact-card {
    height: 70px;
  }

  .compact-card-content {
    padding: 0.75rem;
    gap: 0.75rem;
  }

  .compact-icon-wrapper {
    width: 36px;
    height: 36px;
  }

  .compact-value {
    font-size: 1.25rem;
  }

  .compact-value-small {
    font-size: 0.875rem;
  }

  .compact-title {
    font-size: 0.688rem;
  }

  .sort-option {
    gap: 0.5rem;
  }

  .sort-option-icon {
    font-size: 1rem;
  }

  .sort-option-label {
    font-size: 0.875rem;
  }

  .sort-option-description {
    font-size: 0.688rem;
  }

  .bento-card-large {
    min-height: 140px;
    padding: 1.5rem;
  }

  .bento-card-medium {
    min-height: 120px;
    padding: 1.25rem;
  }

  .bento-card-small {
    min-height: 80px;
    padding: 1rem;
  }

  .search-card {
    padding: 1.5rem;
  }

  .results-heading {
    font-size: 1.5rem;
  }

  .empty-actions {
    flex-direction: column;
    align-items: center;
  }

  .empty-actions button {
    width: 100%;
    max-width: 300px;
  }
}

/* 暗色主题 */
:deep(.dark-theme) .stats-bar,
:deep(.dark-theme) .compact-card,
:deep(.dark-theme) .bento-card,
:deep(.dark-theme) .search-card,
:deep(.dark-theme) .empty-card {
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .stat-item:hover {
  background: rgba(255, 255, 255, 0.05);
}

:deep(.dark-theme) .stat-number,
:deep(.dark-theme) .stat-text,
:deep(.dark-theme) .compact-value,
:deep(.dark-theme) .compact-value-small,
:deep(.dark-theme) .stat-value {
  color: #e2e8f0;
}

:deep(.dark-theme) .stat-label,
:deep(.dark-theme) .compact-title,
:deep(.dark-theme) .stat-title {
  color: #cbd5e1;
}

:deep(.dark-theme) .stat-subtitle {
  color: #94a3b8;
}

:deep(.dark-theme) .modern-search-input {
  background: rgba(30, 41, 59, 0.9);
  border-color: rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .sort-option-label {
  color: #e2e8f0;
}

:deep(.dark-theme) .sort-option-description {
  color: #94a3b8;
}

:deep(.dark-theme) .results-heading {
  background: linear-gradient(135deg, #e2e8f0 0%, #cbd5e1 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

:deep(.dark-theme) .results-subtitle {
  color: #94a3b8;
}

:deep(.dark-theme) .loading-text {
  color: #94a3b8;
}

:deep(.dark-theme) .empty-title {
  color: #e2e8f0;
}

:deep(.dark-theme) .empty-subtitle {
  color: #94a3b8;
}

:deep(.dark-theme) .modern-pagination {
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
}
</style>
