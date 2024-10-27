<script setup lang="ts">
import { reactive, ref } from "vue"
import axios from "axios";
import { useForm } from 'vuestic-ui';
import { logout, duplicateUsername, getUsername } from "@/composables/user";

const authorize = defineModel<boolean>({ required: true })
const { isLoading, isValid, validateAsync } = useForm('formRef')

const modal = ref(false);
const form = reactive({
    username: getUsername()
})

async function beforeOk(hide: () => void) {
    await validateAsync();
    if (isLoading.value || !isValid.value) { return; }

    await axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        username: form.username,
    });

    hide();
    logout();
    authorize.value = false;
}

</script>

<template>
    <div class="flex-grow-0">
        <VaButton icon="ms-edit" background-opacity="0" color="textPrimary"
                  @click="modal = true;" class="flex-grow-0" />
        <VaModal v-model="modal" ok-text="Save" size="auto"
                 :before-ok="beforeOk"
                 @open="() => form.username = getUsername()">
            <VaForm ref="formRef" class="w-80 flex flex-col items-center">
                <VaInput v-model="form.username" label="New Username" :rules="[
                    ((x) => x.length >= 6 || 'username too short'),
                    duplicateUsername
                ]" class="w-4/5 flex-grow-0" />
            </VaForm>
        </VaModal>
    </div>
</template>

<style scoped></style>
