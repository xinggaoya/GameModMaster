<template>
  <div class="trainer-skeleton">
    <div v-if="type === 'card'" class="card-skeleton">
      <!-- 卡片骨架 -->
      <div class="image-skeleton"></div>
      <div class="content-skeleton">
        <div class="title-skeleton"></div>
        <div class="tag-skeleton"></div>
        <div class="info-skeleton"></div>
        <div class="info-skeleton short"></div>
      </div>
    </div>

    <div v-else-if="type === 'detail'" class="detail-skeleton">
      <!-- 详情页骨架 -->
      <div class="header-skeleton">
        <div class="back-skeleton"></div>
        <div class="title-skeleton"></div>
      </div>
      <div class="detail-content">
        <div class="image-area">
          <div class="detail-image-skeleton"></div>
        </div>
        <div class="info-area">
          <div class="info-grid">
            <div class="info-item-skeleton" v-for="i in 4" :key="i"></div>
          </div>
          <div class="divider-skeleton"></div>
          <div class="buttons-skeleton">
            <div class="button-skeleton primary"></div>
            <div class="button-skeleton"></div>
          </div>
        </div>
      </div>
      <div class="divider-skeleton"></div>
      <div class="description-skeleton">
        <div class="desc-line" v-for="i in 6" :key="i"></div>
      </div>
    </div>

    <div v-else-if="type === 'list'" class="list-skeleton">
      <!-- 列表骨架 -->
      <div class="list-item-skeleton" v-for="i in count" :key="i">
        <div class="list-image-skeleton"></div>
        <div class="list-content-skeleton">
          <div class="list-title-skeleton"></div>
          <div class="list-info-skeleton"></div>
          <div class="list-info-skeleton short"></div>
        </div>
        <div class="list-actions-skeleton"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'

const props = defineProps({
  // 骨架屏类型：card(卡片), detail(详情页), list(列表项)
  type: {
    type: String,
    default: 'card',
    validator: (value: string) => ['card', 'detail', 'list'].includes(value),
  },
  // 列表项数量（当type为list时有效）
  count: {
    type: Number,
    default: 3,
  },
})
</script>

<style scoped>
.trainer-skeleton {
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
  }
  100% {
    opacity: 0.6;
  }
}

/* 卡片骨架样式 */
.card-skeleton {
  display: flex;
  flex-direction: column;
  border-radius: 8px;
  overflow: hidden;
  background-color: var(--card-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  height: 280px;
}

.image-skeleton {
  width: 100%;
  height: 160px;
  background-color: rgba(150, 150, 150, 0.1);
}

.content-skeleton {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.title-skeleton {
  height: 18px;
  width: 85%;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
}

.tag-skeleton {
  height: 20px;
  width: 60px;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
  margin-left: auto;
}

.info-skeleton {
  height: 14px;
  width: 100%;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
}

.info-skeleton.short {
  width: 70%;
}

/* 详情页骨架样式 */
.detail-skeleton {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.header-skeleton {
  display: flex;
  align-items: center;
  gap: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid rgba(150, 150, 150, 0.1);
}

.back-skeleton {
  width: 80px;
  height: 32px;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
}

.detail-content {
  display: grid;
  grid-template-columns: minmax(300px, 2fr) 3fr;
  gap: 32px;
}

.detail-image-skeleton {
  width: 100%;
  height: 200px;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 8px;
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 24px;
}

.info-item-skeleton {
  height: 50px;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
}

.divider-skeleton {
  height: 1px;
  width: 100%;
  background-color: rgba(150, 150, 150, 0.1);
  margin: 16px 0;
}

.buttons-skeleton {
  display: flex;
  justify-content: center;
  gap: 16px;
  margin-top: 24px;
}

.button-skeleton {
  height: 36px;
  width: 120px;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
}

.button-skeleton.primary {
  background-color: rgba(100, 150, 200, 0.1);
}

.description-skeleton {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.desc-line {
  height: 14px;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
}

.desc-line:nth-child(odd) {
  width: 95%;
}

.desc-line:nth-child(even) {
  width: 85%;
}

/* 列表骨架样式 */
.list-skeleton {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.list-item-skeleton {
  display: flex;
  align-items: center;
  padding: 12px;
  gap: 16px;
  background-color: var(--card-color);
  border-radius: 4px;
  height: 80px;
}

.list-image-skeleton {
  width: 100px;
  height: 56px;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
  flex-shrink: 0;
}

.list-content-skeleton {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1;
}

.list-title-skeleton {
  height: 18px;
  width: 60%;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
}

.list-info-skeleton {
  height: 12px;
  width: 80%;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
}

.list-info-skeleton.short {
  width: 50%;
}

.list-actions-skeleton {
  width: 80px;
  height: 28px;
  background-color: rgba(150, 150, 150, 0.1);
  border-radius: 4px;
  flex-shrink: 0;
}

/* 媒体查询适配 */
@media (max-width: 768px) {
  .detail-content {
    grid-template-columns: 1fr;
  }

  .list-item-skeleton {
    height: auto;
    flex-direction: column;
    align-items: flex-start;
  }

  .list-image-skeleton {
    width: 100%;
    height: 120px;
  }

  .list-content-skeleton {
    width: 100%;
  }
}
</style>
