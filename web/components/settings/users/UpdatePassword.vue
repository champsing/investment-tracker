<script setup lang="ts">
import { reactive, ref } from "vue"
import axios from "axios";
import { useForm } from 'vuestic-ui';
import { logout } from "@/composables/user";

const authorize = defineModel<boolean>({ required: true })
const { isLoading, isValid, reset, validate } = useForm('formRef')

const modal = ref(false);
const form = reactive({
    oldPassword: '',
    newPassword: '',
    repPassword: '',
    status200: true,
})

async function beforeOk(hide: () => void) {
    validate();
    if (isLoading.value || !isValid.value) { return; }

    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        password: [form.oldPassword, form.newPassword]
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        form.status200 = false;
        validate();
    })
}
</script>

<template>
    <div class="flex-grow-0">
        <VaButton icon="ms-edit" background-opacity="0" color="textPrimary"
                  @click="modal = true" />
        <VaModal v-model="modal" ok-text="Save" size="auto" @open="reset"
                 :before-ok="beforeOk">
            <VaForm ref="formRef" class="w-80 flex flex-col items-center">
                <VaInput v-model="form.oldPassword" label="Old Password"
                         type="password" @input="(_) => form.status200 = true"
                         :rules="[(_) => form.status200 || 'incorrect password']"
                         class="w-4/5 flex-grow-0 mt-2" />
                <VaInput v-model="form.newPassword" label="New Password"
                         type="password"
                         :rules="[(x) => x.length >= 8 || 'password too short']"
                         class="w-4/5 flex-grow-0 mt-2" />
                <VaInput v-model="form.repPassword" label="Repeat Password"
                         type="password"
                         :rules="[(x) => x == form.newPassword || 'password does not match']"
                         class="w-4/5 flex-grow-0 mt-2" />
            </VaForm>
        </VaModal>
    </div>
</template>

<style scoped></style>
