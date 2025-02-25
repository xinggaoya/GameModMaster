import './assets/styles/theme.css'
import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { create } from 'naive-ui'

import App from './App.vue'
import router from './router'

// 初始化Tauri应用
import '@tauri-apps/api'

const naive = create()

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(naive)

app.mount('#app')
