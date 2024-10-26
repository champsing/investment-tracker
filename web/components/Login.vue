<script setup lang="ts">
import { reactive } from "vue";
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
            axios.post("/api/user/username", {
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
            localStorage.setItem('token', response.data);
            modal.wait = false;
            hide()
        }).catch(_ => {
            modal.err2 = "wrong password";
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
</script>

<template>
    <VaButton @click="showModal()">Login</VaButton>
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

<style scoped></style>
