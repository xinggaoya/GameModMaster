<template>
  <div class="downloads-view animate-fade">
    <section class="header-section">
      <div class="title-section">
        <h1 class="page-title">我的修改器收藏</h1>
        <p class="page-subtitle">管理您已下载的游戏修改器</p>
      </div>

      <div class="controls-section">
        <NInputGroup>
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

          <NDropdown :options="sortOptions" v-model:value="sortBy">
            <NButton>
              排序
              <template #icon>
                <NIcon><FilterOutline /></NIcon>
              </template>
            </NButton>
          </NDropdown>

          <NButton @click="toggleViewMode('grid')" :quaternary="viewMode !== 'grid'">
            <template #icon>
              <NIcon><AppsOutline /></NIcon>
            </template>
          </NButton>

          <NButton @click="toggleViewMode('list')" :quaternary="viewMode !== 'list'">
            <template #icon>
              <NIcon><ListOutline /></NIcon>
            </template>
          </NButton>
        </NInputGroup>
      </div>
    </section>

    <NDivider />

    <!-- 加载状态 -->
    <div v-if="store.isLoading" class="loading-container">
      <NSpin size="large" />
    </div>

    <!-- 空状态 -->
    <NEmpty
      v-else-if="!filteredTrainers.length"
      class="empty-state"
      description="没有找到已下载的修改器"
    >
      <template #extra>
        <NButton @click="router.push('/')">浏览修改器</NButton>
      </template>
    </NEmpty>

    <!-- 网格视图 -->
    <div v-else-if="viewMode === 'grid'" class="grid-view">
      <NGrid cols="1 s:2 m:3 l:4" responsive="screen" :x-gap="16" :y-gap="24">
        <NGridItem v-for="trainer in filteredTrainers" :key="trainer.id">
          <GameCard :trainer="trainer" showButtons="downloaded" />
        </NGridItem>
      </NGrid>
    </div>

    <!-- 列表视图 -->
    <div v-else class="list-view">
      <NDataTable
        :columns="columns"
        :data="filteredTrainers"
        :pagination="{ pageSize: 10 }"
        :bordered="false"
        striped
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, h } from 'vue'
import { useRouter } from 'vue-router'
import {
  PlayOutline,
  TrashOutline,
  InformationCircleOutline,
  SearchOutline,
  FilterOutline,
  AppsOutline,
  ListOutline,
} from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import GameCard from '@/components/common/GameCard.vue'
import type { Trainer } from '../types'

const router = useRouter()
const store = useTrainerStore()

// 视图模式
const viewMode = ref('grid') // 'grid' 或 'list'

// 搜索与筛选
const searchQuery = ref('')
const sortBy = ref('installTime')

// 排序选项
const sortOptions = [
  { label: '安装时间', value: 'installTime' },
  { label: '上次启动', value: 'lastLaunch' },
  { label: '名称', value: 'name' },
]

// 当前展示的训练器列表
const filteredTrainers = computed(() => {
  let result = [...store.downloadedTrainers]

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(
      (trainer) =>
        trainer.name.toLowerCase().includes(query) ||
        trainer.game_version.toLowerCase().includes(query),
    )
  }

  // 排序
  if (sortBy.value === 'installTime') {
    result.sort((a, b) => {
      const timeA = new Date(a.install_time || 0).getTime()
      const timeB = new Date(b.install_time || 0).getTime()
      return timeB - timeA // 降序，最新安装的排前面
    })
  } else if (sortBy.value === 'lastLaunch') {
    result.sort((a, b) => {
      const timeA = a.last_launch_time ? new Date(a.last_launch_time).getTime() : 0
      const timeB = b.last_launch_time ? new Date(b.last_launch_time).getTime() : 0
      return timeB - timeA
    })
  } else if (sortBy.value === 'name') {
    result.sort((a, b) => a.name.localeCompare(b.name))
  }

  return result
})

// 表格列定义
const columns = [
  {
    title: '修改器',
    key: 'name',
    render(row: Trainer) {
      return h(
        NSpace,
        { align: 'center' },
        {
          default: () => [
            h(NAvatar, {
              src: row.thumbnail,
              round: false,
              objectFit: 'cover',
              size: 'small',
            }),
            h('div', { style: 'display: flex; flex-direction: column;' }, [
              h('div', { style: 'font-weight: 600;' }, row.name),
              h(
                'div',
                { style: 'font-size: 12px; color: var(--text-tertiary);' },
                `v${row.version}`,
              ),
            ]),
          ],
        },
      )
    },
  },
  {
    title: '游戏版本',
    key: 'game_version',
    render(row: Trainer) {
      return h(NTag, { size: 'small', round: true }, { default: () => row.game_version })
    },
  },
  {
    title: '安装时间',
    key: 'install_time',
    render(row: Trainer) {
      return row.install_time ? formatDate(row.install_time) : '未知'
    },
  },
  {
    title: '最后使用',
    key: 'last_launch_time',
    render(row: Trainer) {
      return row.last_launch_time ? formatDate(row.last_launch_time) : '未使用'
    },
  },
  {
    title: '操作',
    key: 'actions',
    render(row: Trainer) {
      return h(
        NSpace,
        { justify: 'end' },
        {
          default: () => [
            h(
              NButton,
              {
                quaternary: true,
                size: 'small',
                onClick: () => router.push(`/detail/${row.id}`),
              },
              { icon: () => h(NIcon, null, { default: () => h(InformationCircleOutline) }) },
            ),

            h(
              NButton,
              {
                quaternary: true,
                size: 'small',
                type: 'success',
                onClick: () => handleLaunch(row),
              },
              { icon: () => h(NIcon, null, { default: () => h(PlayOutline) }) },
            ),

            h(
              NButton,
              {
                quaternary: true,
                size: 'small',
                type: 'error',
                onClick: () => handleDelete(row),
              },
              { icon: () => h(NIcon, null, { default: () => h(TrashOutline) }) },
            ),
          ],
        },
      )
    },
  },
]

// 格式化日期
const formatDate = (dateString: string | undefined | null) => {
  if (!dateString) return '未知'
  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

// 获取已安装训练器信息
const getInstallInfo = (id: string) => {
  return store.installedTrainers.find((t) => t.id === id)
}

// 启动训练器
const handleLaunch = async (trainer: Trainer) => {
  try {
    await store.launchTrainer(trainer.id)
  } catch (error) {
    console.error('启动失败:', error)
  }
}

// 删除训练器
const handleDelete = async (trainer: Trainer) => {
  if (confirm(`确定要删除修改器"${trainer.name}"吗？`)) {
    try {
      await store.deleteTrainer(trainer.id)
    } catch (error) {
      console.error('删除失败:', error)
    }
  }
}

// 搜索处理
const handleSearch = () => {
  // 搜索通过计算属性处理
}

// 切换视图模式
const toggleViewMode = (mode: string) => {
  viewMode.value = mode
}

onMounted(async () => {
  if (store.downloadedTrainers.length === 0) {
    // 加载数据
    await store.initialize()
  }
})
</script>

<style scoped>
.downloads-view {
  width: 100%;
}

.header-section {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.title-section {
  flex: 1;
  min-width: 250px;
}

.page-title {
  font-size: 1.75rem;
  font-weight: 600;
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.page-subtitle {
  color: var(--text-tertiary);
  font-size: 0.95rem;
  margin: 0;
}

.controls-section {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.loading-container,
.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 200px;
  margin: 2rem 0;
}

.grid-view,
.list-view {
  margin-top: 1.5rem;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .header-section {
    flex-direction: column;
    align-items: stretch;
  }

  .controls-section {
    width: 100%;
  }

  .page-title {
    font-size: 1.5rem;
  }
}
</style>
