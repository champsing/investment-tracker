<script setup lang="ts">
import { getUsername } from "@/composables/user";
import UpdateUsername from "./users/UpdateUsername.vue";
import UpdatePassword from "./users/UpdatePassword.vue";
const authorize = defineModel<boolean>({ required: true })
</script>

<template>
    <VaCard stripe stripe-color="success">
        <VaCardTitle>User Setting</VaCardTitle>
        <VaCardContent class="flex flex-col">
            <div class="flex items-center justify-between">
                <div class="flex-grow-0">username: <span class="font-bold">{{
                    getUsername()
                        }}</span>
                </div>
                <UpdateUsername v-model:model-value="authorize" />
            </div>
            <div class="flex items-center justify-between mt-2">
                <div class="flex-grow-0">password: ●●●●●●●●</div>
                <UpdatePassword v-model:model-value="authorize" />
            </div>
        </VaCardContent>
    </VaCard>
</template>

<style scoped></style>
