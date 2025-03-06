<template>
  <div class="downloads-view animate-fade">
    <section class="header-section">
      <div class="title-section">
        <h1 class="page-title">我的修改器收藏</h1>
        <p class="page-subtitle">管理您已下载的游戏修改器</p>
      </div>

      <div class="controls-section">
        <NInputGroup class="search-box">
          <NInput
            v-model:value="searchQuery"
            placeholder="搜索已下载修改器..."
            clearable
            @keydown.enter="handleSearch"
          >
            <template #prefix>
              <NIcon><SearchOutline /></NIcon>
            </template>
          </NInput>
        </NInputGroup>

        <NDropdown
          trigger="click"
          :options="sortOptions"
          @select="(key: string) => (sortBy = key)"
          :value="sortBy"
        >
          <NButton class="sort-button">
            <template #icon>
              <NIcon><FilterOutline /></NIcon>
            </template>
            {{ sortOptions.find((o) => o.key === sortBy)?.label || '排序' }}
            <NIcon style="margin-left: 4px">
              <ChevronDownOutline />
            </NIcon>
          </NButton>
        </NDropdown>
      </div>
    </section>

    <NDivider />

    <section class="stats-section" v-if="filteredTrainers.length > 0">
      <NStatistic class="statistic-item">
        <template #label>
          <div class="statistic-label">
            <NIcon class="statistic-icon"><DownloadOutline /></NIcon>
            <span>已下载修改器</span>
          </div>
        </template>
        {{ store.downloadedTrainers.length }} 个
      </NStatistic>

      <NStatistic class="statistic-item">
        <template #label>
          <div class="statistic-label">
            <NIcon class="statistic-icon"><SearchOutline /></NIcon>
            <span>当前显示</span>
          </div>
        </template>
        {{ filteredTrainers.length }} 个
      </NStatistic>
    </section>

    <div v-if="store.isLoading" class="loading-container">
      <NSpin size="large" />
    </div>

    <NEmpty
      v-else-if="!filteredTrainers.length"
      class="empty-state"
      :description="searchQuery ? `没有找到与 '${searchQuery}' 相关的修改器` : '没有已下载的修改器'"
    >
      <template #extra>
        <NSpace>
          <NButton @click="router.push('/')">浏览修改器</NButton>
          <NButton v-if="searchQuery" @click="clearSearch">清除搜索</NButton>
        </NSpace>
      </template>
    </NEmpty>

    <div v-else class="grid-view">
      <NGrid cols="1 s:2 m:3 l:4" responsive="screen" :x-gap="16" :y-gap="24">
        <NGridItem v-for="trainer in filteredTrainers" :key="trainer.id">
          <GameCard :trainer="trainer" showButtons="downloaded" />
        </NGridItem>
      </NGrid>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  PlayOutline,
  TrashOutline,
  InformationCircleOutline,
  SearchOutline,
  FilterOutline,
  ChevronDownOutline,
  DownloadOutline,
  TimeOutline,
} from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import GameCard from '@/components/common/GameCard.vue'
import type { Trainer } from '@/types'
import { useMessage, useDialog, NButton, NIcon } from 'naive-ui'

const router = useRouter()
const store = useTrainerStore()
const message = useMessage()
const dialog = useDialog()

// 状态
const searchQuery = ref('')
const sortBy = ref('installTime') // 默认按安装时间排序

// 排序选项
const sortOptions = [
  {
    label: '安装时间',
    key: 'installTime',
    icon: TimeOutline,
  },
  {
    label: '使用时间',
    key: 'lastUsed',
    icon: TimeOutline,
  },
  {
    label: '名称',
    key: 'name',
    icon: SearchOutline,
  },
]

const filteredTrainers = computed(() => {
  let result = [...store.downloadedTrainers]

  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(
      (trainer) =>
        trainer.name.toLowerCase().includes(query) ||
        trainer.game_version.toLowerCase().includes(query),
    )
  }

  switch (sortBy.value) {
    case 'installTime':
      result.sort((a, b) => {
        const timeA = a.install_time ? new Date(a.install_time).getTime() : 0
        const timeB = b.install_time ? new Date(b.install_time).getTime() : 0
        return timeB - timeA
      })
      break
    case 'lastUsed':
      result.sort((a, b) => {
        const timeA = a.last_launch_time ? new Date(a.last_launch_time).getTime() : 0
        const timeB = b.last_launch_time ? new Date(b.last_launch_time).getTime() : 0
        return timeB - timeA
      })
      break
    case 'name':
      result.sort((a, b) => a.name.localeCompare(b.name))
      break
  }

  return result
})

const handleSearch = () => {
  // 搜索是通过 filteredTrainers 计算属性自动处理的
}

const clearSearch = () => {
  searchQuery.value = ''
}

const viewDetails = (id: string) => {
  router.push(`/detail/${id}`)
}

const handleDelete = async (trainer: Trainer) => {
  dialog.warning({
    title: '确认删除',
    content: `确定要删除修改器"${trainer.name}"吗？`,
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await store.deleteTrainer(trainer.id)
        message.success('修改器已删除')
      } catch (error) {
        console.error('删除失败:', error)
        message.error(error instanceof Error ? error.message : '删除失败')
      }
    },
  })
}

const handleLaunch = async (trainer: Trainer) => {
  try {
    message.loading('正在启动修改器...')
    await store.launchTrainer(trainer.id)
    message.success('修改器启动成功')
  } catch (error) {
    console.error('启动失败:', error)
    message.error(error instanceof Error ? error.message : '启动失败')
  }
}

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

onMounted(() => {
  if (store.downloadedTrainers.length === 0) {
    store.initialize()
  }
})
</script>

<style scoped>
.downloads-view {
  padding-bottom: 2rem;
}

.header-section {
  margin-bottom: 1.5rem;
}

.title-section {
  margin-bottom: 1.5rem;
}

.page-title {
  font-size: 1.75rem;
  font-weight: 700;
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.page-subtitle {
  font-size: 1rem;
  color: var(--text-secondary);
  margin: 0;
}

.controls-section {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 1rem;
}

.search-box {
  flex: 1;
  min-width: 240px;
}

.sort-button {
  min-width: 120px;
  display: flex;
  align-items: center;
}

.stats-section {
  display: flex;
  gap: 2rem;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.statistic-item {
  background-color: var(--card-bg-subtle);
  padding: 1rem;
  border-radius: var(--radius-md);
  flex: 1;
  min-width: 200px;
}

.statistic-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.statistic-icon {
  color: var(--primary);
}

.grid-view {
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

@media (max-width: 640px) {
  .controls-section {
    flex-direction: column;
  }

  .search-box {
    width: 100%;
  }

  .page-title {
    font-size: 1.5rem;
  }
}
</style>
