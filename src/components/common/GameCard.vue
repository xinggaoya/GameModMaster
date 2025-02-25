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

// 防止事件冒泡
const stopPropagation = (e: Event) => {
  e.stopPropagation()
}

// 处理下载按钮点击
const handleDownload = async (e: Event) => {
  stopPropagation(e)
  try {
    // 先获取最新的trainer详情
    const latestTrainerDetail = await store.getTrainerDetail(props.trainer.id)

    // 使用最新的trainer详情进行下载
    await store.downloadTrainer(latestTrainerDetail)
    message.success('下载成功')
  } catch (error) {
    console.error('下载失败:', error)
    message.error(error instanceof Error ? error.message : '下载失败')
  }
}

// 处理启动按钮点击
const handleLaunch = async (e: Event) => {
  stopPropagation(e)
  try {
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
        message.success('删除成功')
      } catch (error) {
        console.error('删除失败:', error)
        message.error(error instanceof Error ? error.message : '删除失败')
      }
    },
  })
}

// 处理查看详情按钮点击
const handleViewDetails = (e: Event) => {
  stopPropagation(e)
  router.push(`/detail/${props.trainer.id}`)
}
</script>

<template>
  <NCard class="game-card" :class="{ 'is-loading': loading }" hoverable @click="handleCardClick">
    <template #cover>
      <div class="card-cover">
        <NSkeleton v-if="loading" :width="240" :height="135" />
        <NImage v-else :src="trainer.thumbnail" :alt="trainer.name" object-fit="cover" />

        <!-- 状态标签 -->
        <NTag v-if="!loading" class="status-tag" size="small" :type="statusTag.type" round>
          {{ statusTag.text }}
        </NTag>
      </div>
    </template>

    <NSkeleton v-if="loading" text :repeat="2" />
    <template v-else>
      <h3 class="card-title">
        <NEllipsis :tooltip="false">
          {{ trainer.name }}
        </NEllipsis>
      </h3>

      <div class="card-meta">
        <div class="meta-item">
          <NIcon><TimeOutline /></NIcon>
          <span>{{ formatDate(trainer.last_update) }}</span>
        </div>
        <NTag size="small" round>v{{ trainer.version }}</NTag>
      </div>

      <div class="card-description">
        <NEllipsis :line-clamp="2" :tooltip="false">
          {{ trainer.description || '暂无描述' }}
        </NEllipsis>
      </div>

      <div class="card-actions">
        <!-- 默认模式：详情+下载 -->
        <NSpace v-if="showButtons === 'default'" justify="space-between">
          <!-- 详情按钮 -->
          <NButton size="small" secondary @click="handleViewDetails">
            <template #icon>
              <NIcon><EyeOutline /></NIcon>
            </template>
            详情
          </NButton>

          <!-- 下载按钮 -->
          <NButton size="small" type="primary" @click="handleDownload">
            <template #icon>
              <NIcon><DownloadOutline /></NIcon>
            </template>
            下载
          </NButton>
        </NSpace>

        <!-- 已下载模式：详情+删除+启动 -->
        <NSpace v-else-if="showButtons === 'downloaded'" justify="space-between">
          <!-- 详情按钮 -->
          <NButton size="small" secondary @click="handleViewDetails">
            <template #icon>
              <NIcon><EyeOutline /></NIcon>
            </template>
            详情
          </NButton>

          <!-- 删除按钮 -->
          <NButton size="small" type="error" @click="handleDelete">
            <template #icon>
              <NIcon><TrashOutline /></NIcon>
            </template>
            删除
          </NButton>

          <!-- 启动按钮 -->
          <NButton size="small" type="success" @click="handleLaunch">
            <template #icon>
              <NIcon><PlayOutline /></NIcon>
            </template>
            启动
          </NButton>
        </NSpace>
      </div>
    </template>
  </NCard>
</template>

<style scoped>
.game-card {
  cursor: pointer;
  transition:
    transform var(--transition-fast),
    box-shadow var(--transition-fast);
  height: 100%;
  display: flex;
  flex-direction: column;
}

.game-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-md);
}

.game-card.is-loading {
  cursor: default;
}

.card-cover {
  position: relative;
  height: 135px;
  overflow: hidden;
  background: var(--gray-200);
}

.card-cover :deep(img) {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform var(--transition-normal);
}

.game-card:hover .card-cover :deep(img) {
  transform: scale(1.05);
}

.status-tag {
  position: absolute;
  top: 10px;
  right: 10px;
  z-index: 2;
}

.card-title {
  margin: 0.5rem 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
  font-family: var(--font-gaming);
}

.card-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
  font-size: 0.8rem;
  color: var(--text-tertiary);
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.meta-item :deep(.n-icon) {
  font-size: 0.9rem;
}

.card-description {
  margin-bottom: 1rem;
  font-size: 0.85rem;
  color: var(--text-secondary);
  flex-grow: 1;
}

.card-actions {
  margin-top: auto;
}

/* 修改按钮布局，确保多个按钮能够均匀分布 */
.card-actions :deep(.n-space) {
  width: 100%;
}

.card-actions :deep(.n-button) {
  padding: 0 8px;
}
</style>
