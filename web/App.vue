<script setup lang="ts">
import { computed, ref } from "vue"
import {
    useColors,
    VaSidebar,
    VaSidebarItem,
    VaSidebarItemContent,
    VaSidebarItemTitle,
    VaIcon
} from "vuestic-ui";
import Login from './components/Login.vue';
import axios from "axios";

// dark theme
const { applyPreset, colors } = useColors();
applyPreset('dark');

// auth token
const auth = ref(false);
function setToken(token: string) {
    localStorage.setItem('token', token);
    auth.value = true;
}
function refreshToken() {
    let token = localStorage.getItem('token');
    if (token == null) {
        auth.value = false;
    } else {
        axios.post("/api/auth/refresh", {
            token: token
        }).then(response => {
            setToken(response.data);
        }).catch(_ => {
            auth.value = false;
        });
    }
}
refreshToken();
setInterval(() => {
    refreshToken();
}, 1000 * 60 * 10); // 10 minutes

// sidebar
const minimize = ref(false);
const icon = computed(() => minimize.value ? 'ms-left_panel_open' : 'ms-left_panel_close')
</script>

<template>
    <div class="background" :style="{ backgroundColor: colors.backgroundPrimary }">
        <template v-if="auth">
            <div class="flex">
                <div class="flex-grow-0">
                    <VaSidebar :minimized="minimize" minimized-width="64px" class="h-full min-h-screen">
                        <VaSidebarItem @click="minimize = !minimize">
                            <VaSidebarItemContent>
                                <VaIcon :name="icon" />
                            </VaSidebarItemContent>
                        </VaSidebarItem>
                        <VaSidebarItem to="/" :active="$route.path == '/'">
                            <VaSidebarItemContent>
                                <VaIcon name="ms-dashboard" />
                                <VaSidebarItemTitle>Overview</VaSidebarItemTitle>
                            </VaSidebarItemContent>
                        </VaSidebarItem>
                        <VaSidebarItem>
                            <VaSidebarItemContent>
                                <VaIcon name="ms-monitoring" />
                                <VaSidebarItemTitle>Investments</VaSidebarItemTitle>
                            </VaSidebarItemContent>
                        </VaSidebarItem>
                        <VaSidebarItem to="/settings" :active="$route.path == '/settings'">
                            <VaSidebarItemContent>
                                <VaIcon name="ms-settings" />
                                <VaSidebarItemTitle>Settings</VaSidebarItemTitle>
                            </VaSidebarItemContent>
                        </VaSidebarItem>
                    </VaSidebar>
                </div>
                <div class="p-4">
                    <RouterView />
                </div>
            </div>
        </template>
        <template v-else>
            <Login @token="(value) => setToken(value)" />
        </template>
    </div>
</template>

<style scoped>
.background {
    height: 100%;
    width: 100%;
    min-height: 100vh;
    min-width: 100vw;
}
</style>
