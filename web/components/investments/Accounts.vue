<script setup lang="ts">
import axios from 'axios';
import { ref } from 'vue';
import { Account } from '@/composables/interface';

const accounts = ref<Account[]>([]);

axios.post('/api/investment/account/fetch', {
    token: localStorage.getItem('token'),
}).then(response => {
    accounts.value = response.data
})
</script>

<template>
    <div class="px-4 mt-2 grid grid-cols-4 gap-2">
        <VaCard v-for="account in accounts" class="h-36">
            <VaCardTitle>{{ account.alias }}</VaCardTitle>
            <VaCardContent>
                123
            </VaCardContent>
        </VaCard>
        <VaCard class="h-36 flex items-center justify-center">
            <VaIcon name="ms-add" size="64px" class="flex-grow-0"/>
        </VaCard>
    </div>
</template>

<style scoped></style>
