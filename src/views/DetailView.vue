<template>
  <div class="detail-container">
    <NCard>
      <template #header>
        <div class="header-container">
          <NButton @click="router.back()" class="back-button">
            <template #icon>
              <NIcon><ArrowBack /></NIcon>
            </template>
            返回
          </NButton>
          <h2 class="trainer-title">{{ trainer?.name }}</h2>
        </div>
      </template>

      <NSpin :show="loading">
        <template v-if="trainer">
          <div class="trainer-detail">
            <!-- 基本信息部分 -->
            <div class="basic-info">
              <div class="image-section">
                <NImage
                  :src="trainer.thumbnail"
                  :alt="trainer.name"
                  class="trainer-thumbnail"
                  :preview-disabled="true"
                  fallback-src="/placeholder.png"
                />
              </div>

              <div class="info-section">
                <NGrid :cols="2" :x-gap="12" :y-gap="12">
                  <NGridItem>
                    <NStatistic label="版本" :value="trainer.version" />
                  </NGridItem>
                  <NGridItem>
                    <NStatistic label="游戏版本" :value="trainer.game_version" />
                  </NGridItem>
                  <NGridItem>
                    <NStatistic label="下载次数" :value="trainer.download_count" />
                  </NGridItem>
                  <NGridItem>
                    <NStatistic label="最后更新" :value="formattedDate" />
                  </NGridItem>
                </NGrid>

                <NDivider />

                <NSpace class="action-buttons" justify="center" size="large">
                  <n-button
                    type="primary"
                    size="large"
                    :loading="downloading"
                    @click="handleDownload"
                  >
                    {{ isDownloaded ? '重新下载' : '下载修改器' }}
                  </n-button>
                  <n-button
                    v-if="isDownloaded"
                    type="error"
                    size="large"
                    :loading="deleting"
                    @click="handleDelete"
                  >
                    删除
                  </n-button>
                </NSpace>
              </div>
            </div>

            <!-- 详细信息部分 -->
            <NDivider>详细说明</NDivider>
            <div class="description-section">
              <NCard class="description-card">
                <div class="description-content" v-html="trainer.description"></div>
              </NCard>
            </div>
          </div>
        </template>
      </NSpin>
    </NCard>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ArrowBack } from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import type { Trainer } from '../types'
import {
  NCard,
  NButton,
  NIcon,
  NSpin,
  NDescriptions,
  NDescriptionsItem,
  NSpace,
  NImage,
  NDivider,
  NTag,
  NGrid,
  NGridItem,
  NStatistic,
} from 'naive-ui'

const route = useRoute()
const router = useRouter()
const store = useTrainerStore()

const loading = ref(true)
const downloading = ref(false)
const deleting = ref(false)
const trainer = ref<Trainer | null>(null)

const isDownloaded = computed(() => {
  return store.downloadedTrainers.some((t) => t.id === trainer.value?.id)
})

const formattedDate = computed(() => {
  if (!trainer.value?.last_update) return ''
  return new Date(trainer.value.last_update).toLocaleString()
})

const fetchTrainerDetail = async () => {
  loading.value = true
  try {
    const result = await store.getTrainerDetail(route.params.id as string)
    trainer.value = result
  } catch (error) {
    window.$message?.error('获取修改器详情失败')
  } finally {
    loading.value = false
  }
}

const handleDownload = async () => {
  if (!trainer.value) return
  downloading.value = true
  try {
    await store.downloadTrainer(trainer.value)
    window.$message?.success('下载成功')
  } catch (error) {
    console.error('下载失败:', error)
    window.$message?.error(error instanceof Error ? error.message : '下载失败')
  } finally {
    downloading.value = false
  }
}

const handleDelete = async () => {
  if (!trainer.value) return
  deleting.value = true
  try {
    await store.deleteTrainer(trainer.value.id)
    window.$message?.success('删除成功')
    router.back()
  } catch (error) {
    window.$message?.error('删除失败')
  } finally {
    deleting.value = false
  }
}

onMounted(() => {
  fetchTrainerDetail()
})
</script>

<style scoped>
.header-container {
  display: flex;
  align-items: center;
  gap: 16px;
}

.back-button {
  flex-shrink: 0;
}

.trainer-title {
  margin: 0;
  font-size: 1.5em;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.trainer-detail {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.basic-info {
  display: grid;
  grid-template-columns: minmax(300px, 2fr) 3fr;
  gap: 32px;
  align-items: start;
}

.image-section {
  position: relative;
  width: 100%;
  border-radius: 8px;
  overflow: hidden;
}

.trainer-thumbnail {
  width: 100%;
  aspect-ratio: 16/9;
  object-fit: cover;
}

.info-section {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.action-buttons {
  margin-top: auto;
}

.description-section {
  margin-top: 16px;
}

.description-card {
  width: 100%;
  background-color: var(--card-color);
}

.description-content {
  line-height: 1.6;
}

.description-content :deep(p) {
  margin: 8px 0;
}

/* 响应式布局 */
@media (max-width: 1024px) {
  .basic-info {
    grid-template-columns: 1fr;
  }

  .image-section {
    max-width: 600px;
    margin: 0 auto;
  }
}

@media (max-width: 768px) {
  .header-container {
    flex-direction: column;
    align-items: flex-start;
  }

  .trainer-title {
    font-size: 1.2em;
  }
}
</style>
