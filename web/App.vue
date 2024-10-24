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

// dark theme
useColors().applyPreset('dark');
</script>

<template>
    <div class="background" :style="{ backgroundColor: useColors().colors.backgroundPrimary }">
        <VaNavbar>
            <template #left>
                <VaNavbarItem>
                    <img src="@assets/icon.svg" alt="logo" height="32" width="32" />
                </VaNavbarItem>
            </template>

            <template #center>
                <VaNavbarItem>
                    <VaButton icon="ms-dashboard" background-opacity="0" color="textPrimary" to="/">
                        Overview
                    </VaButton>
                </VaNavbarItem>
                <VaNavbarItem>
                    <VaButton icon="ms-monitoring" background-opacity="0" color="textPrimary" to="/investments">
                        Investments
                    </VaButton>
                </VaNavbarItem>
            </template>
        </VaNavbar>
        <div class="p-4">
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

.login-card {
    height: 300px;
    width: 400px;
}

.va-modal__dialog {
    width: 400px;
    --va-modal-dialog-height: 300px;
}

.va-modal__message {
    flex-grow: 1;
}

.va-modal__inner {
    height: 100%;
}
</style>