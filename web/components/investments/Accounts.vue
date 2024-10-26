<script setup lang="ts">
import { VaButton } from 'vuestic-ui';
</script>

<template>
    <div class="grid grid-cols-4 gap-4">
        <div class="h-0"></div>
        <div class="h-0"></div>
        <VaButton>Button 1</VaButton>
        <VaButton>Button 2</VaButton>
    </div>
    <VaDivider orientation="left">
        <span class="px-2 font-bold">left</span>
    </VaDivider>
    <div class="grid grid-cols-4 gap-4">
        <VaCard>
            <VaCardTitle>Title</VaCardTitle>
            <VaCardContent>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit.
            </VaCardContent>
        </VaCard>
        <VaCard>
            <VaCardTitle>Title</VaCardTitle>
            <VaCardContent>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit.
            </VaCardContent>
        </VaCard>
        <VaCard>
            <VaCardTitle>Title</VaCardTitle>
            <VaCardContent>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit.
            </VaCardContent>
        </VaCard>
        <VaCard>
            <VaCardTitle>Title</VaCardTitle>
            <VaCardContent>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit.
            </VaCardContent>
        </VaCard>
        <VaCard>
            <VaCardTitle>Title</VaCardTitle>
            <VaCardContent>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit.
            </VaCardContent>
        </VaCard>
        <VaCard>
            <div class="p-auto w-full h-full">
                <div>123</div>
            </div>
        </VaCard>
    </div>
</template>

<style scoped></style>
