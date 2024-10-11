<script setup lang="ts">
import axios from "axios";
import { reactive, ref } from "vue";
import { VaButton, VaDivider } from "vuestic-ui";

interface Account { id: string, alias: string, tag: string }

// fetch data
function fetch() {
    axios.post("/api/investment/account/query", {
        token: localStorage.getItem('token')
    }).then(response => {
        console.log(response.data)
        accounts.value = response.data
    })
}

function upsertModal(id: string) {
    modal.upsert = true
    modalForm.id = id
    if (id == "") {
        modal.update = false
        modalForm.tag = "NRA"
    } else {
        modal.update = true
        let account = accounts.value.filter(x => x.id == id)[0]
        modalForm.tag = account.tag
    }
}
function upsert() {
    if (modalForm.id.length > 0) {
        if (modalForm.alias.length == 0) {
            modalForm.alias = modalForm.id
        }

        axios.post("/api/investment/account/upsert", {
            token: localStorage.getItem('token'),
            account: {
                id: modalForm.id,
                alias: modalForm.alias,
                tag: modalForm.tag,
            }
        }).then(_ => {
            fetch()
        })
        clean()
    } else {
        modal.upsert = true
    }
}
function removeModal(id: string) {
    modalForm.id = id
    modal.delete = true
}
// function remove() {
//     axios.post("/api/investment/account/delete", {
//         token: localStorage.getItem('token'),
//         id: modalForm.id,
//     }).then(_ => {
//         fetch()
//     })
//     clean()
// }
function clean() {
    modalForm.id = ""
    modalForm.alias = ""
    modalForm.tag = "NRA"
}

const accounts = ref<Account[]>([])
const modal = reactive({
    upsert: false,
    update: false,
    delete: false,
})
const modalForm = reactive({
    id: "",
    alias: "",
    tag: "",
})
const tagOptions = [
    "NRA",
    "TFSA",
    "RRSP",
    "FHSA",
]

fetch()
</script>

<template>
    <div class="flex">
        <div></div>
        <div></div>
        <div></div>
        <VaButton>Close</VaButton>
    </div>
    <VaDivider />
    <div class="flex flex-col">
        <VaButton @click="upsertModal('')">Add</VaButton>
        <VaAccordion>
            <VaCollapse v-for="(account, i) in accounts" :key="i">
                <template #header="{ value, attrs, iconAttrs }">
                    <div v-bind="attrs"
                        class="w-full flex items-center border-solid border-b-2 border-[var(--va-background-border)] p-2">
                        <VaIcon name="va-arrow-down" size="large" v-bind="iconAttrs"
                            :class="value ? 'flex-grow-0' : 'rotate-[-90deg] flex-grow-0'" />
                        <VaChip color="success" class="w-16 flex-grow-0">
                            {{ account.tag }}
                        </VaChip>
                        <div class="ml-3 text-xl flex-grow-0">
                            {{ account.id == account.alias ? account.id : account.alias + "(" + account.id + ")" }}
                        </div>

                        <VaSpacer />

                        <VaButton preset="plain" size="large" icon="edit" @click="upsertModal(account.id)"
                            class="flex-grow-0" />
                        <VaButton preset="plain" size="large" icon="delete" @click="removeModal(account.id)"
                            class="ml-3 flex-grow-0" />
                    </div>
                </template>
            </VaCollapse>
        </VaAccordion>
    </div>
    <VaModal v-model="modal.upsert" ok-text="Apply" @ok="upsert()" @cancel="clean()">
        <div class="h-full flex flex-col items-center justify-evenly">
            <VaInput v-model="modalForm.id" label="Account ID" name="Account ID" :readonly="modal.update"
                :rules="[(i) => i.length > 3 || `invalid account ID`]" class="w-3/5 flex-grow-0" />
            <VaInput v-model="modalForm.alias" label="Alias Name" name="Alias Name" class="w-3/5 flex-grow-0" />
            <VaSelect v-model="modalForm.tag" label="Account Type" :options="tagOptions" searchable
                highlight-matched-text class="flex-grow-0 w-3/5" />
        </div>
    </VaModal>
</template>

<style scoped></style>
