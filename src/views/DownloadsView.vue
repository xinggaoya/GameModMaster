<template>
  <div class="modern-downloads-view">
    <!-- 顶部标题区域 -->
    <div class="downloads-header">
      <div class="header-content">
        <div class="header-info">
          <div class="title-section">
            <h1 class="page-title">我的修改器收藏</h1>
            <p class="page-subtitle">管理和启动您已下载的游戏修改器</p>
          </div>
        </div>

        <!-- 快速统计 -->
        <div class="quick-stats">
          <div class="stat-item">
            <div class="stat-icon-wrapper">
              <NIcon class="stat-icon">
                <DownloadOutline />
              </NIcon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ store.downloadedTrainers.length }}</div>
              <div class="stat-label">已下载</div>
            </div>
          </div>

          <div class="stat-item">
            <div class="stat-icon-wrapper">
              <NIcon class="stat-icon">
                <GameControllerOutline />
              </NIcon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ store.installedTrainers.length }}</div>
              <div class="stat-label">已安装</div>
            </div>
          </div>

          <div class="stat-item">
            <div class="stat-icon-wrapper">
              <NIcon class="stat-icon">
                <PlayOutline />
              </NIcon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ getRecentlyUsedCount() }}</div>
              <div class="stat-label">最近使用</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和控制区域 -->
    <div class="search-control-section">
      <div class="search-card">
        <div class="search-content">
          <div class="search-area">
            <NInput
              v-model:value="searchQuery"
              placeholder="搜索已下载的修改器..."
              size="large"
              class="search-input"
              @keydown.enter="handleSearch"
              clearable
            >
              <template #prefix>
                <NIcon class="search-icon">
                  <SearchOutline />
                </NIcon>
              </template>
            </NInput>

            <NSelect
              v-model:value="sortBy"
              :options="sortOptions"
              size="large"
              class="sort-select"
              placeholder="排序方式"
            >
              <template #render-label="{ option }">
                <div class="sort-option">
                  <NIcon class="sort-option-icon">
                    <component :is="option.icon" />
                  </NIcon>
                  <div class="sort-option-content">
                    <div class="sort-option-label">{{ option.label }}</div>
                  </div>
                </div>
              </template>
            </NSelect>

            <NButton
              size="large"
              class="refresh-button"
              @click="refreshData"
              :loading="store.isLoading"
            >
              <template #icon>
                <NIcon><RefreshOutline /></NIcon>
              </template>
              刷新
            </NButton>
          </div>

          <!-- 批量操作 -->
          <div class="batch-actions" v-if="filteredTrainers.length > 0">
            <NButton size="large" quaternary @click="launchAll">
              <template #icon>
                <NIcon><PlayOutline /></NIcon>
              </template>
              全部启动
            </NButton>

            <NButton size="large" quaternary @click="exportList">
              <template #icon>
                <NIcon><DownloadOutline /></NIcon>
              </template>
              导出列表
            </NButton>
          </div>
        </div>
      </div>
    </div>

    <!-- 加载状态 -->
    <div v-if="store.isLoading" class="loading-container">
      <div class="loading-content">
        <NSpin size="large" />
        <p class="loading-text">正在加载修改器列表...</p>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else-if="!filteredTrainers.length" class="empty-state">
      <div class="empty-card">
        <div class="empty-content">
          <div class="empty-icon">
            <NIcon size="64">
              <DownloadOutline />
            </NIcon>
          </div>
          <h3 class="empty-title">
            {{ searchQuery ? `没有找到 "${searchQuery}" 相关的修改器` : '还没有下载任何修改器' }}
          </h3>
          <p class="empty-subtitle">
            {{ searchQuery ? '尝试使用不同的关键词搜索' : '去浏览页面下载您需要的修改器吧' }}
          </p>
          <div class="empty-actions">
            <NButton @click="router.push('/')" type="primary" size="large">
              <template #icon>
                <NIcon><GameControllerOutline /></NIcon>
              </template>
              浏览修改器
            </NButton>
            <NButton v-if="searchQuery" @click="clearSearch" size="large">
              清除搜索
            </NButton>
          </div>
        </div>
      </div>
    </div>

    <!-- 修改器网格 -->
    <div v-else class="trainers-section">
      <!-- 结果统计 -->
      <div class="results-info">
        <p class="results-text">
          显示 {{ filteredTrainers.length }} 个修改器
          <span v-if="searchQuery">，搜索关键词： "{{searchQuery}}"</span>
        </p>
      </div>

      <!-- 网格布局 -->
      <div class="trainers-grid">
        <NGrid
          cols="1 s:2 m:3 l:4 xl:5"
          responsive="screen"
          :x-gap="20"
          :y-gap="24"
        >
          <NGridItem
            v-for="trainer in filteredTrainers"
            :key="trainer.id"
            class="trainer-item"
          >
            <div class="trainer-card-wrapper">
              <GameCard :trainer="trainer" showButtons="downloaded" />
            </div>
          </NGridItem>
        </NGrid>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  PlayOutline,
  TrashOutline,
  SearchOutline,
  FilterOutline,
  ChevronDownOutline,
  DownloadOutline,
  TimeOutline,
  RefreshOutline,
  GameControllerOutline,
  SparklesOutline,
  FolderOpenOutline,
} from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import GameCard from '@/components/common/GameCard.vue'
import type { Trainer } from '@/types'
import { useMessage, useDialog } from 'naive-ui'

const router = useRouter()
const store = useTrainerStore()
const message = useMessage()
const dialog = useDialog()

// 状态管理
const searchQuery = ref('')
const sortBy = ref('installTime')

// 排序选项
const sortOptions = [
  {
    label: '安装时间',
    value: 'installTime',
    icon: TimeOutline,
  },
  {
    label: '使用时间',
    value: 'lastUsed',
    icon: PlayOutline,
  },
  {
    label: '名称排序',
    value: 'name',
    icon: SearchOutline,
  },
  {
    label: '游戏版本',
    value: 'gameVersion',
    icon: GameControllerOutline,
  },
]

// 过滤和排序后的修改器列表
const filteredTrainers = computed(() => {
  let result = [...store.downloadedTrainers]

  // 搜索过滤
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(
      (trainer) =>
        trainer.name.toLowerCase().includes(query) ||
        trainer.game_version.toLowerCase().includes(query) ||
        trainer.description?.toLowerCase().includes(query)
    )
  }

  // 排序
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
    case 'gameVersion':
      result.sort((a, b) => a.game_version.localeCompare(b.game_version))
      break
  }

  return result
})

// 获取最近使用的修改器数量
const getRecentlyUsedCount = () => {
  const sevenDaysAgo = new Date()
  sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 7)

  return store.downloadedTrainers.filter(trainer => {
    if (!trainer.last_launch_time) return false
    return new Date(trainer.last_launch_time) > sevenDaysAgo
  }).length
}

// 搜索处理
const handleSearch = () => {
  // 搜索通过计算属性自动处理
}

// 清除搜索
const clearSearch = () => {
  searchQuery.value = ''
}

// 刷新数据
const refreshData = async () => {
  try {
    await store.initialize()
    message.success('数据已刷新')
  } catch (error) {
    console.error('刷新失败:', error)
    message.error('刷新失败，请稍后再试')
  }
}

// 批量启动
const launchAll = async () => {
  if (filteredTrainers.value.length === 0) {
    message.warning('没有可启动的修改器')
    return
  }

  dialog.info({
    title: '批量启动',
    content: `确定要启动所有 ${filteredTrainers.value.length} 个修改器吗？`,
    positiveText: '确定启动',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        message.loading('正在批量启动修改器...')

        // 并行启动所有修改器
        const launchPromises = filteredTrainers.value.map(trainer =>
          store.launchTrainer(trainer.id).catch(error => {
            console.error(`启动 ${trainer.name} 失败:`, error)
            return { trainer, error }
          })
        )

        const results = await Promise.all(launchPromises)
        const failures = results.filter((result: any) => result.error)

        if (failures.length === 0) {
          message.success(`成功启动 ${filteredTrainers.value.length} 个修改器`)
        } else {
          message.warning(`成功启动 ${filteredTrainers.value.length - failures.length} 个修改器，${failures.length} 个启动失败`)
        }
      } catch (error) {
        console.error('批量启动失败:', error)
        message.error('批量启动失败，请稍后再试')
      }
    },
  })
}

// 导出列表
const exportList = () => {
  if (filteredTrainers.value.length === 0) {
    message.warning('没有可导出的修改器')
    return
  }

  try {
    const csvContent = [
      ['名称', '版本', '游戏版本', '安装时间', '最后使用时间'],
      ...filteredTrainers.value.map(trainer => [
        trainer.name,
        trainer.version,
        trainer.game_version,
        trainer.install_time || '未知',
        trainer.last_launch_time || '从未使用'
      ])
    ].map(row => row.join(',')).join('\n')

    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    const url = URL.createObjectURL(blob)
    link.setAttribute('href', url)
    link.setAttribute('download', `修改器列表_${new Date().toLocaleDateString()}.csv`)
    link.style.visibility = 'hidden'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)

    message.success('列表已导出')
  } catch (error) {
    console.error('导出失败:', error)
    message.error('导出失败，请稍后再试')
  }
}

// 组件挂载时初始化
onMounted(() => {
  if (store.downloadedTrainers.length === 0) {
    store.initialize()
  }
})
</script>

<style scoped>
/* 现代化下载页布局 */
.modern-downloads-view {
  width: 100%;
  min-height: 100vh;
  background: transparent;
}

/* 顶部标题区域 */
.downloads-header {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 24px;
  padding: 2rem;
  margin-bottom: 2rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  position: relative;
  overflow: hidden;
}

.downloads-header::before {
  content: '';
  position: absolute;
  top: 0;
  right: 0;
  width: 300px;
  height: 300px;
  background: radial-gradient(circle, rgba(124, 58, 237, 0.1) 0%, transparent 70%);
  border-radius: 50%;
  transform: translate(50%, -50%);
}

.header-content {
  position: relative;
  z-index: 1;
  max-width: 1400px;
  margin: 0 auto;
}

.title-section {
  margin-bottom: 2rem;
  text-align: center;
}

.page-title {
  margin: 0 0 0.75rem 0;
  font-size: 2.5rem;
  font-weight: 800;
  background: linear-gradient(135deg, #7c3aed 0%, #0891b2 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  line-height: 1.2;
}

.page-subtitle {
  margin: 0;
  font-size: 1.125rem;
  color: #6b7280;
  font-weight: 500;
}

/* 快速统计 */
.quick-stats {
  display: flex;
  justify-content: center;
  gap: 2rem;
  flex-wrap: wrap;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem 2rem;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 20px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  min-width: 160px;
}

.stat-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.stat-icon-wrapper {
  width: 56px;
  height: 56px;
  border-radius: 18px;
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 4px 16px rgba(124, 58, 237, 0.3);
  transition: transform 0.3s ease;
}

.stat-item:hover .stat-icon-wrapper {
  transform: scale(1.1) rotate(5deg);
}

.stat-icon {
  font-size: 1.5rem;
}

.stat-content {
  text-align: center;
}

.stat-value {
  font-size: 1.75rem;
  font-weight: 800;
  color: #1f2937;
  line-height: 1;
  margin-bottom: 0.25rem;
}

.stat-label {
  font-size: 0.875rem;
  color: #6b7280;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
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
  border-radius: 24px;
  padding: 2rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.search-content {
  max-width: 1400px;
  margin: 0 auto;
}

.search-area {
  display: flex;
  gap: 1rem;
  align-items: center;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.search-input {
  flex: 1;
  min-width: 300px;
  background: rgba(255, 255, 255, 0.9);
  border: 2px solid rgba(0, 0, 0, 0.1);
  border-radius: 16px !important;
  transition: all 0.3s ease;
}

.search-input:focus-within {
  border-color: #7c3aed;
  box-shadow: 0 0 0 4px rgba(124, 58, 237, 0.1);
  transform: translateY(-2px);
}

.search-icon {
  color: #9ca3af;
  transition: color 0.3s ease;
}

.search-input:focus-within .search-icon {
  color: #7c3aed;
}

.sort-select {
  min-width: 180px;
}

.sort-option {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem;
}

.sort-option-icon {
  font-size: 1.125rem;
  color: #7c3aed;
}

.sort-option-content {
  flex: 1;
}

.sort-option-label {
  font-weight: 600;
  color: #1f2937;
}

.refresh-button {
  border-radius: 16px;
  padding: 0 1.5rem;
  font-weight: 600;
  transition: all 0.3s ease;
}

.refresh-button:hover {
  transform: translateY(-2px);
}

/* 批量操作 */
.batch-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
}

.batch-actions .n-button {
  border-radius: 16px;
  padding: 0 1.5rem;
  font-weight: 600;
  transition: all 0.3s ease;
}

.batch-actions .n-button:hover {
  transform: translateY(-2px);
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

/* 空状态 */
.empty-state {
  display: flex;
  justify-content: center;
  padding: 2rem;
}

.empty-card {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 24px;
  padding: 4rem 2rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  max-width: 500px;
  width: 100%;
  text-align: center;
}

.empty-content {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.empty-icon {
  margin-bottom: 2rem;
  padding: 2rem;
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

/* 修改器区域 */
.trainers-section {
  max-width: 1400px;
  margin: 0 auto;
}

.results-info {
  margin-bottom: 2rem;
  text-align: center;
}

.results-text {
  font-size: 1rem;
  color: #6b7280;
  font-weight: 500;
  margin: 0;
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

.trainer-item {
  animation: fadeInUp 0.6s ease-out backwards;
}

.trainer-item:nth-child(1) { animation-delay: 0.1s; }
.trainer-item:nth-child(2) { animation-delay: 0.2s; }
.trainer-item:nth-child(3) { animation-delay: 0.3s; }
.trainer-item:nth-child(4) { animation-delay: 0.4s; }
.trainer-item:nth-child(5) { animation-delay: 0.5s; }

.trainer-card-wrapper {
  height: 100%;
  transition: transform 0.3s ease;
}

.trainer-card-wrapper:hover {
  transform: translateY(-4px);
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .search-area {
    flex-direction: column;
    align-items: stretch;
  }

  .search-input {
    min-width: auto;
  }

  .sort-select {
    width: 100%;
  }

  .batch-actions {
    flex-direction: column;
    align-items: stretch;
  }
}

@media (max-width: 768px) {
  .downloads-header {
    padding: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .page-title {
    font-size: 2rem;
  }

  .quick-stats {
    gap: 1rem;
  }

  .stat-item {
    padding: 1rem 1.5rem;
    min-width: 140px;
  }

  .stat-value {
    font-size: 1.5rem;
  }

  .search-card {
    padding: 1.5rem;
  }

  .empty-card {
    padding: 2rem 1.5rem;
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

@media (max-width: 480px) {
  .quick-stats {
    flex-direction: column;
    align-items: center;
  }

  .stat-item {
    width: 100%;
    max-width: 300px;
  }

  .page-title {
    font-size: 1.75rem;
  }
}

/* 暗色主题 */
:deep(.dark-theme) .downloads-header,
:deep(.dark-theme) .search-card,
:deep(.dark-theme) .empty-card {
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .page-title {
  background: linear-gradient(135deg, #a78bfa 0%, #67e8f9 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

:deep(.dark-theme) .page-subtitle,
:deep(.dark-theme) .loading-text,
:deep(.dark-theme) .results-text {
  color: #94a3b8;
}

:deep(.dark-theme) .stat-item {
  background: rgba(30, 41, 59, 0.8);
  border-color: rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .stat-value {
  color: #e2e8f0;
}

:deep(.dark-theme) .stat-label {
  color: #94a3b8;
}

:deep(.dark-theme) .search-input {
  background: rgba(30, 41, 59, 0.9);
  border-color: rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .sort-option-label {
  color: #e2e8f0;
}

:deep(.dark-theme) .empty-title {
  color: #e2e8f0;
}

:deep(.dark-theme) .empty-subtitle {
  color: #94a3b8;
}
</style>
