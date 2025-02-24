<script setup lang="ts">
import { onMounted, defineComponent } from 'vue'
import { useTrainerStore } from '../stores/trainer'
import { NCard, NSpace, NImage, NButton, NEmpty } from 'naive-ui'

defineComponent({
  name: 'TrainerList',
})

const store = useTrainerStore()

onMounted(async () => {
  console.log('TrainerList: 开始初始化')
  try {
    await store.initialize()
    console.log('TrainerList: 初始化完成', {
      trainers: store.trainers.length,
      isLoading: store.isLoading,
      error: store.error,
    })
  } catch (err) {
    console.error('TrainerList: 初始化失败:', err)
  }
})
</script>

<template>
  <div class="trainer-list">
    <div v-if="store.error" class="error-message">
      {{ store.error }}
    </div>
    <div v-else-if="store.trainers.length === 0 && !store.isLoading" class="empty-state">
      <NEmpty description="暂无修改器数据" />
    </div>
    <div v-else class="trainer-grid">
      <NCard v-for="trainer in store.trainers" :key="trainer.id" class="trainer-card" hoverable>
        <template #cover>
          <div class="image-wrapper">
            <NImage
              :src="trainer.thumbnail"
              :alt="trainer.name"
              object-fit="cover"
              class="trainer-image"
              :preview-disabled="true"
              fallback-src="/placeholder.png"
            />
          </div>
        </template>
        <NSpace vertical>
          <h3 class="trainer-name">{{ trainer.name }}</h3>
          <div class="trainer-info">
            <p>版本: {{ trainer.version }}</p>
            <p>游戏版本: {{ trainer.game_version }}</p>
            <p>更新时间: {{ new Date(trainer.last_update).toLocaleDateString() }}</p>
          </div>
          <div class="trainer-actions">
            <NButton
              type="primary"
              @click="store.downloadTrainer(trainer)"
              :disabled="store.isLoading"
            >
              下载
            </NButton>
          </div>
        </NSpace>
      </NCard>
    </div>
  </div>
</template>

<style scoped>
.trainer-list {
  padding: 20px;
  min-height: 100%;
  position: relative;
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

.trainer-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.trainer-card {
  transition: transform 0.2s;
}

.trainer-card:hover {
  transform: translateY(-5px);
}

.image-wrapper {
  position: relative;
  width: 100%;
  height: 200px;
  overflow: hidden;
}

.trainer-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.trainer-name {
  margin: 0;
  font-size: 1.2em;
  font-weight: 600;
}

.trainer-info {
  font-size: 0.9em;
  color: var(--text-color-2);
}

.trainer-info p {
  margin: 5px 0;
}

.trainer-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 10px;
}
</style>
