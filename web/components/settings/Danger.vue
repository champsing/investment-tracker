<script setup lang="ts">
import { VaButton } from 'vuestic-ui';
// TODO: add request to backend
// TODO: add modal for confirmation
</script>

<template>
    <VaCard stripe stripe-color="danger">
        <VaCardTitle>Danger Zone</VaCardTitle>
        <VaCardContent class="grid grid-cols-2 gap-4">
            <VaButton>Delete User</VaButton>
            <VaButton>Erase Data</VaButton>
        </VaCardContent>
    </VaCard>
</template>
