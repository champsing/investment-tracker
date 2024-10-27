<script setup lang="ts">
import { reactive } from "vue"
import axios from "axios";
import { VaButton } from 'vuestic-ui';
import { logout, validUsername, validPassword, getUsername, matchPassword } from "@/composables/user";
const authorize = defineModel<boolean>({ required: true })

const usernameForm = reactive({
    show: false,
    wait: false,
    username: '',
    error: '',
})

function usernameInput() {
    validUsername(usernameForm.username, (e) => usernameForm.error = e)
}

function usernameShowModal() {
    usernameForm.show = true;
    usernameForm.username = getUsername();
    usernameForm.error = '';
}

function usernameBeforeOk(hide: () => void) {
    if (usernameForm.error != '' || usernameForm.wait) {
        return;
    }

    usernameForm.wait = true
    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        username: usernameForm.username,
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        usernameForm.error = 'please try again';
    }).finally(() => usernameForm.wait = false)
}

const passwordForm = reactive({
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
    validPassword(passwordForm.newPassword, (e) => passwordForm.newPasswordErr = e)
}

function repeatPasswordInput() {
    matchPassword(passwordForm.newPassword, passwordForm.repeatPassword, (e) => passwordForm.newPasswordErr = e)
}

function passwordShowModal() {
    passwordForm.show = true;
    passwordForm.oldPassword = '';
    passwordForm.newPassword = '';
    passwordForm.repeatPassword = '';
    passwordForm.oldPasswordErr = '';
    passwordForm.newPasswordErr = '';
    passwordForm.repeatPasswordErr = '';
}

function passwordBeforeOk(hide: () => void) {
    if (passwordForm.oldPasswordErr != '' || passwordForm.newPasswordErr != '' ||
        passwordForm.repeatPasswordErr != '' || passwordForm.wait) {
        return;
    }

    passwordForm.wait = true
    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        password: [passwordForm.oldPassword, passwordForm.newPassword]
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        passwordForm.oldPasswordErr = 'wrong password';
    }).finally(() => passwordForm.wait = false)
}
</script>

<template>
    <VaCard stripe stripe-color="success">
        <VaCardTitle>User Setting</VaCardTitle>
        <VaCardContent class="flex flex-col">
            <div class="flex items-center justify-around">
                <div>username: <span class="font-bold">{{ getUsername()
                        }}</span>
                </div>
                <VaButton icon="ms-edit" background-opacity="0"
                          color="textPrimary" @click="usernameShowModal()"
                          class="flex-grow-0" />
            </div>
            <div class="flex items-center justify-around mt-2">
                <div>password: ●●●●●●●●</div>
                <VaButton icon="ms-edit" background-opacity="0"
                          color="textPrimary" @click="passwordShowModal()"
                          class="flex-grow-0" />
            </div>
        </VaCardContent>
    </VaCard>
    <VaModal v-model="usernameForm.show" ok-text="Save" size="auto"
             :before-ok="usernameBeforeOk">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="usernameForm.username" label="New Username"
                     name="New Username" immediate-validation
                     :error="usernameForm.error != ''"
                     :error-messages="usernameForm.error"
                     @input="usernameInput()" class="w-4/5 flex-grow-0" />
        </div>
    </VaModal>
    <VaModal v-model="passwordForm.show" ok-text="Save" size="auto"
             :before-ok="passwordBeforeOk">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="passwordForm.oldPassword" label="Old Password"
                     name="Old Password" type="password" immediate-validation
                     :error="passwordForm.oldPasswordErr != ''"
                     :error-messages="passwordForm.oldPasswordErr"
                     @input="passwordForm.oldPasswordErr = ''"
                     class="w-4/5 flex-grow-0 mt-2" />
            <VaInput v-model="passwordForm.newPassword" label="New Password"
                     name="New Password" type="password" immediate-validation
                     :error="passwordForm.newPasswordErr != ''"
                     :error-messages="passwordForm.newPasswordErr"
                     @input="newPasswordInput()"
                     class="w-4/5 flex-grow-0 mt-2" />
            <VaInput v-model="passwordForm.repeatPassword"
                     label="Repeat Password" name="Repeat Password"
                     type="password" immediate-validation
                     :error="passwordForm.repeatPasswordErr != ''"
                     :error-messages="passwordForm.repeatPasswordErr"
                     @input="repeatPasswordInput()"
                     class="w-4/5 flex-grow-0 mt-2" />
        </div>
    </VaModal>
</template>

<style scoped></style>
