<script setup lang="ts">
import axios from 'axios';
import { ref } from 'vue';
import { Account } from '@/composables/account';
import NewAccount from './accounts/NewAccount.vue';

const accounts = ref<Account[]>([]);

// const

function fetch() {
    axios.post('/api/investment/account/fetch', {
        token: localStorage.getItem('token'),
    }).then(response => {
        accounts.value = response.data;
    })
}

fetch();
</script>

<template>
    <div class="px-4 mt-4 grid grid-cols-4 gap-4">
        <VaCard v-for="account in accounts" class="h-36">
            <VaCardTitle>
                <div class="-mt-2 -mb-2 h-7 flex items-center">
                    <VaChip size="small" class="text-xs flex-shrink-0">
                        {{ account.kind }}
                    </VaChip>
                    <div class="ml-2 text-xs flex-shrink-0">
                        {{ account.alias }}
                    </div>
                    <div v-if="account.alias != account.name"
                         class="ml-2 text-xs font-normal text-ellipsis truncate">
                        ({{ account.name }})
                    </div>
                </div>
            </VaCardTitle>
            <VaCardContent>
                <div class="flex items-center justify-between">
                    <div class="text-xl flex-grow-0">CAD 2,000,000</div>
                    <div class="flex-grow-0 flex flex-col items-end">
                        <div>+1,000.11 (+2.34%)</div>
                        <div>+14,000.11 (+11.34%)</div>
                    </div>
                </div>
            </VaCardContent>
        </VaCard>
        <NewAccount @insert="() => fetch()" />
    </div>
</template>

<style scoped></style>
