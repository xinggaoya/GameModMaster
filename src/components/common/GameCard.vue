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
const store = useTrainerStore() // 引入状态管理
const message = useMessage() // 使用Naive UI的消息提示
const dialog = useDialog() // 使用Naive UI的对话框

// 格式化日期
const formatDate = (dateString: string) => {
  if (!dateString) return '未知'
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
}

// 卡片点击处理
const handleCardClick = () => {
  if (!props.loading) {
    router.push(`/detail/${props.trainer.id}`)
  }
}

// 计算下载状态标签
const statusTag = computed(() => {
  if (props.trainer.installed_path) {
    return { text: '已安装', type: 'success' as const, icon: '' }
  }
  return { text: '未安装', type: 'default' as const, icon: '' }
})

// 计算是否已下载
const isDownloaded = computed(() => {
  return store.downloadedTrainers.some((t) => t.id === props.trainer.id)
})

// 防止事件冒泡
const stopPropagation = (e: Event) => {
  e.stopPropagation()
}

// 处理下载按钮点击
const handleDownload = async (e: Event) => {
  stopPropagation(e)
  try {
    // 显示加载中
    message.loading('正在获取修改器信息...')

    // 先获取最新的trainer详情
    const latestTrainerDetail = await store.getTrainerDetail(props.trainer.id)

    message.loading('准备下载修改器...')
    // 使用最新的trainer详情进行下载
    await store.downloadTrainer(latestTrainerDetail)
    message.success('修改器下载成功')
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
    message.success('修改器启动成功')
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
    content: `确定要删除修改器"${props.trainer.name}"吗？`,
    positiveText: '确定',
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
  <NCard
    class="game-card"
    :class="{ loading }"
    hoverable
    @click="handleCardClick"
    content-style="padding: 0"
  >
    <!-- 图片容器 -->
    <div class="card-image-container">
      <div class="card-image-wrapper">
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
      </div>

      <!-- 状态标签 -->
      <div class="status-tag" v-if="isDownloaded">
        <NTag type="success" size="small">已下载</NTag>
      </div>
    </div>

    <!-- 卡片内容 -->
    <div class="card-content">
      <h3 class="card-title" :title="trainer.name">{{ trainer.name }}</h3>

      <div class="card-meta">
        <div class="meta-item">
          <NIcon class="meta-icon"><TimeOutline /></NIcon>
          <span>{{ formatDate(trainer.last_update) }}</span>
        </div>

        <div class="meta-item">
          <NIcon class="meta-icon"><DownloadOutline /></NIcon>
          <span>{{ trainer.download_count }}</span>
        </div>
      </div>

      <!-- 按钮区域 -->
      <div class="card-actions">
        <!-- 默认按钮：详情和下载 -->
        <template v-if="showButtons === 'default'">
          <NButton type="primary" secondary size="small" @click.stop="handleCardClick">
            <template #icon>
              <NIcon><EyeOutline /></NIcon>
            </template>
            详情
          </NButton>

          <NButton
            type="primary"
            ghost
            size="small"
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
          <NButton type="info" secondary size="small" @click.stop="handleCardClick">
            <template #icon>
              <NIcon><EyeOutline /></NIcon>
            </template>
            详情
          </NButton>

          <NButton type="error" ghost size="small" @click="handleDelete">
            <template #icon>
              <NIcon><TrashOutline /></NIcon>
            </template>
            删除
          </NButton>

          <NButton type="success" ghost size="small" @click="handleLaunch">
            <template #icon>
              <NIcon><PlayOutline /></NIcon>
            </template>
            启动
          </NButton>
        </template>
      </div>
    </div>
  </NCard>
</template>

<style scoped>
.game-card {
  position: relative;
  transition:
    transform 0.2s ease,
    box-shadow 0.2s ease;
  height: 100%;
  display: flex;
  flex-direction: column;
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.game-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg);
}

.card-image-container {
  position: relative;
  overflow: hidden;
  aspect-ratio: 16/9;
  width: 100%;
}

.card-image-wrapper {
  width: 100%;
  height: 100%;
}

.card-image {
  width: 100%;
  height: 100%;
  object-position: center;
  transition: transform 0.3s ease;
  border-radius: 0;
}

.game-card:hover .card-image {
  transform: scale(1.05);
}

.status-tag {
  position: absolute;
  top: 8px;
  right: 8px;
}

.card-content {
  display: flex;
  flex-direction: column;
  padding: 16px;
  flex-grow: 1;
}

.card-title {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-meta {
  display: flex;
  justify-content: space-between;
  margin-bottom: 16px;
  font-size: 12px;
  color: var(--text-color-secondary);
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.meta-icon {
  font-size: 14px;
}

.card-actions {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  margin-top: auto;
}

.card-actions button {
  flex: 1;
}

.loading {
  opacity: 0.7;
  pointer-events: none;
}
</style>
