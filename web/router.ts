import { createWebHistory, createRouter } from "vue-router";
import Overview from "./components/Overview.vue";
import Settings from "./components/Settings.vue";
import Investment from "./components/Investment.vue";

const routes = [
    { path: "/", component: Overview },
    { path: "/settings", component: Settings },
    { path: "/investment", component: Investment }
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;