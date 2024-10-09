<script setup lang="ts">
import { ref, reactive, Ref } from "vue"
import axios from "axios";
import { VaDataTable, VaButton } from 'vuestic-ui';
import { userGroup } from '@/composables/auth';

// fetch data
function fetch() {
    axios.post("/api/auth/all_users", {
        token: localStorage.getItem('token')
    }).then(response => {
        console.log(response.data)
        users.value = response.data
    })
}

// upsert
function upsertModal(username: string) {
    modal.upsert = true
    upsertForm.username = username
    if (username == "") {
        modal.update = false
        upsertForm.group = "Viewer"
    } else {
        modal.update = true
        let user = users.value.filter(x => x.username == username)[0]
        upsertForm.group = user.group
    }
}
function upsert() {
    if (upsertForm.username.length > 3 && upsertForm.password.length > 7) {
        axios.post("/api/auth/upsert", {
            token: localStorage.getItem('token'),
            username: upsertForm.username,
            password: upsertForm.password,
            group: upsertForm.group,
        }).then(_ => {
            fetch()
        })
        clean()
    } else {
        modal.upsert = true
    }
}
function deleteModal(username: string) {
    deleteForm.username = username
    modal.delete = true
}
function remove() {
    axios.post("/api/auth/delete", {
        token: localStorage.getItem('token'),
        username: deleteForm.username,
    }).then(_ => {
        fetch()
    })
    clean()
}
function clean() {
    upsertForm.username = ""
    upsertForm.password = ""
    upsertForm.group = "Viewer"

    deleteForm.username = ""
}
function logout() {
    localStorage.removeItem('token');
    location.reload();
}

const users: Ref<{ username: string, group: string }[]> = ref([])
const columns = [
    { key: "username" },
    { key: "group", label: "permission" },
    { key: "username", name: "actions", label: "actions", width: 80 },
]
const modal = reactive({
    upsert: false,
    update: false,
    delete: false,
})
const upsertForm = reactive({
    username: "",
    password: "",
    group: "Viewer",
})
const deleteForm = reactive({
    username: ""
})

fetch()
</script>

<template>
    <VaCard>
        <VaCardTitle>Users</VaCardTitle>
        <template v-if="userGroup() == 'Editor'">
            <VaCardBlock horizontal>
                <VaCardBlock class="flex-auto flex items-center justify-evenly">
                    <VaButton class="flex-grow-0 w-24" @click="upsertModal('')">Add Users</VaButton>
                    <VaButton class="flex-grow-0 w-24" @click="logout()">&nbsp;&nbsp;Logout&nbsp;</VaButton>
                </VaCardBlock>
                <VaCardBlock class="flex-auto">
                    <VaDataTable :items="users" :columns="columns" height="160px" sticky-header>
                        <template #cell(actions)="{ value }">
                            <VaButton preset="plain" icon="edit" @click="upsertModal(value)" />
                            <VaButton preset="plain" icon="delete" class="ml-3" @click="deleteModal(value)" />
                        </template>
                    </VaDataTable>
                </VaCardBlock>
            </VaCardBlock>
        </template>
        <template v-else>
            <VaCardContent>
                <VaButton class="flex-grow-0 w-24" @click="logout()">&nbsp;&nbsp;Logout&nbsp;</VaButton>
            </VaCardContent>
        </template>
    </VaCard>
    <VaModal v-model="modal.upsert" ok-text="Apply" @ok="upsert()" @cancel="clean()">
        <div class="h-full flex flex-col items-center justify-evenly">
            <VaInput v-model="upsertForm.username" label="Username" name="Username" :readonly="modal.update"
                :rules="[(i) => i.length > 3 || `username too short`]" class="w-3/5 flex-grow-0" />
            <VaInput v-model="upsertForm.password" label="Password" type="password" name="Password"
                :rules="[(i) => i.length > 7 || `password too short`]" class="w-3/5 flex-grow-0" />
            <VaSelect v-model="upsertForm.group" :options="['Viewer', 'Editor']" class="w-3/5 flex-grow-0" />
        </div>
    </VaModal>
    <VaModal v-model="modal.delete" ok-text="Apply" @ok="remove()" @cancel="clean()">
        <div class="h-full flex flex-col items-center justify-center">
            <div class="flex-grow-0">
                Are you sure to delete user <span class="font-bold">{{ deleteForm.username }}</span>?
            </div>
        </div>
    </VaModal>
</template>

<style scoped></style>
