<script setup lang="ts">
import { ref } from "vue"
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
const minSidebar = ref(false)
</script>

<template>
    <div class="background" :style="{ backgroundColor: colors.backgroundPrimary }">
        <template v-if="auth">
            <div class="flex">
                <VaSidebar :minimized="minSidebar" minimized-width="64px" class="h-full min-h-screen">
                    <VaSidebarItem>
                        <VaSidebarItemContent>
                            <VaIcon name="ms-left_panel_close" />
                            <VaSidebarItemTitle></VaSidebarItemTitle>
                        </VaSidebarItemContent>
                    </VaSidebarItem>
                    <VaSidebarItem>
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
                    <VaSidebarItem>
                        <VaSidebarItemContent>
                            <VaIcon name="ms-settings" />
                            <VaSidebarItemTitle>Settings</VaSidebarItemTitle>
                        </VaSidebarItemContent>
                    </VaSidebarItem>
                </VaSidebar>

                <RouterView />
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
