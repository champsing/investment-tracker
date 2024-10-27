<script setup lang="ts">
import { reactive } from "vue"
import axios from "axios";
import { VaButton } from 'vuestic-ui';
import { logout, validUsername, getUsername } from "@/composables/user";
const authorize = defineModel<boolean>({ required: true })

const form = reactive({
    show: false,
    wait: false,
    username: '',
    error: '',
})

function input() {
    validUsername(form.username, (e) => form.error = e)
}

function showModal() {
    form.show = true;
    form.username = getUsername();
    form.error = '';
}

function beforeOk(hide: () => void) {
    if (form.error != '' || form.wait) {
        return;
    }

    form.wait = true
    axios.post('/api/user/update', {
        token: localStorage.getItem('token'),
        username: form.username,
    }).then(_ => {
        hide();
        logout();
        authorize.value = false;
    }).catch(_ => {
        form.error = 'please try again';
    }).finally(() => form.wait = false)
}
</script>

<template>
    <div class="flex-grow-0">
        <VaButton icon="ms-edit" background-opacity="0" color="textPrimary"
                  @click="showModal()" class="flex-grow-0" />
        <VaModal v-model="form.show" ok-text="Save" size="auto"
                 :before-ok="beforeOk">
            <div class="w-80 flex flex-col items-center">
                <VaInput v-model="form.username" label="New Username"
                         name="New Username" immediate-validation
                         :error="form.error != ''" :error-messages="form.error"
                         @input="input()" class="w-4/5 flex-grow-0" />
            </div>
        </VaModal>
    </div>
</template>

<style scoped></style>
