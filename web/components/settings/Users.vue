<script setup lang="ts">
import { ref, reactive } from "vue"
import axios from "axios";
import { VaDataTable, VaButton } from 'vuestic-ui';

const users = ref([])

function fetch() {
    axios.post("/api/auth/all_users", {
        token: localStorage.getItem('token')
    }).then(response => {
        console.log(response.data)
        users.value = response.data
    })
}
function show(username: string) {
    modal.show = true
    form.username = username
    if (username == "") {
        form.group = "Viewer"
    } else {
        // TODO: based on `users`
    }
}
function cancel() {
    form.username = ""
    form.password = ""
    form.group = "Viewer"
}
function submit() {
    axios.put("/api/auth/insert", {
        token: localStorage.getItem('token'),
        username: form.username,
        password: form.password,
        group: form.group,
    }).then(_ => {
        fetch()
    })
    cancel()
}
let modal = reactive({
    show: false,
})
let form = reactive({
    username: "",
    password: "",
    group: "",
})

fetch()

function logout() {
    localStorage.removeItem('token');
    location.reload();
}
</script>

<template>
    <div class="flex">
        <div class="flex-grow-1 flex items-center justify-evenly">
            <VaButton class="flex-grow-0 w-24" @click="show('')">Add Users</VaButton>
            <VaButton class="flex-grow-0 w-24" @click="logout()">&nbsp;&nbsp;Logout&nbsp;</VaButton>
        </div>
        <div class="flex-grow-1">
            <VaDataTable :items="users">
            </VaDataTable>
        </div>
    </div>
    <VaModal v-model="modal.show" ok-text="Apply" @ok="submit()" @cancel="cancel()">
        <div class="h-full flex flex-col items-center justify-evenly">
            <VaInput v-model="form.username" label="Username" name="Username"
                :rules="[(i) => i.length > 3 || `username too short`]" class="w-3/5 flex-grow-0" />
            <VaInput v-model="form.password" label="Password" type="password" name="Password"
                :rules="[(i) => i.length > 7 || `password too short`]" class="w-3/5 flex-grow-0" />
            <VaSelect v-model="form.group" :options="['Viewer', 'Editor']" class="w-3/5 flex-grow-0" />
        </div>
    </VaModal>
</template>

<style scoped></style>
