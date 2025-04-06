<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import {
  Search,
  GameControllerOutline,
  DownloadOutline,
  StatsChartOutline,
  PeopleOutline,
  RefreshOutline,
  FilterOutline,
} from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import {
  NCard,
  NInput,
  NInputGroup,
  NButton,
  NIcon,
  NGrid,
  NGridItem,
  NSpace,
  NText,
  NEmpty,
  NSpin,
  NPagination,
  NDropdown,
  NSelect,
} from 'naive-ui'

// 导入自定义组件
import GameCard from '@/components/common/GameCard.vue'
import StatusCard from '@/components/common/StatusCard.vue'

const router = useRouter()
const store = useTrainerStore()

// 状态变量
const searchQuery = ref('')
const isLoading = computed(() => store.isLoading)
const totalPages = computed(() => store.totalPages)
const trainers = computed(() => store.trainers)
const sortBy = ref('latest') // 默认排序：最新

// 排序选项
const sortOptions = [
  { label: '最新更新', value: 'latest', icon: 'TimeOutline' },
  { label: '下载次数', value: 'downloads', icon: 'DownloadOutline' },
  { label: '名称', value: 'name', icon: 'TextOutline' },
]

// 计算错误信息，确保返回字符串
const errorMessage = computed(() => store.error || '')

// 本地数据统计
const statistics = computed(() => [
  {
    title: '已下载修改器',
    value: store.downloadedTrainers.length,
    icon: DownloadOutline,
    color: '#6366f1',
  },
  {
    title: '已安装修改器',
    value: store.installedTrainers.length,
    icon: GameControllerOutline,
    color: '#2dd4bf',
  },
  {
    title: '最近使用',
    value: getLastUsedGame(),
    icon: PeopleOutline,
    color: '#f43f5e',
  },
  {
    title: '上次使用时间',
    value: getLastUsedTime(),
    icon: StatsChartOutline,
    color: '#f59e0b',
  },
])

// 获取最后使用的游戏名称
function getLastUsedGame() {
  const trainers = [...store.downloadedTrainers]
  if (trainers.length === 0) return '无'

  trainers.sort((a, b) => {
    const timeA = a.last_launch_time ? new Date(a.last_launch_time).getTime() : 0
    const timeB = b.last_launch_time ? new Date(b.last_launch_time).getTime() : 0
    return timeB - timeA
  })

  return trainers[0]?.name || '无'
}

// 获取上次使用时间
function getLastUsedTime() {
  const trainers = [...store.downloadedTrainers]
  if (trainers.length === 0) return '无使用记录'

  trainers.sort((a, b) => {
    const timeA = a.last_launch_time ? new Date(a.last_launch_time).getTime() : 0
    const timeB = b.last_launch_time ? new Date(b.last_launch_time).getTime() : 0
    return timeB - timeA
  })

  if (!trainers[0]?.last_launch_time) return '无使用记录'

  return formatDate(trainers[0].last_launch_time)
}

// 格式化日期
const formatDate = (dateString: string | undefined | null) => {
  if (!dateString) return '未知'
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

// 处理搜索
const handleSearch = () => {
  if (searchQuery.value.trim()) {
    store.currentPage = 1 // 搜索时重置页码
    store.searchTrainers(searchQuery.value)
  } else {
    // 如果搜索框为空，恢复默认列表
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
    window.scrollTo({ top: 0, behavior: 'smooth' }) // 滚动到顶部
  } catch (error) {
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
  } catch (error) {
    // 错误处理由store处理
  }
}

// 处理排序变化
watch(sortBy, (newValue) => {
  const sortedTrainers = [...trainers.value]

  switch (newValue) {
    case 'latest':
      sortedTrainers.sort(
        (a, b) => new Date(b.last_update).getTime() - new Date(a.last_update).getTime(),
      )
      break
    case 'downloads':
      sortedTrainers.sort((a, b) => b.download_count - a.download_count)
      break
    case 'name':
      sortedTrainers.sort((a, b) => a.name.localeCompare(b.name))
      break
  }

  // 注意：实际应用中可能需要在store中实现排序逻辑
  // 这里只是前端排序的示例
})

// 格式化数字，添加千位分隔符
const formatNumber = (num: number): string => {
  return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
}

onMounted(async () => {
  if (trainers.value.length === 0) {
    await store.fetchTrainers(1)
  }
})
</script>

<template>
  <div class="home-view animate-fade">
    <!-- 欢迎区域 -->
    <section class="welcome-section">
      <!-- 装饰元素 -->
      <div class="decoration-circle decoration-circle-1"></div>
      <div class="decoration-circle decoration-circle-2"></div>
      <div class="decoration-dots"></div>

      <div class="welcome-content">
        <h1 class="welcome-title">GAME MOD MASTER</h1>

        <!-- 搜索框 -->
        <div class="search-container">
          <NInputGroup>
            <NInput
              v-model:value="searchQuery"
              placeholder="搜索游戏修改器..."
              clearable
              @keydown.enter="handleSearch"
            />
            <NButton type="primary" @click="handleSearch">
              <template #icon>
                <NIcon><Search /></NIcon>
              </template>
              搜索
            </NButton>
          </NInputGroup>
        </div>
      </div>
    </section>

    <!-- 统计卡片 -->
    <section class="stats-section">
      <NGrid cols="1 s:2 m:4" responsive="screen" :x-gap="16" :y-gap="16">
        <NGridItem v-for="(stat, index) in statistics" :key="index">
          <StatusCard
            :title="stat.title"
            :value="typeof stat.value === 'number' ? formatNumber(stat.value) : stat.value"
            :icon="stat.icon"
            :color="stat.color"
            :loading="isLoading"
          />
        </NGridItem>
      </NGrid>
    </section>

    <!-- 修改器部分标题和控制栏 -->
    <section class="trainers-section">
      <div class="section-header">
        <h2 class="section-title">
          {{ searchQuery ? `"${searchQuery}" 的搜索结果` : '热门修改器' }}
        </h2>

        <div class="controls">
          <NSelect v-model:value="sortBy" :options="sortOptions" size="small" style="width: 160px">
            <template #default>
              <div style="display: flex; align-items: center; gap: 4px">
                <NIcon><FilterOutline /></NIcon>
                <span>{{ sortOptions.find((o) => o.value === sortBy)?.label || '排序' }}</span>
              </div>
            </template>
          </NSelect>

          <NButton size="small" @click="handleRefresh" :loading="isLoading">
            <template #icon>
              <NIcon><RefreshOutline /></NIcon>
            </template>
            刷新
          </NButton>
        </div>
      </div>
    </section>

    <!-- 错误显示 -->
    <NCard v-if="errorMessage" class="error-card">
      <NText type="error">{{ errorMessage }}</NText>
    </NCard>

    <!-- 加载状态 -->
    <div v-else-if="isLoading" class="loading-container">
      <NSpin size="large" />
    </div>

    <!-- 空状态 -->
    <NEmpty
      v-else-if="trainers.length === 0"
      :description="searchQuery ? `没有找到与 '${searchQuery}' 相关的修改器` : '没有找到任何修改器'"
      class="empty-state"
    >
      <template #extra>
        <NButton @click="store.fetchTrainers(1)">浏览所有修改器</NButton>
      </template>
    </NEmpty>

    <!-- 修改器网格 -->
    <section v-else class="trainers-grid">
      <NGrid cols="1 s:2 m:3 l:4" responsive="screen" :x-gap="16" :y-gap="24">
        <NGridItem v-for="trainer in trainers" :key="trainer.id">
          <GameCard :trainer="trainer" />
        </NGridItem>
      </NGrid>
    </section>

    <!-- 分页 -->
    <div class="pagination-container" v-if="totalPages > 1">
      <NPagination
        v-model:page="store.currentPage"
        :page-count="totalPages"
        :page-sizes="[12, 24, 36]"
        show-size-picker
        @update:page="handlePageChange"
      />
    </div>
  </div>
</template>

<style scoped>
.home-view {
  width: 100%;
}

.welcome-section {
  background: white;
  padding: 4rem 2rem;
  border-radius: var(--radius-lg);
  margin-bottom: 2rem;
  box-shadow: var(--shadow-md);
  position: relative;
  overflow: hidden;
  z-index: 1;
}

:deep(.dark) .welcome-section,
.dark .welcome-section {
  background: #1e1e1e;
  box-shadow: 0 4px 20px 0 rgba(0, 0, 0, 0.2);
}

/* 装饰元素 - 点阵 */
.decoration-dots {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: radial-gradient(rgba(100, 100, 250, 0.1) 1px, transparent 1px);
  background-size: 20px 20px;
  opacity: 0.5;
}

/* 装饰元素 - 圆形 */
.decoration-circle {
  position: absolute;
  border-radius: 50%;
  filter: blur(60px);
  z-index: -1;
}

.decoration-circle-1 {
  width: 300px;
  height: 300px;
  background: rgba(89, 91, 255, 0.1);
  top: -100px;
  left: -100px;
  animation: float 15s ease-in-out infinite;
}

.decoration-circle-2 {
  width: 250px;
  height: 250px;
  background: rgba(89, 91, 255, 0.08);
  bottom: -80px;
  right: -60px;
  animation: float 18s ease-in-out infinite reverse;
}

@keyframes float {
  0% {
    transform: translateY(0) translateX(0);
  }
  50% {
    transform: translateY(-20px) translateX(20px);
  }
  100% {
    transform: translateY(0) translateX(0);
  }
}

.welcome-content {
  position: relative;
  z-index: 1;
  text-align: center;
  max-width: 800px;
  margin: 0 auto;
}

.welcome-title {
  margin: 0 0 2.5rem 0;
  color: #595bff;
  font-size: 3rem;
  font-weight: 800;
  letter-spacing: 2px;
  text-align: center;
  animation: fadeIn 1s ease-out;
}

:deep(.dark) .welcome-title,
.dark .welcome-title {
  color: #818cf8;
}

@keyframes fadeIn {
  0% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}

.welcome-subtitle {
  color: rgba(255, 255, 255, 0.8);
  font-size: 1.1rem;
  margin-bottom: 2rem;
  animation: fadeSlideIn 0.8s ease-out 0.2s backwards;
}

.search-container {
  max-width: 600px;
  margin: 0 auto;
  animation: fadeIn 1s ease-out 0.3s backwards;
}

.search-container :deep(.n-input) {
  border-color: #e5e7eb;
}

:deep(.dark) .search-container :deep(.n-input),
.dark .search-container :deep(.n-input) {
  border-color: #333;
}

.stats-section {
  margin-bottom: 2.5rem;
}

.trainers-section {
  margin-bottom: 1.5rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.controls {
  display: flex;
  gap: 8px;
  align-items: center;
}

.trainers-grid {
  margin-bottom: 2rem;
}

.error-card {
  margin-bottom: 2rem;
}

.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 300px;
}

.empty-state {
  padding: 3rem 0;
}

.pagination-container {
  display: flex;
  justify-content: center;
  margin: 2rem 0;
}

@media (max-width: 640px) {
  .welcome-section {
    padding: 2rem 1rem;
  }

  .welcome-title {
    font-size: 1.8rem;
  }

  .section-header {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
