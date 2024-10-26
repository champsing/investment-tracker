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
    <div :style="{ backgroundColor: useColors().colors.backgroundPrimary }"
         class="h-full min-h-screen w-screen">
        <VaNavbar class="va-navbar-p-slim">
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
        <RouterView />
    </div>
</template>

<style scoped>
.va-navbar-p-slim {
    --va-navbar-padding-y: 0.6rem;
}
</style>
