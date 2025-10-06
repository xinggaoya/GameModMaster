<template>
  <div class="modern-detail-view">
    <!-- 顶部导航区域 -->
    <div class="detail-header">
      <div class="header-content">
        <NButton
          @click="router.back()"
          circle
          size="large"
          class="back-button"
        >
          <template #icon>
            <NIcon><ArrowBackOutline /></NIcon>
          </template>
        </NButton>

        <div class="header-info" v-if="trainer">
          <h1 class="trainer-title">{{ trainer.name }}</h1>
          <div class="trainer-meta">
            <NRate :value="4.8" readonly size="small" />
            <span class="meta-text">4.8 分</span>
            <span class="meta-separator">•</span>
            <span class="meta-text">{{ formatNumber(trainer.download_count) }} 次下载</span>
          </div>
        </div>

        <div class="header-actions">
          <NButton
            circle
            size="large"
            quaternary
            @click="toggleFavorite"
            :class="{ active: isFavorite }"
          >
            <template #icon>
              <NIcon><HeartOutline /></NIcon>
            </template>
          </NButton>

          <NButton
            circle
            size="large"
            quaternary
            @click="handleShare"
          >
            <template #icon>
              <NIcon><ShareOutline /></NIcon>
            </template>
          </NButton>
        </div>
      </div>
    </div>

    <!-- 主内容区域 -->
    <div class="detail-content">
      <NSpin :show="loading" size="large">
        <template v-if="trainer">
          <div class="detail-layout">
            <!-- 左侧：封面和主要操作 -->
            <div class="cover-section">
              <!-- 封面卡片 -->
              <div class="cover-card">
                <div class="cover-wrapper">
                  <NImage
                    :src="trainer.thumbnail"
                    :alt="trainer.name"
                    class="cover-image"
                    width="100%"
                    :preview-disabled="true"
                    fallback-src="/placeholder.png"
                    object-fit="cover"
                  />
                  <div class="cover-overlay">
                    <div class="status-badge">
                      <NTag
                        :type="isDownloaded ? 'success' : 'warning'"
                        size="large"
                        round
                      >
                        {{ isDownloaded ? '已下载' : '未下载' }}
                      </NTag>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 操作按钮区域 -->
              <div class="action-area">
                <!-- 下载状态 -->
                <div class="download-area">
                  <NButton
                    v-if="!isDownloading && !isDownloaded"
                    type="primary"
                    size="large"
                    block
                    class="download-button"
                    @click="handleDownload"
                  >
                    <template #icon>
                      <NIcon><DownloadOutline /></NIcon>
                    </template>
                    下载修改器
                  </NButton>

                  <div v-else-if="isDownloading" class="progress-area">
                    <NProgress
                      :percentage="downloadProgress"
                      type="line"
                      :height="8"
                      :show-indicator="false"
                      processing
                      class="download-progress"
                    />
                    <div class="progress-text">
                      <span class="progress-percentage">{{ Math.round(downloadProgress) }}%</span>
                      <span class="progress-label">正在下载...</span>
                    </div>
                  </div>

                  <div v-else-if="isDownloaded" class="downloaded-actions">
                    <NButton
                      type="success"
                      size="large"
                      block
                      class="launch-button"
                      @click="handleLaunch"
                    >
                      <template #icon>
                        <NIcon><PlayOutline /></NIcon>
                      </template>
                      启动修改器
                    </NButton>

                    <NButton
                      quaternary
                      size="large"
                      block
                      class="delete-button"
                      @click="handleDelete"
                    >
                      <template #icon>
                        <NIcon><TrashOutline /></NIcon>
                      </template>
                      删除
                    </NButton>
                  </div>
                </div>

                <!-- 快速统计 -->
                <div class="quick-stats">
                  <div class="stat-item">
                    <NIcon class="stat-icon" size="16">
                      <StarOutline />
                    </NIcon>
                    <span class="stat-value">4.8</span>
                  </div>
                  <div class="stat-separator"></div>
                  <div class="stat-item">
                    <NIcon class="stat-icon" size="16">
                      <DownloadOutline />
                    </NIcon>
                    <span class="stat-value">{{ formatNumber(trainer.download_count) }}</span>
                  </div>
                  <div class="stat-separator"></div>
                  <div class="stat-item">
                    <NIcon class="stat-icon" size="16">
                      <TimeOutline />
                    </NIcon>
                    <span class="stat-value">{{ formatDate(trainer.last_update) }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 右侧：详细信息 -->
            <div class="info-section">
              <!-- 便当盒统计信息 -->
              <div class="stats-grid">
                <div
                  v-for="(stat, index) in trainerStats"
                  :key="index"
                  class="stat-card"
                  :style="{ '--accent-color': stat.color }"
                >
                  <div class="stat-icon-wrapper">
                    <NIcon class="stat-icon">
                      <component :is="stat.icon" />
                    </NIcon>
                  </div>
                  <div class="stat-content">
                    <div class="stat-label">{{ stat.label }}</div>
                    <div class="stat-value">{{ stat.value }}</div>
                  </div>
                </div>
              </div>

              <!-- 描述信息卡片 -->
              <div class="info-card">
                <div class="card-header">
                  <h3 class="card-title">
                    <NIcon class="card-icon">
                      <SparklesOutline />
                    </NIcon>
                    修改器描述
                  </h3>
                </div>
                <div class="card-content">
                  <div
                    class="description-text"
                    :class="{ collapsed: !isExpanded && trainer.description.length > 300 }"
                  >
                    {{ trainer.description || '暂无描述信息' }}
                  </div>
                  <div class="expand-control" v-if="trainer.description.length > 300">
                    <NButton text @click="toggleExpand" class="expand-button">
                      {{ isExpanded ? '收起' : '展开更多' }}
                      <template #icon>
                        <NIcon>
                          <component :is="isExpanded ? ChevronUpOutline : ChevronDownOutline" />
                        </NIcon>
                      </template>
                    </NButton>
                  </div>
                </div>
              </div>

              <!-- 本地安装信息 -->
              <div v-if="isDownloaded" class="info-card">
                <div class="card-header">
                  <h3 class="card-title">
                    <NIcon class="card-icon">
                      <FolderOpenOutline />
                    </NIcon>
                    本地安装信息
                  </h3>
                </div>
                <div class="card-content">
                  <div class="installation-info">
                    <div class="info-item">
                      <div class="info-label">
                        <NIcon class="info-icon">
                          <FolderOpenOutline />
                        </NIcon>
                        安装位置
                      </div>
                      <div class="info-value">
                        <code class="install-path">{{ trainer.installed_path }}</code>
                        <NButton text size="small" @click="openFolder" class="folder-button">
                          <template #icon>
                            <NIcon><LinkOutline /></NIcon>
                          </template>
                          打开文件夹
                        </NButton>
                      </div>
                    </div>

                    <NDivider class="info-divider" />

                    <div class="info-item">
                      <div class="info-label">
                        <NIcon class="info-icon">
                          <TimeOutline />
                        </NIcon>
                        安装时间
                      </div>
                      <div class="info-value">
                        {{ formatDate(trainer.install_time || '') }}
                      </div>
                    </div>

                    <NDivider class="info-divider" />

                    <div class="info-item">
                      <div class="info-label">
                        <NIcon class="info-icon">
                          <PlayOutline />
                        </NIcon>
                        上次启动
                      </div>
                      <div class="info-value">
                        {{
                          trainer.last_launch_time
                            ? formatDate(trainer.last_launch_time)
                            : '未启动过'
                        }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 安全信息 -->
              <div class="info-card security-card">
                <div class="card-header">
                  <h3 class="card-title">
                    <NIcon class="card-icon security-icon">
                      <ShieldCheckmarkOutline />
                    </NIcon>
                    安全验证
                  </h3>
                </div>
                <div class="card-content">
                  <div class="security-items">
                    <div class="security-item verified">
                      <NIcon class="security-check">
                        <ShieldCheckmarkOutline />
                      </NIcon>
                      <span class="security-text">已通过安全扫描</span>
                    </div>
                    <div class="security-item verified">
                      <NIcon class="security-check">
                        <ShieldCheckmarkOutline />
                      </NIcon>
                      <span class="security-text">无恶意代码</span>
                    </div>
                    <div class="security-item verified">
                      <NIcon class="security-check">
                        <ShieldCheckmarkOutline />
                      </NIcon>
                      <span class="security-text">用户验证安全</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </NSpin>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  ArrowBackOutline,
  CodeOutline,
  GameControllerOutline,
  DownloadOutline,
  TimeOutline,
  PlayOutline,
  TrashOutline,
  ChevronDownOutline,
  ChevronUpOutline,
  LinkOutline,
  StarOutline,
  HeartOutline,
  ShareOutline,
  ShieldCheckmarkOutline,
  SparklesOutline,
  FolderOpenOutline,
} from '@vicons/ionicons5'
import { useTrainerStore } from '@/stores/trainer'
import {
  useMessage,
  useDialog,
  NProgress,
  NButton,
  NCard,
  NIcon,
  NDescriptions,
  NDescriptionsItem,
  NFlex,
  NGrid,
  NGridItem,
  NImage,
  NSpace,
  NSpin,
  NTag,
  NStatistic,
  NRate,
  NAvatar,
  NDivider,
} from 'naive-ui'
import type { Trainer } from '@/types'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const route = useRoute()
const store = useTrainerStore()
const message = useMessage()
const dialog = useDialog()

// 状态管理
const loading = ref(true)
const trainer = ref<Trainer | null>(null)
const isExpanded = ref(false)
const downloadProgress = ref(0)
const isDownloading = ref(false)
const userRating = ref(0)
const isFavorite = ref(false)

// 计算属性
const isDownloaded = computed(() =>
  store.downloadedTrainers.some((t) => t.id === trainer.value?.id),
)

const trainerId = computed(() => route.params.id as string)

// 统计信息
const trainerStats = computed(() => {
  if (!trainer.value) return []

  return [
    {
      label: '版本',
      value: trainer.value.version,
      icon: CodeOutline,
      color: '#7c3aed'
    },
    {
      label: '游戏版本',
      value: trainer.value.game_version,
      icon: GameControllerOutline,
      color: '#0891b2'
    },
    {
      label: '下载量',
      value: formatNumber(trainer.value.download_count),
      icon: DownloadOutline,
      color: '#059669'
    },
    {
      label: '评分',
      value: '4.8',
      icon: StarOutline,
      color: '#d97706'
    }
  ]
})

// 获取修改器详情
const fetchTrainerDetail = async () => {
  try {
    loading.value = true
    trainer.value = await store.getTrainerDetail(trainerId.value)
  } catch (error) {
    console.error('获取修改器详情失败:', error)
    message.error('获取修改器详情失败，请稍后再试')
  } finally {
    loading.value = false
  }
}

// 格式化数字
const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}

// 格式化日期
const formatDate = (dateString: string) => {
  if (!dateString) return '未知'
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

// 下载修改器
const handleDownload = async () => {
  if (!trainer.value) return

  try {
    isDownloading.value = true
    downloadProgress.value = 0

    // 模拟下载进度
    const interval = setInterval(() => {
      if (downloadProgress.value < 90) {
        downloadProgress.value += Math.random() * 15
      }
    }, 300)

    await store.downloadTrainer(trainer.value)

    clearInterval(interval)
    downloadProgress.value = 100
    message.success('修改器下载成功！')

    await fetchTrainerDetail()
  } catch (error) {
    console.error('下载失败:', error)
    message.error(error instanceof Error ? error.message : '下载失败')
  } finally {
    isDownloading.value = false
  }
}

// 启动修改器
const handleLaunch = async () => {
  if (!trainer.value) return

  try {
    message.loading('正在启动修改器...')
    await store.launchTrainer(trainer.value.id)
    message.success('修改器启动成功！')

    await fetchTrainerDetail()
  } catch (error) {
    console.error('启动失败:', error)

    const errorMsg = error instanceof Error ? error.message : String(error)
    if (
      errorMsg.includes('Permission') ||
      errorMsg.includes('权限') ||
      errorMsg.includes('elevated privileges')
    ) {
      dialog.warning({
        title: '需要管理员权限',
        content: '启动修改器需要管理员权限。您可以选择以管理员身份重启应用程序，或者取消操作。',
        positiveText: '以管理员身份重启',
        negativeText: '取消',
        onPositiveClick: async () => {
          try {
            message.info('正在以管理员身份重启应用...')
            await invoke('restart_as_admin')
          } catch (e) {
            message.error('重启失败，请手动以管理员身份运行应用')
          }
        },
      })
    } else {
      message.error(errorMsg)
    }
  }
}

// 删除修改器
const handleDelete = async () => {
  if (!trainer.value) return

  const trainerId = trainer.value.id
  const trainerName = trainer.value.name

  dialog.warning({
    title: '确认删除',
    content: `确定要删除修改器"${trainerName}"吗？此操作不可撤销。`,
    positiveText: '确定删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await store.deleteTrainer(trainerId)
        message.success('修改器已删除')
        await fetchTrainerDetail()
      } catch (error) {
        console.error('删除失败:', error)
        message.error(error instanceof Error ? error.message : '删除失败')
      }
    },
  })
}

// 分享功能
const handleShare = () => {
  if (navigator.share && trainer.value) {
    navigator.share({
      title: trainer.value.name,
      text: trainer.value.description,
      url: window.location.href
    }).catch(() => {
      // 降级到复制到剪贴板
      navigator.clipboard.writeText(window.location.href)
      message.success('链接已复制到剪贴板')
    })
  } else {
    // 复制到剪贴板
    navigator.clipboard.writeText(window.location.href)
    message.success('链接已复制到剪贴板')
  }
}

// 切换收藏
const toggleFavorite = () => {
  isFavorite.value = !isFavorite.value
  message.success(isFavorite.value ? '已添加到收藏' : '已取消收藏')
}

// 打开文件夹
const openFolder = () => {
  if (trainer.value?.installed_path) {
    // 实现打开文件夹的逻辑
    message.info('正在打开文件夹...')
  }
}

// 切换描述展开/收起
const toggleExpand = () => {
  isExpanded.value = !isExpanded.value
}

// 组件挂载时获取数据
onMounted(fetchTrainerDetail)
</script>

<style scoped>
/* 现代化详情页布局 */
.modern-detail-view {
  width: 100%;
  min-height: 100vh;
  background: transparent;
}

/* 顶部导航区域 */
.detail-header {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 24px;
  padding: 1.5rem 2rem;
  margin-bottom: 2rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  position: sticky;
  top: 2rem;
  z-index: 10;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  max-width: 1400px;
  margin: 0 auto;
}

.back-button {
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  flex-shrink: 0;
}

.back-button:hover {
  background: rgba(255, 255, 255, 1);
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.header-info {
  flex: 1;
  min-width: 0;
}

.trainer-title {
  margin: 0 0 0.5rem 0;
  font-size: 1.75rem;
  font-weight: 800;
  background: linear-gradient(135deg, #1f2937 0%, #374151 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  line-height: 1.2;
}

.trainer-meta {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.meta-text {
  font-size: 0.875rem;
  color: #6b7280;
  font-weight: 500;
}

.meta-separator {
  color: #d1d5db;
  font-weight: 400;
}

.header-actions {
  display: flex;
  gap: 0.75rem;
  flex-shrink: 0;
}

.header-actions .n-button.active {
  color: #dc2626;
  background: rgba(220, 38, 38, 0.1);
}

/* 主内容区域 */
.detail-content {
  max-width: 1400px;
  margin: 0 auto;
}

.detail-layout {
  display: grid;
  grid-template-columns: 380px 1fr;
  gap: 2rem;
  align-items: start;
}

/* 左侧：封面区域 */
.cover-section {
  position: sticky;
  top: 8rem;
}

.cover-card {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 24px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  margin-bottom: 1.5rem;
}

.cover-wrapper {
  position: relative;
  width: 100%;
  aspect-ratio: 16/10;
  overflow: hidden;
}

.cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.cover-card:hover .cover-image {
  transform: scale(1.05);
}

.cover-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    to bottom,
    transparent 0%,
    transparent 50%,
    rgba(0, 0, 0, 0.1) 100%
  );
  display: flex;
  align-items: flex-start;
  justify-content: flex-end;
  padding: 1rem;
}

.status-badge {
  animation: slideInDown 0.5s ease-out;
}

@keyframes slideInDown {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 操作区域 */
.action-area {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 24px;
  padding: 1.5rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.download-area {
  margin-bottom: 1.5rem;
}

.download-button {
  height: 56px;
  border-radius: 16px;
  font-weight: 700;
  font-size: 1rem;
  transition: all 0.3s ease;
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  border: none;
}

.download-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 32px rgba(124, 58, 237, 0.4);
}

.progress-area {
  text-align: center;
}

.download-progress {
  margin-bottom: 1rem;
}

.progress-text {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

.progress-percentage {
  font-size: 1.5rem;
  font-weight: 800;
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.progress-label {
  font-size: 0.875rem;
  color: #6b7280;
  font-weight: 500;
}

.downloaded-actions {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.launch-button {
  height: 48px;
  border-radius: 16px;
  font-weight: 700;
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  border: none;
  transition: all 0.3s ease;
}

.launch-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 32px rgba(5, 150, 105, 0.4);
}

.delete-button {
  height: 44px;
  border-radius: 12px;
  font-weight: 600;
  transition: all 0.3s ease;
}

.delete-button:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

/* 快速统计 */
.quick-stats {
  display: flex;
  justify-content: space-around;
  align-items: center;
  padding: 1rem 0;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
  flex: 1;
}

.stat-icon {
  color: #7c3aed;
}

.stat-value {
  font-size: 0.875rem;
  font-weight: 700;
  color: #1f2937;
  text-align: center;
}

.stat-separator {
  width: 1px;
  height: 32px;
  background: rgba(0, 0, 0, 0.1);
}

/* 右侧：信息区域 */
.info-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

/* 便当盒统计网格 */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.stat-card {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 20px;
  padding: 1.5rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 4px;
  height: 100%;
  background: var(--accent-color);
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.stat-icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: 16px;
  background: var(--accent-color);
  background: linear-gradient(135deg, var(--accent-color) 0%, color-mix(in srgb, var(--accent-color) 80%, black) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.stat-icon {
  font-size: 1.25rem;
}

.stat-content {
  flex: 1;
}

.stat-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 0.25rem;
}

.stat-value {
  font-size: 1.25rem;
  font-weight: 800;
  color: #1f2937;
  line-height: 1.1;
}

/* 信息卡片 */
.info-card {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 24px;
  padding: 2rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
}

.info-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 48px rgba(0, 0, 0, 0.15);
}

.card-header {
  margin-bottom: 1.5rem;
}

.card-title {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin: 0;
  font-size: 1.25rem;
  font-weight: 700;
  color: #1f2937;
}

.card-icon {
  color: #7c3aed;
  font-size: 1.25rem;
}

.security-icon {
  color: #059669;
}

.card-content {
  color: #374151;
}

.description-text {
  font-size: 1rem;
  line-height: 1.7;
  color: #374151;
  white-space: pre-line;
}

.description-text.collapsed {
  max-height: 6rem;
  overflow: hidden;
  position: relative;
}

.description-text.collapsed::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2rem;
  background: linear-gradient(
    to bottom,
    transparent 0%,
    rgba(255, 255, 255, 0.95) 100%
  );
}

.expand-control {
  display: flex;
  justify-content: center;
  margin-top: 1rem;
}

.expand-button {
  font-weight: 600;
  color: #7c3aed;
}

/* 安装信息 */
.installation-info {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.info-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 600;
  color: #374151;
  font-size: 0.875rem;
}

.info-icon {
  color: #7c3aed;
  font-size: 1rem;
}

.info-value {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.install-path {
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  background: rgba(0, 0, 0, 0.05);
  padding: 0.5rem 0.75rem;
  border-radius: 8px;
  font-size: 0.875rem;
  color: #374151;
  word-break: break-all;
  flex: 1;
  min-width: 200px;
}

.folder-button {
  font-weight: 600;
  color: #7c3aed;
  white-space: nowrap;
}

.info-divider {
  margin: 0;
}

/* 安全信息 */
.security-card {
  border-left: 4px solid #059669;
}

.security-items {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.security-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  background: rgba(5, 150, 105, 0.05);
  border-radius: 12px;
  border: 1px solid rgba(5, 150, 105, 0.2);
}

.security-item.verified {
  background: rgba(5, 150, 105, 0.1);
  border-color: rgba(5, 150, 105, 0.3);
}

.security-check {
  color: #059669;
  font-size: 1.125rem;
}

.security-text {
  font-weight: 600;
  color: #047857;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .detail-layout {
    grid-template-columns: 320px 1fr;
    gap: 1.5rem;
  }

  .cover-section {
    position: relative;
    top: 0;
  }

  .detail-header {
    position: relative;
    top: 0;
  }
}

@media (max-width: 768px) {
  .detail-header {
    padding: 1rem 1.5rem;
    margin-bottom: 1.5rem;
  }

  .header-content {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .trainer-title {
    font-size: 1.5rem;
  }

  .detail-layout {
    grid-template-columns: 1fr;
    gap: 1.5rem;
  }

  .cover-section {
    position: relative;
    top: 0;
  }

  .stats-grid {
    grid-template-columns: 1fr;
  }

  .info-card {
    padding: 1.5rem;
  }

  .install-path {
    min-width: auto;
  }

  .quick-stats {
    flex-direction: column;
    gap: 1rem;
  }

  .stat-separator {
    width: 32px;
    height: 1px;
  }
}

/* 暗色主题 */
:deep(.dark-theme) .detail-header,
:deep(.dark-theme) .cover-card,
:deep(.dark-theme) .action-area,
:deep(.dark-theme) .info-card,
:deep(.dark-theme) .stat-card {
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .trainer-title {
  background: linear-gradient(135deg, #e2e8f0 0%, #cbd5e1 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

:deep(.dark-theme) .meta-text {
  color: #94a3b8;
}

:deep(.dark-theme) .stat-value,
:deep(.dark-theme) .card-title {
  color: #e2e8f0;
}

:deep(.dark-theme) .stat-label {
  color: #94a3b8;
}

:deep(.dark-theme) .description-text {
  color: #cbd5e1;
}

:deep(.dark-theme) .info-label {
  color: #e2e8f0;
}

:deep(.dark-theme) .install-path {
  background: rgba(0, 0, 0, 0.3);
  color: #cbd5e1;
}

:deep(.dark-theme) .back-button,
:deep(.dark-theme) .header-actions .n-button {
  background: rgba(30, 41, 59, 0.8);
  border-color: rgba(255, 255, 255, 0.1);
}
</style>
