<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useMessage, useDialog } from 'naive-ui'
import {
  EyeOutline,
  TrashOutline,
  PlayOutline,
  TimeOutline,
  DownloadOutline,
  StarOutline,
  HeartOutline,
  SparklesOutline,
  GameControllerOutline,
} from '@vicons/ionicons5'
import type { Trainer } from '@/types'
import { useTrainerStore } from '@/stores/trainer'

const props = defineProps<{
  trainer: Trainer
  loading?: boolean
  // 控制显示哪些按钮："default"(详情+下载) 或 "downloaded"(详情+删除+启动)
  showButtons?: 'default' | 'downloaded'
}>()

// 设置默认值
const showButtons = props.showButtons || 'default'

const router = useRouter()
const store = useTrainerStore()
const message = useMessage()
const dialog = useDialog()

// 计算属性
const isDownloaded = computed(() => {
  return store.downloadedTrainers.some((t) => t.id === props.trainer.id)
})

const isInstalled = computed(() => {
  return props.trainer.installed_path ? true : false
})

const statusInfo = computed(() => {
  if (isInstalled.value) {
    return {
      text: '已安装',
      type: 'success' as const,
      color: '#059669',
      bgColor: 'rgba(5, 150, 105, 0.1)',
      borderColor: 'rgba(5, 150, 105, 0.3)'
    }
  } else if (isDownloaded.value) {
    return {
      text: '已下载',
      type: 'info' as const,
      color: '#0891b2',
      bgColor: 'rgba(8, 145, 178, 0.1)',
      borderColor: 'rgba(8, 145, 178, 0.3)'
    }
  } else {
    return {
      text: '未下载',
      type: 'default' as const,
      color: '#6b7280',
      bgColor: 'rgba(107, 114, 128, 0.1)',
      borderColor: 'rgba(107, 114, 128, 0.3)'
    }
  }
})

// 格式化日期
const formatDate = (dateString: string) => {
  if (!dateString) return '未知'
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
  })
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

// 防止事件冒泡
const stopPropagation = (e: Event) => {
  e.stopPropagation()
}

// 卡片点击处理
const handleCardClick = () => {
  if (!props.loading) {
    router.push(`/detail/${props.trainer.id}`)
  }
}

// 处理收藏
const handleFavorite = (e: Event) => {
  stopPropagation(e)
  // 这里可以实现收藏逻辑
  message.success('已添加到收藏')
}

// 处理下载按钮点击
const handleDownload = async (e: Event) => {
  stopPropagation(e)
  try {
    message.loading('正在获取修改器信息...')
    const latestTrainerDetail = await store.getTrainerDetail(props.trainer.id)

    message.loading('准备下载修改器...')
    await store.downloadTrainer(latestTrainerDetail)
    message.success('修改器下载成功！')
  } catch (error) {
    console.error('下载失败:', error)
    message.error(error instanceof Error ? error.message : '下载失败')
  }
}

// 处理启动按钮点击
const handleLaunch = async (e: Event) => {
  stopPropagation(e)
  try {
    message.loading('正在启动修改器...')
    await store.launchTrainer(props.trainer.id)
    message.success('修改器启动成功！')
  } catch (error) {
    console.error('启动失败:', error)
    message.error(error instanceof Error ? error.message : '启动失败')
  }
}

// 处理删除按钮点击
const handleDelete = async (e: Event) => {
  stopPropagation(e)

  dialog.warning({
    title: '确认删除',
    content: `确定要删除修改器"${props.trainer.name}"吗？此操作不可撤销。`,
    positiveText: '确定删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await store.deleteTrainer(props.trainer.id)
        message.success('修改器已删除')
      } catch (error) {
        console.error('删除失败:', error)
        message.error(error instanceof Error ? error.message : '删除失败')
      }
    },
  })
}
</script>

<template>
  <div
    class="modern-game-card"
    :class="{ loading: props.loading }"
    @click="handleCardClick"
  >
    <!-- 卡片主体 -->
    <div class="card-wrapper">
      <!-- 图片区域 -->
      <div class="card-image-section">
        <div class="image-wrapper">
          <NImage
            class="card-image"
            :src="trainer.thumbnail"
            width="100%"
            fallback-src="/placeholder.png"
            :alt="trainer.name"
            :preview-disabled="true"
            object-fit="cover"
            loading="lazy"
          />
          <!-- 悬停时的渐变遮罩 -->
          <div class="image-overlay">
            <div class="overlay-content">
              <NIcon class="view-icon">
                <EyeOutline />
              </NIcon>
              <span class="view-text">查看详情</span>
            </div>
          </div>
        </div>

        <!-- 状态标签 -->
        <div class="status-badge">
          <div
            class="status-tag"
            :style="{
              backgroundColor: statusInfo.bgColor,
              borderColor: statusInfo.borderColor,
              color: statusInfo.color
            }"
          >
            <NIcon class="status-icon" size="12">
              <GameControllerOutline v-if="isInstalled" />
              <DownloadOutline v-else-if="isDownloaded" />
              <SparklesOutline v-else />
            </NIcon>
            <span class="status-text">{{ statusInfo.text }}</span>
          </div>
        </div>

        <!-- 收藏按钮 -->
        <div class="favorite-button" @click="handleFavorite">
          <NIcon class="favorite-icon">
            <HeartOutline />
          </NIcon>
        </div>
      </div>

      <!-- 内容区域 -->
      <div class="card-content-section">
        <!-- 标题 -->
        <h3 class="card-title" :title="trainer.name">
          {{ trainer.name }}
        </h3>

        <!-- 元数据 -->
        <div class="card-meta">
          <div class="meta-item">
            <div class="meta-icon-wrapper">
              <NIcon class="meta-icon">
                <TimeOutline />
              </NIcon>
            </div>
            <span class="meta-text">{{ formatDate(trainer.last_update) }}</span>
          </div>

          <div class="meta-item">
            <div class="meta-icon-wrapper">
              <NIcon class="meta-icon">
                <DownloadOutline />
              </NIcon>
            </div>
            <span class="meta-text">{{ formatNumber(trainer.download_count) }}</span>
          </div>

          <div class="meta-item">
            <div class="meta-icon-wrapper">
              <NIcon class="meta-icon">
                <StarOutline />
              </NIcon>
            </div>
            <span class="meta-text">4.8</span>
          </div>
        </div>

        <!-- 游戏版本 -->
        <div class="game-version">
          <NIcon class="version-icon">
            <GameControllerOutline />
          </NIcon>
          <span class="version-text">{{ trainer.game_version }}</span>
        </div>

        <!-- 操作按钮区域 -->
        <div class="card-actions">
          <!-- 默认按钮：详情和下载 -->
          <template v-if="showButtons === 'default'">
            <NButton
              size="small"
              class="action-button detail-button"
              @click.stop="handleCardClick"
            >
              <template #icon>
                <NIcon><EyeOutline /></NIcon>
              </template>
              详情
            </NButton>

            <NButton
              type="primary"
              size="small"
              class="action-button download-button"
              @click="handleDownload"
              :disabled="isDownloaded"
            >
              <template #icon>
                <NIcon><DownloadOutline /></NIcon>
              </template>
              {{ isDownloaded ? '已下载' : '下载' }}
            </NButton>
          </template>

          <!-- 已下载按钮：详情、删除和启动 -->
          <template v-else-if="showButtons === 'downloaded'">
            <NButton
              size="small"
              class="action-button detail-button"
              @click.stop="handleCardClick"
            >
              <template #icon>
                <NIcon><EyeOutline /></NIcon>
              </template>
              详情
            </NButton>

            <NButton
              size="small"
              class="action-button delete-button"
              @click="handleDelete"
            >
              <template #icon>
                <NIcon><TrashOutline /></NIcon>
              </template>
              删除
            </NButton>

            <NButton
              type="success"
              size="small"
              class="action-button launch-button"
              @click="handleLaunch"
            >
              <template #icon>
                <NIcon><PlayOutline /></NIcon>
              </template>
              启动
            </NButton>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 现代化游戏卡片 */
.modern-game-card {
  position: relative;
  width: 100%;
  height: 100%;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 20px;
  overflow: hidden;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.modern-game-card:hover {
  transform: translateY(-6px) scale(1.02);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
}

.modern-game-card.loading {
  opacity: 0.7;
  pointer-events: none;
}

.card-wrapper {
  height: 100%;
  display: flex;
  flex-direction: column;
}

/* 图片区域 */
.card-image-section {
  position: relative;
  width: 100%;
  aspect-ratio: 16/9;
  overflow: hidden;
}

.image-wrapper {
  position: relative;
  width: 100%;
  height: 100%;
}

.card-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.4s ease;
}

.modern-game-card:hover .card-image {
  transform: scale(1.08);
}

/* 悬停遮罩 */
.image-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    to bottom,
    transparent 0%,
    transparent 40%,
    rgba(0, 0, 0, 0.7) 100%
  );
  display: flex;
  align-items: flex-end;
  justify-content: center;
  padding: 1.5rem;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.modern-game-card:hover .image-overlay {
  opacity: 1;
}

.overlay-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  color: white;
  transform: translateY(10px);
  transition: transform 0.3s ease;
}

.modern-game-card:hover .overlay-content {
  transform: translateY(0);
}

.view-icon {
  font-size: 1.5rem;
}

.view-text {
  font-size: 0.875rem;
  font-weight: 600;
}

/* 状态标签 */
.status-badge {
  position: absolute;
  top: 1rem;
  right: 1rem;
  z-index: 2;
}

.status-tag {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.375rem 0.75rem;
  border-radius: 20px;
  border: 2px solid;
  font-size: 0.75rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  transition: all 0.3s ease;
}

.status-tag:hover {
  transform: translateY(-2px);
}

.status-icon {
  font-size: 0.75rem;
}

.status-text {
  line-height: 1;
}

/* 收藏按钮 */
.favorite-button {
  position: absolute;
  top: 1rem;
  left: 1rem;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  z-index: 2;
}

.favorite-button:hover {
  background: rgba(255, 255, 255, 1);
  transform: scale(1.1);
}

.favorite-icon {
  color: #ef4444;
  font-size: 1rem;
  transition: transform 0.3s ease;
}

.favorite-button:hover .favorite-icon {
  transform: scale(1.2);
}

/* 内容区域 */
.card-content-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 1.25rem;
  gap: 1rem;
}

.card-title {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 700;
  color: #1f2937;
  line-height: 1.3;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

/* 元数据 */
.card-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.25rem 0.5rem;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 12px;
  font-size: 0.75rem;
}

.meta-icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 8px;
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  color: white;
}

.meta-icon {
  font-size: 0.75rem;
  color: white;
}

.meta-text {
  font-weight: 600;
  color: #4b5563;
  line-height: 1;
}

/* 游戏版本 */
.game-version {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: linear-gradient(135deg, rgba(124, 58, 237, 0.1) 0%, rgba(8, 145, 178, 0.1) 100%);
  border-radius: 12px;
  border: 1px solid rgba(124, 58, 237, 0.2);
  margin-bottom: 1rem;
}

.version-icon {
  color: #7c3aed;
  font-size: 0.875rem;
}

.version-text {
  font-size: 0.75rem;
  font-weight: 600;
  color: #4c1d95;
  line-height: 1;
}

/* 操作按钮 */
.card-actions {
  display: flex;
  gap: 0.5rem;
  margin-top: auto;
}

.action-button {
  flex: 1;
  height: 36px;
  border-radius: 12px;
  font-weight: 600;
  font-size: 0.75rem;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.action-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.4), transparent);
  transition: left 0.5s ease;
}

.action-button:hover::before {
  left: 100%;
}

.detail-button {
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(0, 0, 0, 0.1);
  color: #374151;
}

.detail-button:hover {
  background: rgba(255, 255, 255, 1);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.download-button {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  border: none;
  color: white;
}

.download-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(124, 58, 237, 0.4);
}

.download-button:disabled {
  background: #9ca3af;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.delete-button {
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(239, 68, 68, 0.3);
  color: #dc2626;
}

.delete-button:hover {
  background: rgba(239, 68, 68, 0.1);
  border-color: #dc2626;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.2);
}

.launch-button {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  border: none;
  color: white;
}

.launch-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(5, 150, 105, 0.4);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .modern-game-card {
    border-radius: 16px;
  }

  .card-content-section {
    padding: 1rem;
    gap: 0.75rem;
  }

  .card-title {
    font-size: 1rem;
  }

  .card-meta {
    gap: 0.25rem;
  }

  .meta-item {
    padding: 0.1875rem 0.375rem;
    font-size: 0.6875rem;
  }

  .meta-icon-wrapper {
    width: 16px;
    height: 16px;
    border-radius: 6px;
  }

  .meta-icon {
    font-size: 0.6875rem;
  }

  .action-button {
    height: 32px;
    font-size: 0.6875rem;
  }

  .status-badge {
    top: 0.75rem;
    right: 0.75rem;
  }

  .favorite-button {
    top: 0.75rem;
    left: 0.75rem;
    width: 28px;
    height: 28px;
  }
}

@media (max-width: 480px) {
  .card-content-section {
    padding: 0.75rem;
  }

  .card-title {
    font-size: 0.875rem;
  }

  .game-version {
    padding: 0.375rem 0.5rem;
    margin-bottom: 0.75rem;
  }

  .version-text {
    font-size: 0.6875rem;
  }

  .action-button {
    height: 28px;
    font-size: 0.625rem;
  }
}

/* 暗色主题 */
:deep(.dark-theme) .modern-game-card {
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .card-title {
  color: #e2e8f0;
}

:deep(.dark-theme) .meta-item {
  background: rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .meta-text {
  color: #cbd5e1;
}

:deep(.dark-theme) .game-version {
  background: linear-gradient(135deg, rgba(124, 58, 237, 0.2) 0%, rgba(8, 145, 178, 0.2) 100%);
  border-color: rgba(124, 58, 237, 0.3);
}

:deep(.dark-theme) .version-text {
  color: #a78bfa;
}

:deep(.dark-theme) .detail-button {
  background: rgba(30, 41, 59, 0.8);
  border-color: rgba(255, 255, 255, 0.1);
  color: #e2e8f0;
}

:deep(.dark-theme) .detail-button:hover {
  background: rgba(30, 41, 59, 1);
}

:deep(.dark-theme) .delete-button {
  background: rgba(30, 41, 59, 0.8);
  border-color: rgba(239, 68, 68, 0.3);
  color: #f87171;
}

:deep(.dark-theme) .delete-button:hover {
  background: rgba(239, 68, 68, 0.2);
  border-color: #dc2626;
}

:deep(.dark-theme) .favorite-button {
  background: rgba(30, 41, 59, 0.8);
  border-color: rgba(255, 255, 255, 0.1);
}
</style>
