<script setup lang="ts">
import Users from './settings/Users.vue';
import Danger from './settings/Danger.vue';

const authorize = defineModel<boolean>({ required: true })
</script>

<template>
    <div class="px-4 gap-2 columns-4">
        <div class="setting-item">
            <Users v-model:model-value="authorize" />
        </div>
        <div class="setting-item">
            <Danger />
        </div>
    </div>
</template>

<style scoped>
.setting-item {
    @apply inline-block;
    @apply w-full;
    @apply mt-2;
}
</style>
