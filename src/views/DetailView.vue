<template>
  <div class="detail-view animate-fade">
    <!-- 顶部导航栏 -->
    <NFlex align="center" justify="start" :wrap="false" class="header-wrapper">
      <NButton @click="router.back()" quaternary circle class="back-button">
        <template #icon>
          <NIcon><ArrowBackOutline /></NIcon>
        </template>
      </NButton>
      <div class="title-container">
        <h1 class="game-title" v-if="trainer">{{ trainer.name }}</h1>
      </div>
    </NFlex>

    <!-- 主内容区 -->
    <NSpin :show="loading" size="large">
      <template v-if="trainer">
        <NCard class="detail-card">
          <NGrid :cols="24" :x-gap="16">
            <!-- 左侧：封面和信息 -->
            <NGridItem :span="24" :md="8">
              <div class="cover-section">
                <!-- 封面图 -->
                <NCard class="cover-card" :bordered="false" content-style="padding: 0;">
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
                    <div class="status-badge">
                      <NTag type="success" v-if="isDownloaded" size="large">已下载</NTag>
                      <NTag type="warning" v-else size="large">未下载</NTag>
                    </div>
                  </div>
                </NCard>

                <!-- 操作按钮区域 -->
                <div class="action-buttons">
                  <!-- 下载按钮/进度条 -->
                  <div class="download-section">
                    <NButton
                      v-if="!isDownloading && !isDownloaded"
                      type="primary"
                      size="large"
                      block
                      @click="handleDownload"
                    >
                      <template #icon>
                        <NIcon><DownloadOutline /></NIcon>
                      </template>
                      下载修改器
                    </NButton>

                    <NProgress
                      v-else-if="isDownloading"
                      :percentage="downloadProgress"
                      type="line"
                      :height="24"
                      :show-indicator="true"
                      processing
                    >
                      正在下载 {{ downloadProgress }}%
                    </NProgress>

                    <!-- 如果已下载，显示启动和删除按钮 -->
                    <NSpace v-else-if="isDownloaded" justify="space-between" :wrap="false">
                      <NButton type="success" size="large" style="flex: 1" @click="handleLaunch">
                        <template #icon>
                          <NIcon><PlayOutline /></NIcon>
                        </template>
                        启动
                      </NButton>

                      <NButton type="error" size="large" style="flex: 1" @click="handleDelete">
                        <template #icon>
                          <NIcon><TrashOutline /></NIcon>
                        </template>
                        删除
                      </NButton>
                    </NSpace>
                  </div>
                </div>
              </div>
            </NGridItem>

            <!-- 右侧：详细信息 -->
            <NGridItem :span="24" :md="16">
              <div class="info-section">
                <!-- 基本信息 -->
                <NGrid :cols="2" :x-gap="12" :y-gap="12" responsive="screen">
                  <NGridItem>
                    <NCard class="info-card">
                      <NFlex align="center">
                        <div class="icon-wrapper">
                          <NIcon><CodeOutline /></NIcon>
                        </div>
                        <div>
                          <div class="info-label">版本</div>
                          <div class="info-value">{{ trainer.version }}</div>
                        </div>
                      </NFlex>
                    </NCard>
                  </NGridItem>

                  <NGridItem>
                    <NCard class="info-card">
                      <NFlex align="center">
                        <div class="icon-wrapper">
                          <NIcon><GameControllerOutline /></NIcon>
                        </div>
                        <div>
                          <div class="info-label">游戏版本</div>
                          <div class="info-value">{{ trainer.game_version }}</div>
                        </div>
                      </NFlex>
                    </NCard>
                  </NGridItem>

                  <NGridItem>
                    <NCard class="info-card">
                      <NFlex align="center">
                        <div class="icon-wrapper">
                          <NIcon><DownloadOutline /></NIcon>
                        </div>
                        <div>
                          <div class="info-label">下载次数</div>
                          <div class="info-value">{{ trainer.download_count }}</div>
                        </div>
                      </NFlex>
                    </NCard>
                  </NGridItem>

                  <NGridItem>
                    <NCard class="info-card">
                      <NFlex align="center">
                        <div class="icon-wrapper">
                          <NIcon><TimeOutline /></NIcon>
                        </div>
                        <div>
                          <div class="info-label">最近更新</div>
                          <div class="info-value">{{ formatDate(trainer.last_update) }}</div>
                        </div>
                      </NFlex>
                    </NCard>
                  </NGridItem>
                </NGrid>

                <!-- 描述信息 -->
                <NCard class="description-card" :bordered="false">
                  <template #header>
                    <div class="card-title">修改器描述</div>
                  </template>

                  <div
                    class="description-content"
                    :class="{ collapsed: !isExpanded && trainer.description.length > 300 }"
                  >
                    {{ trainer.description || '暂无描述' }}
                  </div>

                  <div class="expand-toggle" v-if="trainer.description.length > 300">
                    <NButton text @click="toggleExpand">
                      {{ isExpanded ? '收起' : '展开' }}
                      <template #icon>
                        <NIcon>
                          <component :is="isExpanded ? ChevronUpOutline : ChevronDownOutline" />
                        </NIcon>
                      </template>
                    </NButton>
                  </div>
                </NCard>

                <!-- 本地安装信息 -->
                <NCard v-if="isDownloaded" class="installation-card" :bordered="false">
                  <template #header>
                    <div class="card-title">本地安装信息</div>
                  </template>

                  <NDescriptions :column="1" bordered>
                    <NDescriptionsItem label="安装位置">
                      <NFlex align="center">
                        <span class="install-path">{{ trainer.installed_path }}</span>
                        <NButton text size="small">
                          <template #icon>
                            <NIcon><LinkOutline /></NIcon>
                          </template>
                          打开文件夹
                        </NButton>
                      </NFlex>
                    </NDescriptionsItem>

                    <NDescriptionsItem label="安装时间">
                      {{ formatDate(trainer.install_time || '') }}
                    </NDescriptionsItem>

                    <NDescriptionsItem label="上次启动">
                      {{
                        trainer.last_launch_time ? formatDate(trainer.last_launch_time) : '未启动过'
                      }}
                    </NDescriptionsItem>
                  </NDescriptions>
                </NCard>
              </div>
            </NGridItem>
          </NGrid>
        </NCard>
      </template>
    </NSpin>
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
} from 'naive-ui'
import type { Trainer } from '@/types'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const route = useRoute()
const store = useTrainerStore()
const message = useMessage()
const dialog = useDialog()

// 状态
const loading = ref(true)
const trainer = ref<Trainer | null>(null)
const isExpanded = ref(false)
const downloadProgress = ref(0)
const isDownloading = ref(false)

// 计算属性
const isDownloaded = computed(() =>
  store.downloadedTrainers.some((t) => t.id === trainer.value?.id),
)

const trainerId = computed(() => route.params.id as string)

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

// 下载修改器
const handleDownload = async () => {
  if (!trainer.value) return

  try {
    isDownloading.value = true
    downloadProgress.value = 0

    // 模拟下载进度
    const interval = setInterval(() => {
      if (downloadProgress.value < 90) {
        downloadProgress.value += 10
      }
    }, 300)

    await store.downloadTrainer(trainer.value)

    clearInterval(interval)
    downloadProgress.value = 100
    message.success('修改器下载成功')

    // 重新获取修改器信息，确保状态更新
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
    message.success('修改器启动成功')

    // 重新获取修改器信息，确保状态更新
    await fetchTrainerDetail()
  } catch (error) {
    console.error('启动失败:', error)

    // 检查是否是权限错误
    const errorMsg = error instanceof Error ? error.message : String(error)
    if (
      errorMsg.includes('Permission') ||
      errorMsg.includes('权限') ||
      errorMsg.includes('elevated privileges')
    ) {
      // 显示需要管理员权限的对话框
      dialog.warning({
        title: '需要管理员权限',
        content: '启动修改器需要管理员权限。您可以选择以管理员身份重启应用程序，或者取消操作。',
        positiveText: '以管理员身份重启',
        negativeText: '取消',
        onPositiveClick: async () => {
          // 使用Tauri API请求以管理员身份重启应用
          try {
            message.info('正在以管理员身份重启应用...')
            // 调用Windows API重启应用
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

  try {
    await store.deleteTrainer(trainer.value.id)
    message.success('修改器已删除')

    // 重新获取修改器信息，确保状态更新
    await fetchTrainerDetail()
  } catch (error) {
    console.error('删除失败:', error)
    message.error(error instanceof Error ? error.message : '删除失败')
  }
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

// 切换描述展开/收起
const toggleExpand = () => {
  isExpanded.value = !isExpanded.value
}

onMounted(fetchTrainerDetail)
</script>

<style scoped>
.detail-view {
  padding-bottom: 2rem;
}

.header-wrapper {
  margin-bottom: 1.5rem;
}

.back-button {
  margin-right: 1rem;
}

.game-title {
  font-size: 1.75rem;
  font-weight: 700;
  margin: 0;
  color: var(--text-primary);
}

.detail-card {
  margin-bottom: 1.5rem;
}

.cover-section {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  height: 100%;
}

.cover-card {
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.cover-wrapper {
  position: relative;
  width: 100%;
  aspect-ratio: 16/9;
}

.cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.status-badge {
  position: absolute;
  top: 1rem;
  right: 1rem;
  z-index: 1;
}

.action-buttons {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.download-section {
  width: 100%;
}

.info-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  height: 100%;
}

.info-card {
  height: 100%;
  background-color: var(--card-bg-subtle);
  border: none;
  border-radius: var(--radius-md);
}

.icon-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 50%;
  background-color: var(--primary-subtle);
  color: var(--primary);
  margin-right: 0.75rem;
}

.icon-wrapper :deep(svg) {
  width: 1.25rem;
  height: 1.25rem;
}

.info-label {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin-bottom: 0.25rem;
}

.info-value {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.description-card,
.installation-card {
  background-color: var(--card-bg-subtle);
}

.card-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.description-content {
  font-size: 0.95rem;
  line-height: 1.6;
  color: var(--text-secondary);
  white-space: pre-line;
}

.description-content.collapsed {
  max-height: 8rem;
  overflow: hidden;
  position: relative;
}

.description-content.collapsed::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 3rem;
  background: linear-gradient(transparent, var(--card-bg-subtle));
}

.expand-toggle {
  display: flex;
  justify-content: center;
  margin-top: 0.5rem;
}

.installation-card {
  flex-grow: 1;
}

.install-path {
  font-family: monospace;
  background-color: var(--card-bg);
  padding: 0.25rem 0.5rem;
  border-radius: var(--radius-sm);
  margin-right: 0.5rem;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

@media (max-width: 768px) {
  .game-title {
    font-size: 1.5rem;
  }

  .cover-section {
    margin-bottom: 1.5rem;
  }
}
</style>
