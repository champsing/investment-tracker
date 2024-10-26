<script setup lang="ts">
import { reactive } from "vue"
import axios from "axios";
import { VaButton } from 'vuestic-ui';
import { logout, validUsr, validPwd, matchPwd } from "@/composables/user";
const authorize = defineModel<boolean>({ required: true })

const usrModal = reactive({
    show: false,
    wait: false,
    username1: '',
    err1: '',
})

function username(): string {
    return localStorage.getItem('username');
}

function showUsrModal() {
    usrModal.show = true;
    usrModal.username1 = username();
    usrModal.err1 = '';
}

function beforeOkUsr(hide: () => void) {
    if (usrModal.err1 != '') {
        return;
    }
    if (usrModal.wait) { return; }

    usrModal.wait = true
    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        username: usrModal.username1,
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        usrModal.err1 = 'please try again';
    }).finally(() => usrModal.wait = false)
}

const pwdModal = reactive({
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
    pwdModal.show = true;
    pwdModal.password1 = '';
    pwdModal.password2 = '';
    pwdModal.password3 = '';
    pwdModal.err1 = '';
    pwdModal.err2 = '';
    pwdModal.err3 = '';
}

function beforeOkPwd(hide: () => void) {
    if (pwdModal.err1 != '' || pwdModal.err2 != '' || pwdModal.err3 != '') {
        return;
    }
    if (pwdModal.wait) { return; }

    pwdModal.wait = true
    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        password: [pwdModal.password1, pwdModal.password2]
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        pwdModal.err1 = 'wrong password';
    }).finally(() => pwdModal.wait = false)
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
    <VaModal v-model="usrModal.show" ok-text="Save" size="auto"
             :before-ok="beforeOkUsr">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="usrModal.username1" label="New Username"
                     name="New Username" immediate-validation
                     :error="usrModal.err1 != ''"
                     :error-messages="usrModal.err1" @input="validUsr(usrModal)"
                     class="w-4/5 flex-grow-0" />
        </div>
    </VaModal>
    <VaModal v-model="pwdModal.show" ok-text="Save" size="auto"
             :before-ok="beforeOkPwd">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="pwdModal.password1" label="Old Password"
                     name="Old Password" type="password" immediate-validation
                     :error="pwdModal.err1 != ''"
                     :error-messages="pwdModal.err1" @input="pwdModal.err1 = ''"
                     class="w-4/5 flex-grow-0 mt-2" />
            <VaInput v-model="pwdModal.password2" label="New Password"
                     name="New Password" type="password" immediate-validation
                     :error="pwdModal.err2 != ''"
                     :error-messages="pwdModal.err2" @input="validPwd(pwdModal)"
                     class="w-4/5 flex-grow-0 mt-2" />
            <VaInput v-model="pwdModal.password3" label="Repeat Password"
                     name="Repeat Password" type="password" immediate-validation
                     :error="pwdModal.err3 != ''"
                     :error-messages="pwdModal.err3" @input="matchPwd(pwdModal)"
                     class="w-4/5 flex-grow-0 mt-2" />
        </div>
    </VaModal>
</template>

<style scoped></style>
