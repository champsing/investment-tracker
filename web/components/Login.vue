<script setup lang="ts">
import { reactive } from "vue";
import { VaInput, VaButton, VaModal } from 'vuestic-ui';
import axios from "axios";
import { logout, login, validUsr, validPwd, matchPwd } from '@composables/user';

const authorize = defineModel<boolean>({ required: true })

const form = reactive({
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
    form.wait = false;

    form.username1 = '';
    form.password2 = '';
    form.password3 = '';

    form.err1 = '';
    form.err2 = '';
    form.err3 = '';
}

function showModal() {
    form.show = true
    form.login = true
    reset();
}

function validPwd2() {
    if (form.login) {
        form.err2 = ''
    } else {
        validPwd(form);
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
    if (form.err1 != '' || form.err2 != '' || form.err3 != '') {
        return;
    }

    if (form.wait) { return; }

    form.wait = true
    if (form.login) {
        axios.post("/api/user/login", {
            username: form.username1,
            password: form.password2,
        }).then(response => {
            hide();
            login(response.data);
            authorize.value = true;
        }).catch(_ => {
            form.err2 = "wrong password";
            authorize.value = false;
        }).finally(() => {
            form.wait = false;
        })
    } else {
        axios.post("/api/user/register", {
            username: form.username1,
            password: form.password2,
        }).then(_ => {
            form.login = true;
        }).catch(_ => {
            form.err2 = "please try again";
        }).finally(() => {
            form.wait = false;
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
    <VaModal v-model="form.show" size="auto" :ok-text="okText()"
             :before-ok="beforeOk">
        <div class="w-80 flex flex-col items-center">
            <VaInput v-model="form.username1" label="Username" name="Username"
                     immediate-validation :error="form.err1 != ''"
                     :error-messages="form.err1"
                     @input="form.login || validUsr(form)"
                     class="w-4/5 flex-grow-0" />
            <VaInput v-model="form.password2" label="Password" name="Password"
                     type="password" immediate-validation
                     :error="form.err2 != ''" :error-messages="form.err2"
                     @input="validPwd2()" class="w-4/5 flex-grow-0 mt-2" />
            <template v-if="form.login">
                <div class="w-4/5 flex-grow-0 mt-4">
                    Don't have an account?
                    <span class="text-bold cursor-pointer underline" @click="reset();
                    form.login = false">Sign up</span>
                </div>
            </template>
            <template v-else>
                <VaInput v-model="form.password3" label="Repeat Password"
                         name="Repeat Password" type="password"
                         immediate-validation :error="form.err3 != ''"
                         :error-messages="form.err3" @input="matchPwd(form)"
                         class="w-4/5 flex-grow-0 mt-2" />
                <div class="w-4/5 flex-grow-0 mt-4">
                    Already have an account?
                    <span class="text-bold cursor-pointer underline"
                          @click="reset(); form.login = true">Log
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
