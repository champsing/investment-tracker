import { createWebHistory, createRouter } from "vue-router";
import Overview from "./components/Overview.vue";
import Settings from "./components/Settings.vue";

const routes = [
    { path: "/", component: Overview },
    { path: "/settings", component: Settings }
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;