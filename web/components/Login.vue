<script setup lang="ts">
import { reactive } from "vue";
import { VaInput, VaButton, VaModal } from 'vuestic-ui';
import axios from "axios";
import { logout, login, validUsr, validPwd, matchPwd } from '@composables/user';

const authorize = defineModel<boolean>({ required: true })

const modal = reactive({
    show: false,
    login: true,
    wait: false,

    username1: '',
    err1: '',
    password2: '',
    err2: '',
    password3: '',
    err3: '',
})

function reset() {
    modal.wait = false;

    modal.username1 = '';
    modal.password2 = '';
    modal.password3 = '';

    modal.err1 = '';
    modal.err2 = '';
    modal.err3 = '';
}

function showModal() {
    modal.show = true
    modal.login = true
    reset();
}

function validPwd2() {
    if (modal.login) {
        modal.err2 = ''
    } else {
        validPwd(modal);
    }
}

function okText() {
    if (modal.login) {
        return "Login"
    } else {
        return "Signup"
    }
}

function beforeOk(hide: () => void) {
    if (modal.err1 != '' || modal.err2 != '' || modal.err3 != '') {
        return;
    }

    if (modal.wait) { return; }

    modal.wait = true
    if (modal.login) {
        axios.post("/api/user/login", {
            username: modal.username1,
            password: modal.password2,
        }).then(response => {
            hide();
            login(response.data);
            authorize.value = true;
        }).catch(_ => {
            modal.err2 = "wrong password";
            authorize.value = false;
        }).finally(() => {
            modal.wait = false;
        })
    } else {
        axios.post("/api/user/register", {
            username: modal.username1,
            password: modal.password2,
        }).then(_ => {
            modal.login = true;
        }).catch(_ => {
            modal.err2 = "please try again";
        }).finally(() => {
            modal.wait = false;
        })
    }
}

function username(): string {
    return localStorage.getItem('username');
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
                    {{ username() }}
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
    <VaModal v-model="modal.show" size="auto" :ok-text="okText()"
             :before-ok="beforeOk">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="modal.username1" label="Username" name="Username"
                     immediate-validation :error="modal.err1 != ''"
                     :error-messages="modal.err1"
                     @input="modal.login || validUsr(modal)"
                     class="w-4/5 flex-grow-0" />
            <VaInput v-model="modal.password2" label="Password" name="Password"
                     type="password" immediate-validation
                     :error="modal.err2 != ''" :error-messages="modal.err2"
                     @input="validPwd2()" class="w-4/5 flex-grow-0 mt-2" />
            <template v-if="modal.login">
                <div class="w-4/5 flex-grow-0 mt-4">
                    Don't have an account?
                    <span class="text-bold cursor-pointer underline" @click="reset();
                    modal.login = false">Sign up</span>
                </div>
            </template>
            <template v-else>
                <VaInput v-model="modal.password3" label="Repeat Password"
                         name="Repeat Password" type="password"
                         immediate-validation :error="modal.err3 != ''"
                         :error-messages="modal.err3" @input="matchPwd(modal)"
                         class="w-4/5 flex-grow-0 mt-2" />
                <div class="w-4/5 flex-grow-0 mt-4">
                    Already have an account?
                    <span class="text-bold cursor-pointer underline"
                          @click="reset(); modal.login = true">Log
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
