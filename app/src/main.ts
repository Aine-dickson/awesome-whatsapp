import './assets/tailwind.css'
import piniaPluginPersistedState from 'pinia-plugin-persistedstate'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(createPinia().use(piniaPluginPersistedState))
app.use(router)

app.mount('#app')
