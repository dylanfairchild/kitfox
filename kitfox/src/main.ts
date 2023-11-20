import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import Home from "./views/Home.vue";
import Storage from "./views/Storage.vue";
import Settings from "./views/Settings.vue";

// Pinia
import { createPinia } from "pinia";

// Vue Router
import { createRouter, createWebHashHistory } from "vue-router";

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
// import { aliases, fa } from 'vuetify/iconsets/fa'
import '@mdi/font/css/materialdesignicons.css'
import { aliases, mdi } from 'vuetify/iconsets/mdi'

const vuetify = createVuetify({
    components,
    directives,
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
            mdi,
        }
    }
})

const routes = [
    { path: '/', name: "home", component: Home },
    { path: '/settings', name: "settings", component: Settings },
    { path: '/storage/:id', name: "storage", component: Storage }
]

const pinia = createPinia()

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

createApp(App).use(vuetify).use(router).use(pinia).mount("#app");
