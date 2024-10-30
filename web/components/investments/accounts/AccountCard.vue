<script setup lang="ts">
import { ref } from 'vue';
import { Account } from '@/composables/account';
const props = defineProps<{ account: Account }>()
const emits = defineEmits<{ click: [] }>()

const hover = ref(false);
</script>

<template>
    <VaHover v-model="hover" class="h-36">
        <VaCard @click="emits('click')" :gradient="hover" class="h-full">
            <VaCardTitle>
                <div class="-mt-2 -mb-2 h-7 flex items-center">
                    <VaChip size="small" square readonly
                            class="text-xs flex-grow-0 flex-shrink-0">
                        {{ props.account.kind }}
                    </VaChip>
                    <div class="ml-2 text-xs flex-grow-0 flex-shrink-0">
                        {{ props.account.alias }}
                    </div>
                    <div v-if="props.account.alias != props.account.name"
                         class="ml-2 text-xs font-normal min-w-0 truncate flex-grow flex-shrink">
                        ({{ props.account.name }})
                    </div>
                </div>
            </VaCardTitle>
            <VaCardContent>
                <div class="flex items-center justify-between">
                    <div class="text-xl flex-grow-0">USD XXXXXXX.XX</div>
                    <div class="flex-grow-0 flex flex-col items-end">
                        <div>+XXXX.XX (+XX.XX%)</div>
                        <div>+XXXX.XX (+XX.XX%)</div>
                    </div>
                </div>
            </VaCardContent>
        </VaCard>
    </VaHover>
</template>

<style scoped></style>
