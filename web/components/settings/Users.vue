<script setup lang="ts">
import { ref, reactive } from "vue"
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
    modalForm.username = username
    if (username == "") {
        modal.update = false
        modalForm.group = "Viewer"
    } else {
        modal.update = true
        let user = users.value.filter(x => x.username == username)[0]
        modalForm.group = user.group
    }
}
function upsert() {
    if (modalForm.username.length > 3 && modalForm.password.length > 7) {
        axios.post("/api/auth/upsert", {
            token: localStorage.getItem('token'),
            username: modalForm.username,
            password: modalForm.password,
            group: modalForm.group,
        }).then(_ => {
            fetch()
        })
        clean()
    } else {
        modal.upsert = true
    }
}
// remove
function removeModal(username: string) {
    modalForm.username = username
    modal.delete = true
}
function remove() {
    axios.post("/api/auth/delete", {
        token: localStorage.getItem('token'),
        username: modalForm.username,
    }).then(_ => {
        fetch()
    })
    clean()
}
// clean
function clean() {
    modalForm.username = ""
    modalForm.password = ""
    modalForm.group = "Viewer"
}
function logout() {
    localStorage.removeItem('token');
    location.reload();
}

const users = ref<{ username: string, group: string }[]>([])
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
const modalForm = reactive({
    username: "",
    password: "",
    group: "Viewer",
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
                            <VaButton preset="plain" icon="delete" class="ml-3" @click="removeModal(value)" />
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
            <VaInput v-model="modalForm.username" label="Username" name="Username" :readonly="modal.update"
                :rules="[(i) => i.length > 3 || `username too short`]" class="w-3/5 flex-grow-0" />
            <VaInput v-model="modalForm.password" label="Password" type="password" name="Password"
                :rules="[(i) => i.length > 7 || `password too short`]" class="w-3/5 flex-grow-0" />
            <VaSelect v-model="modalForm.group" :options="['Viewer', 'Editor']" class="w-3/5 flex-grow-0" />
        </div>
    </VaModal>
    <VaModal v-model="modal.delete" ok-text="Apply" @ok="remove()" @cancel="clean()">
        <div class="h-full flex flex-col items-center justify-center">
            <div class="flex-grow-0">
                Are you sure to delete user <span class="font-bold">{{ modalForm.username }}</span>?
            </div>
        </div>
    </VaModal>
</template>

<style scoped></style>
