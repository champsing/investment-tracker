<script setup lang="ts">
import { reactive } from "vue";
import { VaInput, VaButton, VaModal } from 'vuestic-ui';
import axios from "axios";
import {
    logout, login, validUsername, validPassword, matchPassword,
    getUsername
} from '@composables/user';

const authorize = defineModel<boolean>({ required: true })

const form = reactive({
    show: false,
    login: true,
    wait: false,

    username: '',
    usernameErr: '',
    password: '',
    passwordErr: '',
    repeatPassword: '',
    repeatPasswordErr: '',
})

function showModal() {
    form.show = true;
    form.login = true;

    form.username = '';
    form.usernameErr = '';
    form.password = '';
    form.passwordErr = '';
    form.repeatPassword = '';
    form.repeatPasswordErr = '';
}

function switchModal() {
    form.login = !form.login;

    form.username = '';
    form.usernameErr = '';
    form.password = '';
    form.passwordErr = '';
    form.repeatPassword = '';
    form.repeatPasswordErr = '';
}

function usernameInput() {
    if (!form.login) {
        validUsername(form.username, (e) => form.usernameErr = e)
    }
}

function passwordInput() {
    if (form.login) {
        form.passwordErr = ''
    } else {
        validPassword(form.password, (e) => form.passwordErr = e);
    }
}

function repeatPasswordInput() {
    if (!form.login) {
        matchPassword(form.password, form.repeatPassword, (e) => form.repeatPasswordErr = e);
    }
}

function okText() {
    if (form.login) {
        return "Login"
    } else {
        return "Signup"
    }
}

function beforeOk(hide: () => void) {
    if (form.usernameErr != '' || form.passwordErr != '' ||
        form.repeatPasswordErr != '' || form.wait) {
        return;
    }

    form.wait = true
    if (form.login) {
        axios.post("/api/user/login", {
            username: form.username,
            password: form.password,
        }).then(response => {
            hide();
            login(response.data);
            authorize.value = true;
        }).catch(_ => {
            form.passwordErr = "wrong password";
            authorize.value = false;
        }).finally(() => {
            form.wait = false;
        })
    } else {
        axios.post("/api/user/register", {
            username: form.username,
            password: form.password,
        }).then(_ => {
            form.login = true;
        }).catch(_ => {
            form.passwordErr = "please try again";
        }).finally(() => {
            form.wait = false;
        })
    }
}

function rotateToken() {
    let token = localStorage.getItem('token');
    if (token != null) {
        axios.post("/api/user/rotate", {
            token: token
        }).then(response => {
            login(response.data);
            authorize.value = true;
        }).catch(_ => {
            logout();
            authorize.value = false;
        })
    } else {
        authorize.value = false;
    }
}
rotateToken();
setInterval(rotateToken, 1000 * 60);
</script>

<template>
    <template v-if="!authorize">
        <VaButton icon="ms-login" background-opacity="0" color="textPrimary"
                  @click="showModal()" class="login-button" />
    </template>
    <template v-else>
        <VaButtonDropdown background-opacity="0" color="textPrimary" hide-icon
                          :close-on-content-click="false"
                          placement="bottom-start" class="login-button">
            <template #label>
                <VaIcon name="ms-account_circle" size="24px" />
            </template>
            <div class="flex flex-col">
                <div class="text-sm text-center">
                    {{ getUsername() }}
                </div>
                <VaDivider />
                <VaButton background-opacity="0" color="textPrimary"
                          to="/settings">
                    Settings
                </VaButton>
                <VaButton background-opacity="0" color="textPrimary"
                          @click="authorize = false; logout()">
                    Logout
                </VaButton>
            </div>
        </VaButtonDropdown>
    </template>
    <VaModal v-model="form.show" size="auto" :ok-text="okText()"
             :before-ok="beforeOk">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="form.username" label="Username" name="Username"
                     immediate-validation :error="form.usernameErr != ''"
                     :error-messages="form.usernameErr" @input="usernameInput()"
                     class="w-4/5 flex-grow-0" />
            <VaInput v-model="form.password" label="Password" name="Password"
                     type="password" immediate-validation
                     :error="form.passwordErr != ''"
                     :error-messages="form.passwordErr" @input="passwordInput()"
                     class="w-4/5 flex-grow-0 mt-2" />
            <template v-if="form.login">
                <div class="w-4/5 flex-grow-0 mt-4">
                    Don't have an account?
                    <span class="text-bold cursor-pointer underline"
                          @click="switchModal()">Sign up</span>
                </div>
            </template>
            <template v-else>
                <VaInput v-model="form.repeatPassword" label="Repeat Password"
                         name="Repeat Password" type="password"
                         immediate-validation
                         :error="form.repeatPasswordErr != ''"
                         :error-messages="form.repeatPasswordErr"
                         @input="repeatPasswordInput()"
                         class="w-4/5 flex-grow-0 mt-2" />
                <div class="w-4/5 flex-grow-0 mt-4">
                    Already have an account?
                    <span class="text-bold cursor-pointer underline"
                          @click="switchModal()">Log
                        in</span>
                </div>
            </template>
        </div>
    </VaModal>
</template>

<style>
.login-button {
    --va-button-size: 32px;
    --va-button-content-py: 4px;
}
</style>
