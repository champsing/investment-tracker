<script setup lang="ts">
import { ref } from "vue"
import { useColors } from "vuestic-ui";
import Login from './components/Login.vue';
import axios from "axios";

const { applyPreset, colors } = useColors();
applyPreset('dark')

const auth = ref(false);
setInterval(() => {
    let token = localStorage.getItem('token')
    if (token != null) {
        axios.post("/api/auth/refresh", {
            token: token
        }).then(response => {
            localStorage.setItem('token', response.data)
            auth.value = true
        }).catch(_ => {
            auth.value = false
        })
    } else {
        auth.value = false
    }
}, 1000 * 60 * 10);

function updateToken(token: string) {
    localStorage.setItem('token', token)
    auth.value = true
}
</script>

<template>
    <div class="background" :style="{ backgroundColor: colors.backgroundPrimary }">
        <template v-if="auth">
            <div class="text-white text-lg"> Success </div>
        </template>
        <template v-else>
            <Login @token="(value) => updateToken(value)" />
        </template>
    </div>
</template>

<style scoped>
.background {
    height: 100%;
    width: 100%;
    min-height: 100vh;
    min-width: 100vw;
}
</style>
