<script setup lang="ts">
import { ref } from "vue"
import { useColors } from "vuestic-ui";
import Login from "./components/Login.vue";

// dark theme
useColors().applyPreset('dark');

// authorize
const authorize = ref(false);
</script>

<template>
    <div class="background"
         :style="{ backgroundColor: useColors().colors.backgroundPrimary }">
        <VaNavbar>
            <template #left>
                <VaNavbarItem>
                    <img src="@assets/icon.svg" alt="logo" height="32"
                         width="32" />
                </VaNavbarItem>
            </template>

            <template #center>
                <VaNavbarItem>
                    <VaButton icon="ms-dashboard" background-opacity="0"
                              color="textPrimary" to="/">
                        Overview
                    </VaButton>
                </VaNavbarItem>
                <VaNavbarItem>
                    <VaButton icon="ms-monitoring" background-opacity="0"
                              color="textPrimary" to="/investments">
                        Investments
                    </VaButton>
                </VaNavbarItem>
            </template>

            <template #right>
                <VaNavbarItem>
                    <Login v-model:model-value="authorize" />
                </VaNavbarItem>
            </template>
        </VaNavbar>
        <div class="px-4 pt-2">
            <RouterView />
        </div>
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
<style lang="scss">
// ensure browser scroll bar is dark
:root {
    color-scheme: dark;
}

// make navigation bar slimmer
.va-navbar {
    --va-navbar-padding-y: 0.6rem;
}
</style>