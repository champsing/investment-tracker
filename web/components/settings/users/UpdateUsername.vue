<script setup lang="ts">
import { reactive, ref } from "vue"
import axios from "axios";
import { VaButton, useForm } from 'vuestic-ui';
import { logout, hasUsername, getUsername } from "@/composables/user";

const authorize = defineModel<boolean>({ required: true })
const { isLoading, isValid } = useForm('formRef')

const modal = ref(false);
const form = reactive({
    username: getUsername()
})

function beforeOk(hide: () => void) {
    if (isLoading.value || !isValid.value) { return; }

    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        username: form.username,
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    })
}

</script>

<template>
    <div class="flex-grow-0">
        <VaButton icon="ms-edit" background-opacity="0" color="textPrimary"
                  @click="modal = true;" class="flex-grow-0" />
        <VaModal v-model="modal" ok-text="Save" size="auto"
                 :before-ok="beforeOk"
                 @open="() => form.username = getUsername()">
            <VaForm ref="formRef" immediate
                    class="w-80 flex flex-col items-center">
                <VaInput v-model="form.username" label="New Username" :rules="[
                    ((x) => x.length >= 6 || 'username too short'),
                    hasUsername
                ]" class="w-4/5 flex-grow-0" />
            </VaForm>
        </VaModal>
    </div>
</template>

<style scoped></style>
