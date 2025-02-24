<template>
  <div class="downloads-container">
    <NCard title="已下载的修改器">
      <div v-if="!store.downloadedTrainers.length" class="empty-state">
        <NEmpty description="暂无已下载的修改器" />
      </div>
      <div v-else class="downloads-list">
        <NCard
          v-for="trainer in store.downloadedTrainers"
          :key="trainer.id"
          class="trainer-card"
          size="small"
        >
          <div class="card-content">
            <!-- 左侧图片 -->
            <div class="card-image">
              <NImage
                :src="trainer.thumbnail"
                :alt="trainer.name"
                class="trainer-image"
                :preview-disabled="true"
                fallback-src="/placeholder.png"
              />
            </div>

            <!-- 中间信息 -->
            <div class="info-content">
              <div class="info-header">
                <NEllipsis :line-clamp="1" class="trainer-name">
                  {{ trainer.name }}
                </NEllipsis>
                <NTag size="tiny" :bordered="false" type="success">{{ trainer.version }}</NTag>
              </div>
              <div class="info-body">
                <div class="game-info">
                  <NText depth="3" class="game-version">游戏版本: {{ trainer.game_version }}</NText>
                  <NText depth="3" class="install-time" v-if="getInstallInfo(trainer.id)">
                    安装于: {{ formatDate(getInstallInfo(trainer.id)?.install_time) }}
                  </NText>
                </div>
                <div class="launch-info" v-if="getInstallInfo(trainer.id)?.last_launch_time">
                  <NText depth="3">
                    上次启动: {{ formatDate(getInstallInfo(trainer.id)?.last_launch_time) }}
                  </NText>
                </div>
              </div>
            </div>

            <!-- 右侧按钮组 -->
            <div class="action-buttons">
              <NSpace align="center" :size="8">
                <NButton
                  size="small"
                  quaternary
                  circle
                  type="info"
                  @click="router.push({ name: 'detail', params: { id: trainer.id } })"
                >
                  <template #icon>
                    <NIcon><InformationCircleOutline /></NIcon>
                  </template>
                </NButton>
                <NButton
                  size="small"
                  quaternary
                  circle
                  type="success"
                  @click="handleLaunch(trainer)"
                >
                  <template #icon>
                    <NIcon><PlayOutline /></NIcon>
                  </template>
                </NButton>
                <NButton size="small" quaternary circle type="error" @click="handleDelete(trainer)">
                  <template #icon>
                    <NIcon><TrashBinOutline /></NIcon>
                  </template>
                </NButton>
              </NSpace>
            </div>
          </div>
        </NCard>
      </div>
    </NCard>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTrainerStore } from '../stores/trainer'
import type { Trainer } from '../types'
import { NCard, NEmpty, NImage, NSpace, NText, NButton, NEllipsis, NTag, NIcon } from 'naive-ui'
import { InformationCircleOutline, PlayOutline, TrashBinOutline } from '@vicons/ionicons5'

const router = useRouter()
const store = useTrainerStore()

onMounted(() => {
  store.initialize()
})

const handleLaunch = async (trainer: Trainer) => {
  try {
    await store.launchTrainer(trainer.id)
    window.$message?.success('启动成功')
  } catch (error) {
    window.$message?.error('启动失败')
  }
}

const handleDelete = async (trainer: Trainer) => {
  try {
    await store.deleteTrainer(trainer.id)
    window.$message?.success('删除成功')
  } catch (error) {
    window.$message?.error('删除失败')
  }
}

const getInstallInfo = (trainerId: string) => {
  return store.installedTrainers.find((t) => t.id === trainerId)
}

const formatDate = (dateString: string | null | undefined) => {
  if (!dateString) return '未知'
  return new Date(dateString).toLocaleString()
}
</script>

<style scoped>
.downloads-container {
  padding: 16px;
  max-width: 1200px;
  margin: 0 auto;
}

.empty-state {
  padding: 32px;
  text-align: center;
}

.downloads-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.trainer-card {
  transition: all 0.2s ease;
  border: 1px solid var(--divider-color);
}

.trainer-card:hover {
  transform: translateX(2px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.card-content {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px;
}

.card-image {
  width: 100px;
  height: 56px;
  flex-shrink: 0;
  border-radius: 4px;
  overflow: hidden;
}

.trainer-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.trainer-card:hover .trainer-image {
  transform: scale(1.05);
}

.info-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.info-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.trainer-name {
  flex: 1;
  font-size: 14px;
  font-weight: 500;
  line-height: 1.4;
  min-width: 0;
}

.info-body {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.game-info {
  display: flex;
  gap: 16px;
}

.game-version,
.install-time {
  font-size: 12px;
}

.launch-info {
  font-size: 12px;
}

.action-buttons {
  flex-shrink: 0;
}

.icon {
  font-size: 16px;
  line-height: 1;
}

/* 响应式布局 */
@media (max-width: 640px) {
  .downloads-container {
    padding: 12px;
  }

  .card-content {
    padding: 8px;
    gap: 12px;
  }

  .card-image {
    width: 80px;
    height: 45px;
  }

  .info-content {
    gap: 6px;
  }

  .game-info {
    flex-direction: column;
    gap: 4px;
  }

  .trainer-name {
    font-size: 13px;
  }

  .game-version,
  .install-time,
  .launch-info {
    font-size: 11px;
  }
}
</style>
