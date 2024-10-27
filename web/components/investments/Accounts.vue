<script setup lang="ts">
import axios from 'axios';
import { ref } from 'vue';
import { Account } from '@/composables/interface';

const accounts = ref<Account[]>([]);

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
                123
            </VaCardContent>
        </VaCard>
        <VaCard class="h-36 flex items-center justify-center">
            <VaIcon name="ms-add" size="64px" class="flex-grow-0" />
        </VaCard>
    </div>
</template>

<style scoped></style>
