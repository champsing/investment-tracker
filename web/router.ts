import { createWebHistory, createRouter } from "vue-router";
import Overview from "./components/Overview.vue";
import Settings from "./components/Settings.vue";
import Investments from "./components/Investments.vue";

const routes = [
    { path: "/", component: Overview },
    { path: "/settings", component: Settings },
    { path: "/investments", component: Investments }
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;