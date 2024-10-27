<script setup lang="ts">
import { reactive } from "vue"
import axios from "axios";
import { VaButton } from 'vuestic-ui';
import { logout, validPassword, matchPassword } from "@/composables/user";
const authorize = defineModel<boolean>({ required: true })

const form = reactive({
    show: false,
    wait: false,
    oldPassword: '',
    oldPasswordErr: '',
    newPassword: '',
    newPasswordErr: '',
    repeatPassword: '',
    repeatPasswordErr: '',
})

function newPasswordInput() {
    validPassword(form.newPassword, (e) => form.newPasswordErr = e)
}

function repeatPasswordInput() {
    matchPassword(form.newPassword, form.repeatPassword, (e) => form.newPasswordErr = e)
}

function showModal() {
    form.show = true;
    form.oldPassword = '';
    form.newPassword = '';
    form.repeatPassword = '';
    form.oldPasswordErr = '';
    form.newPasswordErr = '';
    form.repeatPasswordErr = '';
}

function beforeOk(hide: () => void) {
    if (form.oldPasswordErr != '' || form.newPasswordErr != '' ||
        form.repeatPasswordErr != '' || form.wait) {
        return;
    }

    form.wait = true
    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        password: [form.oldPassword, form.newPassword]
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        form.oldPasswordErr = 'wrong password';
    }).finally(() => form.wait = false)
}
</script>

<template>
    <div class="flex-grow-0">
        <VaButton icon="ms-edit" background-opacity="0" color="textPrimary"
                  @click="showModal()" />
        <VaModal v-model="form.show" ok-text="Save" size="auto"
                 :before-ok="beforeOk">
            <div class="w-80 flex flex-col items-center">
                <VaInput v-model="form.oldPassword" label="Old Password"
                         name="Old Password" type="password"
                         immediate-validation :error="form.oldPasswordErr != ''"
                         :error-messages="form.oldPasswordErr"
                         @input="form.oldPasswordErr = ''"
                         class="w-4/5 flex-grow-0 mt-2" />
                <VaInput v-model="form.newPassword" label="New Password"
                         name="New Password" type="password"
                         immediate-validation :error="form.newPasswordErr != ''"
                         :error-messages="form.newPasswordErr"
                         @input="newPasswordInput()"
                         class="w-4/5 flex-grow-0 mt-2" />
                <VaInput v-model="form.repeatPassword" label="Repeat Password"
                         name="Repeat Password" type="password"
                         immediate-validation
                         :error="form.repeatPasswordErr != ''"
                         :error-messages="form.repeatPasswordErr"
                         @input="repeatPasswordInput()"
                         class="w-4/5 flex-grow-0 mt-2" />
            </div>
        </VaModal>
    </div>
</template>

<style scoped></style>
