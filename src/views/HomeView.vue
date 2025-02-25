<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  Search,
  GameControllerOutline,
  DownloadOutline,
  StatsChartOutline,
  PeopleOutline,
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
} from 'naive-ui'

// 导入自定义组件
import GameCard from '@/components/common/GameCard.vue'
import StatusCard from '@/components/common/StatusCard.vue'

const router = useRouter()
const store = useTrainerStore()

const searchQuery = ref('')
const isLoading = computed(() => store.isLoading)
const totalPages = computed(() => store.totalPages)
const trainers = computed(() => store.trainers)

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

const handleSearch = () => {
  store.currentPage = 1 // 搜索时重置页码
  store.searchTrainers(searchQuery.value)
}

const handlePageChange = async (page: number) => {
  try {
    store.currentPage = page
    await store.fetchTrainers(page)
    window.scrollTo({ top: 0, behavior: 'smooth' }) // 滚动到顶部
  } catch (error) {
    // 错误处理由store处理
  }
}

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
      <div class="welcome-content">
        <h1 class="welcome-title">
          <span class="game-title">Game Mod Master</span>
        </h1>
        <p class="welcome-subtitle">一站式游戏修改器管理平台</p>

        <!-- 搜索框 -->
        <div class="search-container">
          <NInputGroup>
            <NInput
              v-model:value="searchQuery"
              placeholder="搜索游戏修改器..."
              clearable
              @keydown.enter="handleSearch"
            />
            <NButton type="primary" ghost @click="handleSearch">
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

    <!-- 修改器部分标题 -->
    <section class="trainers-section">
      <h2 class="section-title">热门修改器</h2>
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
    <NEmpty v-else-if="trainers.length === 0" description="没有找到任何修改器" class="empty-state">
      <template #extra>
        <NButton @click="store.fetchTrainers(1)">刷新</NButton>
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
  background: linear-gradient(120deg, var(--primary) 0%, var(--secondary-dark) 100%);
  padding: 3rem 2rem;
  border-radius: var(--radius-lg);
  margin-bottom: 2rem;
  box-shadow: var(--shadow-md);
  position: relative;
  overflow: hidden;
}

.welcome-section::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: url('/src/assets/wallhaven-p9pgjp.jpg');
  background-size: cover;
  opacity: 0.1;
  z-index: 0;
}

.welcome-content {
  position: relative;
  z-index: 1;
  text-align: center;
  max-width: 800px;
  margin: 0 auto;
}

.welcome-title {
  font-size: 2.5rem;
  margin-bottom: 0.5rem;
  color: white;
}

.welcome-subtitle {
  font-size: 1.2rem;
  margin-bottom: 2rem;
  color: rgba(255, 255, 255, 0.9);
}

.search-container {
  max-width: 600px;
  margin: 0 auto;
}

.stats-section {
  margin-bottom: 2rem;
}

.trainers-section {
  margin-bottom: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.trainers-grid {
  margin-bottom: 2rem;
}

.error-card {
  margin-bottom: 2rem;
}

.loading-container,
.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 200px;
  margin-bottom: 2rem;
}

.pagination-container {
  display: flex;
  justify-content: center;
  margin: 2rem 0;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .welcome-section {
    padding: 2rem 1rem;
  }

  .welcome-title {
    font-size: 2rem;
  }

  .welcome-subtitle {
    font-size: 1rem;
  }

  .section-title {
    font-size: 1.25rem;
  }
}
</style>
