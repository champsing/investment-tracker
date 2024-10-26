<script setup lang="ts">
import { reactive, ref } from "vue";
import { VaInput, VaButton, VaModal } from 'vuestic-ui';
import axios from "axios";

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
const auth = ref(false)

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

function validateUsername1() {
    if (modal.login == false) {
        if (modal.username1.length < 6) {
            modal.err1 = 'username too short'
        } else {
            axios.post("/api/user/exist", {
                username: modal.username1,
            }).then(response => {
                if (response.data) {
                    modal.err1 = 'username already exists'
                } else {
                    modal.err1 = ''
                }
            }).catch(_ => {
            })
        }
    }
}

function validatePassword2() {
    if (modal.login == false) {
        if (modal.password2.length < 8) {
            modal.err2 = 'password too short'
        } else {
            modal.err2 = ''
        }
    } else {
        modal.err2 = ''
    }
}

function validatePassword3() {
    if (modal.password2 != modal.password3) {
        modal.err3 = 'password does not match'
    } else {
        modal.err3 = ''
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
            localStorage.setItem('token', response.data.token);
            localStorage.setItem('username', response.data.username);
            auth.value = true;
            modal.wait = false;
            hide()
        }).catch(_ => {
            modal.err2 = "wrong password";
            auth.value = false;
            modal.wait = false;
        })
    } else {
        axios.post("/api/user/register", {
            username: modal.username1,
            password: modal.password2,
        }).then(_ => {
            modal.wait = false;
            modal.login = true;
        }).catch(_ => {
            modal.err2 = "please try again";
            modal.wait = false;
        })
    }
}

function username(): string {
    return localStorage.getItem('username');
}

function logout() {
    localStorage.removeItem('token')
    localStorage.removeItem('username')
    auth.value = false;
}

function rotateToken() {
    let token = localStorage.getItem('token');
    if (token != null) {
        axios.post("/api/user/rotate", {
            token: token
        }).then(response => {
            localStorage.setItem('token', response.data.token);
            localStorage.setItem('username', response.data.username);
            auth.value = true;
        }).catch(_ => {
            logout();
        })
    } else {
        auth.value = false;
    }
}
rotateToken();
setInterval(rotateToken, 1000 * 60);
</script>

<template>
    <template v-if="!auth">
        <VaButton icon="ms-login" @click="showModal()" class="login-button">
            Login</VaButton>
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
                <VaButton background-opacity="0" color="textPrimary">
                    Settings
                </VaButton>
                <VaButton background-opacity="0" color="textPrimary"
                          @click="logout()">
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
                     :error-messages="modal.err1" @input="validateUsername1()"
                     class="w-4/5 flex-grow-0" />
            <VaInput v-model="modal.password2" label="Password" name="Password"
                     type="password" immediate-validation
                     :error="modal.err2 != ''" :error-messages="modal.err2"
                     @input="validatePassword2()"
                     class="w-4/5 flex-grow-0 mt-2" />
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
                         :error-messages="modal.err3"
                         @input="validatePassword3()"
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
