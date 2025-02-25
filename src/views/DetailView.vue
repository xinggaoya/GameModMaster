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
          <NSpace vertical size="large">
            <!-- 上半部分：封面和信息 -->
            <NGrid :cols="24" :x-gap="16">
              <!-- 左侧：封面图 -->
              <NGridItem :span="24" :md="8">
                <NCard class="cover-card" :bordered="false" content-style="padding: 0;">
                  <div class="cover-wrapper">
                    <NImage
                      :src="trainer.thumbnail"
                      :alt="trainer.name"
                      class="cover-image"
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
              </NGridItem>

              <!-- 右侧：信息和操作按钮 -->
              <NGridItem :span="24" :md="16">
                <NSpace vertical size="large" style="height: 100%">
                  <!-- 信息区 -->
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
                            <div class="info-value">
                              {{
                                typeof trainer.download_count === 'string'
                                  ? parseInt(trainer.download_count) || 0
                                  : trainer.download_count || 0
                              }}
                            </div>
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
                            <div class="info-label">最后更新</div>
                            <div class="info-value">{{ formattedDate }}</div>
                          </div>
                        </NFlex>
                      </NCard>
                    </NGridItem>
                  </NGrid>
                </NSpace>
              </NGridItem>
            </NGrid>

            <!-- 统一的操作按钮区域 -->
            <NSpace vertical size="small" class="action-buttons">
              <NButton
                block
                type="primary"
                size="large"
                :loading="downloading"
                @click="handleDownload"
              >
                {{ isDownloaded ? '重新下载' : '下载修改器' }}
                <template #icon>
                  <NIcon>
                    <DownloadOutline v-if="!isDownloaded" />
                    <RefreshOutline v-else />
                  </NIcon>
                </template>
              </NButton>

              <NFlex :wrap="false" justify="space-between" v-if="isDownloaded">
                <NButton
                  type="success"
                  size="large"
                  @click="handleLaunch"
                  style="flex: 1; margin-right: 8px"
                >
                  <template #icon>
                    <NIcon><PlayOutline /></NIcon>
                  </template>
                  启动
                </NButton>

                <NButton
                  type="error"
                  size="large"
                  :loading="deleting"
                  @click="confirmDelete"
                  style="flex: 1"
                >
                  <template #icon>
                    <NIcon><TrashOutline /></NIcon>
                  </template>
                  删除
                </NButton>
              </NFlex>
            </NSpace>

            <!-- 下半部分：标签页信息 -->
            <NTabs type="segment" size="large" animated>
              <NTab name="description" tab="详细介绍">
                <NCard class="tab-card">
                  <div class="description-content" v-html="trainer.description"></div>
                </NCard>
              </NTab>

              <NTab name="instructions" tab="使用说明">
                <NCard class="tab-card">
                  <h3>使用说明</h3>
                  <ol class="instruction-list">
                    <li>下载后，系统会自动安装修改器</li>
                    <li>点击启动按钮，等待游戏与修改器一起启动</li>
                    <li>
                      游戏中通常使用 <NTag size="small">F1</NTag> -
                      <NTag size="small">F12</NTag> 等按键激活修改器菜单
                    </li>
                    <li>调整修改器设置，开启您需要的功能</li>
                    <li>享受游戏！</li>
                  </ol>

                  <NDivider />

                  <NFlex align="center" class="note-box">
                    <NIcon color="#f90" size="20"><AlertCircleOutline /></NIcon>
                    <span>注意：请遵守游戏社区规则，仅在单人游戏模式中使用修改器。</span>
                  </NFlex>
                </NCard>
              </NTab>

              <NTab v-if="isDownloaded" name="details" tab="安装信息">
                <NCard class="tab-card">
                  <NDescriptions label-placement="left" :column="1" bordered>
                    <NDescriptionsItem label="安装路径">
                      <span class="ellipsis-text">
                        {{ getInstallInfo()?.installed_path || '未知' }}
                      </span>
                    </NDescriptionsItem>
                    <NDescriptionsItem label="安装时间">
                      {{
                        getInstallInfo()?.install_time
                          ? formatDate(getInstallInfo()?.install_time)
                          : '未知'
                      }}
                    </NDescriptionsItem>
                    <NDescriptionsItem label="上次启动">
                      {{
                        getInstallInfo()?.last_launch_time
                          ? formatDate(getInstallInfo()?.last_launch_time)
                          : '未启动过'
                      }}
                    </NDescriptionsItem>
                  </NDescriptions>
                </NCard>
              </NTab>
            </NTabs>
          </NSpace>
        </NCard>
      </template>

      <!-- 未找到游戏的情况 -->
      <template v-else-if="!loading">
        <NResult status="404" title="未找到修改器" description="您请求的修改器不存在或已被删除">
          <template #footer>
            <NButton @click="router.push('/')">返回首页</NButton>
          </template>
        </NResult>
      </template>

      <!-- 骨架屏 -->
      <template #description>
        <NSkeleton v-if="loading" text :repeat="6" />
      </template>
    </NSpin>

    <!-- 删除确认对话框 -->
    <NModal
      v-model:show="showDeleteModal"
      preset="dialog"
      title="确认删除"
      positive-text="确认"
      negative-text="取消"
      @positive-click="handleDelete"
    >
      <template #icon>
        <NIcon><AlertCircleOutline /></NIcon>
      </template>
      <div>确定要删除修改器"{{ trainer?.name }}"吗？该操作无法撤销。</div>
    </NModal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  ArrowBackOutline,
  DownloadOutline,
  PlayOutline,
  TrashOutline,
  TimeOutline,
  RefreshOutline,
  CodeOutline,
  GameControllerOutline,
  AlertCircleOutline,
} from '@vicons/ionicons5'
import { useMessage } from 'naive-ui'
import { useTrainerStore } from '../stores/trainer'
import type { Trainer } from '../types'

const route = useRoute()
const router = useRouter()
const store = useTrainerStore()
const message = useMessage()

const loading = ref(true)
const downloading = ref(false)
const deleting = ref(false)
const trainer = ref<Trainer | null>(null)
const showDeleteModal = ref(false)

const isDownloaded = computed(() => {
  return store.downloadedTrainers.some((t) => t.id === trainer.value?.id)
})

const formattedDate = computed(() => {
  if (!trainer.value?.last_update) return '未知'
  return formatDate(trainer.value.last_update)
})

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

const getInstallInfo = () => {
  if (!trainer.value) return null
  return store.installedTrainers.find((t) => t.id === trainer.value?.id)
}

const fetchTrainerDetail = async () => {
  loading.value = true
  try {
    const result = await store.getTrainerDetail(route.params.id as string)
    trainer.value = result
  } catch (error) {
    message.error('获取修改器详情失败')
  } finally {
    loading.value = false
  }
}

const handleDownload = async () => {
  if (!trainer.value) return
  downloading.value = true
  try {
    await store.downloadTrainer(trainer.value)
    message.success('下载成功')
  } catch (error) {
    console.error('下载失败:', error)
    message.error(error instanceof Error ? error.message : '下载失败')
  } finally {
    downloading.value = false
  }
}

const handleLaunch = async () => {
  if (!trainer.value) return
  try {
    await store.launchTrainer(trainer.value.id)
    message.success('启动成功')
  } catch (error) {
    message.error('启动失败')
  }
}

const confirmDelete = () => {
  showDeleteModal.value = true
}

const handleDelete = async () => {
  if (!trainer.value) return
  deleting.value = true
  try {
    await store.deleteTrainer(trainer.value.id)
    message.success('删除成功')
    router.back()
  } catch (error) {
    message.error('删除失败')
  } finally {
    deleting.value = false
  }
}

onMounted(() => {
  fetchTrainerDetail()
})
</script>

<style scoped>
.detail-view {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  min-height: 70vh;
}

.header-wrapper {
  margin-bottom: 16px;
}

.back-button {
  margin-right: 8px;
}

.title-container {
  flex: 1;
  text-align: center;
}

.game-title {
  font-size: 1.75rem;
  font-weight: 700;
  margin: 0;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  font-family: var(--font-gaming);
}

.detail-card {
  margin-bottom: 24px;
  border-radius: 12px;
}

.cover-card {
  border-radius: 12px;
  overflow: hidden;
  box-shadow: var(--shadow-md);
}

.cover-wrapper {
  position: relative;
  aspect-ratio: 16/9;
}

.cover-image {
  width: 100%;
  height: 100%;
}

.status-badge {
  position: absolute;
  bottom: 12px;
  right: 12px;
  z-index: 5;
}

.icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background-color: var(--bg-tertiary);
  color: var(--primary);
  margin-right: 12px;
}

.info-card {
  height: 100%;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
}

.info-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-sm);
}

.info-label {
  font-size: 0.85rem;
  color: var(--text-tertiary);
}

.info-value {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.action-buttons {
  width: 100%;
  max-width: 600px;
  margin: 0 auto;
}

.tab-card {
  margin-top: 12px;
  background-color: var(--bg-secondary);
  border-radius: 12px;
}

.instruction-list {
  padding-left: 20px;
  line-height: 1.8;
}

.note-box {
  padding: 12px;
  background-color: rgba(255, 153, 0, 0.1);
  border-left: 4px solid #f90;
  border-radius: 4px;
  gap: 8px;
}

.description-content {
  line-height: 1.6;
}

.description-content :deep(p) {
  margin: 12px 0;
}

.ellipsis-text {
  display: block;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 删除旧的响应式布局控制 */
@media (max-width: 768px) {
  .cover-card {
    margin-bottom: 16px;
  }

  .game-title {
    font-size: 1.5rem;
  }
}
</style>
