<template>
  <div
    :class="['modern-trainer-card', { 'compact-mode': compact, 'loading': loading }]"
    @click="handleClick"
  >
    <!-- 非紧凑模式：完整卡片布局 -->
    <div v-if="!compact" class="card-full">
      <!-- 图片区域 -->
      <div class="card-image-section">
        <div class="image-wrapper">
          <NImage
            :src="trainer.thumbnail"
            :alt="trainer.name"
            class="trainer-image"
            :preview-disabled="true"
            fallback-src="/placeholder.png"
            object-fit="cover"
          />
          <!-- 悬停遮罩 -->
          <div class="image-overlay">
            <div class="overlay-content">
              <NButton quaternary circle class="detail-button">
                <template #icon>
                  <NIcon><Search /></NIcon>
                </template>
              </NButton>
            </div>
          </div>
        </div>

        <!-- 状态标签 -->
        <div class="status-badge">
          <NTag size="small" :bordered="false" type="success">
            {{ trainer.version }}
          </NTag>
        </div>
      </div>

      <!-- 内容区域 -->
      <div class="card-content">
        <div class="trainer-header">
          <NEllipsis :line-clamp="2" class="trainer-name">
            {{ trainer.name }}
          </NEllipsis>
        </div>

        <div class="trainer-info">
          <div class="info-item">
            <NIcon class="info-icon">
              <GameControllerOutline />
            </NIcon>
            <span class="info-text">{{ trainer.game_version }}</span>
          </div>

          <div class="info-item">
            <NIcon class="info-icon">
              <TimeOutline />
            </NIcon>
            <span class="info-text">{{ formatDate(trainer.last_update) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 紧凑模式：横向布局 -->
    <div v-else class="card-compact">
      <!-- 左侧图片 -->
      <div class="compact-image">
        <NImage
          :src="trainer.thumbnail"
          :alt="trainer.name"
          class="trainer-image-compact"
          :preview-disabled="true"
          fallback-src="/placeholder.png"
        />
      </div>

      <!-- 中间内容 -->
      <div class="compact-content">
        <div class="compact-header">
          <NEllipsis :line-clamp="1" class="compact-name">
            {{ trainer.name }}
          </NEllipsis>
          <NTag size="small" :bordered="false" type="success">
            {{ trainer.version }}
          </NTag>
        </div>

        <div class="compact-info">
          <span class="version-info">{{ trainer.game_version }}</span>
          <span class="update-info">{{ formatDate(trainer.last_update) }}</span>
        </div>
      </div>

      <!-- 右侧操作按钮 -->
      <div class="compact-actions">
        <slot name="actions"></slot>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'
import {
  Search,
  GameControllerOutline,
  TimeOutline
} from '@vicons/ionicons5'
import { NImage, NEllipsis, NTag, NButton, NIcon } from 'naive-ui'
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
  loading: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['click'])

// 格式化日期
const formatDate = (dateString: string) => {
  if (!dateString) return '未知'
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
  })
}

// 处理点击事件
const handleClick = (e: MouseEvent) => {
  if (!props.loading) {
    emit('click', e)
  }
}
</script>

<style scoped>
/* 现代化修改器卡片 */
.modern-trainer-card {
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

.modern-trainer-card:hover {
  transform: translateY(-6px) scale(1.02);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
}

.modern-trainer-card.loading {
  opacity: 0.7;
  pointer-events: none;
}

/* 非紧凑模式：完整卡片 */
.card-full {
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

.trainer-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.4s ease;
}

.modern-trainer-card:hover .trainer-image {
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

.modern-trainer-card:hover .image-overlay {
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

.modern-trainer-card:hover .overlay-content {
  transform: translateY(0);
}

.detail-button {
  color: #fff !important;
  background: rgba(255, 255, 255, 0.2) !important;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  transition: all 0.3s ease;
}

.detail-button:hover {
  background: rgba(255, 255, 255, 0.3) !important;
  transform: scale(1.1);
}

/* 状态标签 */
.status-badge {
  position: absolute;
  top: 1rem;
  right: 1rem;
  z-index: 2;
}

/* 内容区域 */
.card-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 1.25rem;
  gap: 1rem;
}

.trainer-header {
  margin-bottom: 0.5rem;
}

.trainer-name {
  font-size: 1.125rem;
  font-weight: 700;
  color: #1f2937;
  line-height: 1.3;
  margin-bottom: 0.5rem;
}

.trainer-info {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 12px;
  font-size: 0.875rem;
}

.info-icon {
  color: #7c3aed;
  font-size: 1rem;
}

.info-text {
  font-weight: 600;
  color: #4b5563;
  line-height: 1;
}

/* 紧凑模式：横向布局 */
.card-compact {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  height: 100%;
}

.compact-image {
  width: 80px;
  height: 45px;
  flex-shrink: 0;
  border-radius: 12px;
  overflow: hidden;
}

.trainer-image-compact {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.modern-trainer-card:hover .trainer-image-compact {
  transform: scale(1.1);
}

.compact-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.compact-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
}

.compact-name {
  font-size: 1rem;
  font-weight: 700;
  color: #1f2937;
  flex: 1;
  min-width: 0;
}

.compact-info {
  display: flex;
  align-items: center;
  gap: 1rem;
  font-size: 0.75rem;
  color: #6b7280;
}

.version-info,
.update-info {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.compact-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-shrink: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .modern-trainer-card {
    border-radius: 16px;
  }

  .card-content {
    padding: 1rem;
    gap: 0.75rem;
  }

  .trainer-name {
    font-size: 1rem;
  }

  .info-item {
    padding: 0.375rem 0.5rem;
    font-size: 0.8rem;
  }

  .card-compact {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 0.75rem;
  }

  .compact-image {
    width: 100%;
    height: auto;
    aspect-ratio: 16/9;
  }

  .compact-actions {
    margin-top: 0.5rem;
    width: 100%;
    justify-content: flex-end;
  }
}

@media (max-width: 480px) {
  .card-content {
    padding: 0.75rem;
  }

  .trainer-name {
    font-size: 0.875rem;
  }

  .compact-name {
    font-size: 0.875rem;
  }

  .compact-info {
    font-size: 0.688rem;
    gap: 0.75rem;
  }
}

/* 暗色主题 */
:deep(.dark-theme) .modern-trainer-card {
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .trainer-name,
:deep(.dark-theme) .compact-name {
  color: #e2e8f0;
}

:deep(.dark-theme) .info-item {
  background: rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .info-text {
  color: #cbd5e1;
}

:deep(.dark-theme) .compact-info {
  color: #94a3b8;
}

:deep(.dark-theme) .detail-button {
  background: rgba(255, 255, 255, 0.1) !important;
  border-color: rgba(255, 255, 255, 0.2);
}

:deep(.dark-theme) .detail-button:hover {
  background: rgba(255, 255, 255, 0.2) !important;
}
</style>
