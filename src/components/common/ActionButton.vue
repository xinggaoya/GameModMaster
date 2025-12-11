<template>
  <div class="modern-action-button-wrapper">
    <NButton
      :size="size"
      :quaternary="quaternary"
      :circle="circle"
      :type="type"
      :loading="loading"
      :disabled="disabled"
      v-bind="$attrs"
      :class="['modern-action-button', buttonClass]"
    >
      <template #icon v-if="icon">
        <NIcon class="button-icon">
          <component :is="icon" />
        </NIcon>
      </template>
      <span v-if="$slots.default" class="button-text">
        <slot></slot>
      </span>
    </NButton>
  </div>
</template>

<script setup lang="ts">
import { computed, useSlots } from 'vue'
import { NButton, NIcon } from 'naive-ui'
import type { Component } from 'vue'

// 引入Naive UI的类型
type ButtonSize = 'tiny' | 'small' | 'medium' | 'large'
type ButtonType = 'default' | 'tertiary' | 'primary' | 'info' | 'success' | 'warning' | 'error'

const props = defineProps({
  // 按钮尺寸
  size: {
    type: String as () => ButtonSize,
    default: 'medium',
  },
  // 是否为四级按钮（最低强调）
  quaternary: {
    type: Boolean,
    default: false,
  },
  // 是否为圆形按钮
  circle: {
    type: Boolean,
    default: false,
  },
  // 按钮类型
  type: {
    type: String as () => ButtonType,
    default: 'default',
  },
  // 图标组件
  icon: {
    type: Object as () => Component,
    default: null,
  },
  // 加载状态
  loading: {
    type: Boolean,
    default: false,
  },
  // 禁用状态
  disabled: {
    type: Boolean,
    default: false,
  },
  // 变体样式
  variant: {
    type: String as () => 'default' | 'gradient' | 'glass' | 'outlined',
    default: 'default',
  },
})

const slots: ReturnType<typeof useSlots> = useSlots()

// 计算按钮样式类
const buttonClass = computed((): Record<string, boolean> => {
  return {
    [`variant-${props.variant}`]: true,
    'with-icon': !!props.icon,
    'with-text': !props.circle && !!slots.default,
  }
})
</script>

<style scoped>
/* 现代化操作按钮 */
.modern-action-button-wrapper {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.modern-action-button {
  position: relative;
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 16px;
  font-weight: 600;
  letter-spacing: 0.02em;
}

/* 默认变体 */
.modern-action-button.variant-default {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  color: #374151;
}

.modern-action-button.variant-default:hover {
  background: rgba(255, 255, 255, 1);
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.1);
}

/* 渐变变体 */
.modern-action-button.variant-gradient {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  border: none;
  color: white;
  position: relative;
}

.modern-action-button.variant-gradient::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  transition: left 0.5s ease;
}

.modern-action-button.variant-gradient:hover::before {
  left: 100%;
}

.modern-action-button.variant-gradient:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 30px rgba(124, 58, 237, 0.4);
}

/* 玻璃态变体 */
.modern-action-button.variant-glass {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  color: #374151;
}

.modern-action-button.variant-glass:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: translateY(-2px);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
}

/* 轮廓变体 */
.modern-action-button.variant-outlined {
  background: transparent;
  border: 2px solid #7c3aed;
  color: #7c3aed;
}

.modern-action-button.variant-outlined:hover {
  background: rgba(124, 58, 237, 0.1);
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(124, 58, 237, 0.2);
}

/* 按钮尺寸优化 */
:deep(.modern-action-button.n-button--tiny) {
  border-radius: 8px;
  font-size: 0.625rem;
  height: 24px;
  padding: 0 8px;
}

:deep(.modern-action-button.n-button--small) {
  border-radius: 12px;
  font-size: 0.75rem;
  height: 32px;
  padding: 0 12px;
}

:deep(.modern-action-button.n-button--medium) {
  border-radius: 14px;
  font-size: 0.875rem;
  height: 40px;
  padding: 0 16px;
}

:deep(.modern-action-button.n-button--large) {
  border-radius: 16px;
  font-size: 1rem;
  height: 48px;
  padding: 0 20px;
}

/* 圆形按钮优化 */
:deep(.modern-action-button.n-button--circle) {
  border-radius: 50%;
}

/* 图标样式 */
.button-icon {
  transition: transform 0.3s ease;
}

.modern-action-button:hover .button-icon {
  transform: scale(1.1);
}

/* 文本样式 */
.button-text {
  position: relative;
  z-index: 1;
}

/* 加载状态 */
:deep(.modern-action-button.n-button--loading) {
  pointer-events: none;
}

/* 禁用状态 */
:deep(.modern-action-button.n-button--disabled) {
  opacity: 0.5;
  pointer-events: none;
  transform: none !important;
}

/* 不同类型的颜色主题 */
:deep(.modern-action-button.n-button--primary) {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  border: none;
  color: white;
}

:deep(.modern-action-button.n-button--info) {
  background: linear-gradient(135deg, #0891b2 0%, #0e7490 100%);
  border: none;
  color: white;
}

:deep(.modern-action-button.n-button--success) {
  background: linear-gradient(135deg, #059669 0%, #047857 100%);
  border: none;
  color: white;
}

:deep(.modern-action-button.n-button--warning) {
  background: linear-gradient(135deg, #d97706 0%, #b45309 100%);
  border: none;
  color: white;
}

:deep(.modern-action-button.n-button--error) {
  background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  border: none;
  color: white;
}

/* 四级按钮样式 */
:deep(.modern-action-button.n-button--quaternary) {
  background: transparent;
  border: 1px solid rgba(0, 0, 0, 0.1);
  color: #6b7280;
}

:deep(.modern-action-button.n-button--quaternary:hover) {
  background: rgba(0, 0, 0, 0.05);
  border-color: rgba(0, 0, 0, 0.2);
  color: #374151;
}

/* 响应式设计 */
@media (max-width: 768px) {
  :deep(.modern-action-button.n-button--large) {
    height: 44px;
    font-size: 0.875rem;
  }

  :deep(.modern-action-button.n-button--medium) {
    height: 36px;
    font-size: 0.75rem;
  }
}

/* 暗色主题 */
:deep(.dark-theme) .modern-action-button.variant-default {
  background: rgba(30, 41, 59, 0.8);
  border-color: rgba(255, 255, 255, 0.1);
  color: #e2e8f0;
}

:deep(.dark-theme) .modern-action-button.variant-default:hover {
  background: rgba(30, 41, 59, 1);
}

:deep(.dark-theme) .modern-action-button.variant-glass {
  background: rgba(255, 255, 255, 0.05);
  border-color: rgba(255, 255, 255, 0.1);
  color: #e2e8f0;
}

:deep(.dark-theme) .modern-action-button.variant-glass:hover {
  background: rgba(255, 255, 255, 0.1);
}

:deep(.dark-theme) .modern-action-button.n-button--quaternary {
  background: transparent;
  border-color: rgba(255, 255, 255, 0.1);
  color: #94a3b8;
}

:deep(.dark-theme) .modern-action-button.n-button--quaternary:hover {
  background: rgba(255, 255, 255, 0.05);
  border-color: rgba(255, 255, 255, 0.2);
  color: #e2e8f0;
}
</style>
