<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Search } from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import type { Trainer } from '../types'
import {
  NCard,
  NInput,
  NInputGroup,
  NButton,
  NIcon,
  NGrid,
  NGridItem,
  NSpace,
  NText,
  NImage,
  NEmpty,
  NTag,
  NEllipsis,
  NSpin,
  NPagination,
} from 'naive-ui'

const router = useRouter()
const store = useTrainerStore()

const searchQuery = ref('')
const currentPage = ref(1)
const totalPages = ref(10) // TODO: 从后端获取总页数
const downloading = ref<string | null>(null)

const handleSearch = () => {
  store.searchTrainers(searchQuery.value)
}

const handlePageChange = (page: number) => {
  store.fetchTrainers(page)
}

const handleDownload = async (trainer: Trainer) => {
  if (downloading.value) return
  downloading.value = trainer.id
  try {
    await store.downloadTrainer(trainer)
    window.$message?.success('下载成功')
  } catch (error) {
    window.$message?.error('下载失败')
  } finally {
    downloading.value = null
  }
}

onMounted(() => {
  store.initialize()
})
</script>

<template>
  <div class="home-container">
    <NCard>
      <template #header>
        <div class="header-container">
          <h2>游戏修改器</h2>
          <NInputGroup>
            <NInput
              v-model:value="searchQuery"
              placeholder="搜索游戏修改器..."
              @keydown.enter="handleSearch"
              clearable
            />
            <NButton type="primary" @click="handleSearch" :disabled="store.isLoading">
              <template #icon>
                <NIcon><Search /></NIcon>
              </template>
              搜索
            </NButton>
          </NInputGroup>
        </div>
      </template>

      <NSpin :show="store.isLoading">
        <div v-if="store.error" class="error-message">
          {{ store.error }}
        </div>
        <NEmpty
          v-else-if="!store.trainers.length"
          description="暂无修改器数据"
          class="empty-state"
        />
        <template v-else>
          <NGrid :cols="3" :x-gap="16" :y-gap="16" responsive="screen">
            <NGridItem v-for="trainer in store.trainers" :key="trainer.id">
              <NCard class="trainer-card" hoverable>
                <template #cover>
                  <div class="image-container">
                    <NImage
                      :src="trainer.thumbnail"
                      :alt="trainer.name"
                      class="trainer-image"
                      :preview-disabled="true"
                      fallback-src="/placeholder.png"
                    />
                  </div>
                </template>
                <div class="card-content">
                  <NEllipsis :line-clamp="1" class="trainer-name">
                    {{ trainer.name }}
                  </NEllipsis>
                  <div class="trainer-info">
                    <NSpace vertical size="small">
                      <NText depth="3" class="info-text">
                        <span class="label">版本:</span>
                        <NEllipsis style="max-width: 150px; display: inline-block">
                          {{ trainer.version }}
                        </NEllipsis>
                      </NText>
                      <NText depth="3" class="info-text">
                        <span class="label">游戏版本:</span>
                        <NEllipsis style="max-width: 150px; display: inline-block">
                          {{ trainer.game_version }}
                        </NEllipsis>
                      </NText>
                      <NText depth="3" class="info-text">
                        <span class="label">更新时间:</span>
                        {{ new Date(trainer.last_update).toLocaleDateString() }}
                      </NText>
                    </NSpace>
                  </div>
                  <div class="trainer-actions">
                    <NSpace justify="end">
                      <NButton
                        type="primary"
                        size="small"
                        @click="router.push({ name: 'detail', params: { id: trainer.id } })"
                      >
                        详情
                      </NButton>
                      <NButton
                        type="success"
                        size="small"
                        :loading="downloading === trainer.id"
                        :disabled="downloading !== null"
                        @click="handleDownload(trainer)"
                      >
                        下载
                      </NButton>
                    </NSpace>
                  </div>
                </div>
              </NCard>
            </NGridItem>
          </NGrid>

          <div class="pagination-container">
            <NPagination
              v-model:page="currentPage"
              :page-count="totalPages"
              :on-update:page="handlePageChange"
            />
          </div>
        </template>
      </NSpin>
    </NCard>
  </div>
</template>

<style scoped>
.home-container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.header-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.error-message {
  text-align: center;
  color: var(--error-color);
  padding: 20px;
}

.empty-state {
  padding: 40px;
  text-align: center;
}

.trainer-card {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.image-container {
  position: relative;
  width: 100%;
  padding-top: 56.25%; /* 16:9 比例 */
  overflow: hidden;
}

.trainer-image {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.card-content {
  display: flex;
  flex-direction: column;
  flex: 1;
  padding: 12px;
}

.trainer-name {
  font-size: 1.1em;
  font-weight: 600;
  margin-bottom: 8px;
}

.trainer-info {
  flex: 1;
  margin-bottom: 12px;
}

.info-text {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9em;
}

.label {
  color: var(--text-color-3);
  white-space: nowrap;
}

.trainer-actions {
  margin-top: auto;
}

.pagination-container {
  margin-top: 20px;
  display: flex;
  justify-content: center;
}

/* 响应式布局 */
@media (max-width: 1200px) {
  :deep(.n-grid) {
    --cols: 2 !important;
  }
}

@media (max-width: 768px) {
  :deep(.n-grid) {
    --cols: 1 !important;
  }

  .header-container {
    flex-direction: column;
    gap: 16px;
  }
}
</style>
