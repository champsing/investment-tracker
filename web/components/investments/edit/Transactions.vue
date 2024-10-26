<script setup lang="ts">
import { reactive, ref } from "vue"
import { VaCard, VaButton, VaDataTable } from 'vuestic-ui';
import { getUsername } from "@/composables/user";

let edit_mode = ref(true)

let data = [
    {
        date: "2020-01-01",
        account: "ABCXYZ",
        action: "Buy",
        content: "VT@100.00CAD",
        count: "10",
        fee: "0.5CAD",
        comment: "bla bla"
    },
    {
        date: "2020-01-02",
        account: "ABCXYZ",
        action: "Sell",
        content: "VT@100.00CAD",
        count: "10",
        fee: "0.5CAD",
        comment: "bla bla"
    },
    {
        date: "2020-01-03",
        account: "ABCXYZ",
        action: "Deposit",
        content: "CAD",
        count: "100",
        fee: "0.5CAD",
        comment: "bla bla"
    },
    {
        date: "2020-01-01",
        account: "ABCXYZ",
        action: "Withdrawal",
        content: "CAD",
        count: "100",
        fee: "0.5CAD",
        comment: "bla bla"
    }
]

function upsert() {
    // if (upsertForm.username.length > 3 && upsertForm.password.length > 7) {
    //     axios.post("/api/auth/upsert", {
    //         token: localStorage.getItem('token'),
    //         username: upsertForm.username,
    //         password: upsertForm.password,
    //         group: upsertForm.group,
    //     }).then(_ => {
    //         fetch()
    //     })
    //     clean()
    // } else {
    //     modal.upsert = true
    // }
}
function clean() {

}

const modal = reactive({
    upsert: false,
    update: false,
    delete: false,
})
const upsertForm = reactive({
    id: "00000000-0000-0000-0000-000000000000",
    date: new Date(Date.now()),
    account: "",
    action: {
        type: "",
        value: ["", 0.0],
        stock: ["", 0.0],
        cash: ["", 0.0],
    },
    fee: ["", 0.0],
    comment: "",
})
const actionOptions = [
    "Buy",
    "Sell",
    "Deposit",
    "Withdrawal",
]
</script>

<template>
    <VaCard>
        <VaCardTitle>
            Transactions
            <template v-if="getUsername() == 'Editor'">
                <VaButton preset="plain" icon="edit" size="small" @click="edit_mode = !edit_mode" class="ml-3" />
            </template>
        </VaCardTitle>
        <VaCardBlock vertical>
            <VaButton v-if="edit_mode" class="w-full" @click="modal.upsert = true">
                Add Transactions
            </VaButton>
            <VaDataTable :items="data" />
        </VaCardBlock>
    </VaCard>
    <VaModal v-model="modal.upsert" ok-text="Apply" @ok="upsert()" @cancel="clean()">
        <div class="h-full flex flex-col items-center justify-evenly">
            <VaSelect v-model="upsertForm.action.type" label="Action" :options="actionOptions" searchable
                highlight-matched-text class="flex-grow-0 w-4/5" />
            <VaDateInput v-model="upsertForm.date" class="flex-grow-0 w-4/5" />
            <VaTextarea v-model="upsertForm.comment" class="flex-grow-0 w-4/5" />
            <!-- <VaInput v-model="upsertForm.username" label="Username" name="Username" :readonly="modal.update"
                :rules="[(i) => i.length > 3 || `username too short`]" class="w-3/5 flex-grow-0" />
            <VaInput v-model="upsertForm.password" label="Password" type="password" name="Password"
                :rules="[(i) => i.length > 7 || `password too short`]" class="w-3/5 flex-grow-0" />
            <VaSelect v-model="upsertForm.group" :options="['Viewer', 'Editor']" class="w-3/5 flex-grow-0" /> -->
        </div>
    </VaModal>
</template>

<style scoped></style>
