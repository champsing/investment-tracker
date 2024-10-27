<script setup lang="ts">
import { VaCard, VaModal, VaInput, VaSelect } from 'vuestic-ui';
import { reactive } from 'vue';
import { kindOptions } from '@/composables/account';

const form = reactive({
    show: false,
    name: "",
    alias: "",
    kind: "",
})

</script>

<template>
    <div>
        <VaCard class="h-36 flex items-center justify-center"
                @click="form.show = true">
            <VaIcon name="ms-add" size="64px" class="flex-grow-0" />
        </VaCard>
        <VaModal v-model="form.show" ok-text="Save" size="auto">
            <div class="w-80 flex flex-col items-center">
                <VaInput v-model="form.name" label="Account ID"
                         name="Account ID" immediate-validation
                         class="w-4/5 flex-grow-0" />
                <VaInput v-model="form.alias" label="Account Alias (Optional)"
                         name="Account Alias" immediate-validation
                         class="w-4/5 flex-grow-0 mt-2" />
                <VaSelect v-model="form.kind" :options="kindOptions"
                          placeholder="Select an option" label="Account Kind"
                          name="Account Kind" class="w-4/5 flex-grow-0 mt-2" />
            </div>
        </VaModal>
    </div>
</template>
