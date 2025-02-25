<template>
  <NCard :class="['trainer-card', { 'compact-mode': compact }]" hoverable @click="handleClick">
    <template #cover v-if="!compact">
      <div class="image-container">
        <NImage
          :src="trainer.thumbnail"
          :alt="trainer.name"
          class="trainer-image"
          :preview-disabled="true"
          fallback-src="/placeholder.png"
          object-fit="cover"
        />
        <div class="image-overlay">
          <slot name="overlay-content">
            <NButton quaternary circle class="detail-button">
              <template #icon>
                <NIcon><Search /></NIcon>
              </template>
            </NButton>
          </slot>
        </div>
      </div>
    </template>

    <div :class="['card-content', { 'with-image': compact }]">
      <div v-if="compact" class="compact-image">
        <NImage
          :src="trainer.thumbnail"
          :alt="trainer.name"
          class="trainer-image-compact"
          :preview-disabled="true"
          fallback-src="/placeholder.png"
        />
      </div>

      <div class="trainer-content">
        <div class="trainer-header">
          <NEllipsis :line-clamp="1" class="trainer-name">
            {{ trainer.name }}
          </NEllipsis>
          <NTag size="small" :bordered="false" type="success">
            {{ trainer.version }}
          </NTag>
        </div>

        <div class="trainer-info">
          <NTooltip trigger="hover">
            <template #trigger>
              <NText depth="3" class="game-version">
                支持游戏版本: {{ trainer.game_version }}
              </NText>
            </template>
            {{ trainer.game_version }}
          </NTooltip>

          <NText depth="3" class="update-time">
            更新于: {{ formatDate(trainer.last_update) }}
          </NText>

          <slot name="extra-info"></slot>
        </div>
      </div>

      <div v-if="compact" class="action-buttons">
        <slot name="actions"></slot>
      </div>
    </div>
  </NCard>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'
import { Search } from '@vicons/ionicons5'
import { NCard, NImage, NEllipsis, NTag, NText, NTooltip, NButton, NIcon } from 'naive-ui'
import type { Trainer } from '@/types'

const props = defineProps({
  trainer: {
    type: Object as () => Trainer,
    required: true,
  },
  compact: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['click'])

const formatDate = (dateString: string) => {
  try {
    return new Date(dateString).toLocaleDateString()
  } catch (e) {
    return dateString
  }
}

const handleClick = (e: MouseEvent) => {
  emit('click', e)
}
</script>

<style scoped>
.trainer-card {
  height: 100%;
  transition: all var(--transition-duration) var(--transition-timing);
  cursor: pointer;
}

.trainer-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 12px var(--shadow-color);
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
  transition: transform var(--transition-duration) var(--transition-timing);
}

.image-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity var(--transition-duration) var(--transition-timing);
}

.trainer-card:hover .image-overlay {
  opacity: 1;
}

.trainer-card:hover .trainer-image {
  transform: scale(1.05);
}

.detail-button {
  color: #fff !important;
  background: rgba(255, 255, 255, 0.2) !important;
  backdrop-filter: blur(4px);
}

.detail-button:hover {
  background: rgba(255, 255, 255, 0.3) !important;
}

.card-content {
  padding: 16px;
}

.with-image {
  display: flex;
  padding: 12px;
  gap: 16px;
  align-items: center;
}

.compact-image {
  width: 100px;
  height: 56px;
  flex-shrink: 0;
  border-radius: 4px;
  overflow: hidden;
}

.trainer-image-compact {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform var(--transition-duration) var(--transition-timing);
}

.trainer-card:hover .trainer-image-compact {
  transform: scale(1.05);
}

.trainer-content {
  flex: 1;
  min-width: 0;
}

.trainer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  margin-bottom: 12px;
}

.trainer-name {
  font-size: var(--font-size-base);
  font-weight: 600;
  flex: 1;
  min-width: 0;
}

.compact-mode .trainer-name {
  font-size: var(--font-size-small);
}

.trainer-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.game-version,
.update-time {
  font-size: var(--font-size-small);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.compact-mode .game-version,
.compact-mode .update-time {
  font-size: var(--font-size-mini);
}

.action-buttons {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  flex-shrink: 0;
}

/* 响应式布局 */
@media (max-width: 768px) {
  .with-image {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .compact-image {
    width: 100%;
    height: auto;
    aspect-ratio: 16 / 9;
  }

  .action-buttons {
    margin-top: 12px;
    width: 100%;
    justify-content: flex-end;
  }
}
</style>
