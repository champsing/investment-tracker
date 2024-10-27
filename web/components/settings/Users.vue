<script setup lang="ts">
import { reactive } from "vue"
import axios from "axios";
import { VaButton } from 'vuestic-ui';
import { logout, validUsr, validPwd, matchPwd } from "@/composables/user";
const authorize = defineModel<boolean>({ required: true })

const usernameForm = reactive({
    show: false,
    wait: false,
    username1: '',
    err1: '',
})

function username(): string {
    return localStorage.getItem('username');
}

function showUsrModal() {
    usernameForm.show = true;
    usernameForm.username1 = username();
    usernameForm.err1 = '';
}

function beforeOkUsr(hide: () => void) {
    if (usernameForm.err1 != '') {
        return;
    }
    if (usernameForm.wait) { return; }

    usernameForm.wait = true
    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        username: usernameForm.username1,
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        usernameForm.err1 = 'please try again';
    }).finally(() => usernameForm.wait = false)
}

const passwordForm = reactive({
    show: false,
    wait: false,
    password1: '',
    err1: '',
    password2: '',
    err2: '',
    password3: '',
    err3: '',
})

function showPwdModal() {
    passwordForm.show = true;
    passwordForm.password1 = '';
    passwordForm.password2 = '';
    passwordForm.password3 = '';
    passwordForm.err1 = '';
    passwordForm.err2 = '';
    passwordForm.err3 = '';
}

function beforeOkPwd(hide: () => void) {
    if (passwordForm.err1 != '' || passwordForm.err2 != '' || passwordForm.err3 != '') {
        return;
    }
    if (passwordForm.wait) { return; }

    passwordForm.wait = true
    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        password: [passwordForm.password1, passwordForm.password2]
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        passwordForm.err1 = 'wrong password';
    }).finally(() => passwordForm.wait = false)
}
</script>

<template>
    <VaCard stripe stripe-color="success">
        <VaCardTitle>User Setting</VaCardTitle>
        <VaCardContent class="flex flex-col">
            <div class="flex items-center justify-around">
                <div>username: <span class="font-bold">{{ username() }}</span>
                </div>
                <VaButton icon="ms-edit" background-opacity="0"
                          color="textPrimary" @click="showUsrModal()"
                          class="flex-grow-0" />
            </div>
            <div class="flex items-center justify-around mt-2">
                <div>password: ●●●●●●●●</div>
                <VaButton icon="ms-edit" background-opacity="0"
                          color="textPrimary" @click="showPwdModal()"
                          class="flex-grow-0" />
            </div>
        </VaCardContent>
    </VaCard>
    <VaModal v-model="usernameForm.show" ok-text="Save" size="auto"
             :before-ok="beforeOkUsr">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="usernameForm.username1" label="New Username"
                     name="New Username" immediate-validation
                     :error="usernameForm.err1 != ''"
                     :error-messages="usernameForm.err1" @input="validUsr(usernameForm)"
                     class="w-4/5 flex-grow-0" />
        </div>
    </VaModal>
    <VaModal v-model="passwordForm.show" ok-text="Save" size="auto"
             :before-ok="beforeOkPwd">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="passwordForm.password1" label="Old Password"
                     name="Old Password" type="password" immediate-validation
                     :error="passwordForm.err1 != ''"
                     :error-messages="passwordForm.err1" @input="passwordForm.err1 = ''"
                     class="w-4/5 flex-grow-0 mt-2" />
            <VaInput v-model="passwordForm.password2" label="New Password"
                     name="New Password" type="password" immediate-validation
                     :error="passwordForm.err2 != ''"
                     :error-messages="passwordForm.err2" @input="validPwd(passwordForm)"
                     class="w-4/5 flex-grow-0 mt-2" />
            <VaInput v-model="passwordForm.password3" label="Repeat Password"
                     name="Repeat Password" type="password" immediate-validation
                     :error="passwordForm.err3 != ''"
                     :error-messages="passwordForm.err3" @input="matchPwd(passwordForm)"
                     class="w-4/5 flex-grow-0 mt-2" />
        </div>
    </VaModal>
</template>

<style scoped></style>
